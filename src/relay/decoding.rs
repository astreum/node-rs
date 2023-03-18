use astro_format::decode;
use fides::decrypt;
use fides::encrypt;
use fides::x25519;

use crate::relay::Relay;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::SystemTime;
use super::Peer;
use super::envelope::Envelope;
use super::message::Message;
use super::ping::Ping;
use super::topic::Topic;

impl Relay {
	
	pub fn decoding(&self, sender: Sender<(Message, IpAddr)>) {

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

										let plain_message_option = if envelope.encrypted {

											match peers_clone.lock() {

												Ok(peers) => {

													match peers.get(&incoming_address) {
														
														Some(peer) => {

															match astro_format::decode(&envelope.message) {

																Ok(cipher_fields) => {

																if cipher_fields.len() == 2 {

																	let cipher_nonce_res = cipher_fields[0].try_into();

																	match cipher_nonce_res {

																		Ok(cipher_nonce) => {

																		match decrypt(&peer.shared_key, cipher_nonce, cipher_fields[1]) {
									
																			Ok(decrypted) => Some(decrypted),
										
																			Err(_) => None,
										
																		}

																		},

																		Err(_) => None,

																	}

																} else {
									
																	None
									
																}

																},

																Err(_) => None,
															
															}

														},

														None => None,

													}

												},

												Err(_) => None,

											}

										} else {

											Some(envelope.message)
                      
										};

										match plain_message_option {

											Some(plain_message) => {

												match Message::try_from(&plain_message[..]) {

													Ok(message) => {

														match message.topic {

															Topic::Ping => {

																match Ping::try_from(&message.body[..]) {

																	Ok(ping) => {

																		let peer = Peer {
																			incoming_port: ping.incoming_port,
																			shared_key: x25519::shared_key(&private_key, &ping.public_key),
																			timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
																		};

																		match peers_clone.lock() {

																			Ok(mut peers) => {

																				match peers.get(&incoming_address) {

																					Some(_) => (),

																					None => {

																						let response_ping = Ping {
																							incoming_port,
																							public_key,
																							validator,
																						};

																						let response_envelope = Envelope::new(false, (&response_ping).into());

																						match outgoing_queue_clone.lock() {

																							Ok(mut outgoing_queue) => {

																								let response_bytes = (&response_envelope).into();

																								let recipient_socket = SocketAddr::new(incoming_address, ping.incoming_port);

																								outgoing_queue.push((response_bytes, recipient_socket))

																							},

																							Err(_) => ()

																						}

																					}

																				}

																				peers.insert(incoming_address, peer);

																				let peer_public_key_str = ping.public_key
																					.iter()
																					.fold(String::new(), |acc, x| format!("{}{:08b}", acc, x));

																				match peer_route_clone.lock() {

																					Ok(mut peer_route) => {

																						let _ = peer_route.add(&incoming_address, &peer_public_key_str);

																					},

																					Err(_) => (),

																				}

																				if ping.validator {
																					
																					match consensus_route_clone.lock() {

																						Ok(mut consensus_route) => {
																							
																							let _ = consensus_route.add(&incoming_address, &peer_public_key_str);
																						
																						},
																						
																						Err(_) => (),
																					
																					}
																				}

																			}

																			Err(_) => (),

																		}

																	},

																	Err(_) => (),

																}

															},

															Topic::RouteRequest => {

																match consensus_route_clone.lock() {

																	Ok(consensus_route) => {

																		let samples = consensus_route.samples();

																		let mut sample_socket_addresses_bytes = Vec::new();

																		match peers_clone.lock() {

																			Ok(peers) => {

																				for sample in samples {

																					match peers.get(&sample) {

																						Some(peer) => {

																							let sample_socket_address = SocketAddr::new(sample, peer.incoming_port);

																							let sample_socket_address_str = sample_socket_address.to_string();

																							let sample_socket_address_bytes = sample_socket_address_str.into_bytes();

																							sample_socket_addresses_bytes.push(sample_socket_address_bytes);

																						},

																						None => (),
																					}
																					
																				}

																			},

																			Err(_) => (),

																		}

																		let sample_socket_addresses_bytes_slices: Vec<&[u8]> = sample_socket_addresses_bytes
																			.iter()
																			.map(|x| x.as_slice())
																			.collect();

																		let encoded_samples = astro_format::encode(&sample_socket_addresses_bytes_slices);

																		let route_message = Message::new(&encoded_samples, &Topic::Route);

																		match peers_clone.lock() {

																			Ok(peers) => {

																				match peers.get(&incoming_address) {

																					Some(peer) => {

																						let route_message_bytes: Vec<u8> = (&route_message).into();

																						match encrypt(&peer.shared_key, &route_message_bytes) {

																							Ok((nonce, cipher)) => {

																								let encrypted_message = astro_format::encode(&[&nonce[..], &cipher[..]][..]);

																								let route_envelope = Envelope::new(true, encrypted_message);

																								match outgoing_queue_clone.lock() {

																									Ok(mut outgoing_queue) => {

																										outgoing_queue.push(((&route_envelope).into(), SocketAddr::new(incoming_address, peer.incoming_port)))

																									},

																									Err(_) => (),

																								}

																							},
																							
																							Err(_) => (),
																						
																						}
																					
																					},

																					None => (),

																				}

																			},

																			Err(_) => (),

																		}

																	},

																	Err(_) => (),

																}

																// sample consensus route

																// send samples 
															
															},

															Topic::Route => {

																match decode(&message.body) {

																	Ok(sample_bytes) => {

																		let mut sample_socket_addresses = vec![];

																		for sample in sample_bytes {

																			match String::from_utf8(sample.to_vec()) {

																				Ok(sample_socket_address_str) => {

																					match SocketAddr::from_str(&sample_socket_address_str) {

																						Ok(sample_socket_address) => sample_socket_addresses.push(sample_socket_address),

																						Err(_) => (),

																					}

																				},

																				Err(_) => (),

																			}
																			
																		}

																		let ping = Ping {
																			incoming_port,
																			public_key,
																			validator,
																		};

																		let ping_bytes: Vec<u8> = (&ping).into();

																		let ping_message = Message::new(&ping_bytes, &Topic::Ping);

																		let ping_envelope = Envelope::new(false, (&ping_message).into());

																		let ping_envelope_bytes: Vec<u8> = (&ping_envelope).into();

																		match outgoing_queue_clone.lock() {

																			Ok(mut outgoing_queue) => {

																				for sample_socket_address in sample_socket_addresses {

																					outgoing_queue.push((ping_envelope_bytes.clone(), sample_socket_address))
																				}
																			},

																			Err(_) => (),

																		}

																	},

																	Err(_) => (),

																}

															},

															Topic::BlockRequest => {

																let _ = sender.send((message, incoming_address));

															}, // forward

															Topic::Block => {

																let _ = sender.send((message, incoming_address));

															},

															Topic::Transaction => {

																let _ = sender.send((message, incoming_address));

															}

														}

													},

													Err(_) => (),

												}

											},

											None => (),

										};

									},

									Err(_) => (),

								}

							},

							None => (),

						}

					},

        			Err(_) => (),

				}

			}

		});

	}
	
}
