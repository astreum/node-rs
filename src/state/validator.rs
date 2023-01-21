use std::{error::Error, collections::HashMap};

use opis::Integer;

use crate::{address::Address, account::Account, CONSENSUS_ADDRESS};

use super::State;

impl State {

    pub fn validator(&mut self, target_time: &u64) -> Result<(Address, HashMap<Address, Account>), Box<dyn Error>> {
        
        let changed_accounts: HashMap<Address, Account> = HashMap::new();

        let consensus_account = self.accounts_store.get(&CONSENSUS_ADDRESS)?;

        let time_diff = target_time - &self.latest_block.time;

        let misses = if time_diff > 3 {

            (time_diff - 1) / 3

        } else {

            0

        };

        let mut random: Integer = (*self.latest_block.delay_output).into();

        if misses > 0 {
            
            random = random.lfsr(misses.try_into()?);
                
        };

        let total_stake = consensus_account.storage
            .iter()
            .fold(Integer::zero(), |acc, x| acc + x.1[..].into());
        
        random = random.modulo(&total_stake)?;

        let mut validator: Option<Address> = None;

        for x in consensus_account.storage {

            random = random - x.1[..].into();

            if random <= Integer::zero() {

                validator = Some(Address(x.0[..].try_into()?));
                
                break;
            
            }

        }

        match validator {
            Some(res) => Ok((res, changed_accounts)),
            None => Err("")?,
        }

    }
    
}