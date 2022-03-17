use std::collections::HashMap;
use neutrondb::Store;
use crate::account::Account;
use crate::block::Block;
use crate::transaction::Transaction;
use std::sync::{Arc, Mutex};
use pulsar_network::{Network, Route};
use opis::Int;
use crate::{ FIRST_ADDRESS, NOVA_ADDRESS };
use crate::NOVA_SLOTS_STORE_ID;
use crate::NOVA_STAKE_STORE_ID;
use std::convert::TryInto;

#[derive(Debug)]
pub struct State {
    pub accounts: Arc<Mutex<HashMap<[u8; 32], Account>>>,
    pub accounts_store: Arc<Mutex<Store>>,
    pub blocks_store: Arc<Mutex<Store>>,
    pub current_block: Arc<Mutex<Block>>,
    pub pending_transactions: Arc<Mutex<HashMap<[u8; 32], Transaction>>>,
    pub network: Arc<Mutex<Network>>
}

impl State {

    pub fn get(chain: &str) -> Self {

        let block_chain: u8 = match chain {
            "main" => 1,
            "test" => 2,
            _ => panic!("{} is not a supported chain!", chain)
        };

        let network_route: Route = match chain {
            "main" => Route::MainValidation,
            "test" => Route::TestValidation,
            _ => panic!("{} is not a supported chain!", chain)
        };

        let network = Network::configure(network_route);

        let nova_stake_store: HashMap<[u8; 32], [u8; 32]> = HashMap::from([
            (FIRST_ADDRESS, Int::one().to_ext_bytes(32).try_into().unwrap())
        ]);

        let nova_slot_store: HashMap<[u8; 32], [u8; 32]> = HashMap::from([
            (FIRST_ADDRESS, Int::from_decimal("201600").to_ext_bytes(32).try_into().unwrap())
        ]);

        let nova_storage = HashMap::from([
            (NOVA_STAKE_STORE_ID, nova_stake_store),
            (NOVA_SLOTS_STORE_ID, nova_slot_store)
        ]);

        let mut nova_account = Account {
            hash: [0_u8; 32],
            balance: Int::one(),
            counter: Int::zero(),
            storage: nova_storage,
            storage_hash: [0_u8; 32]
        };

        nova_account.hash = nova_account.hash();

        nova_account.storage_hash = nova_account.storage_hash();

        let mut first_account = Account {
            hash: [0_u8; 32],
            balance: Int::zero(),
            counter: Int::zero(),
            storage: HashMap::new(),
            storage_hash: [0_u8; 32]
        };

        first_account.hash = first_account.hash();

        let accounts = HashMap::from([
            (NOVA_ADDRESS, nova_account),
            (FIRST_ADDRESS, first_account)
        ]);

        let res: State = State {
            accounts: Arc::new(Mutex::new(accounts)),
            accounts_store: Arc::new(Mutex::new(Store::connect("accounts"))),
            blocks_store: Arc::new(Mutex::new(Store::connect(&format!("blocks_{}", &chain)))),
            current_block: Arc::new(Mutex::new(Block::genesis(block_chain))),
            pending_transactions: Arc::new(Mutex::new(HashMap::new())),
            network: Arc::new(Mutex::new(network))
        };

        res

    }
}