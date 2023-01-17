use fides::hash::blake_3;
use fides::merkle_tree::root;
use opis::Integer;
use super::Block;

impl Block {

    pub fn update_details_hash(&mut self) {

        self.details_hash = self.details_hash()
        
    }

    pub fn details_hash(&self) -> [u8; 32] {

        let chain_bytes: Vec<u8> = (&self.chain).into();

        let delay_difficulty_bytes: Vec<u8> = Integer::from(&self.delay_difficulty).into();

        let number_bytes: Vec<u8> = (&self.number).into();

        let solar_used_bytes: Vec<u8> = Integer::from(&self.solar_used).into();

        let time_bytes: Vec<u8> = Integer::from(&self.time).into();

        root(
            blake_3,
            &[
                &self.accounts_hash,
                &chain_bytes,
                &self.data,
                &delay_difficulty_bytes,
                &self.delay_output,
                &number_bytes,
                &self.previous_block_hash,
                &self.receipts_hash,
                &self.signature,
                &solar_used_bytes,
                &time_bytes,
                &self.transactions_hash,
                &self.validator.0,
            ]
        )
        
    }

}
