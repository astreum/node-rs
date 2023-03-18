use std::{sync::{Arc, Mutex}, collections::BTreeMap};
use neutrondb::Store;
use opis::Integer;
use crate::block::Block;
use crate::transaction::Transaction;
use crate::state::State;
use crate::relay::Relay;
pub mod stake;
pub mod listen;
mod new;
pub mod transaction;
pub mod update;
pub mod validate;

#[derive(Debug)]
pub struct App {
    blocks_store: Arc<Mutex<Store<Integer, Block>>>,
    pending_transactions: Arc<Mutex<BTreeMap<[u8; 32], Transaction>>>,
    pub relay: Arc<Mutex<Relay>>,
    state: Arc<Mutex<State>>,
}
