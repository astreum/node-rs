use fides::hash::blake_3;
use fides::merkle_tree::root;
use opis::Integer;
use super::Block;

impl Block {

    pub fn update_details_hash(&mut self) {

        self.details_hash = self.details_hash()
        
    }

    pub fn details_hash(&self) -> [u8; 32] {

        let accounts_hash_bytes: Vec<u8> = self.accounts_hash.into();

        let chain_bytes: Vec<u8> = (&self.chain).into();

        let number_bytes: Vec<u8> = (&self.number).into();

        let previous_block_hash_bytes: Vec<u8> = self.previous_block_hash.into();

        let receipts_hash_bytes: Vec<u8> = self.receipts_hash.into();

        let signature_bytes: Vec<u8> = self.signature.to_vec();

        let solar_used_bytes: Vec<u8> = Integer::from(&self.solar_used).into();

        let time_bytes: Vec<u8> = Integer::from(&self.time).into();

        let transactions_hash_bytes: Vec<u8> = self.transactions_hash.into();

        let validator_bytes: Vec<u8> = self.validator.into();

        root(
            blake_3,
            &[
                &accounts_hash_bytes,
                &chain_bytes,
                &self.data,
                &self.delay_output,
                &number_bytes,
                &previous_block_hash_bytes,
                &receipts_hash_bytes,
                &signature_bytes,
                &solar_used_bytes,
                &time_bytes,
                &transactions_hash_bytes,
                &validator_bytes,
            ]
        )
        
    }

}
