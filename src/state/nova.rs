use astro_format::string;
use neutrondb::Store;
use opis::{Int, modulo};

use crate::{blocks::Block, NOVA_ADDRESS, accounts::Account};

pub fn validator_selection(accounts_store: &Store, latest_block: &Block, target_time: &Int) -> [u8;32] {

    let nova_address_string = string::encode::bytes(&NOVA_ADDRESS);

    let nova_account_string = accounts_store.get(&nova_address_string).unwrap();

    let nova_account_bytes = string::decode::bytes(&nova_account_string).unwrap();

    let nova_account = Account::from_bytes(&nova_account_bytes).unwrap();
        
    let time_diff = target_time - &latest_block.time;

    let three = Int::from_decimal("3");

    let misses = if time_diff > three {
        (time_diff - Int::one()) / three
    } else {
        Int::zero()
    };

    let mut random = Int::from_bytes(&latest_block.hash());

    if misses > Int::zero() {
        
        let shifts = u64::from_be_bytes(misses.to_ext_bytes(8).try_into().unwrap());

        for _ in 0..shifts {
            random = random.lfsr();
        }
            
    };

    let sum = nova_account.storage.iter().fold(Int::zero(), |acc, x| acc + Int::from_bytes(x.1));
    
    random = modulo(&random, &sum);

    let mut validator = [0_u8;32];

    for x in nova_account.storage {

        random = random - Int::from_bytes(&x.1);

        if random <= Int::zero() {

            validator = x.0;
            
            break;
        
        }

    }

    validator

}
