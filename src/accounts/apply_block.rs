use crate::account::Account;
use crate::accounts::Accounts;
use crate::block::Block;
use astro_format::string;
use fides::{merkle_root, hash};
use neutrondb::Store;
use opis::Int;
use std::collections::HashMap;
use std::error::Error;
use super::receipt::{Receipt, Status};

impl Accounts {

    pub fn apply_block(&self, block: &Block) -> Result<HashMap<[u8;32], Account>, Box<dyn Error>> {

        let mut accounts: HashMap<[u8;32], Account> = HashMap::new();

        let mut receipts: Vec<Receipt> = Vec::new();

        for tx in block.transactions.iter() {
            
            if &tx.solar_price >= &block.solar_price {

                let tx_cost = Int::from_decimal("1000");
    
                if tx.solar_limit >= tx_cost {
    
                    match get_account(&tx.sender, &accounts, &self.store) {
    
                        Some(mut sender) => {
        
                            if sender.counter == tx.counter {
        
                                let mut solar_used = Int::zero();
        
                                let tx_fee =  &tx_cost * &block.solar_price;
        
                                sender.balance -= tx_fee;
        
                                solar_used += tx_cost;
    
                                if tx.sender != tx.recipient {
        
                                    match get_account(&tx.recipient, &accounts, &self.store) {
            
                                        Some(mut recipient) => {
            
                                            if sender.balance >= tx.value {
            
                                                sender.balance -= tx.value.clone();
            
                                                recipient.balance += tx.value.clone();

                                                accounts.insert(tx.sender, sender);

                                                accounts.insert(tx.recipient, recipient);
            
                                                let receipt = Receipt {
                                                    solar_used: solar_used,
                                                    status: Status::Accepted
                                                };
    
                                                receipts.push(receipt);
                                                
                                            } else {

                                                accounts.insert(tx.sender, sender);
            
                                                let receipt = Receipt {
                                                    solar_used: solar_used,
                                                    status: Status::BalanceError
                                                };

                                                receipts.push(receipt);
            
                                            }
                                        },
            
                                        None => {
            
                                            let remaining_tx_solar = &tx.solar_limit - &solar_used;
            
                                            let account_creation_cost = Int::from_decimal("200000");
                                            
                                            if remaining_tx_solar >= account_creation_cost {
            
                                                let account_creation_fee = &account_creation_cost * &block.solar_price;
                                                
                                                if sender.balance >= account_creation_fee {
            
                                                    solar_used += account_creation_cost;
            
                                                    sender.balance -= account_creation_fee;
                                                
                                                    if sender.balance >= tx.value {
                                                        
                                                        sender.balance -= tx.value.clone();
            
                                                        let mut recipient = Account::new();
    
                                                        recipient.balance = tx.value.clone();

                                                        accounts.insert(tx.sender, sender);

                                                        accounts.insert(tx.recipient, recipient);
            
                                                        let receipt = Receipt {
                                                            solar_used: solar_used,
                                                            status: Status::Accepted
                                                        };

                                                        receipts.push(receipt);
            
                                                    } else {
                                                        
                                                        accounts.insert(tx.sender, sender);

                                                        let receipt = Receipt {
                                                            solar_used: solar_used,
                                                            status: Status::BalanceError
                                                        };
            
                                                        receipts.push(receipt);
            
                                                    }
            
                                                } else {

                                                    accounts.insert(tx.sender, sender);
            
                                                    let receipt = Receipt {
                                                        solar_used: solar_used,
                                                        status: Status::BalanceError
                                                    };
            
                                                    receipts.push(receipt);
            
                                                }
            
                                            }  else {

                                                accounts.insert(tx.sender, sender);
            
                                                let receipt = Receipt {
                                                    solar_used: solar_used,
                                                    status: Status::SolarError
                                                };
            
                                                receipts.push(receipt);
            
                                            } 
                                        }
                                    }
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        },
                        None => break
                    }
                } else {
                    break;
                }
            } else {
                break;
            }

        }

        let receipts_hash = merkle_root(receipts.iter().map(|x| x.hash()).collect());

        let accounts_hash = merkle_root(
            self.details
                .iter()
                .map(|x| {

                    let details_hash = match accounts.get(x.0) {
                        Some(account) => account.hash(),
                        None => *x.1
                    };

                    let address_hash = hash(x.0);

                    merkle_root(vec![address_hash, details_hash])
                    
                })
                .collect()
        );

        if [
            receipts_hash == block.receipts_hash,
            accounts_hash == block.accounts_hash
        ]
        .iter()
        .any(|&x| x == false)
        {
            Err("Block application failed!")?
        } else {
            Ok(accounts)
        }
        
    }

}

fn get_account(address: &[u8;32], accounts: &HashMap<[u8;32], Account>, store: &Store) -> Option<Account> {

    match accounts.get(address) {

        Some(account) => Some(account.clone()),

        None => {

            let address_string = string::encode::bytes(address);

            match store.get(&address_string) {

                Some(encoded) => {

                    let bytes = string::decode::bytes(&encoded).unwrap();

                    let account = Account::from_bytes(&bytes).unwrap();

                    Some(account)
                },

                None => None
                
            }
        }
    }
}
