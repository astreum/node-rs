use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use crate::relay::Relay;
use fides::x25519;
use rand::Rng;
use super::envelope::Envelope;
use super::message::Message;
use super::ping::Ping;
use super::route::Route;

impl Relay {
    
    pub fn new(incoming_port: &u16, validator: &bool) -> Result<Relay, Box<dyn Error>> {

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

        let seeding_envelope = Envelope::new(
            false,
            (&seeding_ping).into()
        );
        
        for seeder in &seeders {

            let _ = outgoing_queue.push(((&seeding_envelope).into(), *seeder));

        }

        let (sender, receiver): (Sender<(Message, IpAddr)>, Receiver<(Message, IpAddr)>) = channel();        

        let relay = Relay {
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
            peer_route: Arc::new(Mutex::new(Route::new())),
            receiver: Arc::new(Mutex::new(receiver)),
        };

        relay.incoming();

        relay.decoding(sender);

        relay.outgoing();

        relay.liveness();

        Ok(relay)

    }

}