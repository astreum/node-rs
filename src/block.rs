mod block_hash;
mod details_hash;
mod from;
mod into;
mod new;
mod sign;
mod verify;
use opis::Integer;
use crate::address::Address;
use crate::chain::Chain;
use crate::transaction::Transaction;

#[derive(Clone, Debug)]
pub struct Block {
    pub accounts_hash: [u8; 32],
    pub block_hash: [u8; 32],
    pub chain: Chain,
    pub data: Vec<u8>,
    pub delay_difficulty: u64,
    pub delay_output: Vec<u8>,
    pub details_hash: [u8; 32],
    pub number: Integer,
    pub previous_block_hash: [u8; 32],
    pub receipts_hash: [u8; 32],
    pub signature: [u8; 64],
    pub solar_used: u64,
    pub time: u64,
    pub transactions: Vec<Transaction>,
    pub transactions_hash: [u8; 32],
    pub validator: Address,
}

