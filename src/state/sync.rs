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

            let next_block_message = Message::new(Context::BlockRequest, &next_block_number.to_bytes());
            
            let network = network_clone.lock().unwrap();

            let nearest_peer = network.nearest_peer();
            
            network.send(next_block_message, nearest_peer);

            let messages = network.messages();

            drop(network);

            for (message, peer) in &messages {

                match message.context {
                    
                    Context::Block => {

                        println!("block received from ...");
    
                        match Block::from_bytes(&message.body) {
                            
                            Ok(block) => {

                                let mut accounts = accounts_clone.lock().unwrap();

                                let latest_block = latest_block_clone.lock().unwrap();

                                match block.apply(&mut accounts, &latest_block) {

                                    Ok(_) => {

                                        let network = network_clone.lock().unwrap();

                                        network.broadcast(message);

                                        let mut blocks_store = blocks_store_clone.lock().unwrap();

                                        let block_key = string::encode::bytes(&block.number.to_bytes());

                                        let block_value = string::encode::bytes(&block.to_bytes());

                                        blocks_store.put(&block_key, &block_value).unwrap();

                                    },
                                    
                                    Err(_) => ()

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
