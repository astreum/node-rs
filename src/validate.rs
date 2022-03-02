use neutrondb::Store;
use pulsar_network::{Message, MessageKind, Network, Routes};
use std::collections::HashMap;
use std::str;
use crate::block::Block;
use crate::state::State;
use crate::transaction::{CancelTransaction, Transaction};
use astro_notation::{ encode, decode };
use std::thread;
use std::convert::TryInto;

pub fn blocks(password: &str, chain: u8) {

    let mut app_store: Store = Store::connect("app");

    let priv_key = app_store.get("priv_key").unwrap();

    let pub_key = app_store.get("pub_key").unwrap();

    let mut blocks_store: Store = Store::connect("blocks");

    let all_blocks = blocks_store.get_all().unwrap();

    let blocks: Vec<Block> = all_blocks
        .iter()
        .map(|x| Block::from_bytes(&decode::as_bytes(&x.1)).unwrap())
        .collect();

    let mut pending_transactions: HashMap<[u8; 32], Transaction>;

    let mut state: State = State::new(blocks, chain);

    let network_route: Routes = match chain {
        1 => Routes::MainValidation,
        2 => Routes::TestValidation
    };

    let network = Network::config(network_route);

    let messages = network.connect();

    thread::spawn(move || {
        
        let mut current_validator: [u8; 32];

        let private_key: [u8; 32] = decode::as_bytes(&priv_key).try_into().unwrap();

        let public_key: [u8; 32] = decode::as_bytes(&pub_key).try_into().unwrap();

        loop {

            if current_validator == public_key {

            }

        }

    });

    loop {

        for (message, peer) in messages {

            match message.kind {
                
                MessageKind::Block => {

                    match Block::from_bytes(&message.body) {
                        
                        Ok(block) => {

                            if block.previous_block_hash == state.current_block.hash() {
                                
                                match state.transform(block) {

                                    Ok(_) => {

                                        blocks_store.put(&encode::bytes(&block.hash().to_vec()), &encode::bytes(&message.body))

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

                            match pending_transactions.get(&cancel_transaction.transaction_hash) {

                                Some(tx) => {
                                    
                                    match cancel_transaction.verify(*tx) {
                                        
                                        true => {
                                            pending_transactions.remove(&cancel_transaction.transaction_hash);
                                            network.broadcast(message)
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

                    match blocks_store.get(&encode::bytes(&message.body)) {
                        Some(r) => {
                            let message: Message = Message::new(decode::as_bytes(&r), MessageKind::Block);
                            network.send(message, peer)
                        },
                        _ => ()
                    }

                },

                MessageKind::Transaction => {
                    
                    match Transaction::from_bytes(&message.body) {

                        Ok(tx) => {

                            let tx_hash: [u8; 32] = tx.hash();

                            match pending_transactions.get(&tx_hash) {
            
                                None => {

                                    match state.is_tx_applicable(tx) {
                                        true => {
                                            pending_transactions.insert(tx_hash, tx);
                                            network.broadcast(message)
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

    }

}