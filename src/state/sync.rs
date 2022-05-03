use astro_format::string;
use crate::block::Block;
use crate::State;
use crate::transaction::Transaction;
use crate::transaction::cancel::CancelTransaction;
use opis::Int;
use pulsar_network::{Message, Context};
use std::{sync::Arc, thread};

impl State {

    pub fn sync(&self) {

        println!("syncing ...");

        let accounts_clone = Arc::clone(&self.accounts);

        let blocks_store_clone = Arc::clone(&self.blocks_store);

        let pending_transactions_clone = Arc::clone(&self.pending_transactions);

        let network_clone = Arc::clone(&self.network);

        let latest_block_clone = Arc::clone(&self.latest_block);

        thread::spawn(move || {

            let latest_block = latest_block_clone.lock().unwrap();

            let next_block_number = &latest_block.number + &Int::one();

            drop(latest_block);

            let next_block_message = Message::new(Context::Block, &next_block_number.to_bytes());
            
            let network = network_clone.lock().unwrap();
            
            network.broadcast(next_block_message);

            let messages = network.messages();

            drop(network);

            for (message, peer) in &messages {

                match message.context {
                    
                    Context::Block => {

                        println!("block received from ...");
    
                        match Block::from_bytes(&message.body) {
                            
                            Ok(block) => {

                                let latest_block = latest_block_clone.lock().unwrap();

                                let latest_solar_price = latest_block.solar_price.clone();

                                let current_solar_price = if latest_block.solar_used > Int::from_decimal("750000000") {
                                    latest_solar_price + Int::one()
                                } else if latest_block.solar_used < Int::from_decimal("250000000") {
                                    latest_solar_price - Int::one()
                                } else {
                                    latest_solar_price
                                };

                                if [
                                    block.previous_block_hash == latest_block.hash(),
                                    block.solar_price == current_solar_price
                                    // check validator selection
                                ].iter().all(|&x| x) {

                                    let mut accounts = accounts_clone.lock().unwrap();
                                        
                                    match accounts.apply_block(&block) {
    
                                        Ok(updated) => {

                                            for (address, account) in updated {

                                                let address_string = string::encode::bytes(&address);

                                                let account_bytes = account.to_bytes();

                                                let account_string = string::encode::bytes(&account_bytes);

                                                accounts.store.put(&address_string, &account_string).unwrap();

                                                accounts.details.insert(address, account.hash());

                                            }

                                            let mut blocks_store = blocks_store_clone.lock().unwrap();

                                            let block_number_string = string::encode::bytes(&block.number.to_bytes());

                                            let block_bytes = block.to_bytes();

                                            let block_string = string::encode::bytes(&block_bytes);

                                            blocks_store.put(&block_number_string, &block_string).unwrap();

                                        },
                                        _ => ()
                                    }
                                }
                            },
                            _ => ()
                        }
                    },

                    Context::CancelTransaction => {

                        println!("transaction cancellation ...");
    
                        match CancelTransaction::from_bytes(&message.body) {
    
                            Ok(cancel_transaction) => {

                                let mut pending_transactions = pending_transactions_clone.lock().unwrap();
    
                                match pending_transactions.get(&cancel_transaction.transaction_hash) {
    
                                    Some(tx) => {
                                        
                                        match cancel_transaction.verify(tx) {
                                            
                                            true => {

                                                pending_transactions.remove(&cancel_transaction.transaction_hash);

                                                let network = network_clone.lock().unwrap();

                                                network.broadcast(message);

                                                drop(network)

                                            },
                                            false => ()
                                        }
                                    },
                                    _ => ()
                                }
                            },
                            _ => ()
                        }
    
                    },

                    Context::BlockRequest => {

                        println!("block request ...");

                        let blocks_store = blocks_store_clone.lock().unwrap();
    
                        match blocks_store.get(&string::encode::bytes(&message.body)) {
                            
                            Some(r) => {

                                let message_body = string::decode::bytes(&r).unwrap();

                                let message: Message = Message::new(Context::Block, &message_body);
                                
                                let network = network_clone.lock().unwrap();

                                network.send(message, peer);

                                drop(network)

                            },
                            _ => ()
                        }
                    },
    
                    Context::Transaction => {

                        println!("transaction received ...");
                        
                        match Transaction::from_bytes(&message.body) {
    
                            Ok(tx) => {
    
                                let tx_hash: [u8; 32] = tx.hash();

                                let mut pending_transactions = pending_transactions_clone.lock().unwrap();
    
                                match pending_transactions.get(&tx_hash) {
                
                                    None => {

                                        match true {
                                            
                                            true => {

                                                pending_transactions.insert(tx_hash, tx);
                                                
                                                let network = network_clone.lock().unwrap();

                                                network.broadcast(message);

                                                drop(network)

                                            },
                                            
                                            false => ()
                                        
                                        }
                                    
                                    },
                                    
                                    Some(_) => ()
                                
                                }
                            
                            },
                            
                            Err(_) => ()
                        
                        }
                    
                    }
                
                }
            
            }
        
        });
    
    }

}
