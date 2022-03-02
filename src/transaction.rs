use std::error::Error;
use std::convert::TryInto;
use fides::{ ed25519, hash };
use crate::merkle_tree_hash;

pub struct CancelTransaction {
    pub transaction_hash: [u8; 32],
    pub signature: [u8; 64]
}

impl CancelTransaction {
    
    pub fn from_bytes(input: &Vec<u8>) -> Result<Self, Box<dyn Error>> {

        if input.len() == 257 {

            Err("Unsupported cancel transaction format!")?

        } else {

            Ok(CancelTransaction {
                transaction_hash: input[..32].try_into().unwrap(),
                signature: input[32..].try_into().unwrap()
            })

        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        [
            self.transaction_hash.to_vec(),
            self.signature.to_vec()
        ].concat()
    }

    pub fn verify(self, tx: Transaction) -> bool {
        ed25519::verify(&self.transaction_hash, &tx.sender, &self.signature)
    }
}

pub struct Transaction {
    pub chain: u8,
    pub counter: [u8; 32],
    pub recipient: [u8; 32],
    pub sender: [u8; 32],
    pub signature: [u8; 64],
    pub solar_limit: [u8; 32],
    pub solar_price: [u8; 32],
    pub value: [u8; 32]
}

impl Transaction {

    pub fn body_hash(self) -> [u8; 32] {
        merkle_tree_hash(vec![
            hash(&vec![self.chain]),
            hash(&self.counter.to_vec()),
            hash(&self.recipient.to_vec()),
            hash(&self.sender.to_vec()),
            hash(&self.solar_limit.to_vec()),
            hash(&self.solar_price.to_vec()),
            hash(&self.value.to_vec())
        ])
    }
    
    pub fn hash(self) -> [u8; 32] {
        merkle_tree_hash(vec![
            hash(&self.body_hash().to_vec()),
            hash(&self.signature.to_vec())
        ])
    }

    pub fn from_bytes(input: &Vec<u8>) -> Result<Self, Box<dyn Error>> {

        if input.len() == 257 {

            Err("Unsupported transaction format!")?

        } else {

            Ok(Transaction {
                chain: input[0],
                counter: input[1..33].try_into().unwrap(),
                recipient: input[33..65].try_into().unwrap(),
                sender: input[65..97].try_into().unwrap(),
                signature: input[97..161].try_into().unwrap(),
                solar_limit: input[161..193].try_into().unwrap(),
                solar_price: input[193..225].try_into().unwrap(),
                value: input[225..].try_into().unwrap()
            })

        }

    }

    pub fn to_bytes(self) -> Vec<u8> {
        
        [
            vec![self.chain],
            self.counter.to_vec(),
            self.recipient.to_vec(),
            self.sender.to_vec(),
            self.signature.to_vec(),
            self.solar_limit.to_vec(),
            self.solar_price.to_vec(),
            self.value.to_vec()
        ].concat()

    }

    pub fn verify(self) -> bool {
        ed25519::verify(&self.body_hash(), &self.sender, &self.signature)
    }

}