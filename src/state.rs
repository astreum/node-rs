pub mod new;
pub mod nova;
pub mod sync;
pub mod validate;
use crate::blocks::Block;
use crate::transactions::Transaction;
use neutrondb::Store;
use pulsar_network::Client;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

pub struct State {
    pub accounts: Arc<Mutex<BTreeMap<[u8;32], [u8;32]>>>,
    pub accounts_store: Arc<Mutex<Store>>,
    pub blocks_store: Arc<Mutex<Store>>,
    pub latest_block: Arc<Mutex<Block>>,
    pub pending_transactions: Arc<Mutex<BTreeMap<[u8; 32], Transaction>>>,
    pub network: Arc<Mutex<Client>>  
}
