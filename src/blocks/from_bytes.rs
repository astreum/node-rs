use astro_format::arrays;
use crate::blocks::Block;
use crate::transactions::Transaction;
use std::error::Error;
use opis::Int;

impl Block {

    pub fn from_bytes(arg: &Vec<u8>) -> Result<Block, Box<dyn Error>> {

        let details = arrays::decode(&arg)?;
        
        if details.len() == 11 {

            let mut txs = Vec::new();

            let txs_bytes = arrays::decode(details[9])?;

            for tx_bytes in txs_bytes {
                let tx = Transaction::from_bytes(tx_bytes)?;
                txs.push(tx)
            }
            
            let block = Block {
                accounts_hash: details[0].try_into().unwrap_or(Err("Accounts hash error!")?),
                chain: Int::from_bytes(details[1]),
                number: Int::from_bytes(details[2]),
                previous_block_hash: details[3].try_into().unwrap_or(Err("Previous block hash error!")?),
                receipts_hash: details[4].try_into().unwrap_or(Err("Receipts hash error!")?),
                signature: details[5].try_into().unwrap_or(Err("Signature error!")?),
                solar_price: Int::from_bytes(details[6]),
                solar_used: Int::from_bytes(details[7]),
                time: Int::from_bytes(details[8]),
                transactions: txs,
                validator: details[10].clone().try_into().unwrap_or(Err("Validator error!")?)
            };
            
            match block.verify() {
                true => Ok(block),
                false => Err("Verification error!")?
            }
                
        } else {
            Err("Block error!")?
        }

    }

}
