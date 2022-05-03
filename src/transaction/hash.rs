use crate::transaction::Transaction;
use fides::{merkle_root, hash};

impl Transaction {

    pub fn hash(&self) -> [u8; 32] {
        merkle_root(vec![
            hash(&self.body_hash()),
            hash(&self.signature)
        ])
    }

}
