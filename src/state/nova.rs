use crate::STELAR_ADDRESS;
use std::{collections::BTreeMap, time::SystemTime};
use astro_format::string;
use opis::Int;

use crate::{block::Block, NOVA_ADDRESS, accounts::Accounts, account::Account};

pub fn validator_selection(accounts: &Accounts, latest_block: &Block, target_time: &Int) -> [u8;32] {

    let nova_address_string = string::encode::bytes(&NOVA_ADDRESS);

    let nova_account_string = accounts.store.get(&nova_address_string).unwrap();

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

pub fn next_solar_price(latest_block: &Block) -> Int {

    if latest_block.solar_used > Int::from_decimal("900000000000") {
                            
        &latest_block.solar_price + &Int::one()
    
    } else if latest_block.solar_used < Int::from_decimal("100000000000") {
        
        if latest_block.solar_price == Int::one() {
            
            latest_block.solar_price.clone()
        
        } else {
            
            &latest_block.solar_price - &Int::one()
        
        }

    } else {
        
        latest_block.solar_price.clone()
    
    }
    
}