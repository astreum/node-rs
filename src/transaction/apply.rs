use crate::account::Account;
use crate::block::Block;
use neutrondb::Store;
use opis::Int;
use std::{collections::HashMap, error::Error};
use super::{receipt::{Receipt, Status}, Transaction};

impl Transaction {

    pub fn apply(
        &self,
        accounts_store: &Store,
        changed_accounts: &mut HashMap<[u8;32], Account>,
        solar_price: &Int
    ) -> Result<Receipt, Box<dyn Error>> {

        if &self.solar_price >= solar_price {

            let self_cost = Int::from_decimal("1000");

            if self.solar_limit >= self_cost {

                match Account::from_accounts(&self.sender, &changed_accounts, accounts_store) {

                    Some(mut sender) => {
    
                        if sender.counter == self.counter {
    
                            let mut solar_used = Int::zero();
    
                            let self_fee =  &self_cost * solar_price;
    
                            sender.balance -= self_fee;
    
                            solar_used += self_cost;

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
        
                                        let remaining_self_solar = &self.solar_limit - &solar_used;
        
                                        let account_creation_cost = Int::from_decimal("200000");
                                        
                                        if remaining_self_solar >= account_creation_cost {
        
                                            let account_creation_fee = &account_creation_cost * solar_price;
                                            
                                            if sender.balance >= account_creation_fee {
        
                                                solar_used += account_creation_cost;
        
                                                sender.balance -= account_creation_fee;
                                            
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
        
                                            } else {

                                                changed_accounts.insert(self.sender, sender);
        
                                                Ok(Receipt { solar_used, status: Status::BalanceError })
        
                                            }
        
                                        }  else {

                                            changed_accounts.insert(self.sender, sender);
        
                                            Ok(Receipt { solar_used, status: Status::SolarError })
        
                                        } 
                                    }
                                }
                            } else {
                                Err("")?
                            }
                        } else {
                            Err("")?
                        }
                    },
                    None => Err("")?
                }
            } else {
                Err("")?
            }
        } else {
            Err("")?
        }
    }
}
