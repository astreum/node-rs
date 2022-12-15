use std::net::SocketAddr;
use std::net::UdpSocket;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use crate::chain::Chain;
use crate::relay::Relay;
use fides::x25519;
use rand::Rng;
use super::{Envelope, Ping};

impl Relay {
    
    pub fn new(
        chain: Chain,
        bootstrap: bool,
        seeders: Vec<SocketAddr>,
        validator: bool
    ) -> Result<Relay, Box<dyn Error>> {

        let incoming_port: u16 = if bootstrap {
            55555
        } else {
            rand::thread_rng().gen_range(49152..65535)
        };

        let incoming_address = format!("127.0.0.1:{}", incoming_port);

        let incoming_socket = UdpSocket::bind(incoming_address)?;

        let outgoing_port: u16 = rand::thread_rng().gen_range(49152..65535);

        let outgoing_address = format!("127.0.0.1:{}", outgoing_port);

        let outgoing_socket = UdpSocket::bind(outgoing_address)?;

        let private_key = x25519::private_key();

        let public_key = x25519::public_key(&private_key);

        let mut outgoing_queue = vec![];

        let seeding_ping = Ping {
            incoming_port,
            public_key,
            validator,
        };

        let seeding_envelope = Envelope::new((&seeding_ping).into(), true);
        
        if !bootstrap {

            for seeder in &seeders {

                let _ = outgoing_queue.push(((&seeding_envelope).into(), *seeder));

            }

        };

        Ok(Relay {
            bootstrap,
            chain,
            incoming_port,
            incoming_queue: Arc::new(Mutex::new(Vec::new())),
            incoming_socket: Arc::new(Mutex::new(incoming_socket)),
            outgoing_queue: Arc::new(Mutex::new(outgoing_queue)),
            outgoing_socket: Arc::new(Mutex::new(outgoing_socket)),
            private_key,
            public_key,
            peers: Arc::new(Mutex::new(HashMap::new())),
            consensus_route: Arc::new(Mutex::new(HashMap::new())),
            seeders,
            validator,
            peer_route: Arc::new(Mutex::new(HashMap::new()))
        })
    }

}