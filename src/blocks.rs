mod apply;
mod body_hash;
mod create;
mod from_bytes;
mod hash;
mod new;
mod next_solar_price;
mod to_bytes;
mod verify;
use crate::transactions::Transaction;
use opis::Int;

#[derive(Clone, Debug)]
pub struct Block {
    pub accounts_hash: [u8; 32],
    pub chain: Int,
    pub number: Int,
    pub previous_block_hash: [u8; 32],
    pub receipts_hash: [u8; 32],
    pub signature: [u8; 64],
    pub solar_price: Int,
    pub solar_used: Int,
    pub time: Int,
    pub transactions: Vec<Transaction>,
    pub validator: [u8; 32]
}
