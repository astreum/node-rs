use astro_format::string;
use fides::{ed25519};
use opis::Int;
use pulsar_network::{Message, Context};
use crate::{state::nova, block::Block};

use super::State;
use std::{sync::Arc, thread, time::{SystemTime, Instant, Duration}};

impl State {

    pub fn validate(&self, private_key: [u8; 32]) {

        println!("validating ...");

        let public_key = ed25519::public_key(&private_key);

        let accounts_clone = Arc::clone(&self.accounts);

        let blocks_store_clone = Arc::clone(&self.blocks_store);

        let latest_block_clone = Arc::clone(&self.latest_block);

        let pending_transactions_clone = Arc::clone(&self.pending_transactions);

        let network_clone = Arc::clone(&self.network);

        thread::spawn(move || {

            let mut now = Instant::now();

            loop {

                if now.elapsed().as_secs() > 1 {

                    let mut blocks_store = blocks_store_clone.lock().unwrap();

                    let mut latest_block = latest_block_clone.lock().unwrap();
                    
                    let mut accounts = accounts_clone.lock().unwrap();

                    let mut current_time = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();

                    let target_time = current_time + (current_time % 3);

                    let validator = nova::validator_selection(&accounts, &latest_block, &Int::from_bytes(&target_time.to_be_bytes()));

                    if validator == public_key {

                        let mut pending_transactions = pending_transactions_clone.lock().unwrap();

                        let (changed_accounts, block) = Block::create(
                            &accounts,
                            &latest_block,
                            &pending_transactions,
                            &private_key,
                            &public_key,
                            &target_time
                        );

                        current_time = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();

                        let new_block_message = Message::new(Context::Block, &block.to_bytes());

                        if current_time <= target_time {

                            while current_time < target_time {

                                thread::sleep(Duration::from_millis(100));

                                current_time = SystemTime::now()
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs();
                                
                            }

                            let network = network_clone.lock().unwrap();

                            network.broadcast(new_block_message);

                            for (address, account) in changed_accounts {

                                accounts.details.insert(address, account.hash());

                                let account_key = string::encode::bytes(&address);

                                let account_value = string::encode::bytes(&account.to_bytes());
                                
                                accounts.store.put(&account_key, &account_value).unwrap();

                            }
                            
                            for tx in &block.transactions {
                                pending_transactions.remove(&tx.hash());
                            }

                            let block_key = string::encode::bytes(&block.number.to_bytes());

                            let block_value = string::encode::bytes(&block.to_bytes());

                            blocks_store.put(&block_key, &block_value).unwrap();
                            
                            *latest_block = block;

                        }

                    };

                    now = Instant::now()

                }
            }
        });
    }

}
