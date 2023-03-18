use std::net::IpAddr;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use self::message::Message;
use self::peer::Peer;
use self::route::Route;
mod broadcast;
mod envelope;
mod decoding;
mod incoming;
mod liveness;
pub mod message;
mod new;
mod outgoing;
mod send;
pub mod topic;
mod ping;
mod route;
pub mod peer;
pub mod bucket;
mod connect;

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
    pub peer_route: Arc<Mutex<Route>>,
    pub consensus_route: Arc<Mutex<Route>>,
    peers: Arc<Mutex<HashMap<IpAddr, Peer>>>,
    pub receiver: Arc<Mutex<Receiver<(Message, IpAddr)>>>,
}
