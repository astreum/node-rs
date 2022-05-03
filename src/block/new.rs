use crate::{block::Block, Chain};
use opis::Int;

impl Block {

    pub fn new(chain: &Chain) -> Self {
        Block {
            accounts_hash: [0_u8; 32],
            chain: match chain {
                Chain::Main => Int::one(),
                Chain::Test => Int::from_decimal("2")
            },
            number: Int::zero(),
            previous_block_hash: [0_u8; 32],
            receipts_hash: [0_u8; 32],
            signature: [0_u8; 64],
            solar_price: Int::one(),
            solar_used: Int::zero(),
            time: Int::zero(),
            transactions: Vec::new(),
            validator: [0_u8; 32]
        }
    }

}
