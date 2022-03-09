use astro_notation::{ decode, encode };
use crate::block::Block;
use crate::state::State;
use crate::transaction::{ CancelTransaction, Transaction };
use crate::transform::apply_block;
use opis::Int;
use pulsar_network::{ Message, MessageKind };
use std::sync::Arc;
use std::thread;

impl State {

    pub fn sync(&self) {

        println!("astreuos: syncing ...");

        let accounts_clone = Arc::clone(&self.accounts);

        let blocks_store_clone = Arc::clone(&self.blocks_store);

        let pending_transactions_clone = Arc::clone(&self.pending_transactions);

        let network_clone = Arc::clone(&self.network);

        let current_block_clone = Arc::clone(&self.current_block);

        thread::spawn(move || {

            let current_block = current_block_clone.lock().unwrap();

            let next_block_number = current_block.number.clone() + Int::one();

            drop(current_block);

            let next_block_message: Message = Message::new(MessageKind::NextBlock, next_block_number.to_ext_bytes(32));
            
            let network = network_clone.lock().unwrap();
            
            network.broadcast(next_block_message);

            let messages = network.listen();

            drop(network);

            for (message, peer) in &messages {

                match message.kind {
                    
                    MessageKind::Block => {
    
                        match Block::from_bytes(&message.body) {
                            
                            Ok(block) => {

                                let current_block = current_block_clone.lock().unwrap();
                                
                                if block.previous_block_hash == current_block.hash {

                                    let mut accounts = accounts_clone.lock().unwrap();
                                        
                                    match apply_block(accounts.clone(), block.clone()) {
    
                                        Ok(new_accounts) => {

                                            *accounts = new_accounts;

                                            let mut blocks_store = blocks_store_clone.lock().unwrap();

                                            blocks_store.put(&encode::bytes(&block.hash.to_vec()), &encode::bytes(&message.body));

                                        },
                                        _ => ()
                                    }
                                }
                            },
                            _ => ()
                        }
                    },

                    MessageKind::CancelTransaction => {
    
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

                    MessageKind::NextBlock => {

                        let blocks_store = blocks_store_clone.lock().unwrap();
    
                        match blocks_store.get(&encode::bytes(&message.body)) {
                            
                            Some(r) => {

                                let message: Message = Message::new(MessageKind::Block, decode::as_bytes(&r), );

                                
                                let network = network_clone.lock().unwrap();

                                network.send(message, peer);

                                drop(network)

                            },
                            _ => ()
                        }
                    },
    
                    MessageKind::Transaction => {
                        
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