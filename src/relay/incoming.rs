use fides::decrypt;
use fides::encrypt;
use fides::x25519;

use crate::relay::Message;
use crate::relay::Relay;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::SystemTime;

use super::Envelope;
use super::Peer;
use super::Ping;
use super::Topic;

impl Relay {
	
	pub fn incoming(&self, sender: Sender<(Message, IpAddr)>) {

		let private_key = self.private_key;

		let incoming_port = self.incoming_port;

		let public_key = self.public_key;

		let validator = self.validator;

		let peers_clone = Arc::clone(&self.peers);

		let incoming_queue_clone = Arc::clone(&self.incoming_queue);

		let outgoing_queue_clone = Arc::clone(&self.outgoing_queue);

		let consensus_route_clone = Arc::clone(&self.consensus_route);

		let peer_route_clone = Arc::clone(&self.peer_route);

		thread::spawn(move || {

			loop {

				match incoming_queue_clone.lock() {

					Ok(mut incoming_queue) => {

						match incoming_queue.pop() {

							Some((incoming_message, incoming_address)) => {

								match Envelope::try_from(&incoming_message[..]) {

									Ok(envelope) => {

										if envelope.ping {

											match Ping::try_from(&envelope.message[..]) {
													
												Ok(ping) => {

													let peer = Peer {
														incoming_port: ping.incoming_port,
														shared_key: x25519::shared_key(
															&private_key,
															&ping.public_key
														),
														timestamp: SystemTime::now()
															.duration_since(SystemTime::UNIX_EPOCH)
															.unwrap()
															.as_secs()
													};

													match peers_clone.lock() {

														Ok(mut peers) => {

															match peers.get(&incoming_address) {

																None => {

																	let response_ping = Ping {
																		incoming_port,
																		public_key,
																		validator,
																	};

																	let response_envelope = Envelope::new(
																		(&response_ping).into(),
																		true
																	);

                                  match outgoing_queue_clone.lock() {

																		Ok(mut outgoing_queue) => {

																			outgoing_queue.push((
																				(&response_envelope).into(),
																				SocketAddr::new(incoming_address, ping.incoming_port)
																			))

																		},

																		Err(_) => ()

																	}

                                  let mut addresses: Vec<Vec<u8>> = Vec::new();

                                  match consensus_route_clone.lock() {

                                    Ok(consensus_route) => {

                                      for (_,bucket) in &consensus_route.0 {

                                        for address in bucket {

                                          addresses.push(address.to_string().into_bytes());

                                        }
                                          
                                      }

                                    },

                                    Err(_) => (),

                                  }

                                  let encoded_addresses = astro_format::encode(
                                    &(addresses
                                      .iter()
                                      .map(|x| x.as_slice())
                                      .collect::<Vec<_>>()
                                    )
                                  );

                                  let routing_message = Message::new(
                                    &encoded_addresses,
                                    &Topic::Routing
                                  );

                                  let routing_message_bytes: Vec<u8> = (&routing_message).into();

                                  match encrypt(&private_key, &routing_message_bytes) {

                                    Ok(encrypted) => {

                                      let routing_envelope = Envelope::new(
                                        astro_format::encode(&[
                                          &encrypted.0,
                                          &encrypted.1
                                        ]),
                                        false
                                      );

                                      match outgoing_queue_clone.lock() {

                                        Ok(mut outgoing_queue) => {
    
                                          outgoing_queue.push((
                                            (&routing_envelope).into(),
                                            SocketAddr::new(incoming_address, ping.incoming_port)
                                          ))
    
                                        },
    
                                        Err(_) => ()
    
                                      }

                                    },

                                    Err(_) => (),

                                }
																},

																_ => ()

															}

															if ping.validator {

																match consensus_route_clone.lock() {

																	Ok(mut consensus_route) => {

																		let _ = consensus_route.add_peer(
																			&incoming_address,
																			&ping.public_key
																				.iter()
																				.fold(
																					String::new(),
																					|acc, x| {
																						format!("{}{:08b}", acc, x)
																					}
																				)
																		);

																	},

																	Err(_) => (),

																}
															
															} else {

																match peer_route_clone.lock() {

																	Ok(mut peer_route) => {

																		let _ = peer_route.add_peer(
																			&incoming_address,
																			&ping.public_key
																				.iter()
																				.fold(
																					String::new(),
																					|acc, x| {
																						format!("{}{:08b}", acc, x)
																					}
																				)
																		);

																	},

																	Err(_) => (),

																}

															}

															peers.insert(incoming_address, peer);

														},

														Err(_) => ()
													}

												},

												Err(_) => (),

											}

										} else {

                      match astro_format::decode(&envelope.message) {

                        Ok(encrypted_fields) => {

                          if encrypted_fields.len() == 2 {

                            if encrypted_fields[1].len() == 12 {

                              match peers_clone.lock() {
                                
                                Ok(peers) => {

                                  match peers.get(&incoming_address) {

                                      Some(peer) => {

                                        match decrypt(
                                          &peer.shared_key,
                                          encrypted_fields[1].try_into().unwrap(),
                                          encrypted_fields[0]
                                        ) {

                                          Ok(decrypted) => {

                                            match Message::try_from(&decrypted[..]) {
                                              
                                              Ok(message) => {

                                                match message.topic {

                                                  Topic::Routing => {

                                                    // ping consensus peers

                                                  },
                                                  
                                                  Topic::Transaction => {
                                                      let _ = sender.send((message, incoming_address));
                                                  },

                                                  Topic::Block => {
                                                      let _ = sender.send((message, incoming_address));
                                                  },
                                                  
                                                  Topic::BlockRequest => {
                                                      let _ = sender.send((message, incoming_address));
                                                  },
                                                
                                                }

                                              },

                                              Err(_) => (),

                                            }

                                          },

                                          Err(_) => ()

                                        }

                                      },

                                      None => ()

                                  }

                                },

                                Err(_) => ()

                              }

                            }

                          }
                        
                        },

                        Err(_) => (),

                      }

										}

									},

									Err(_) => ()
										
								}

							},

							None => ()

						}

					},

					Err(_) => break

				}

			}

		});

  }

}
