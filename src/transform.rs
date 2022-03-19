
use crate::account::{Account, accounts_hash};
use crate::block::Block;
use crate::merkle_tree_hash;
use crate::NOVA_ADDRESS;
use crate::NOVA_SLOTS_STORE_ID;
use crate::NOVA_STAKE_STORE_ID;
use crate::slots;
use crate::transaction::Transaction;
use fides::hash;
use opis::Int;
use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;

#[derive(Clone, Debug)]
pub enum Status {
    Accepted,
    BalanceError,
    SolarError
}

#[derive(Clone, Debug)]
pub struct Receipt {
    pub solar_used: u32,
    pub status: Status
}

impl Receipt {
    pub fn hash(&self) -> [u8; 32] {
        match self.status {
            Status::Accepted => merkle_tree_hash(vec![hash(&self.solar_used.to_be_bytes().to_vec()), hash(&vec![1_u8])]),
            Status::BalanceError => merkle_tree_hash(vec![hash(&self.solar_used.to_be_bytes().to_vec()), hash(&vec![2_u8])]),
            Status::SolarError => merkle_tree_hash(vec![hash(&self.solar_used.to_be_bytes().to_vec()), hash(&vec![3_u8])])
        }
    }
}

pub fn apply_block(mut accounts: &mut HashMap<[u8; 32], Account>, latest_block: Block, previous_block: Block) -> Result<Vec<[u8; 32]>, Box<dyn Error>> {

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
        
        let (updated_addresses, applied_txs, receipts) = apply_txs(&mut accounts, latest_block.transactions, &current_solar_price);

        if receipts.len() == number_of_transactions {
            
            let transactions_hash: [u8; 32] = merkle_tree_hash(applied_txs
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
                accounts_hash(&accounts) == latest_block.accounts_hash,
                solar_used == latest_block.solar_used,
                receipts_hash == latest_block.receipts_hash,
                transactions_hash == latest_block.transactions_hash
            ];

            if checks.iter().any(|&x| x == false) {
                println!("State and Block do not match!");
                Err("State and Block do not match!")?
            } else {
                Ok(updated_addresses)
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

pub fn apply_txs(mut accounts: &mut HashMap<[u8; 32], Account>, txs: Vec<Transaction>, current_solar_price: &Int) -> (Vec<[u8; 32]>, Vec<Transaction>, Vec<Receipt>) {

    let mut all_updated_addresses: Vec<[u8; 32]> = Vec::new();

    let mut applied_txs: Vec<Transaction> = Vec::new();

    let mut receipts: Vec<Receipt> = Vec::new();

    for tx in txs {

        let (updated_addresses, receipt) = apply_tx(&mut accounts, &tx, &current_solar_price);

        match receipt {
            Some(r) => {

                receipts.push(r);

                applied_txs.push(tx);

                for update in updated_addresses{

                    all_updated_addresses.push(update);

                    all_updated_addresses.sort();

                    all_updated_addresses.dedup();

                }

            },

            None => ()
        }
    }

    (all_updated_addresses, applied_txs, receipts)

}

pub fn apply_tx(accounts: &mut HashMap<[u8; 32], Account>, tx: &Transaction, current_solar_price: &Int) -> (Vec<[u8; 32]>, Option<Receipt>) {

    if &tx.solar_price >= current_solar_price {

        if tx.solar_limit >= 1000 {

            match accounts.get(&tx.sender) {

                Some(s) => {

                    let mut sender = s.clone();

                    let mut solar_used: u32 = 0;

                    let transaction_processing_cost: u32 = 1000;

                    let transaction_processing_fee: Int =  &Int::from_decimal(&format!("{}",transaction_processing_cost)) * &current_solar_price;

                    sender.balance -= transaction_processing_fee;

                    solar_used += transaction_processing_cost;

                    match accounts.get(&tx.recipient) {

                        Some(r) => {

                            let mut recipient = r.clone();

                            if sender.balance >= tx.value {

                                sender.balance -= tx.value.clone();

                                recipient.balance += tx.value.clone();

                                let receipt = Receipt { solar_used: solar_used, status: Status::Accepted };

                                accounts.insert(tx.sender, sender);

                                accounts.insert(tx.recipient, recipient);

                                (vec![tx.sender, tx.recipient], Some(receipt))
                                
                            } else {

                                let receipt = Receipt { solar_used: solar_used, status: Status::BalanceError };

                                accounts.insert(tx.sender, sender);

                                (vec![tx.sender], Some(receipt))

                            }
                        },

                        None => {

                            let remaining_tx_solar = tx.solar_limit - solar_used;

                            let account_creation_cost: u32 = 1000000;
                            
                            if remaining_tx_solar >= account_creation_cost {

                                let account_creation_fee: Int = &Int::from_decimal(&format!("{}", account_creation_cost)) * &current_solar_price;
                                
                                if sender.balance >= account_creation_fee {

                                    solar_used += account_creation_cost;

                                    sender.balance -= account_creation_fee;
                                
                                    if sender.balance >= tx.value {
                                        
                                        sender.balance -= tx.value.clone();

                                        let recipient = Account::new(&tx.value);

                                        let receipt = Receipt { solar_used: solar_used, status: Status::Accepted };

                                        accounts.insert(tx.sender, sender);

                                        accounts.insert(tx.recipient, recipient);

                                        (vec![tx.sender, tx.recipient], Some(receipt))

                                    } else {
                                        
                                        let receipt = Receipt { solar_used: solar_used, status: Status::BalanceError };

                                        accounts.insert(tx.sender, sender);

                                        (vec![tx.sender], Some(receipt))

                                    }

                                } else {

                                    let receipt = Receipt { solar_used: solar_used, status: Status::BalanceError };

                                    accounts.insert(tx.sender, sender);
                                    
                                    (vec![tx.sender], Some(receipt))

                                }

                            }  else {

                                let receipt = Receipt { solar_used: solar_used, status: Status::SolarError };

                                accounts.insert(tx.sender, sender);

                                (vec![tx.sender], Some(receipt))

                            } 
                        }
                    }
                },
                None => (vec![], None)
            }
        } else {
            (vec![], None)
        }
    } else {
        (vec![], None)
    }
}