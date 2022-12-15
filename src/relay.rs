use std::error::Error;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

use crate::chain::Chain;

mod envelope;
mod incoming;
mod listen;
mod liveness;
mod message;
mod messages;
mod new;
mod outgoing;
mod topic;

#[derive(Clone, Debug)]
pub struct Relay {
    bootstrap: bool,
    chain: Chain,
    incoming_queue: Arc<Mutex<Vec<(Vec<u8>, IpAddr)>>>,
    incoming_socket: Arc<Mutex<UdpSocket>>,
    incoming_port: u16,
    outgoing_queue: Arc<Mutex<Vec<(Vec<u8>, SocketAddr)>>>,
    outgoing_socket: Arc<Mutex<UdpSocket>>,
    private_key: [u8; 32],
    public_key: [u8; 32],
    seeders: Vec<SocketAddr>,
    validator: bool,
    peer_route: Arc<Mutex<HashMap<String, HashMap<u8, SocketAddr>>>>,
    consensus_route: Arc<Mutex<HashMap<String, HashMap<u8, SocketAddr>>>>,
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
    BlockResponse,
    RouteRequest,
    RouteResponse,
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

pub struct Route(HashMap<String, HashMap<u8, IpAddr>>);

impl Route {

    pub fn add_peer() {}

    pub fn remove_peer() {}
    
}
