use std::{thread, time::{Instant, SystemTime}, sync::Arc, net::SocketAddr};
use super::{Relay, Ping, Envelope};

impl Relay {

    pub fn liveness(&self) {

        let peers_clone = Arc::clone(&self.peers);

        let outgoing_queue_clone = Arc::clone(&self.outgoing_queue);

        let consensus_route_clone = Arc::clone(&self.consensus_route);

        let peer_route_clone = Arc::clone(&self.peer_route); 

        let seeders = self.seeders.clone();

        let liveness_ping = Ping {
            incoming_port: self.incoming_port,
            public_key: self.public_key,
            validator: self.validator,
        };

        thread::spawn(move || {
            
            let mut now = Instant::now();
            
            loop {

                if now.elapsed().as_secs() > 30 {

                    let liveness_envelope = Envelope::new((&liveness_ping).into(), true);

                    match peers_clone.lock() {
                        
                        Ok(peers) => {

                            match outgoing_queue_clone.lock() {
                                
                                Ok(mut outgoing_queue) => {

                                    if peers.is_empty() {

                                        for seeder in &seeders {
        
                                            let _ = outgoing_queue.push((
                                                (&liveness_envelope).into(),
                                                *seeder
                                            ));
                                
                                        }

                                    } else {

                                        let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

                                        for (peer_address, peer) in peers.iter() {
            
                                            if (t - peer.timestamp) > 330 {

                                                match peers_clone.lock() {

                                                    Ok(mut peers) => {
                                                        
                                                        peers.remove(peer_address);
                                                    
                                                    },

                                                    Err(_) => (),

                                                }

                                                match consensus_route_clone.lock() {

                                                    Ok(mut consensus_route) => {

                                                        consensus_route.remove_peer(peer_address)

                                                    },

                                                    Err(_) => (),

                                                }

                                                match peer_route_clone.lock() {

                                                    Ok(mut peer_route) => {

                                                        peer_route.remove_peer(peer_address)

                                                    },

                                                    Err(_) => (),

                                                }
            
                                            }
            
                                            if (t - peer.timestamp) > 300 {
            
                                                outgoing_queue.push((
                                                    (&liveness_envelope).into(),
                                                    SocketAddr::new(*peer_address, peer.incoming_port)
                                                ))
            
                                            }
            
                                        }

                                    }
                                    
                                },

                                Err(_) => ()
                            }


                        },

                        Err(_) => ()
                    }

                    now = Instant::now();

                }

            }

        });

    }

}
