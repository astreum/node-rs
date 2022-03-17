
use crate::account::accounts_hash;
use crate::block::Block;
use crate::slots;
use crate::state::State;
use crate::NOVA_ADDRESS;
use crate::NOVA_SLOTS_STORE_ID;
use crate::NOVA_STAKE_STORE_ID;
use fides::ed25519;
use opis::Int;
use pulsar_network::{ Message, MessageKind };
use std::thread;
use std::time::{Duration,Instant,SystemTime };
use std::sync::Arc;
use std::collections::HashMap;
use std::convert::TryInto;

impl State {

    pub fn validate(&self, private_key: [u8; 32]) {

        println!("astreuos: validating ...");

        let pub_key: [u8; 32] = ed25519::public_key(&private_key);

        let accounts_clone = Arc::clone(&self.accounts);

        let current_block_clone = Arc::clone(&self.current_block);

        let pending_transactions_clone = Arc::clone(&self.pending_transactions);

        let network_clone = Arc::clone(&self.network);

        thread::spawn(move || {

            let slots_in_epoch: Int = Int::from_decimal("201600");

            let mut now = Instant::now();

            loop {

                if now.elapsed().as_secs() > 1 {

                    let mut current_block_lock = current_block_clone.lock().unwrap();

                    let current_block = current_block_lock.clone();
                    
                    let mut accounts = accounts_clone.lock().unwrap();

                    let mut nova_account = accounts.get(&NOVA_ADDRESS).unwrap().clone();

                    if &(&current_block.number + &Int::one()) % &slots_in_epoch == Int::zero() {
                        
                        let nova_stake_store = nova_account.storage.get(&NOVA_STAKE_STORE_ID).unwrap();

                        let total_stake = nova_stake_store.iter().fold(Int::zero(), |acc, x| acc + Int::from_bytes(&x.1.to_vec()));

                        let mut new_nova_stake_store: HashMap<[u8; 32], [u8; 32]> = HashMap::new();

                        for (key, value) in nova_stake_store {

                            let x_slots = &(&Int::from_bytes(&value.to_vec()) / &total_stake) * &slots_in_epoch;

                            new_nova_stake_store.insert(*key, x_slots.to_ext_bytes(32).try_into().unwrap());

                        }

                        nova_account.storage.insert(NOVA_SLOTS_STORE_ID, new_nova_stake_store).unwrap();

                    }

                    let time_diff = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() - current_block.time;

                    let slots_missed: u64 = if time_diff > 3 {
                        (time_diff - 1) / 3
                    } else {
                        0
                    };

                    let nova_slots_store = nova_account.storage.get(&NOVA_SLOTS_STORE_ID).unwrap();
                    
                    let (current_validator, new_nova_slots_store) = &slots::current_validator(nova_slots_store, slots_missed, current_block.hash);
                        
                    if current_validator == &pub_key {

                        let solar_limit: u32 = 1000000;

                        // apply txs 

                        let mut validator = accounts.get(&pub_key).unwrap().clone();
                        
                        validator.balance += Int::from_decimal("1000000000000000000000000");
                        
                        accounts.insert(pub_key, validator);

                        nova_account.storage.insert(NOVA_SLOTS_STORE_ID, new_nova_slots_store.clone());

                        nova_account.storage_hash = nova_account.storage_hash();

                        nova_account.hash = nova_account.hash();

                        accounts.insert(NOVA_ADDRESS, nova_account);

                        while (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() - current_block.time) % 3 != 0 {
                            thread::sleep(Duration::from_millis(100));
                        }

                        let mut new_block: Block = Block {
                            accounts_hash: accounts_hash(&accounts),
                            chain: current_block.chain,
                            hash: [0_u8; 32],
                            number: current_block.number.clone() + Int::one(),
                            previous_block_hash: current_block.hash,
                            receipts_hash: [0_u8; 32],
                            signature: [0_u8; 64],
                            solar_price: current_block.clone().solar_price,
                            solar_used: 1000000 - solar_limit,
                            time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
                            transactions_hash: [0_u8; 32],
                            transactions: Vec::new(),
                            validator: pub_key
                        };

                        new_block.hash = new_block.body_hash();

                        new_block.signature = ed25519::sign(&new_block.hash, &private_key, &pub_key);

                        let new_block_message: Message = Message::new(MessageKind::Block, new_block.to_bytes());

                        let network = network_clone.lock().unwrap();

                        network.broadcast(new_block_message);
                        
                        *current_block_lock = new_block;

                    };

                    now = Instant::now()

                }
            }
        });
    }
}