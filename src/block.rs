use crate::transaction::Transaction;
use crate::merkle_tree_hash;
use fides::{ed25519, hash};
use opis::Int;

use std::error::Error;

use std::convert::TryInto;

#[derive(Clone, Debug)]
pub struct Block {
    pub accounts_hash: [u8; 32],
    pub chain: u8,
    pub hash: [u8; 32],
    pub number: Int,
    pub previous_block_hash: [u8; 32],
    pub receipts_hash: [u8; 32],
    pub signature: [u8; 64],
    pub solar_price: Int,
    pub solar_used: Int,
    pub time: u64,
    pub transactions_hash: [u8; 32],
    pub transactions: Vec<Transaction>,
    pub validator: [u8; 32]
}

impl Block {

    pub fn genesis(chain: u8) -> Self {
        Block {
            accounts_hash: [0_u8; 32],
            chain: chain,
            hash: [0_u8; 32],
            number: Int::zero(),
            previous_block_hash: [0_u8; 32],
            receipts_hash: [0_u8; 32],
            signature: [0_u8; 64],
            solar_price: Int::from_decimal("1000000000000000000"),
            solar_used: Int::zero(),
            time: 1640995200,
            transactions_hash: [0_u8; 32],
            transactions: Vec::new(),
            validator: [0_u8; 32]
        }
    }

    pub fn body_hash(&self) -> [u8; 32] {
        merkle_tree_hash(vec![
            hash(&self.accounts_hash.to_vec()),
            hash(&vec![self.chain]),
            hash(&self.number.clone().to_ext_bytes(32)),
            hash(&self.previous_block_hash.to_vec()),
            hash(&self.receipts_hash.to_vec()),
            hash(&self.solar_price.clone().to_ext_bytes(32)),
            hash(&self.solar_used.clone().to_ext_bytes(32)),
            hash(&[vec![0_u8; 24], self.time.to_be_bytes().to_vec()].concat()),
            hash(&self.transactions_hash.to_vec()),
            hash(&self.validator.to_vec())
        ])
    }

    pub fn hash(&self) -> [u8; 32] {
        merkle_tree_hash(vec![
            hash(&self.clone().body_hash().to_vec()),
            hash(&self.signature.to_vec())
        ])
    }

    pub fn from_bytes(input: &Vec<u8>) -> Result<Block, Box<dyn Error>> {
        
        if input.len() < 385 {

            Err("Unsupported block format!")?
            
        } else {

            let mut block = Block {
                accounts_hash: input[..32].try_into().unwrap(),
                chain: input[32],
                hash: [0_u8; 32],
                number: Int::from_bytes(&input[33..65].to_vec()),
                previous_block_hash: input[65..97].try_into().unwrap(),
                receipts_hash: input[97..129].try_into().unwrap(),
                signature: input[129..193].try_into().unwrap(),
                solar_price: Int::from_bytes(&input[193..225].to_vec()),
                solar_used: Int::from_bytes(&input[225..257].to_vec()),
                time: u64::from_be_bytes(input[257..289][24..].try_into().unwrap()),
                transactions_hash: input[289..321].try_into().unwrap(),
                validator: input[321..].try_into().unwrap(),
                transactions: Vec::new()
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

                    if block.clone().verify() {
                        
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

    pub fn to_bytes(&self) -> Vec<u8> {
        
        let mut res = [
            vec![self.chain],
            self.accounts_hash.to_vec(),
            self.previous_block_hash.to_vec(),
            self.receipts_hash.to_vec(),
            self.signature.to_vec(),
            self.solar_price.clone().to_ext_bytes(32),
            self.solar_used.clone().to_ext_bytes(32),
            [vec![0_u8; 24], self.time.to_be_bytes().to_vec()].concat(),
            self.transactions_hash.to_vec(),
        ].concat();

        for tx in &self.transactions {
            res = [res, tx.to_bytes()].concat()
        }

        res

    }

    pub fn verify(self) -> bool {
        ed25519::verify(&self.clone().body_hash(), &self.validator, &self.signature)
    }
    
}