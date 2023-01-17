use crate::chain::Chain;
use crate::transaction::Transaction;
use opis::Integer;
use std::error::Error;
use super::Block;

impl TryFrom<&[u8]> for Block {

    type Error = Box<dyn Error>;

    fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {
        
        let block_details = astro_format::decode(value)?;

        if block_details.len() == 16 {

            let mut txs = Vec::new();

            let txs_bytes = astro_format::decode(block_details[13])?;

            for tx_bytes in txs_bytes {

                let tx = Transaction::try_from(tx_bytes)?;

                txs.push(tx)

            };

            let block = Block {
                accounts_hash: block_details[0].try_into().unwrap_or(Err("Accounts hash error!")?),
                block_hash: block_details[1].try_into().unwrap_or(Err("Block hash error!")?),
                chain: Chain::try_from(block_details[2]).unwrap_or(Err("Chain error!")?),
                data: block_details[3].to_vec(),
                delay_difficulty: u64::from_be_bytes(block_details[4].try_into().unwrap_or(Err("Block time error!")?)),
                delay_output: block_details[5].to_vec(),
                details_hash: block_details[6].try_into().unwrap_or(Err("Details hash error!")?),
                number: Integer::try_from(block_details[7]).unwrap_or(Err("Number error!")?),
                previous_block_hash: block_details[8].try_into().unwrap_or(Err("Previous block hash error!")?),
                receipts_hash: block_details[9].try_into().unwrap_or(Err("Receipts hash error!")?),
                signature: block_details[10].try_into().unwrap_or(Err("Signature error!")?),
                solar_used: (&Integer::try_from(block_details[11]).unwrap_or(Err("Block number error!")?)).into(),
                time: u64::from_be_bytes(block_details[12].try_into().unwrap_or(Err("Block time error!")?)),
                transactions: txs,
                transactions_hash: block_details[14].clone().try_into().unwrap_or(Err("Validator error!")?),
                validator: block_details[15].clone().try_into().unwrap_or(Err("Validator error!")?),
            };

            match block.verify() {

                Ok(verification) => {

                    match verification {

                        true => Ok(block),

                        false => Err("Block verification error!")?,

                    }

                },

                Err(_) => Err("Block verification error!")?,
                
            }

        } else {

            Err("Block details error!")?

        }

    }
    
}

impl TryFrom<Vec<u8>> for Block {
    type Error = Box<dyn Error>;

    fn try_from(value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        Block::try_from(&value[..])
    }
}