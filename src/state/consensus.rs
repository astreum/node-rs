use std::error::Error;

use neutrondb::Store;
use opis::Integer;

use crate::{block::Block, address::Address, account::Account, CONSENSUS_ADDRESS};

pub fn selection(

    accounts_store: &Store<Address, Account>,
    latest_block: &Block,
    target_time: &u64,

) -> Result<Address, Box<dyn Error>>
    
{

    let consensus_account = accounts_store.get(&CONSENSUS_ADDRESS)?;
        
    let time_diff = target_time - &latest_block.time;

    let misses = if time_diff > 3 {

        (time_diff - 1) / 3

    } else {

        0

    };

    let mut random: Integer = (*latest_block.delay_output).into();

    if misses > 0 {
        
        random = random.lfsr(misses.try_into()?);
            
    };

    let sum = consensus_account.storage
        .iter()
        .fold(Integer::zero(), |acc, x| acc + x.1[..].into());
    
    random = random.modulo(&sum)?;

    let mut validator = Address([0_u8; 32]);

    for x in consensus_account.storage {

        random = random - x.1[..].into();

        if random <= Integer::zero() {

            validator = Address(x.0[..].try_into()?);
            
            break;
        
        }

    }

    Ok(validator)

}