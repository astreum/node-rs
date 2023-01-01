use fides::hash::blake_3;
use fides::merkle_tree::root;
use super::Account;


impl Account {

    pub fn details_hash(&self) -> [u8; 32] {

        let balance: Vec<u8> = (&self.balance).into();

        let counter: Vec<u8> = (&self.counter).into();

        root(
            blake_3,
            &[
                &balance,
                &counter,
                &self.storage_hash
            ]
        )
        
    }

}
