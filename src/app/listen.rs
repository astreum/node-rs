use std::{error::Error, sync::Arc, thread};
use opis::Integer;
use crate::{relay::{Topic, Message}, block::Block, transaction::Transaction};

use super::App;

impl App {

    pub fn listen(&self) -> Result<(), Box<dyn Error>> {

        println!("listening ...");

        let relay_clone = Arc::clone(&self.relay);

        let state_clone = Arc::clone(&self.state);

        let pending_transactions_clone = Arc::clone(&self.pending_transactions);

        let blocks_store_clone = Arc::clone(&self.blocks_store);

        thread::spawn(move || {

            let relay = relay_clone.lock().unwrap();
        
            let messages = relay.messages().unwrap();

            drop(relay);

            loop {

                for (message, peer) in &messages {

                    match message.topic {
                        
                        Topic::Block => {

                            println!("block received ...");

                            match Block::try_from(message.body) {
                                
                                Ok(block) => {

                                    match state_clone.lock() {

                                        Ok(mut state) => {
                                            
                                           match state.transition(&block) {

                                                Ok(_) => println!("block applied!"),

                                                Err(_) => todo!(),
                                            
                                            }
                                        
                                        },
                                        
                                        Err(_) => (),
                                    
                                    }
                                
                                },
                                
                                _ => println!("bad block!"),
                            
                            }
                        
                        },

                        Topic::BlockRequest => {

                            println!("block request ...");

                            let block_number: Integer = message.body.into();

                            match blocks_store_clone.lock() {

                                Ok(blocks_store) => {

                                    match blocks_store.get(&block_number) {
                                
                                        Ok(block) => {

                                            let block_bytes: Vec<u8> = block.into();

                                            let block_message = Message::new(&block_bytes, &Topic::Block);

                                            match relay_clone.lock() {
                                                
                                                Ok(relay) => {

                                                    let _ = relay.send(&peer, &block_message);

                                                },

                                                Err(_) => (),
                                                
                                            }

                                        },

                                        _ => ()

                                    }

                                },

                                Err(_) => (),

                            }

                            
                        },

                        Topic::Transaction => {

                            println!("transaction received ...");
                            
                            match Transaction::try_from(&message.body[..]) {

                                Ok(tx) => {

                                    let tx_hash: [u8; 32] = tx.details_hash;

                                    match pending_transactions_clone.lock() {

                                        Ok(mut pending_transactions) => {

                                            match pending_transactions.get(&tx_hash) {
                    
                                                None => {
                                                    
                                                    pending_transactions.insert(tx_hash, tx);

                                                    match relay_clone.lock() {

                                                        Ok(relay) => {

                                                            let _ = relay.broadcast(&message);

                                                        },

                                                        Err(_) => todo!(),

                                                    }

                                                },

                                                Some(_) => ()

                                            }

                                        },

                                        Err(_) => (),

                                    }
                                    
                                },

                                Err(_) => ()

                            }

                        }
                    
                        _ => ()
                    }

                }

            }

        });

        Ok(())

    }
}