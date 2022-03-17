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
    pub solar_used: u32,
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
            solar_used: 0,
            time: 1647404661,
            transactions_hash: [0_u8; 32],
            transactions: Vec::new(),
            validator: [0_u8; 32]
        }
    }

    pub fn body_hash(&self) -> [u8; 32] {
        merkle_tree_hash(vec![
            hash(&self.accounts_hash.to_vec()),
            hash(&vec![self.chain]),
            hash(&self.number.to_ext_bytes(32)),
            hash(&self.previous_block_hash.to_vec()),
            hash(&self.receipts_hash.to_vec()),
            hash(&self.solar_price.to_ext_bytes(32)),
            hash(&self.solar_used.to_be_bytes().to_vec()),
            hash(&self.time.to_be_bytes().to_vec()),
            hash(&self.transactions_hash.to_vec()),
            hash(&self.validator.to_vec())
        ])
    }

    pub fn hash(&self) -> [u8; 32] {
        merkle_tree_hash(vec![
            hash(&self.body_hash().to_vec()),
            hash(&self.signature.to_vec())
        ])
    }

    pub fn from_bytes(input: &Vec<u8>) -> Result<Block, Box<dyn Error>> {
        
        if input.len() < 302 {
            println!("Unsupported block format!");
            Err("Unsupported block format!")?
        } else {

            let mut block = Block {
                accounts_hash: input[1..33].try_into().unwrap(),
                chain: input[33],
                hash: [0_u8; 32],
                number: Int::from_bytes(&input[34..66].to_vec()),
                previous_block_hash: input[66..98].try_into().unwrap(),
                receipts_hash: input[98..130].try_into().unwrap(),
                signature: input[130..194].try_into().unwrap(),
                solar_price: Int::from_bytes(&input[194..226].to_vec()),
                solar_used: u32::from_be_bytes(input[226..230].try_into().unwrap()),
                time: u64::from_be_bytes(input[230..238].try_into().unwrap()),
                transactions_hash: input[238..270].try_into().unwrap(),
                validator: input[270..].try_into().unwrap(),
                transactions: Vec::new()
            };

            if input.len() == 302 {

                if block.transactions_hash == [0_u8; 32] {
                    if block.verify() {
                        Ok(block)
                    } else {
                        println!("Invalid block signature!");
                        Err("Invalid block signature!")?
                    }
                } else {
                    println!("Invalid transactions hash!");
                    Err("Invalid transactions hash!")?
                }

            } else if (input.len() - 302) % 230 == 0 {

                let mut valid_txs: bool = true;

                let tx_num = (input.len() - 302) / 230;

                for i in 0..tx_num {

                    let start = 302 + (i * 230);

                    let stop = start + 230;

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
                        println!("Invalid block signature!");
                        Err("Invalid block signature!")?
                    }
                } else {
                    println!("Invalid transactions!");
                    Err("Invalid transactions!")?               
                }
            } else {
                println!("Invalid transactions format!");
                Err("Invalid transactions format!")?
            }
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        
        let mut res = [
            vec![1], // version
            self.accounts_hash.to_vec(), // accounts hash
            vec![self.chain], // chain id
            self.number.to_ext_bytes(32), // number
            self.previous_block_hash.to_vec(), // previous block hash
            self.receipts_hash.to_vec(), // receipts hash
            self.signature.to_vec(), // signature
            self.solar_price.to_ext_bytes(32), // solar price
            self.solar_used.to_be_bytes().to_vec(), // solar_used
            self.time.to_be_bytes().to_vec(), // time
            self.transactions_hash.to_vec(), // transactions hash
            self.validator.to_vec() // validator
        ].concat();

        for tx in &self.transactions {
            res = [res, tx.to_bytes()].concat()
        }

        res

    }

    pub fn verify(&self) -> bool {
        ed25519::verify(&self.body_hash(), &self.validator, &self.signature)
    }
    
}