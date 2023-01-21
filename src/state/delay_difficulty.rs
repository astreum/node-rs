use std::error::Error;

use neutrondb::Store;
use opis::Integer;

use crate::block::Block;

use super::State;

impl State {

    pub fn delay_difficulty(&self, blocks_store: &Store<Integer, Block>) -> Result<u64, Box<dyn Error>> {

        let int_100: Integer = (&100_u8).into();

        let result = if &self.latest_block.number > &int_100 {

            let past_block_number = &self.latest_block.number - &int_100;

            let past_block = blocks_store.get(&past_block_number)?;

            let block_time_diff = self.latest_block.time - past_block.time;

            self.latest_block.delay_difficulty * (300 / block_time_diff)

        } else {

            1_u64

        };

        Ok(result)

    }
    
}