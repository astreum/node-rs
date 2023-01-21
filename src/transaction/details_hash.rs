use fides::hash::blake_3;
use fides::merkle_tree;
use super::Transaction;

impl Transaction {

    pub fn update_details_hash(&mut self) {

        self.details_hash = self.details_hash()
        
    }

    pub fn details_hash(&self) -> [u8; 32] {

        let chain_bytes: Vec<u8> = (&self.chain).into();

        let counter_bytes: Vec<u8> = (&self.counter).into();

        let value_bytes: Vec<u8> = (&self.value).into();

        merkle_tree::root(
            blake_3,
            &[
                &chain_bytes,
                &counter_bytes,
                &self.data,
                &self.recipient.0,
                &self.sender.0,
                &value_bytes
            ]
        )

    }

}
