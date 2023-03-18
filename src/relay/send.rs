use std::{net::{SocketAddr, IpAddr}, sync::Arc, error::Error};

use fides::encrypt;

use super::{Relay, Message, envelope::Envelope};

impl Relay {

    pub fn send(&self, address: &IpAddr, message: &Message) -> Result<(), Box<dyn Error>> {
        
        let message_bytes: Vec<u8> = message.into();

        let outgoing_queue_clone = Arc::clone(&self.outgoing_queue);
        
        let peers_clone = Arc::clone(&self.peers);

        let peers_clone_lock = peers_clone.lock();

        match peers_clone_lock {

            Ok(peers) => {

                match peers.get(&address) {

                    Some(peer) => {

                        let encrypted_message = encrypt(&peer.shared_key, &message_bytes)?;

                        let encoded_encrypted_message = astro_format::encode(
                            &[
                                &encrypted_message.0,
                                &encrypted_message.1
                            ]
                        );

                        let envelope = Envelope::new(true, encoded_encrypted_message);

                        match outgoing_queue_clone.lock() {

                            Ok(mut outgoing_queue) => {

                                outgoing_queue.push((
                                    (&envelope).into(),
                                    SocketAddr::new(*address, peer.incoming_port)
                                ));

                                Ok(())

                            },

                            Err(_) => Err("Outgoing queue lock error!")?,

                        }

                        
                    },

                    None => Err("Peer get error!")?,

                }

            },

            Err(_) => Err("Peers lock error!")?,

        }

    }

}