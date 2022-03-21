
use crate::account::Account;
use crate::transaction::{Receipt, Status, Transaction};
use opis::Int;
use std::collections::HashMap;

pub fn run(
    accounts: &HashMap<[u8; 32], Account>,
    tx: &Transaction,
    current_solar_price: &Int
) -> (HashMap<[u8; 32], Account>, Option<Receipt>) {

    let mut res: HashMap<[u8; 32], Account> = HashMap::new();

    if &tx.solar_price >= current_solar_price {

        if tx.solar_limit >= 1000 {

            match accounts.get(&tx.sender) {

                Some(s) => {

                    if s.counter == tx.counter {

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

                                    res.insert(tx.sender, sender);

                                    res.insert(tx.recipient, recipient);

                                    (res, Some(receipt))
                                    
                                } else {

                                    let receipt = Receipt { solar_used: solar_used, status: Status::BalanceError };

                                    res.insert(tx.sender, sender);

                                    (res, Some(receipt))

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

                                            res.insert(tx.sender, sender);

                                            res.insert(tx.recipient, recipient);

                                            (res, Some(receipt))

                                        } else {
                                            
                                            let receipt = Receipt { solar_used: solar_used, status: Status::BalanceError };

                                            res.insert(tx.sender, sender);

                                            (res, Some(receipt))

                                        }

                                    } else {

                                        let receipt = Receipt { solar_used: solar_used, status: Status::BalanceError };

                                        res.insert(tx.sender, sender);
                                        
                                        (res, Some(receipt))

                                    }

                                }  else {

                                    let receipt = Receipt { solar_used: solar_used, status: Status::SolarError };

                                    res.insert(tx.sender, sender);

                                    (res, Some(receipt))

                                } 
                            }
                        }

                    } else {
                        (HashMap::new(), None)
                    }
                },
                None => (HashMap::new(), None)
            }
        } else {
            (HashMap::new(), None)
        }
    } else {
        (HashMap::new(), None)
    }
}
