mod details_hash;
mod from;
mod into;
mod sign;
use crate::address::Address;
use crate::chain::Chain;
use opis::Integer;

#[derive(Clone, Debug)]
pub struct Transaction {
    pub chain: Chain,
    pub counter: Integer,
    pub data: Vec<u8>,
    pub details_hash: [u8; 32],
    pub recipient: Address,
    pub sender: Address,
    pub signature: [u8; 64],
    pub value: Integer,
}
