use crate::accounts::Account;
use crate::transactions::receipt::{ Receipt, Status };
use crate::transactions::Transaction;
use neutrondb::Store;
use opis::Int;
use std::{collections::HashMap, error::Error};

impl Transaction {

    pub fn apply(
        &self,
        accounts_store: &Store,
        changed_accounts: &mut HashMap<[u8;32], Account>,
        solar_price: &Int
    ) -> Result<Receipt, Box<dyn Error>> {

        if &self.solar_price >= solar_price {

            let transaction_cost = Int::from_decimal("1000");

            if self.solar_limit >= transaction_cost {

                match Account::from_accounts(&self.sender, &changed_accounts, accounts_store) {

                    Some(mut sender) => {
    
                        if sender.counter == self.counter {
    
                            let mut solar_used = Int::zero();
    
                            let transaction_fee =  &transaction_cost * solar_price;
    
                            sender.balance -= transaction_fee;
    
                            solar_used += transaction_cost;

                            if self.sender != self.recipient {
    
                                match Account::from_accounts(&self.recipient, &changed_accounts, &accounts_store) {
        
                                    Some(mut recipient) => {
        
                                        if sender.balance >= self.value {
        
                                            sender.balance -= self.value.clone();
        
                                            recipient.balance += self.value.clone();

                                            changed_accounts.insert(self.sender, sender);

                                            changed_accounts.insert(self.recipient, recipient);
        
                                            Ok(Receipt { solar_used, status: Status::Accepted })
                                            
                                        } else {

                                            changed_accounts.insert(self.sender, sender);
        
                                            Ok(Receipt { solar_used, status: Status::BalanceError })
        
                                        }
                                    },
        
                                    None => {
                                            
                                        if sender.balance >= self.value {
                                            
                                            sender.balance -= self.value.clone();

                                            let mut recipient = Account::new();

                                            recipient.balance = self.value.clone();

                                            changed_accounts.insert(self.sender, sender);

                                            changed_accounts.insert(self.recipient, recipient);

                                            Ok(Receipt { solar_used, status: Status::Accepted })

                                        } else {
                                            
                                            changed_accounts.insert(self.sender, sender);

                                            Ok(Receipt { solar_used, status: Status::BalanceError })

                                        }

                                    }
                                }

                            } else {
                                Err("Internal error!")?
                            }

                        } else {
                            Err("Internal error!")?
                        }

                    },

                    None => Err("Internal error!")?

                }

            } else {
                Err("Internal error!")?
            }

        } else {
            Err("Internal error!")?
        }

    }
    
}
