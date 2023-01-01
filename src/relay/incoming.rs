use fides::decrypt;
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
                                                        shared_key: x25519::shared_key(&private_key, &ping.public_key),
                                                        timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
                                                    };

                                                    match peers_clone.lock() {
                                                        Ok(mut peers) => {

                                                            match peers.get(&incoming_address) {

                                                                None => {

                                                                    let response_ping = Ping {
                                                                        incoming_port: incoming_port,
                                                                        public_key: public_key,
                                                                        validator: validator,
                                                                    };

                                                                    let response_envelope = Envelope::new((&response_ping).into(), true);

                                                                    match outgoing_queue_clone.lock() {
                                                                        Ok(mut outgoing_queue) => {
                                                                            outgoing_queue.push((
                                                                                (&response_envelope).into(),
                                                                                SocketAddr::new(incoming_address, ping.incoming_port)
                                                                            ))
                                                                        },

                                                                        Err(_) => ()
                                                                    }

                                                                },

                                                                _ => ()

                                                            }

                                                            peers.insert(incoming_address, peer);

                                                        },

                                                        Err(_) => ()
                                                    }

                                                },

                                                Err(_) => ()
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
                                                                                                Topic::RouteRequest => todo!(),
                                                                                                Topic::RouteResponse => todo!(),
                                                                                                Topic::Transaction => {sender.send((message, incoming_address));},
                                                                                                Topic::Block => {sender.send((message, incoming_address));},
                                                                                                Topic::BlockRequest => {sender.send((message, incoming_address));},
                                                                                            }

                                                                                        },

                                                                                        Err(_) => todo!(),
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

                                                Err(_) => todo!(),
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
