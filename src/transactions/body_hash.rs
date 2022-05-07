use crate::transactions::Transaction;
use fides::{merkle_root, hash};

impl Transaction {

    pub fn body_hash(&self) -> [u8; 32] {
        merkle_root(vec![
            hash(&self.chain.to_bytes()),
            hash(&self.counter.to_bytes()),
            hash(&self.recipient),
            hash(&self.sender),
            hash(&self.solar_limit.to_bytes()),
            hash(&self.solar_price.to_bytes()),
            hash(&self.value.to_bytes())
        ])
    }

}
