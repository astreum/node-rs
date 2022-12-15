
#[derive(Clone, Debug)]
pub struct Block {
    pub accounts_hash: [u8; 32],
    pub chain: Int,
    pub number: Int,
    pub previous_block_hash: [u8; 32],
    pub receipts_hash: [u8; 32],
    pub signature: [u8; 64],
    pub solar_used: Int,
    pub time: Int,
    pub transactions: Vec<Transaction>,
    pub validator: [u8; 32],
}