use std::collections::HashMap;
use std::error::Error;
use neutrondb::Store;
use opis::Integer;
use crate::CONSENSUS_ADDRESS;
use crate::receipt::{Receipt, Status};
use crate::transaction::Transaction;
use crate::address::Address;
use crate::account::Account;

impl Transaction {

    pub fn apply(

        &self,
        accounts_store: &Store<Address, Account>,
        changed_accounts: &mut HashMap<Address, Account>
        
    ) -> Result<Receipt, Box<dyn Error>> {

        match Account::from_accounts(&self.sender, &changed_accounts, accounts_store) {

            Ok(mut sender) => {

                if sender.counter == self.counter {

                    let mut solar_used = 0;

                    let transaction_cost = Integer::from_dec("1000")?;

                    sender.decrease_balance(&transaction_cost)?;

                    solar_used += 1000;

                    if self.sender != self.recipient {

                        if self.recipient == CONSENSUS_ADDRESS {
                            
                            if self.value != Integer::zero() {
                                
                                sender.decrease_balance(&self.value)?;

                                match Account::from_accounts(&self.sender, &changed_accounts, &accounts_store) {

                                    Ok(_consensus) => {

                                        // add stake

                                        Err("")?

                                    }

                                    Err(_) => Err("")?,

                                }

                            } else {

                                // decrease stake by amount in tx data

                                // add value to user account

                                Err("")?

                            }


                        } else {

                            match Account::from_accounts(&self.recipient, &changed_accounts, &accounts_store) {

                                Ok(mut recipient) => {

                                    match sender.decrease_balance(&self.value) {

                                        Ok(_) => {

                                            recipient.increase_balance(&self.value);

                                            sender.increase_counter();

                                            changed_accounts.insert(self.sender, sender);

                                            changed_accounts.insert(self.recipient, recipient);

                                            Ok(Receipt {
                                                solar_used,
                                                status: Status::Accepted
                                            })

                                        },

                                        Err(_) => {

                                            sender.increase_counter();

                                            changed_accounts.insert(self.sender, sender);

                                            Ok(Receipt {
                                                solar_used,
                                                status: Status::BalanceError
                                            })

                                        },

                                    }

                                },

                                Err(_) => {

                                    let account_cost = Integer::from_dec("1000000")?;

                                    match sender.decrease_balance(&account_cost) {

                                        Ok(_) => {

                                            solar_used += 1000000;

                                            match sender.decrease_balance(&self.value) {

                                                Ok(_) => {

                                                    let mut recipient = Account::new();

                                                    recipient.increase_balance(&self.value);

                                                    changed_accounts.insert(self.sender, sender);

                                                    changed_accounts.insert(self.recipient, recipient);

                                                    Ok(Receipt {
                                                        solar_used,
                                                        status: Status::Accepted
                                                    })

                                                },

                                                Err(_) => {

                                                    sender.increase_counter();

                                                    changed_accounts.insert(self.sender, sender);

                                                    Ok(Receipt {
                                                        solar_used,
                                                        status: Status::BalanceError
                                                    })

                                                },

                                            }

                                        },

                                        Err(_) => {

                                            sender.increase_counter();

                                            changed_accounts.insert(self.sender, sender);

                                            Ok(Receipt {
                                                solar_used,
                                                status: Status::BalanceError
                                            })

                                        },

                                    }

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

            Err(_) => Err("Internal error!")?

        }

    }
    
}
