use std::error::Error;
use std::sync::Arc;
use std::thread;
use std::time::{Instant, SystemTime, Duration};
use crate::address::Address;
use crate::relay::{Message, Topic};
use super::App;


impl App {

    pub fn validate(&self, public_key: Address, secret_key: [u8;32]) -> Result<(), Box<dyn Error>> {

        println!("validating ...");

        let relay_clone = Arc::clone(&self.relay);

        let state_clone = Arc::clone(&self.state);

        let blocks_store_clone = Arc::clone(&self.blocks_store);

        let pending_transactions_clone = Arc::clone(&self.pending_transactions);

        thread::spawn(move || {

            let mut now = Instant::now();

            loop {

                if now.elapsed().as_secs() > 1 {

                    match state_clone.lock() {

                        Ok(mut state) => {

                            let mut current_time = SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap()
                                .as_secs();

                            let time_diff = current_time - &state.latest_block.time;

                            let target_time = if time_diff > 3 {

                                current_time

                            } else {

                                &state.latest_block.time + 3

                            };

                            match state.validator(&current_time) {

                                Ok((validator, mut changed_accounts)) => {

                                    if validator == public_key {

                                        match pending_transactions_clone.lock() {

                                            Ok(mut pending_transactions) => {

                                                match blocks_store_clone.lock() {

                                                    Ok(mut blocks_store) => {

                                                        match state.create_block(
                                                            &blocks_store,
                                                            &mut changed_accounts,
                                                            &pending_transactions,
                                                            &public_key,
                                                            &secret_key,
                                                            &target_time
                                                        ) {

                                                            Ok(new_block) => {

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

                                                                            let _ = relay.broadcast(&new_block_message);

                                                                            for (changed_address, changed_account) in changed_accounts {

                                                                                state.accounts.insert(changed_address, changed_account.details_hash());

                                                                                state.accounts_store.put(&changed_address, &changed_account).unwrap();

                                                                            }

                                                                            for tx in &new_block.transactions {

                                                                                pending_transactions.remove(&tx.details_hash);

                                                                            }

                                                                            let _ = blocks_store.put(&new_block.number, &new_block);

                                                                            state.latest_block = new_block;

                                                                        }

                                                                        Err(_) => (),

                                                                    }

                                                                }


                                                            },

                                                            Err(_) => (),

                                                        }


                                                    },

                                                    Err(_) => (),

                                                }

                                            },

                                            Err(_) => (),
                                        }
                                    }
                                },
                                Err(_) => (),

                            }
                        },
                        Err(_) => (),
                    }

                    now = Instant::now();

                }

                thread::sleep(Duration::from_millis(100));

            }

        });

        Ok(())

    }

}
