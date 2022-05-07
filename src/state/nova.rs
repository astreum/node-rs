use crate::STELAR_ADDRESS;
use astro_format::string;
use neutrondb::Store;
use opis::Int;

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
    
    STELAR_ADDRESS

}
