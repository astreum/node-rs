use std::collections::HashMap;
use neutrondb::Store;
use crate::account::Account;
use crate::block::Block;
use crate::transaction::Transaction;
use std::sync::{Arc, Mutex};
use pulsar_network::{Network, Route};
use opis::Int;
use crate::{ FIRST_ADDRESS, NOVA_ADDRESS };


#[derive(Debug)]
pub struct State {
    pub accounts: Arc<Mutex<HashMap<[u8; 32], Account>>>,
    pub accounts_store: Store,
    pub blocks_store: Arc<Mutex<Store>>,
    pub current_block: Arc<Mutex<Block>>,
    pub nova_storage: HashMap<[u8; 32], (usize, [u8; 32])>,
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

        let accounts = HashMap::from([
            (NOVA_ADDRESS,
            Account {
                index: 0,
                balance: Int::one(),
                counter: Int::zero(),
                storage: HashMap::from([
                    (1, HashMap::from([
                        (FIRST_ADDRESS.to_vec(), Int::one().to_ext_bytes(32))
                    ])),
                    (2, HashMap::from([
                        (FIRST_ADDRESS.to_vec(), Int::from_decimal("201600").to_ext_bytes(32))
                    ]))
                ])
            }),
            (FIRST_ADDRESS,
            Account {
                index: 1,
                balance: Int::zero(),
                counter: Int::zero(),
                storage: HashMap::new()
            })
        ]);

        let res: State = State {
            accounts: Arc::new(Mutex::new(accounts)),
            accounts_store: Store::connect("accounts"),
            blocks_store: Arc::new(Mutex::new(Store::connect(&format!("blocks_{}", &chain)))),
            current_block: Arc::new(Mutex::new(Block::genesis(block_chain))),
            nova_storage: HashMap::new(),
            pending_transactions: Arc::new(Mutex::new(HashMap::new())),
            network: Arc::new(Mutex::new(network))
        };

        res

    }
}