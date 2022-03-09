use crate::account::Account;
use crate::block::Block;
use crate::merkle_tree_hash;
use crate::transaction::Transaction;
use fides::hash;
use opis::Int;
use std::collections::HashMap;
use std::error::Error;

#[derive(Clone, Debug)]
pub enum Status {
    Accepted,
    BalanceError,
    SolarError
}

#[derive(Clone, Debug)]
pub struct Receipt {
    solar_used: Int,
    status: Status
}

impl Receipt {
    pub fn hash(self) -> [u8; 32] {
        match self.status {
            Status::Accepted => merkle_tree_hash(vec![hash(&self.solar_used.to_ext_bytes(32)), hash(&vec![1_u8])]),
            Status::BalanceError => merkle_tree_hash(vec![hash(&self.solar_used.to_ext_bytes(32)), hash(&vec![2_u8])]),
            Status::SolarError => merkle_tree_hash(vec![hash(&self.solar_used.to_ext_bytes(32)), hash(&vec![3_u8])])
        }
    }
}

pub fn is_applicable(accounts: &HashMap<[u8; 32], Account>, tx: &Transaction, solar_price: &Int) -> bool {

    if &tx.solar_price >= solar_price {

        if tx.solar_limit >= Int::from_decimal("1000") {

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

pub fn apply_block(mut accounts: HashMap<[u8; 32], Account>, block: Block) -> Result<HashMap<[u8; 32], Account>, Box<dyn Error>> {

    let mut receipts: Vec<Receipt> = Vec::new();

    for tx in &block.transactions {

        if is_applicable(&accounts, tx, &block.solar_price) {

            let mut sender = accounts.get(&tx.sender).unwrap().clone();

            let mut solar_cost: Int = Int::from_decimal("1000");

            let tx_fee: Int =  &solar_cost * &block.solar_price;

            sender.balance -= tx_fee;

            match accounts.get(&tx.recipient) {

                Some(r) => {

                    if sender.balance >= tx.value {

                        sender.balance -= tx.value.clone();

                        let mut recipient = r.clone();

                        recipient.balance += tx.value.clone();

                        let receipt = Receipt { solar_used: solar_cost, status: Status::Accepted };

                        accounts.insert(tx.sender, sender);

                        accounts.insert(tx.recipient, recipient.clone());

                        receipts.push(receipt)
                        
                    } else {

                        accounts.insert(tx.sender, sender);

                        let receipt = Receipt { solar_used: solar_cost, status: Status::BalanceError };

                        receipts.push(receipt)

                    }
                },

                None => {

                    let account_cost = Int::from_decimal("1000");
                    
                    if tx.solar_limit >= Int::from_decimal("2000") {

                        let account_fee: Int = &account_cost * &block.solar_price;
                        
                        if sender.balance >= account_fee {

                            solar_cost += account_cost;

                            sender.balance -= account_fee;
                        
                            if sender.balance >= tx.value {
                                
                                sender.balance -= tx.value.clone();

                                let recipient = Account {
                                    balance: tx.value.clone(),
                                    counter: Int::zero(),
                                    index: accounts.len(),
                                    storage: HashMap::new()
                                };

                                accounts.insert(tx.sender, sender);

                                accounts.insert(tx.recipient, recipient);

                                let receipt = Receipt { solar_used: solar_cost, status: Status::Accepted };

                                receipts.push(receipt)

                            } else {

                                accounts.insert(tx.sender, sender);
                                
                                let receipt = Receipt { solar_used: solar_cost, status: Status::BalanceError };

                                receipts.push(receipt)

                            }

                        } else {

                            accounts.insert(tx.sender, sender);

                            let receipt = Receipt { solar_used: solar_cost, status: Status::BalanceError };
                            
                            receipts.push(receipt)

                        }

                    }  else {

                        accounts.insert(tx.sender, sender);

                        let receipt = Receipt { solar_used: solar_cost, status: Status::SolarError };

                        receipts.push(receipt)

                    } 
                }
            }
        } else {
            break
        }

    }

    if receipts.len() == block.transactions.len() {

        let receipts_hash: [u8; 32] = merkle_tree_hash(receipts
            .iter()
            .map(| x | x.clone().hash() )
            .collect()
        );

        let solar_used: Int = receipts
            .iter()
            .fold( Int::zero(), | acc, x | acc + x.solar_used.clone() );

        let mut validator = accounts.get(&block.validator).unwrap().clone();

        validator.balance += Int::one();

        accounts.insert(block.validator, validator);

        let checks: Vec<bool> = vec![
            accounts_hash(&accounts) == block.accounts_hash,
            solar_used == block.solar_used,
            receipts_hash == block.receipts_hash
        ];

        if checks.iter().any(|&x| x == false) {

            Err("State and Block do not match!")?

        } else {
            Ok(accounts)
        }

    } else {

        Err("Some transactions could not be applied!")?

    }

}

pub fn accounts_hash(_accounts: &HashMap<[u8; 32], Account>) -> [u8; 32] {
    [0_u8; 32]
}
