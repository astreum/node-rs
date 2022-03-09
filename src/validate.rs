use crate::state::State;
use crate::NOVA_ADDRESS;
use std::thread;
use std::time::{ Instant, SystemTime };
use std::sync::Arc;
use fides::ed25519;
use opis::Int;
use std::convert::TryInto;
use pulsar_network::{ Message, MessageKind };
use crate::transform::accounts_hash;
use crate::block::Block;
// use crate::account::Account;
// use std::collections::HashMap;

impl State {

    pub fn validate(&self, private_key: [u8; 32]) {

        println!("astreuos: validating ...");

        let pub_key: [u8; 32] = ed25519::public_key(&private_key);

        let accounts_clone = Arc::clone(&self.accounts);

        let current_block_clone = Arc::clone(&self.current_block);

        // let pending_transactions_clone = Arc::clone(&self.pending_transactions);

        let network_clone = Arc::clone(&self.network);

        thread::spawn(move || {

            let mut now = Instant::now();

            loop {

                if now.elapsed().as_secs() > 2 {

                    let mut current_block = current_block_clone.lock().unwrap();

                    let current_time: u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

                    if current_time - current_block.time > 3 {

                        let mut accounts = accounts_clone.lock().unwrap();

                        let nova_account = accounts.get(&NOVA_ADDRESS).unwrap();

                        let slot_store = &nova_account.storage.get(&2).unwrap();

                        let mut slot_addresses: Vec<&Vec<u8>> = slot_store.iter().map(|x| x.0).collect();

                        slot_addresses.sort_by(
                            |a, b|
                            (Int::from_bytes(&current_block.hash.to_vec()) ^ Int::from_bytes(a))
                            .cmp(
                                &(Int::from_bytes(&current_block.hash.to_vec()) ^ Int::from_bytes(b))
                            ));

                        let current_validator: [u8; 32] = slot_addresses[0].clone().try_into().unwrap();

                        if current_validator == pub_key {

                            let solar_limit: Int = Int::from_decimal("1000000");

                            // let mut pending_transactions = pending_transactions_clone.lock().unwrap();

                            // for tx in &pending_transactions {

                            // }

                            let mut validator = accounts.get(&pub_key).unwrap().clone();
                            
                            validator.balance += Int::from_decimal("1000000000000000000000000");
                            
                            accounts.insert(pub_key, validator);

                            let mut new_block: Block = Block {
                                accounts_hash: accounts_hash(&accounts),
                                chain: current_block.chain,
                                hash: [0_u8; 32],
                                number: current_block.number.clone() + Int::one(),
                                previous_block_hash: current_block.hash,
                                receipts_hash: [0_u8; 32],
                                signature: [0_u8; 64],
                                solar_price: current_block.clone().solar_price,
                                solar_used: Int::from_decimal("1000000") - solar_limit,
                                time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                                transactions_hash: [0_u8; 32],
                                transactions: Vec::new(),
                                validator: pub_key
                            };

                            println!(" * new block hash: {:?}", new_block.hash());

                            new_block.hash = new_block.hash();

                            new_block.signature = ed25519::sign(&new_block.hash, &private_key, &pub_key);

                            let new_block_message: Message = Message::new(MessageKind::Block, new_block.to_bytes());

                            let network = network_clone.lock().unwrap();

                            network.broadcast(new_block_message);

                            drop(network);
                            
                            *current_block = new_block;

                        }

                    }

                    now = Instant::now()

                }

            }
            
        });
    }
}