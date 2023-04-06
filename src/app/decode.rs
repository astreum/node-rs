// decode envelopes and react to messages

use std::{thread, sync::{Arc, Mutex}, net::{IpAddr, SocketAddr, UdpSocket}, collections::{HashMap, BTreeMap}, time::SystemTime, str::FromStr, fs::File, io::Write};

use fides::{decrypt, x25519};
use neutrondb::Store;
use opis::Integer;

use crate::{envelope::Envelope, peer::Peer, message::Message, topic::Topic, ping::Ping, route::{Route, RouteID}, block::Block, transaction::Transaction};

pub fn run(
   blocks_store_clone: Arc<Mutex<Store<Integer, Block>>>,
   consensus_route_clone: Arc<Mutex<Route>>,
   incoming_queue_clone: Arc<Mutex<Vec<(Vec<u8>, IpAddr)>>>,
   outgoing_socket: UdpSocket,
   peer_route_clone: Arc<Mutex<Route>>,
   peers_clone: Arc<Mutex<HashMap<IpAddr, Peer>>>,
   pending_transactions_clone: Arc<Mutex<BTreeMap<[u8; 32], Transaction>>>,
   ping: Ping,
   proposed_blocks_clone: Arc<Mutex<HashMap<[u8; 32], Block>>>,
   relay_key: [u8;32]
) {

   thread::spawn(move || {

      let ping_bytes: Vec<u8> = (&ping).into();

      let ping_message = Message::new(&ping_bytes, &Topic::Ping);

      let ping_message_bytes: Vec<u8> = (&ping_message).into();

      match incoming_queue_clone.lock() {

         Ok(incoming_queue) => {

            match incoming_queue.pop() {
               
               Some((envelope_buffer, sender_ip_address)) => {

                  match Envelope::try_from(&envelope_buffer[..]) {
                     
                     Ok(envelope) => {

                        let plain_message_option =
                        
                           if envelope.encrypted {
                           
                              match peers_clone.lock() {
                              
                                 Ok(peers) => {
                                 
                                    match peers.get(&sender_ip_address) {
                                 
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
                                             
                                             }
                                            
                                             Err(_) => None,
                                          
                                          }

                                       },
                                       
                                       None => None,
                                    
                                    }

                                 }
                                
                                 Err(_) => None
                              
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
                                                   shared_key: x25519::shared_key(&ping.public_address.0, &relay_key),
                                                   timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
                                                };

                                                match peers_clone.lock() {

                                                   Ok(peers) => {
         
                                                      match peers.get(&sender_ip_address) {
               
                                                         Some(_) => (),
               
                                                         None => {
               
                                                            let response_envelope = Envelope::new(false, ping_message_bytes.clone());
                                                            
                                                            let response_bytes: Vec<u8> = (&response_envelope).into();
                                                            
                                                            let recipient_socket = SocketAddr::new(sender_ip_address, ping.incoming_port);
               
                                                            let _ = outgoing_socket.send_to(&response_bytes, &recipient_socket);
               
                                                         }
               
                                                      }

                                                   },
                                                   
                                                   Err(_) => (),

                                                }

                                                match peers_clone.lock() {

                                                   Ok(peers) => {
                                                      
                                                      peers.insert(sender_ip_address, peer);

                                                   },
                                                   
                                                   Err(_) => (),

                                                }
         
                                                let peer_public_key_str = ping.public_address.0
                                                   .iter()
                                                   .fold(String::new(), |acc, x| format!("{}{:08b}", acc, x));

                                                match peer_route_clone.lock() {
                                                   
                                                   Ok(peer_route) => {

                                                      let _ = peer_route.add(&sender_ip_address, &peer_public_key_str);

                                                   },
                                                   
                                                   Err(_) => (),
                                                
                                                }

                                                if ping.validator {

                                                   match consensus_route_clone.lock() {
                                                   
                                                      Ok(consensus_route) => {

                                                         let _ = consensus_route.add(&sender_ip_address, &peer_public_key_str);
   
                                                      },
                                                      
                                                      Err(_) => (),
   
                                                   }
                                                   
                                                   
         
                                                }
         
         
                                             },
         
                                             Err(_) => (),
         
                                          }
         
                                       },
         
                                       Topic::RouteRequest => {

                                          match RouteID::try_from(&message.body[..]) {
                                             
                                             Ok(route_id) => {

                                                let samples = match route_id {
                                                   
                                                   RouteID::Peer => {

                                                      Vec::new()

                                                   },
                                                   
                                                   RouteID::Consensus => {

                                                      match consensus_route_clone.lock() {
                                                         
                                                         Ok(consensus_route) => {
                                                
                                                            consensus_route.samples()
                                                            
                                                         },
                                                        
                                                         Err(_) => Vec::new(),
                                                    
                                                      }

                                                   },
                                                
                                                };

                                                match peers_clone.lock() {
                                                   
                                                   Ok(peers) => {

                                                      let mut sample_socket_addresses_bytes = Vec::new();

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

                                                      let sample_socket_addresses_bytes_slices: Vec<&[u8]> = sample_socket_addresses_bytes
                                                         .iter()
                                                         .map(|x| x.as_slice())
                                                         .collect();
                     
                                                      let encoded_samples = astro_format::encode(&sample_socket_addresses_bytes_slices);
                     
                                                      let route_message = Message::new(&encoded_samples, &Topic::Route);

                                                      let route_message_bytes: Vec<u8> = (&route_message).into();

                                                      let route_envelope = Envelope::new(false, route_message_bytes);

                                                      let envelope_bytes: Vec<u8> = (&route_envelope).into();

                                                      match peers.get(&sender_ip_address) {

                                                         Some(peer) => {

                                                            let _ = outgoing_socket.send_to(&envelope_bytes, &SocketAddr::new(sender_ip_address, peer.incoming_port));

                                                         },

                                                         None => (),

                                                      }

                                                   },
                                                   
                                                   Err(_) => (),
                                                
                                                }

                                             },
                                            
                                             Err(_) => (),

                                          }
                                       
                                       },
         
                                       Topic::Route => {
         
                                          match astro_format::decode(&message.body) {
         
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
         
                                                let ping_envelope = Envelope::new(false, ping_message_bytes.clone());
         
                                                let ping_envelope_bytes: Vec<u8> = (&ping_envelope).into();
                                                
                                                for sample_socket_address in sample_socket_addresses {
         
                                                   let _ = outgoing_socket.send_to(&ping_envelope_bytes, &sample_socket_address);
         
                                                }
         
                                             },
         
                                             Err(_) => (),
         
                                          }
         
                                       },
         
                                       Topic::BlockRequest => {
         
                                          match blocks_store_clone.lock() {
         
                                             Ok(blocks_store) => {
         
                                                let block_number: Integer = message.body.into();
         
                                                match blocks_store.get(&block_number) {
                                                   
                                                   Ok(block) => {

                                                      match peers_clone.lock() {
                                                         
                                                         Ok(peers) => {

                                                            match peers.get(&sender_ip_address) {
                                                               
                                                               Some(peer) => {

                                                                  let block_bytes: Vec<u8> = block.into();
         
                                                                  let block_message = Message::new(&block_bytes, &Topic::Block);
                     
                                                                  let block_envelope = Envelope::new(false, (&block_message).into());
                     
                                                                  let block_envelope_bytes: Vec<u8> = (&block_envelope).into();
                     
                                                                  let peer_socket_address = SocketAddr::new(sender_ip_address, peer.incoming_port);
                     
                                                                  let _ = outgoing_socket.send_to(&block_envelope_bytes, &peer_socket_address);
                                                               
                                                               },
                                                               
                                                               None => (),
                                                            
                                                            }

                                                            
                                                         
                                                         },
                                                        
                                                         Err(_) => (),
                                                    
                                                      }
         
                                                      
         
                                                   },
                                                
                                                   Err(_) => (),
         
                                                }
         
                                             },
                                            
                                             Err(_) => (),
                                          
                                          }
         
                                       },
         
                                       Topic::Block => {
         
                                          match Block::try_from(&message.body[..]) {
         
                                             Ok(block) => {
         
                                                match proposed_blocks_clone.lock() {
                                                   
                                                   Ok(mut proposed_blocks) => {

                                                      proposed_blocks.insert(block.block_hash, block);

                                                   },
                                                   
                                                   Err(_) => (),
                                                
                                                }
                                             
                                             },
         
                                             _ => (),
         
                                          }
         
                                       },
         
                                       Topic::Transaction => {
         
                                          match Transaction::try_from(&message.body[..]) {
                                             
                                             Ok(tx) => {

                                                match pending_transactions_clone.lock() {
                                                   
                                                   Ok(pending_transactions) => {

                                                      match pending_transactions.get(&tx.transaction_hash) {
                                                   
                                                         Some(_) => (),
                                                         
                                                         None => {
               
                                                            pending_transactions.insert(tx.transaction_hash, tx);
               
                                                            // pass to other validators
               
                                                         },
                                                      }
                                                   },
                                                   
                                                   Err(_) => (),
                                                
                                                }

                                             },

                                             Err(_) => (),

                                          }
                                          
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
      
               }
               
               None => (),
      
            }
   
         },
      
         Err(_) => (),
      
      }
   
   });

}
