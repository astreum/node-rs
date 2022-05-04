use std::{collections::BTreeMap, time::SystemTime};
use opis::Int;

use crate::block::Block;

pub fn select(latest_block: &Block, stakes: BTreeMap<[u8;32], [u8;32]>) -> [u8;32] {

    let mut current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
        
    let time_diff = &Int::from_bytes(&current_time.to_be_bytes()) - &latest_block.time;

    let three = Int::from_decimal("3");

    let misses = if time_diff > three {
        (time_diff - Int::one()) / three
    } else {
        Int::zero()
    };

    [0_u8;32]
}

pub fn solar(price: &Int, used: &Int) -> Int {

    if used > &Int::from_decimal("750000000") {
                            
        price + &Int::one()
    
    } else if used < &Int::from_decimal("250000000") {
        
        if price == &Int::one() {
            
            price.clone()
        
        } else {
            
            price - &Int::one()
        
        }

    } else {
        
        price.clone()
    
    }
    
}