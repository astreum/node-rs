use std::error::Error;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
mod broadcast;
mod envelope;
mod incoming;
mod listen;
mod liveness;
mod message;
mod messages;
mod new;
mod outgoing;
mod random_validator;
mod send;
mod topic;

#[derive(Clone, Debug)]
pub struct Relay {
    incoming_queue: Arc<Mutex<Vec<(Vec<u8>, IpAddr)>>>,
    incoming_socket: Arc<Mutex<UdpSocket>>,
    incoming_port: u16,
    outgoing_queue: Arc<Mutex<Vec<(Vec<u8>, SocketAddr)>>>,
    outgoing_socket: Arc<Mutex<UdpSocket>>,
    private_key: [u8; 32],
    public_key: [u8; 32],
    seeders: Vec<SocketAddr>,
    validator: bool,
    peer_route: Arc<Mutex<Route>>,
    consensus_route: Arc<Mutex<Route>>,
    peers: Arc<Mutex<HashMap<IpAddr, Peer>>>
}

#[derive(Clone, Debug)]
pub struct Peer {
    incoming_port: u16,
    shared_key: [u8; 32],
    timestamp: u64
}

#[derive(Clone, Debug)]
pub struct Ping {
    incoming_port: u16,
    public_key: [u8; 32],
    validator: bool
}

impl Into<Vec<u8>> for &Ping {
    fn into(self) -> Vec<u8> {
        astro_format::encode(&[
            &self.incoming_port.to_be_bytes()[..],
            &self.public_key[..],
            if self.validator { &[1_u8] } else { &[0_u8] }
        ])
    }
}

impl TryFrom<&[u8]> for Ping {
    type Error = Box<dyn Error>;

    fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {
        let ping_fields = astro_format::decode(value)?;
        
        if ping_fields.len() == 3 {
            Ok(Ping{
                incoming_port: u16::from_be_bytes(ping_fields[0].try_into()?),
                public_key: ping_fields[1].try_into()?,
                validator: if ping_fields[2] == [1] { true } else { false }
            })
        } else {
            Err("Ping fields error!")?
        }
    }
}

#[derive(Clone, Debug)]
pub enum Topic {
    Block,
    BlockRequest,
    Routing,
    Transaction,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub body: Vec<u8>,
    pub topic: Topic
}

#[derive(Clone, Debug)]
pub struct Envelope {
    message: Vec<u8>,
    nonce: u64,
    ping: bool,
    time: u64
}

#[derive(Clone, Debug)]
pub struct Route(
    HashMap<
        String,
        Vec<IpAddr>
    >
);

impl Route {

    pub fn new() -> Route {
        Route(
            HashMap::new()
        )
    }

    pub fn bucket_key(&self, id: &str) -> Result<String, Box<dyn Error>> {

        if id.len() == 256 {

            let mut result = String::new();

            for i in 1..=256 {

                let bucket_key = &id[0..i];

                match self.0.get(bucket_key) {

                    Some(bucket) => {

                        if bucket.len() < 256 {

                            result = bucket_key.to_string();

                            break;

                        }

                    },

                    None => {

                        result = bucket_key.to_string();

                        break;
                    
                    },

                }
            
            }

            Ok(result)

        } else {

            Err("internal error!")?

        }

    }

    pub fn add_peer(&mut self, address: &IpAddr, id: &str) -> Result<(), Box<dyn Error>> {

        let bucket_key = self.bucket_key(id)?;

        let bucket = match self.0.get(&bucket_key) {

            Some(bucket) => {

                let mut bucket = bucket.clone();
                
                bucket.push(*address);
            
                bucket
            
            },

            None => vec![*address],

        };

        self.0.insert(bucket_key, bucket);

        Ok(())

    }

    pub fn remove_peer(&mut self, address: &IpAddr) {
        
        for (bucket_key, bucket) in &self.0 {

            let mut bucket = bucket.clone();
            
            match bucket.iter().position(|x| x == address) {

                Some(i) => {
                    
                    bucket.remove(i);
                
                    self.0.insert(bucket_key.to_string(), bucket);

                    break;
                
                },
                
                None => (),

            }
            
        }

    }

    // pub fn find_peer(&self, address: &IpAddr) -> bool {

    //     let mut result = false;

    //     for (_,bucket) in &self.0 {

    //         match bucket.contains(address) {

    //             true => {

    //                 result = true;

    //                 break;

    //             },

    //             false => (),

    //         }

    //     }

    //     result

    // }
    
}
