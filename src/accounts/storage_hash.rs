use crate::accounts::Account;
use fides::{merkle_root, hash};

impl Account {

    pub fn storage_hash(&self) -> [u8;32] {
        merkle_root(
            self.storage
                .iter()
                .map(|x| {
                    let joined = [hash(x.0), hash(x.1)].concat();
                    hash(&joined[..])
                })
                .collect()
        )
    }

}