use std::{sync::Arc, error::Error, net::SocketAddr};

use fides::encrypt;

use super::{Relay, Message, Envelope};

impl Relay {

    pub fn broadcast(&self, message: &Message) -> Result<(), Box<dyn Error>> {

        let message_bytes: Vec<u8> = message.into();

        let peers_clone = Arc::clone(&self.peers);

        let consensus_route_clone = Arc::clone(&self.consensus_route);

        let mut validator_messages: Vec<(Vec<u8>, SocketAddr)> = vec![];

        match consensus_route_clone.lock() {

            Ok(consensus_route) => {

                match peers_clone.lock() {

                    Ok(peers) => {

                        for (_,bucket) in consensus_route.iter() {

                            for (_, address) in bucket.iter() {

                                match peers.get(&address.ip()) {

                                    Some(peer) => {

                                        let encrypted_message = encrypt(&peer.shared_key, &message_bytes)?;

                                        let encoded_encrypted_message = astro_format::encode(
                                            &[
                                                &encrypted_message.0,
                                                &encrypted_message.1
                                            ]
                                        );

                                        let envelope = Envelope::new(encoded_encrypted_message, false);

                                        validator_messages.push((
                                            (&envelope).into(),
                                            SocketAddr::new(address.ip(), peer.incoming_port)
                                        ));


                                    },

                                    None => (),

                                }

                            }

                        }
  
                    },

                    Err(_) => (),

                }

            },

            Err(_) => (),

        }

        let outgoing_queue_clone = Arc::clone(&self.outgoing_queue);

        match outgoing_queue_clone.lock() {

            Ok(mut outgoing_queue) => {
                
                for validator_message in validator_messages {

                    outgoing_queue.push(validator_message)
                    
                }

            },

            Err(_) => ()

        };

        Ok(())

    }

}
