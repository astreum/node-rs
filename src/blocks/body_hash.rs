use crate::blocks::Block;
use fides::{merkle_root, hash};

impl Block {

    pub fn body_hash(&self) -> [u8; 32] {

        let transactions_hash = merkle_root(self.transactions.iter().map(|x| x.hash()).collect());

        merkle_root(vec![
            hash(&self.accounts_hash),
            hash(&self.chain.to_bytes()),
            hash(&self.number.to_bytes()),
            hash(&self.previous_block_hash),
            hash(&self.receipts_hash),
            hash(&self.solar_price.to_bytes()),
            hash(&self.solar_used.to_bytes()),
            hash(&self.time.to_bytes()),
            transactions_hash,
            hash(&self.validator)
        ])
    }

}
