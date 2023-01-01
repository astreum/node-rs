use crate::address::Address;
use crate::chain::Chain;
use opis::Integer;
use super::Block;

impl Block {

    pub fn new(chain: &Chain) -> Self {

        Block {
            accounts_hash: [0; 32],
            block_hash: [0; 32],
            chain: chain.clone(),
            data: Vec::new(),
            delay_output: Vec::new(),
            details_hash: [0; 32],
            number: Integer::zero(),
            previous_block_hash: [0; 32],
            receipts_hash: [0; 32],
            signature: [0_u8; 64],
            solar_used: 0,
            time: 0,
            transactions: Vec::new(),
            validator: Address([0_u8; 32]),
            transactions_hash: [0; 32],
        }
        
    }

}
