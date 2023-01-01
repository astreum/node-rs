mod apply;
mod new;
mod consensus;
mod messages;
mod sync;
mod transition;
mod update;
mod validate;
use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::Arc;
use neutrondb::Store;
use opis::Integer;
use crate::transaction::Transaction;
use crate::relay::Relay;
use crate::block::Block;
use crate::account::Account;
use crate::address::Address;

pub struct State {
    pub accounts: Arc<Mutex<BTreeMap<Address, [u8;32]>>>,
    pub accounts_store: Arc<Mutex<Store<Address, Account>>>,
    pub blocks_store: Arc<Mutex<Store<Integer, Block>>>,
    pub latest_block: Arc<Mutex<Block>>,
    pub pending_transactions: Arc<Mutex<BTreeMap<[u8; 32], Transaction>>>,
    pub relay: Arc<Mutex<Relay>>,
}