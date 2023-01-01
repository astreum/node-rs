use std::{sync::Arc, thread, time::{Instant, SystemTime}};

use opis::Integer;

use crate::relay::{Message, Topic};

use super::State;


impl State {

    pub fn update(&self) {

        println!("syncing ...");

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

                    match latest_block_clone.lock() {

                        Ok(latest_block) => {

                            let current_time = SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap()
                                .as_secs();

                            if current_time > (latest_block.time + 3) {

                                let next_block_number = &latest_block.number + &Integer::one();

                                let next_block_number_bytes: Vec<u8> = next_block_number.into();

                                let next_block_message = Message::new(&next_block_number_bytes, &Topic::BlockRequest);

                                match relay_clone.lock() {
                
                                    Ok(relay) => {

                                        match relay.random_validator() {

                                            Ok(random_validator) => {

                                                relay.send(&random_validator, &next_block_message)
                                            
                                            },

                                            Err(_) => Ok(()),

                                        }
                    
                                    },
                    
                                    Err(_) => Ok(()),
                    
                                };
                                
                            }

                        },

                        Err(_) => (),

                    }

                }

            }

        });

    }

}
