
use crate::{merkle_tree_hash, NOVA_ADDRESS, NOVA_SLOTS_STORE_ID, NOVA_STAKE_STORE_ID};
use crate::account::{Account, hashing};
use crate::block::Block;
use crate::nova::slots;
use crate::transaction::apply_many;
use opis::Int;
use std::collections::HashMap;
use std::error::Error;
use std::convert::TryInto;

pub fn run(
    mut accounts: HashMap<[u8; 32], Account>,
    latest_block: Block,
    previous_block: Block
) -> Result<HashMap<[u8; 32], Account>, Box<dyn Error>> {

    let mut nova_account = accounts.get(&NOVA_ADDRESS).unwrap().clone();

    let slots_in_epoch: Int = Int::from_decimal("201600");

    let mut nova_slots_store = nova_account.storage.get(&NOVA_SLOTS_STORE_ID).unwrap().clone();
                    
    if nova_slots_store.is_empty() {

        let nova_stake_store = nova_account.storage.get(&NOVA_STAKE_STORE_ID).unwrap();

        let total_stake = nova_stake_store.iter().fold(Int::zero(), |acc, x| acc + Int::from_bytes(&x.1.to_vec()));

        for (key, value) in nova_stake_store {

            let x_slots = &(&Int::from_bytes(&value.to_vec()) / &total_stake) * &slots_in_epoch;

            nova_slots_store.insert(*key, x_slots.to_ext_bytes(32).try_into().unwrap());

        }

    }

    let time_diff = latest_block.time - previous_block.time;
    
    let slots_missed: u64 = if time_diff > 3 {
        (time_diff - 1) / 3
    } else {
        0
    };
    
    let (current_validator, new_nova_slots_store) = &slots::current_validator(&nova_slots_store, slots_missed, latest_block.hash);
    
    if current_validator == &latest_block.validator {

        let current_solar_price: Int = if previous_block.solar_used > 750000000 {
                            
            previous_block.solar_price + Int::one()
            
            } else if previous_block.solar_used < 250000000 {
                
                if previous_block.solar_price == Int::one() {
                    
                    previous_block.solar_price
                
                } else {
                    
                    previous_block.solar_price - Int::one()
                
                }

            } else {
                
                previous_block.solar_price
            
        };

        let number_of_transactions = latest_block.transactions.len();
        
        let (updated, _, receipts) = apply_many::run(accounts.clone(), latest_block.transactions, &current_solar_price);

        if receipts.len() == number_of_transactions {
            
            let transactions_hash: [u8; 32] = merkle_tree_hash(latest_block.transactions
                .iter()
                .map(| x | x.hash )
                .collect()
            );

            let receipts_hash: [u8; 32] = merkle_tree_hash(receipts
                .iter()
                .map(| x | x.clone().hash() )
                .collect()
            );

            let solar_used: u32 = receipts
                .iter()
                .fold(0, | acc, x | acc + x.solar_used );

            let mut validator = accounts.get(&latest_block.validator).unwrap().clone();

            validator.balance += Int::from_decimal("1000000000000000000000000");

            accounts.insert(latest_block.validator, validator);
        
            nova_account.storage.insert(NOVA_SLOTS_STORE_ID, new_nova_slots_store.clone());

            nova_account.storage_hash = nova_account.storage_hash();

            nova_account.hash = nova_account.hash();

            accounts.insert(NOVA_ADDRESS, nova_account);

            let checks: Vec<bool> = vec![
                hashing::accounts(&accounts) == latest_block.accounts_hash,
                solar_used == latest_block.solar_used,
                receipts_hash == latest_block.receipts_hash,
                transactions_hash == latest_block.transactions_hash
            ];

            if checks.iter().any(|&x| x == false) {
                println!("State and Block do not match!");
                Err("State and Block do not match!")?
            } else {
                Ok(updated)
            }
        } else {
            println!("Some transactions could not be applied!");
            Err("Some transactions could not be applied!")?
        }

    } else {
        println!("Validator not selected!");
        Err("Validator not selected!")?
    }

}