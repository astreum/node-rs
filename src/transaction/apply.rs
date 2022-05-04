use crate::account::Account;
use crate::block::Block;
use neutrondb::Store;
use opis::Int;
use std::{collections::HashMap, error::Error};
use super::{receipt::{Receipt, Status}, Transaction};

pub struct ApplyTxArg<'a> {
    pub accounts_store: &'a Store,
    pub latest_block: &'a Block,
    pub solar_price: &'a Int
}

pub struct ApplyTxRes {
    pub accounts: HashMap<[u8;32], Account>,
    pub receipt: Receipt
}

impl Transaction {

    pub fn apply(&self, apply_tx_arg: ApplyTxArg) -> Result<ApplyTxRes, Box<dyn Error>> {

        let mut accounts = HashMap::new();

        if &self.solar_price >= &apply_tx_arg.solar_price {

            let self_cost = Int::from_decimal("1000");

            if self.solar_limit >= self_cost {

                match Account::from_accounts(&self.sender, &accounts, &apply_tx_arg.accounts_store) {

                    Some(mut sender) => {
    
                        if sender.counter == self.counter {
    
                            let mut solar_used = Int::zero();
    
                            let self_fee =  &self_cost * &apply_tx_arg.solar_price;
    
                            sender.balance -= self_fee;
    
                            solar_used += self_cost;

                            if self.sender != self.recipient {
    
                                match Account::from_accounts(&self.recipient, &accounts, &apply_tx_arg.accounts_store) {
        
                                    Some(mut recipient) => {
        
                                        if sender.balance >= self.value {
        
                                            sender.balance -= self.value.clone();
        
                                            recipient.balance += self.value.clone();

                                            accounts.insert(self.sender, sender);

                                            accounts.insert(self.recipient, recipient);
        
                                            let receipt = Receipt { solar_used, status: Status::Accepted };

                                            Ok(ApplyTxRes { accounts, receipt })
                                            
                                        } else {

                                            accounts.insert(self.sender, sender);
        
                                            let receipt = Receipt { solar_used, status: Status::BalanceError };

                                            Ok(ApplyTxRes { accounts, receipt })
        
                                        }
                                    },
        
                                    None => {
        
                                        let remaining_self_solar = &self.solar_limit - &solar_used;
        
                                        let account_creation_cost = Int::from_decimal("200000");
                                        
                                        if remaining_self_solar >= account_creation_cost {
        
                                            let account_creation_fee = &account_creation_cost * &apply_tx_arg.solar_price;
                                            
                                            if sender.balance >= account_creation_fee {
        
                                                solar_used += account_creation_cost;
        
                                                sender.balance -= account_creation_fee;
                                            
                                                if sender.balance >= self.value {
                                                    
                                                    sender.balance -= self.value.clone();
        
                                                    let mut recipient = Account::new();

                                                    recipient.balance = self.value.clone();

                                                    accounts.insert(self.sender, sender);

                                                    accounts.insert(self.recipient, recipient);
        
                                                    let receipt = Receipt { solar_used, status: Status::Accepted };

                                                    Ok(ApplyTxRes { accounts, receipt })
        
                                                } else {
                                                    
                                                    accounts.insert(self.sender, sender);

                                                    let receipt = Receipt { solar_used, status: Status::BalanceError };
        
                                                    Ok(ApplyTxRes { accounts, receipt })
        
                                                }
        
                                            } else {

                                                accounts.insert(self.sender, sender);
        
                                                let receipt = Receipt { solar_used, status: Status::BalanceError };
        
                                                Ok(ApplyTxRes { accounts, receipt })
        
                                            }
        
                                        }  else {

                                            accounts.insert(self.sender, sender);
        
                                            let receipt = Receipt { solar_used, status: Status::SolarError };
        
                                            Ok(ApplyTxRes { accounts, receipt })
        
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
