#[derive(Clone, Debug)]
pub struct Peer {
    pub incoming_port: u16,
    pub shared_key: [u8; 32],
    pub timestamp: u64
}
