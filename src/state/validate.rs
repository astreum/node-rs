use astro_format::string;
use fides::{ed25519, merkle_root};
use opis::Int;
use pulsar_network::{Message, Context};
use crate::{NOVA_ADDRESS, state::nova, account::Account, transaction, block::{self, Block}};

use super::State;
use std::{sync::Arc, thread, time::{SystemTime, Instant, Duration}};

impl State {

    pub fn validate(&self, private_key: [u8; 32]) {

        println!("validating ...");

        let public_key: [u8; 32] = ed25519::public_key(&private_key);

        let accounts_clone = Arc::clone(&self.accounts);

        let latest_block_clone = Arc::clone(&self.latest_block);

        let pending_transactions_clone = Arc::clone(&self.pending_transactions);

        let network_clone = Arc::clone(&self.network);

        thread::spawn(move || {

            let mut now = Instant::now();

            loop {

                if now.elapsed().as_secs() > 1 {

                    let mut latest_block = latest_block_clone.lock().unwrap();
                    
                    let accounts = accounts_clone.lock().unwrap();

                    let nova_address_string = string::encode::bytes(&NOVA_ADDRESS);

                    let nova_account_string = accounts.store.get(&nova_address_string).unwrap();

                    let nova_account_bytes = string::decode::bytes(&nova_account_string).unwrap();

                    let nova_account = Account::from_bytes(&nova_account_bytes).unwrap();

                    let validator = nova::select(&latest_block, nova_account.storage);

                    if validator == public_key {

                        let mut current_time = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();

                        let target_time = current_time + (current_time % 3);

                        let pending = pending_transactions_clone.lock().unwrap();

                        let mut new_block = Block::create(&accounts, &latest_block, &pending, &public_key);

                        current_time = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();

                        new_block.time = Int::from_bytes(&current_time.to_be_bytes());

                        new_block.signature = ed25519::sign(&new_block.hash(), &private_key);

                        let new_block_message = Message::new(Context::Block, &new_block.to_bytes());

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

                            // save changed accounts

                            // remove transactions from pending 

                            // store block in neutrondb
                            
                            *latest_block = new_block;

                        }

                    };

                    now = Instant::now()

                }
            }
        });
    }

}
