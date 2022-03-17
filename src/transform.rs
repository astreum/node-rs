
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
use std::error::Error;
use std::convert::TryInto;

#[derive(Clone, Debug)]
pub enum Status {
    Accepted,
    BalanceError,
    SolarError
}

#[derive(Clone, Debug)]
pub struct Receipt {
    solar_used: u32,
    status: Status
}

impl Receipt {
    pub fn hash(self) -> [u8; 32] {
        match self.status {
            Status::Accepted => merkle_tree_hash(vec![hash(&self.solar_used.to_be_bytes().to_vec()), hash(&vec![1_u8])]),
            Status::BalanceError => merkle_tree_hash(vec![hash(&self.solar_used.to_be_bytes().to_vec()), hash(&vec![2_u8])]),
            Status::SolarError => merkle_tree_hash(vec![hash(&self.solar_used.to_be_bytes().to_vec()), hash(&vec![3_u8])])
        }
    }
}

pub fn is_applicable(accounts: &HashMap<[u8; 32], Account>, tx: &Transaction, solar_price: &Int) -> bool {

    if &tx.solar_price >= solar_price {

        if tx.solar_limit >= 1000 {

            match accounts.get(&tx.sender) {

                Some(sender) => {

                    if sender.counter == tx.counter {

                        let tx_fee: Int =  &Int::from_decimal("1000") * solar_price;
                        
                        if sender.balance >= tx_fee {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                },
                None => false
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub fn apply_block(
    mut accounts: HashMap<[u8; 32], Account>,
    new_block: Block,
    latest_block: Block
)
-> Result<HashMap<[u8; 32], Account>, Box<dyn Error>> {

    let mut nova_account = accounts.get(&NOVA_ADDRESS).unwrap().clone();

    let slots_in_epoch: Int = Int::from_decimal("201600");

    if &new_block.number % &slots_in_epoch == Int::zero() {

        let nova_stake_store = nova_account.storage.get(&NOVA_STAKE_STORE_ID).unwrap();

        let total_stake = nova_stake_store.iter().fold(Int::zero(), |acc, x| acc + Int::from_bytes(&x.1.to_vec()));

        let mut new_nova_stake_store: HashMap<[u8; 32], [u8; 32]> = HashMap::new();

        for (key, value) in nova_stake_store {

            let x_slots = &(&Int::from_bytes(&value.to_vec()) / &total_stake) * &slots_in_epoch;

            new_nova_stake_store.insert(*key, x_slots.to_ext_bytes(32).try_into().unwrap());

        }

        nova_account.storage.insert(NOVA_SLOTS_STORE_ID, new_nova_stake_store).unwrap();

    }

    let nova_slots_store = nova_account.storage.get(&NOVA_SLOTS_STORE_ID).unwrap();

    let time_diff = new_block.time - latest_block.time;
    
    let slots_missed: u64 = if time_diff > 3 {
        (time_diff - 1) / 3
    } else {
        0
    };
    
    let (current_validator, new_nova_slots_store) = &slots::current_validator(nova_slots_store, slots_missed, latest_block.hash);
    
    if current_validator == &new_block.validator {
        
        let mut receipts: Vec<Receipt> = Vec::new();

        for tx in &new_block.transactions {

            if is_applicable(&accounts, tx, &new_block.solar_price) {

                let mut solar_used: u32 = 0;

                let mut sender = accounts.get(&tx.sender).unwrap().clone();

                let tx_fee: Int =  &Int::from_decimal("1000") * &new_block.solar_price;

                sender.balance -= tx_fee;

                solar_used += 1000;

                match accounts.get(&tx.recipient) {

                    Some(r) => {

                        if sender.balance >= tx.value {

                            sender.balance -= tx.value.clone();

                            let mut recipient = r.clone();

                            recipient.balance += tx.value.clone();

                            let receipt = Receipt { solar_used: solar_used, status: Status::Accepted };

                            accounts.insert(tx.sender, sender);

                            accounts.insert(tx.recipient, recipient.clone());

                            receipts.push(receipt)
                            
                        } else {

                            accounts.insert(tx.sender, sender);

                            let receipt = Receipt { solar_used: solar_used, status: Status::BalanceError };

                            receipts.push(receipt)

                        }
                    },

                    None => {
                        
                        if tx.solar_limit >= 2000 {

                            let account_fee: Int = &Int::from_decimal("1000") * &new_block.solar_price;
                            
                            if sender.balance >= account_fee {

                                solar_used += 1000;

                                sender.balance -= account_fee;
                            
                                if sender.balance >= tx.value {
                                    
                                    sender.balance -= tx.value.clone();

                                    let recipient = Account::new(&tx.value);

                                    accounts.insert(tx.sender, sender);

                                    accounts.insert(tx.recipient, recipient);

                                    let receipt = Receipt { solar_used: solar_used, status: Status::Accepted };

                                    receipts.push(receipt)

                                } else {

                                    accounts.insert(tx.sender, sender);
                                    
                                    let receipt = Receipt { solar_used: solar_used, status: Status::BalanceError };

                                    receipts.push(receipt)

                                }

                            } else {

                                accounts.insert(tx.sender, sender);

                                let receipt = Receipt { solar_used: solar_used, status: Status::BalanceError };
                                
                                receipts.push(receipt)

                            }

                        }  else {

                            accounts.insert(tx.sender, sender);

                            let receipt = Receipt { solar_used: solar_used, status: Status::SolarError };

                            receipts.push(receipt)

                        } 
                    }
                }
            } else {
                break
            }

        }

        if receipts.len() == new_block.transactions.len() {

            let receipts_hash: [u8; 32] = merkle_tree_hash(receipts
                .iter()
                .map(| x | x.clone().hash() )
                .collect()
            );

            let solar_used: u32 = receipts
                .iter()
                .fold(0, | acc, x | acc + x.solar_used );

            let mut validator = accounts.get(&new_block.validator).unwrap().clone();

            validator.balance += Int::from_decimal("1000000000000000000000000");

            accounts.insert(new_block.validator, validator);
        
            nova_account.storage.insert(NOVA_SLOTS_STORE_ID, new_nova_slots_store.clone());

            nova_account.storage_hash = nova_account.storage_hash();

            nova_account.hash = nova_account.hash();

            accounts.insert(NOVA_ADDRESS, nova_account);

            let checks: Vec<bool> = vec![
                accounts_hash(&accounts) == new_block.accounts_hash,
                solar_used == new_block.solar_used,
                receipts_hash == new_block.receipts_hash
            ];

            if checks.iter().any(|&x| x == false) {
                println!("State and Block do not match!");
                Err("State and Block do not match!")?
            } else {
                Ok(accounts)
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