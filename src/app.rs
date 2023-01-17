use std::{sync::{Arc, Mutex}, collections::BTreeMap};

use neutrondb::Store;
use opis::Integer;

use crate::{state::State, relay::Relay, block::Block, transaction::Transaction};

pub mod account_new;
pub mod account_view;
pub mod stake;
pub mod listen;
mod new;
pub mod transaction;
pub mod update;
pub mod validate;

pub struct App {
    state: Arc<Mutex<State>>,
    relay: Arc<Mutex<Relay>>,
    blocks_store: Arc<Mutex<Store<Integer, Block>>>,
    pending_transactions: Arc<Mutex<BTreeMap<[u8; 32], Transaction>>>,
}