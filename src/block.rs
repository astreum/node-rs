use crate::transaction::Transaction;
use crate::merkle_tree_hash;
use fides::{ed25519, hash};

use std::error::Error;

use std::convert::TryInto;

pub struct Block {
    pub accounts_hash: [u8; 32],
    pub chain: u8,
    pub number: [u8; 32],
    pub previous_block_hash: [u8; 32],
    pub receipts_hash: [u8; 32],
    pub signature: [u8; 64],
    pub solar_limit: [u8; 32],
    pub solar_price: [u8; 32],
    pub solar_used: [u8; 32],
    pub time: [u8; 32],
    pub transactions_hash: [u8; 32],
    pub transactions: Vec<Transaction>,
    pub validator: [u8; 32]
}

impl Block {

    pub fn genesis(chain: u8) -> Self {
        Block {
            accounts_hash: [0_u8; 32],
            chain: chain,
            number: [0_u8; 32],
            previous_block_hash: [0_u8; 32],
            receipts_hash: [0_u8; 32],
            signature: [0_u8; 64],
            solar_limit: [0_u8; 32],
            solar_price: [0_u8; 32],
            solar_used: [0_u8; 32],
            time: [0_u8; 32],
            transactions_hash: [0_u8; 32],
            transactions: Vec::new(),
            validator: [0_u8; 32]
        }
    }

    pub fn body_hash(self) -> [u8; 32] {
        merkle_tree_hash(vec![
            hash(&self.accounts_hash.to_vec()),
            hash(&vec![self.chain]),
            hash(&self.number.to_vec()),
            hash(&self.previous_block_hash.to_vec()),
            hash(&self.receipts_hash.to_vec()),
            hash(&self.solar_limit.to_vec()),
            hash(&self.solar_price.to_vec()),
            hash(&self.solar_used.to_vec()),
            hash(&self.time.to_vec()),
            hash(&self.transactions_hash.to_vec()),
            hash(&self.validator.to_vec())
        ])
    }

    pub fn hash(self) -> [u8; 32] {
        merkle_tree_hash(vec![
            hash(&self.body_hash().to_vec()),
            hash(&self.signature.to_vec())
        ])
    }

    pub fn from_bytes(input: &Vec<u8>) -> Result<Block, Box<dyn Error>> {
        
        if input.len() < 417 {

            Err("Unsupported block format!")?
            
        } else {

            let block = Block {
                accounts_hash: input[..32].try_into().unwrap(),
                chain: input[32],
                number: input[33..65].try_into().unwrap(),
                previous_block_hash: input[65..97].try_into().unwrap(),
                receipts_hash: input[97..129].try_into().unwrap(),
                signature: input[129..193].try_into().unwrap(),
                solar_limit: input[193..225].try_into().unwrap(),
                solar_price: input[225..257].try_into().unwrap(),
                solar_used: input[257..289].try_into().unwrap(),
                time: input[289..321].try_into().unwrap(),
                transactions_hash: input[321..385].try_into().unwrap(),
                transactions: Vec::new(),
                validator: input[385..].try_into().unwrap()
            };

            if input.len() - 417 == 0 {

                if block.transactions_hash == [0_u8; 32] {
                    
                    Ok(block)
                
                } else {
                    
                    Err("Invalid transactions hash!")?
                
                }

            } else if (input.len() - 417) % 193 == 0 {

                let mut valid_txs: bool = true;

                let tx_num = (input.len() - 417) / 193;

                let txs_bytes: Vec<Vec<u8>> = Vec::new();

                for i in 0..tx_num {

                    let start = 417 + (i * 193);

                    let stop = start + 193;

                    let tx_bytes = input[start..stop].to_vec();

                    let tx: Transaction = Transaction::from_bytes(&tx_bytes)?;

                    if tx.verify() {

                        block.transactions.push(tx)
                    
                    } else {
                        
                        valid_txs = false;
                        
                        break
                    
                    }

                }

                if valid_txs {

                    if block.verify() {
                        
                        Ok(block)

                    } else {

                        Err("Invalid block signature!")?

                    }
                
                } else {
                    
                    Err("Invalid transactions!")?
                
                }

            } else {
                
                Err("Invalid transactions format!")?
            
            }

        }

    }

    pub fn to_bytes(self) -> Vec<u8> {
        
        let mut res = [
            vec![self.chain],
            self.accounts_hash.to_vec(),
            self.previous_block_hash.to_vec(),
            self.receipts_hash.to_vec(),
            self.signature.to_vec(),
            self.solar_limit.to_vec(),
            self.solar_price.to_vec(),
            self.solar_used.to_vec(),
            self.time.to_vec(),
            self.transactions_hash.to_vec(),
        ].concat();

        for tx in self.transactions {
            res = [res, tx.to_bytes()].concat()
        }

        res

    }

    pub fn verify(self) -> bool {
        ed25519::verify(&self.body_hash(), &self.validator, &self.signature)
    }
    
}