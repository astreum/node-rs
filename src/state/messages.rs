use std::{thread, sync::Arc, error::Error};

use opis::Integer;

use crate::{relay::{Topic, Message}, block::Block, transaction::Transaction};

use super::State;

impl State {

    pub fn messages(&self) -> Result<(), Box<dyn Error>> {

        let accounts_clone = Arc::clone(&self.accounts);

        let accounts_store_clone = Arc::clone(&self.accounts_store);

        let blocks_store_clone = Arc::clone(&self.blocks_store);

        let latest_block_clone = Arc::clone(&self.latest_block);

        let pending_transactions_clone = Arc::clone(&self.pending_transactions);

        let relay_clone = Arc::clone(&self.relay);

        thread::spawn(move || {

            let relay = relay_clone.lock().unwrap();
        
            let messages = relay.messages().unwrap();

            loop {

                for (message, peer) in &messages {

                    match message.topic {
                        
                        Topic::Block => {

                            println!("block received ...");

                            match Block::try_from(message.body) {
                                
                                Ok(block) => {

                                    // state.transition(&block);

                                },

                                _ => (),

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

                                                    relay.send(&peer, &block_message);

                                                },

                                                Err(_) => (),
                                                
                                            }

                                        },

                                        _ => ()

                                    }

                                },

                                Err(_) => todo!(),

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

                                                            relay.broadcast(&message);

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
