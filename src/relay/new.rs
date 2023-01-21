use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use crate::relay::Relay;
use fides::x25519;
use rand::Rng;
use super::Route;
use super::{Envelope, Ping};

impl Relay {
    
    pub fn new(
        incoming_port: &u16,
        validator: &bool
    ) -> Result<Relay, Box<dyn Error>> {

        let seeders_file = File::open("./seeders.txt")?;

        let mut seeders = Vec::new();
        
        for seeder in BufReader::new(seeders_file).lines() {

            let seeder = seeder?;
            
            let socket: SocketAddr = seeder.parse()?;

            seeders.push(socket)

        }

        // let incoming_port: u16 = if bootstrap {
        //     55555
        // } else {
        //     rand::thread_rng().gen_range(49152..65535)
        // };

        let incoming_address = format!("127.0.0.1:{}", incoming_port);

        let incoming_socket = UdpSocket::bind(incoming_address)?;

        let outgoing_port: u16 = rand::thread_rng().gen_range(49152..65535);

        let outgoing_address = format!("127.0.0.1:{}", outgoing_port);

        let outgoing_socket = UdpSocket::bind(outgoing_address)?;

        let private_key = x25519::secret_key();

        let public_key = x25519::public_key(&private_key);

        let mut outgoing_queue = vec![];

        let seeding_ping = Ping {
            incoming_port: incoming_port.clone(),
            public_key,
            validator: validator.clone(),
        };

        let seeding_envelope = Envelope::new((&seeding_ping).into(), true);
        
        for seeder in &seeders {

            let _ = outgoing_queue.push(((&seeding_envelope).into(), *seeder));

        }

        Ok(Relay {
            incoming_port: incoming_port.clone(),
            incoming_queue: Arc::new(Mutex::new(Vec::new())),
            incoming_socket: Arc::new(Mutex::new(incoming_socket)),
            outgoing_queue: Arc::new(Mutex::new(outgoing_queue)),
            outgoing_socket: Arc::new(Mutex::new(outgoing_socket)),
            private_key,
            public_key,
            peers: Arc::new(Mutex::new(HashMap::new())),
            consensus_route: Arc::new(Mutex::new(Route::new())),
            seeders,
            validator: validator.clone(),
            peer_route: Arc::new(Mutex::new(Route::new()))
        })
    }

}