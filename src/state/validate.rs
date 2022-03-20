
use crate::account::accounts_hash;
use crate::block::Block;
use crate::nova::slots;
use crate::state::State;
use crate::transaction::{apply_many, Transaction};
use crate::NOVA_ADDRESS;
use crate::NOVA_SLOTS_STORE_ID;
use crate::NOVA_STAKE_STORE_ID;
use fides::ed25519;
use opis::Int;
use pulsar_network::{ Message, MessageKind };
use std::thread;
use std::time::{Duration,Instant,SystemTime };
use std::sync::Arc;
use std::convert::TryInto;
use crate::merkle_tree_hash;

impl State {

    pub fn validate(&self, private_key: [u8; 32]) {

        println!("validating ...");

        let pub_key: [u8; 32] = ed25519::public_key(&private_key);

        let accounts_clone = Arc::clone(&self.accounts);

        let latest_block_clone = Arc::clone(&self.latest_block);

        let pending_transactions_clone = Arc::clone(&self.pending_transactions);

        let network_clone = Arc::clone(&self.network);

        thread::spawn(move || {

            let slots_in_epoch: Int = Int::from_decimal("201600");

            let mut now = Instant::now();

            loop {

                if now.elapsed().as_secs() > 1 {

                    let mut latest_block_lock = latest_block_clone.lock().unwrap();

                    let latest_block = latest_block_lock.clone();
                    
                    let mut accounts = accounts_clone.lock().unwrap();

                    let mut nova_account = accounts.get(&NOVA_ADDRESS).unwrap().clone();

                    let mut nova_slots_store = nova_account.storage.get(&NOVA_SLOTS_STORE_ID).unwrap().clone();
                    
                    if nova_slots_store.is_empty() {
                        
                        let nova_stake_store = nova_account.storage.get(&NOVA_STAKE_STORE_ID).unwrap();

                        let total_stake = nova_stake_store.iter().fold(Int::zero(), |acc, x| acc + Int::from_bytes(&x.1.to_vec()));

                        for (key, value) in nova_stake_store {

                            let x_slots = &(&Int::from_bytes(&value.to_vec()) / &total_stake) * &slots_in_epoch;

                            nova_slots_store.insert(*key, x_slots.to_ext_bytes(32).try_into().unwrap());

                        }

                    }

                    let time_diff = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() - latest_block.time;

                    let slots_missed: u64 = if time_diff > 3 {
                        (time_diff - 1) / 3
                    } else {
                        0
                    };
                    
                    let (current_validator, new_nova_slots_store) = &slots::current_validator(&nova_slots_store, slots_missed, latest_block.hash);
                        
                    if current_validator == &pub_key {

                        let mut solar_limit: u32 = 1000000000;

                        let current_solar_price: Int = if latest_block.solar_used > 750000000 {
                            
                                latest_block.solar_price + Int::one()
                            
                            } else if latest_block.solar_used < 250000000 {
                                
                                if latest_block.solar_price == Int::one() {
                                    
                                    latest_block.solar_price
                                
                                } else {
                                    
                                    latest_block.solar_price - Int::one()
                                
                                }

                            } else {
                                
                                latest_block.solar_price
                            
                        };

                        let pending_transactions = pending_transactions_clone.lock().unwrap().clone();

                        let mut txs: Vec<Transaction> = Vec::new();
                        
                        for (_,tx) in pending_transactions {

                            if solar_limit >= tx.solar_limit {

                                solar_limit -= tx.solar_limit;

                                txs.push(tx);
                                
                            }

                        }

                        let current_block_transactions_hash_int: Int = Int::from_bytes(&latest_block.transactions_hash.to_vec());

                        txs.sort_by_key(|k| &Int::from_bytes(&k.hash.to_vec()) ^ &current_block_transactions_hash_int);

                        let (updated, applied_txs, receipts) = apply_many::run(accounts.clone(), txs, &current_solar_price);

                        for (address, acc) in updated {
                            accounts.insert(address, acc);
                        }

                        let mut validator = accounts.get(&pub_key).unwrap().clone();
                        
                        validator.balance += Int::from_decimal("1000000000000000000000000");
                        
                        accounts.insert(pub_key, validator);

                        nova_account.storage.insert(NOVA_SLOTS_STORE_ID, new_nova_slots_store.clone());

                        nova_account.storage_hash = nova_account.storage_hash();

                        nova_account.hash = nova_account.hash();

                        accounts.insert(NOVA_ADDRESS, nova_account);

                        let receipts_hash: [u8; 32] = merkle_tree_hash(receipts
                            .iter()
                            .map(| x | x.hash() )
                            .collect()
                        );

                        let transactions_hash: [u8; 32] = merkle_tree_hash(applied_txs
                            .iter()
                            .map(| x | x.hash )
                            .collect()
                        );
            
                        let solar_used: u32 = receipts
                            .iter()
                            .fold(0, | acc, x | acc + x.solar_used );

                        let accs_hash: [u8; 32] = accounts_hash(&accounts);

                        while (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() - latest_block.time) % 3 != 0 {
                            thread::sleep(Duration::from_millis(100));
                        }

                        let mut new_block: Block = Block {
                            accounts_hash: accs_hash,
                            body_hash: [0_u8; 32],
                            chain: latest_block.chain,
                            hash: [0_u8; 32],
                            number: latest_block.number + Int::one(),
                            previous_block_hash: latest_block.hash,
                            receipts_hash: receipts_hash,
                            signature: [0_u8; 64],
                            solar_price: current_solar_price,
                            solar_used: solar_used,
                            time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                            transactions_hash: transactions_hash,
                            transactions: applied_txs,
                            validator: pub_key
                        };

                        new_block.body_hash = new_block.body_hash();

                        new_block.hash = new_block.hash();

                        new_block.signature = ed25519::sign(&new_block.body_hash, &private_key, &pub_key);

                        let new_block_message: Message = Message::new(MessageKind::Block, new_block.to_bytes());

                        let network = network_clone.lock().unwrap();

                        network.broadcast(new_block_message);
                        
                        *latest_block_lock = new_block;

                    };

                    now = Instant::now()

                }
            }
        });
    }
}