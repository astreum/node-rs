mod body_hash;
pub mod cancel;
mod from_bytes;
mod hash;
mod new;
mod to_bytes;
mod verify;
use opis::Int;

#[derive(Clone, Debug)]
pub struct Transaction {
    pub chain: Int,
    pub counter: Int,
    pub recipient: [u8; 32],
    pub sender: [u8; 32],
    pub signature: [u8; 64],
    pub solar_limit: Int,
    pub solar_price: Int,
    pub value: Int,
}
