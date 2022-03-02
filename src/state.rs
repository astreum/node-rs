use std::collections::HashMap;
use neutrondb::Store;
use crate::account::Account;
use crate::block::Block;
use crate::transaction::Transaction;
use std::error::Error;
use crate::merkle_tree_hash;
use opis::Int;
use fides::hash;
use std::convert::TryInto;

pub enum Status {
    Accepted,
    AccountError,
    BalanceError,
}

pub struct Receipt {
    solar_used: [u8; 32],
    status: Status
}

impl Receipt {
    pub fn hash(self) -> [u8; 32] {
        match self.status {
            Accepted => merkle_tree_hash(vec![hash(&self.solar_used.to_vec()), hash(vec![1_u8])]),
            AccountError => merkle_tree_hash(vec![hash(&self.solar_used.to_vec()), hash(vec![2_u8])]),
            BalanceError => merkle_tree_hash(vec![hash(&self.solar_used.to_vec()), hash(vec![3_u8])])
        }
    }
}

pub struct State {
    pub accounts: HashMap<[u8; 32], Account>,
    pub accounts_store: Store,
    pub current_block: Block,
    pub nova_storage: HashMap<[u8; 32], (usize, [u8; 32])>
}

impl State {

    pub fn new(mut blocks: Vec<Block>, chain: u8) -> Self {
        
        blocks.sort_by(|a, b| Int::from_bytes(&a.number.to_vec()).cmp(&Int::from_bytes(&b.number.to_vec())));

        let mut res: State = State {
            accounts: HashMap::new(),
            accounts_store: Store::connect("accounts"),
            current_block: Block::genesis(chain),
            nova_storage: HashMap::new()
        };

        for block in blocks {
            res.transform(block).unwrap()
        }

        res

    }

    pub fn accounts_hash(self) -> [u8; 32] {
        
        let mut accounts: Vec<&Account> = self.accounts
            .iter()
            .map(|x| x.1)
            .collect();

        accounts.sort_by(|a, b| a.index.cmp(&b.index));

        let account_hashes: Vec<[u8; 32]> = accounts
            .iter()
            .map(|x| x.hash())
            .collect();

        merkle_tree_hash(&account_hashes)
    }

    pub fn nova_storage_hash(self) -> [u8; 32] {

        let mut nova_objects: Vec<(usize, &[u8; 32], [u8; 32])> = self.nova_storage
            .iter()
            .map(|x| (x.1.0, x.0, x.1.1))
            .collect();

        nova_objects.sort_by(|a, b| a.0.cmp(&b.0));


        let mut object_hashes: Vec<[u8; 32]> = nova_objects
            .iter()
            .map(|x| {
                merkle_tree_hash(vec![
                    hash(&x.1.to_vec()),
                    hash(&x.1.to_vec())
                ])
            })
            .collect();

        merkle_tree_hash(&object_hashes)

    }
    
    pub fn transform(&mut self, mut block: Block) -> Result<(), Box<dyn Error>> {

        let tx_hashes: Vec<[u8; 32]> = block.transactions
            .iter()
            .map(|x| x.hash())
            .collect();

        if merkle_tree_hash(&tx_hashes) == block.transactions_hash {

            let mut receipts: Vec<Receipt> = Vec::new();
        
            for tx in block.transactions {
                receipts.push(self.apply_tx(tx))
            }

            let receipts_hashes: Vec<[u8; 32]> = receipts.iter().map(|x|x.hash()).collect();

            let receipts_solar_used: Vec<[u8; 32]> = receipts.iter().map(|x| x.solar_used).collect();

            let solar_used: Int = Int::zero();

            for solar in receipts_solar_used {
                solar_used += Int::from_bytes(&solar.to_vec())
            }

            let checks: Vec<bool> = vec![
                self.accounts_hash() == block.accounts_hash,
                merkle_tree_hash(&receipts_hashes) == block.receipts_hash,
                solar_used == Int::from_bytes(&block.solar_used.to_vec())
            ];

            if checks.iter().any(|&x| x == false) {

                while receipts.is_empty() || block.transactions.is_empty() {
                    self.reverse_tx(block.transactions.pop().unwrap(), receipts.pop().unwrap())
                }

                Err("State and Block do not match!")?

            } else {

                self.current_block = block;

                Ok(())

            }

        } else {
            Err("Transactions and Transactions Hash do not match!")?
        } 

    }

    pub fn apply_tx(&mut self, mut tx: Transaction) -> Receipt {

        let mut solar_cost: u16 = 1000;

        match self.accounts.get(&tx.recipient) {
            Some(_) => (),
            None => solar_cost += 1000
        };

        let mut required_balance: Int = Int::from_decimal(format!("{}",solar_cost)) * Int::from_bytes(&tx.solar_price.to_vec());

        required_balance += Int::from_bytes(&tx.value.to_vec());

        match self.accounts.get(&tx.sender) {
            
            Some(&(mut s)) => {

                if Int::from_bytes(&s.balance.to_vec()) >= required_balance {

                    s.remove_balance(required_balance);

                    match self.accounts.get(&tx.recipient) {

                        Some(&(mut r)) => {

                            r.add_balance(tx.value);

                            self.accounts.insert(tx.recipient, r);

                        },

                        None => {

                            let new_account = Account {
                                address: tx.recipient,
                                balance: tx.value,
                                counter: [0_u8; 32],
                                index: self.accounts.len(),
                                storage: [0_u8; 32]
                            };

                            self.accounts.insert(tx.recipient, new_account);
                            
                        }

                    }

                    self.accounts.insert(tx.sender, s);

                    let solar_used = Int::from_decimal(format!("{}",solar_cost));

                    Receipt {
                        solar_used: solar_used.to_ext_bytes(32).try_into().unwrap(),
                        status: Status::Accepted
                    }

                } else {

                    Receipt {
                        solar_used: [0_u8; 32],
                        status: Status::BalanceError
                    }

                }

            },

            None => Receipt {
                solar_used: [0_u8; 32],
                status: Status::AccountError
            }

        }

    }

    pub fn reverse_tx(&mut self, tx: Transaction, rt: Receipt) {

        match rt.status {

            Accepted => {

                let solar_used = Int::from_bytes(&rt.solar_used.to_vec());

                if solar_used == Int::from_decimal("2000") {
                    
                    self.accounts.remove(&tx.recipient);
                    
                    let mut sender = self.accounts.get(&tx.sender).unwrap();

                    sender.add_balance(tx.value);

                    self.accounts.insert(tx.sender, *sender);

                } else {

                    let mut recipient = self.accounts.get(&tx.recipient).unwrap();

                    recipient.remove_balance(Int::from_bytes(&tx.value.to_vec()));

                    self.accounts.insert(tx.recipient, *recipient);

                    let mut sender = self.accounts.get(&tx.sender).unwrap();

                    sender.add_balance(tx.value);

                    self.accounts.insert(tx.sender, *sender);

                } 

            },

            _ => ()

        }

    }

    pub fn is_tx_applicable(&self, tx: Transaction) -> bool {

        let mut solar_cost: u16 = 1000;

        match self.accounts.get(&tx.recipient) {
            Some(_) => (),
            None => solar_cost += 1000
        };

        let mut required_balance: Int = Int::from_decimal(format!("{}",solar_cost)) * Int::from_bytes(&tx.solar_price.to_vec());

        required_balance += Int::from_bytes(&tx.value.to_vec());

        match self.accounts.get(&tx.sender) {
            
            Some(s) => {
                if Int::from_bytes(&s.balance.to_vec()) >= required_balance {
                    true
                } else {
                    false
                }
            },
            None => false
        }

    }

}