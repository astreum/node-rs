use crate::accounts::Account;
use fides::{merkle_root, hash};

impl Account {
    
    pub fn hash(&self) -> [u8;32] {
        merkle_root(vec![
            hash(&self.balance.to_bytes()),
            hash(&self.counter.to_bytes()),
            self.storage_hash()
        ])
    }

}