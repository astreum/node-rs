
use crate::merkle_tree_hash;
use fides::{ed25519, hash};
use opis::Int;
use std::error::Error;
use std::convert::TryInto;

pub mod apply_many;
pub mod apply_one;

#[derive(Clone, Debug)]
pub struct Transaction {
    pub chain: u8,
    pub counter: Int,
    pub hash: [u8; 32],
    pub recipient: [u8; 32],
    pub sender: [u8; 32],
    pub signature: [u8; 64],
    pub solar_limit: u32,
    pub solar_price: Int,
    pub value: Int
}

impl Transaction {

    pub fn body_hash(&self) -> [u8; 32] {
        merkle_tree_hash(vec![
            hash(&vec![self.chain]),
            hash(&self.counter.to_ext_bytes(32)),
            hash(&self.recipient.to_vec()),
            hash(&self.sender.to_vec()),
            hash(&self.solar_limit.to_be_bytes().to_vec()),
            hash(&self.solar_price.to_ext_bytes(32)),
            hash(&self.value.to_ext_bytes(32))
        ])
    }
    
    pub fn hash(&self) -> [u8; 32] {
        merkle_tree_hash(vec![
            hash(&self.body_hash().to_vec()),
            hash(&self.signature.to_vec())
        ])
    }

    pub fn from_bytes(input: &Vec<u8>) -> Result<Self, Box<dyn Error>> {

        if input.len() != 230 {
            Err("Transaction must be 230 bytes!")?
        } else {

            let mut tx = Transaction {
                chain: input[1],
                counter: Int::from_bytes(&input[2..34].to_vec()),
                hash: [0_u8; 32],
                recipient: input[34..66].try_into().unwrap(),
                sender: input[66..98].try_into().unwrap(),
                signature: input[98..162].try_into().unwrap(),
                solar_limit: u32::from_be_bytes(input[162..166].try_into().unwrap()),
                solar_price: Int::from_bytes(&input[166..198].to_vec()),
                value: Int::from_bytes(&input[198..].to_vec()),
            };

            tx.hash = tx.hash();

            Ok(tx)

        }

    }

    pub fn to_bytes(&self) -> Vec<u8> {
        [
            vec![1],
            vec![self.chain],
            self.counter.to_ext_bytes(32),
            self.recipient.to_vec(),
            self.sender.to_vec(),
            self.signature.to_vec(),
            self.solar_limit.to_be_bytes().to_vec(),
            self.solar_price.to_ext_bytes(32),
            self.value.to_ext_bytes(32)
        ].concat()

    }

    pub fn verify(&self) -> bool {
        ed25519::verify(&self.body_hash(), &self.sender, &self.signature)
    }

}

#[derive(Copy, Clone, Debug)]
pub struct CancelTransaction {
    pub transaction_hash: [u8; 32],
    pub signature: [u8; 64]
}

impl CancelTransaction {
    
    pub fn from_bytes(input: &Vec<u8>) -> Result<Self, Box<dyn Error>> {

        if input.len() != 97 {
            Err("Cancel transaction must be 97 bytes!")?
        } else {
            Ok(CancelTransaction {
                transaction_hash: input[1..33].try_into().unwrap(),
                signature: input[33..].try_into().unwrap()
            })
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        [
            vec![1],
            self.transaction_hash.to_vec(),
            self.signature.to_vec()
        ].concat()
    }

    pub fn verify(&self, tx: &Transaction) -> bool {
        ed25519::verify(&self.transaction_hash, &tx.sender, &self.signature)
    }
}

#[derive(Clone, Debug)]
pub struct Receipt {
    pub solar_used: u32,
    pub status: Status
}

impl Receipt {
    pub fn hash(&self) -> [u8; 32] {
        match self.status {
            Status::Accepted => merkle_tree_hash(vec![hash(&self.solar_used.to_be_bytes().to_vec()), hash(&vec![1_u8])]),
            Status::BalanceError => merkle_tree_hash(vec![hash(&self.solar_used.to_be_bytes().to_vec()), hash(&vec![2_u8])]),
            Status::SolarError => merkle_tree_hash(vec![hash(&self.solar_used.to_be_bytes().to_vec()), hash(&vec![3_u8])])
        }
    }
}

#[derive(Clone, Debug)]
pub enum Status {
    Accepted,
    BalanceError,
    SolarError
}