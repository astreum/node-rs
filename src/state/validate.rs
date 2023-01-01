use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::time::SystemTime;
use crate::address::Address;
use crate::block::Block;
use crate::relay::Message;
use crate::relay::Topic;
use crate::state::consensus;

use super::State;

impl State {

    pub fn validate(&self, validator_address: Address, secret_key: [u8; 32]) {

        println!("validating ...");

        let accounts_clone = Arc::clone(&self.accounts);

        let accounts_store_clone = Arc::clone(&self.accounts_store);

        let blocks_store_clone = Arc::clone(&self.blocks_store);

        let latest_block_clone = Arc::clone(&self.latest_block);

        let pending_transactions_clone = Arc::clone(&self.pending_transactions);

        let relay_clone = Arc::clone(&self.relay);

        thread::spawn(move || {

            let mut now = Instant::now();

            loop {

                if now.elapsed().as_secs() > 1 {

                    let mut accounts = accounts_clone.lock().unwrap();

                    let mut accounts_store = accounts_store_clone.lock().unwrap();

                    let mut latest_block = latest_block_clone.lock().unwrap();

                    let mut current_time = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    let time_diff = current_time - &latest_block.time;

                    let target_time = if time_diff > 3 {

                        current_time
                
                    } else {
                
                        latest_block.time + 3
                
                    };

                    match consensus::selection(&accounts_store, &latest_block, &current_time) {

                        Ok(validator_selection) => {

                            if validator_selection == validator_address {

                                let mut pending_transactions = pending_transactions_clone.lock().unwrap();

                                match Block::create(
                                    &accounts,
                                    &accounts_store,
                                    &latest_block,
                                    &pending_transactions,
                                    &secret_key,
                                    &validator_address,
                                    &target_time
                                ) {

                                    Ok((changed_accounts, new_block)) => {

                                        current_time = SystemTime::now()
                                            .duration_since(SystemTime::UNIX_EPOCH)
                                            .unwrap()
                                            .as_secs();

                                        let new_block_bytes: Vec<u8> = new_block.clone().into();

                                        let new_block_message = Message::new(
                                            &new_block_bytes,
                                            &Topic::Block
                                        );

                                        if current_time <= target_time {

                                            while current_time < target_time {

                                                thread::sleep(Duration::from_millis(100));

                                                current_time = SystemTime::now()
                                                    .duration_since(SystemTime::UNIX_EPOCH)
                                                    .unwrap()
                                                    .as_secs();
                                                
                                            }

                                            match relay_clone.lock() {

                                                Ok(relay) => {

                                                    relay.broadcast(&new_block_message);

                                                    for (changed_address, changed_account) in changed_accounts {

                                                        accounts.insert(changed_address, changed_account.details_hash());
        
                                                        accounts_store.put(&changed_address, &changed_account).unwrap();
        
                                                    }
                                                    
                                                    for tx in &new_block.transactions {
        
                                                        pending_transactions.remove(&tx.details_hash);
        
                                                    }
        
                                                    match blocks_store_clone.lock() {
        
                                                        Ok(mut blocks_store) => {
        
                                                            blocks_store.put(&new_block.number, &new_block);
        
                                                        },
        
                                                        Err(_) => (),
        
                                                    }
        
                                                    *latest_block = new_block;
                                                    
                                                }

                                                Err(_) => (),

                                            }

                                        }


                                    },

                                    Err(_) => (),

                                }

                            };
                        },

                        Err(_) => (),
                    }

                    now = Instant::now()

                }

            }

        });

    }

}
