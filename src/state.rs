use std::collections::HashMap;
use neutrondb::Store;
use crate::account::Account;
use crate::block::Block;
use crate::transaction::Transaction;
use std::sync::{Arc, Mutex};
use pulsar_network::{Network, Route};
use opis::Int;
use crate::{FIRST_ADDRESS, NOVA_ADDRESS};
use crate::NOVA_SLOTS_STORE_ID;
use crate::NOVA_STAKE_STORE_ID;
use std::convert::TryInto;
use astro_notation::{encode, decode};
use std::str;

#[derive(Debug)]
pub struct State {
    pub accounts: Arc<Mutex<HashMap<[u8; 32], Account>>>,
    pub accounts_store: Arc<Mutex<Store>>,
    pub blocks_store: Arc<Mutex<Store>>,
    pub latest_block: Arc<Mutex<Block>>,
    pub pending_transactions: Arc<Mutex<HashMap<[u8; 32], Transaction>>>,
    pub network: Arc<Mutex<Network>>
}

impl State {

    pub fn current(chain: &str) -> Self {

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

        

        let mut accounts = HashMap::new();

        let mut accounts_store: Store = Store::connect(&format!("accounts_{}", &chain));

        match accounts_store.get_all() {
            
            Some(r) => {

                for (address, details) in r {

                    let address_array: [u8; 32] = decode::as_bytes(&address).try_into().unwrap();

                    let acc: Account = Account::from_astro(&details);

                    accounts.insert(address_array, acc);

                }
            },

            None => {
                
                let mut nova_account = Account::new(&Int::from_decimal("1000000000000000000000000"));

                let nova_stake_store: HashMap<[u8; 32], [u8; 32]> = HashMap::from([
                    (FIRST_ADDRESS, Int::from_decimal("1000000000000000000000000").to_ext_bytes(32).try_into().unwrap())
                ]);

                let nova_slot_store: HashMap<[u8; 32], [u8; 32]> = HashMap::from([
                    (FIRST_ADDRESS, Int::from_decimal("201600").to_ext_bytes(32).try_into().unwrap())
                ]);

                let nova_storage = HashMap::from([
                    (NOVA_STAKE_STORE_ID, nova_stake_store),
                    (NOVA_SLOTS_STORE_ID, nova_slot_store)
                ]);
                
                nova_account.storage = nova_storage;

                nova_account.storage_hash = nova_account.storage_hash();

                nova_account.hash = nova_account.hash();
                
                let first_account = Account::new(&Int::zero());

                accounts_store.put(&encode::bytes(&NOVA_ADDRESS.to_vec()), &nova_account.to_astro());

                accounts.insert(NOVA_ADDRESS, nova_account);

                accounts_store.put(&encode::bytes(&FIRST_ADDRESS.to_vec()), &first_account.to_astro());

                accounts.insert(FIRST_ADDRESS, first_account);

            }
                

        }

        let mut latest_block: Block = Block::genesis(block_chain);

        let mut block_store: Store = Store::connect(&format!("blocks_{}", &chain));

        match block_store.get("latest_block") {
            Some(r) => latest_block = Block::from_bytes(&decode::as_bytes(&r)).unwrap(),
            None => block_store.put("latest_block", &encode::bytes(&latest_block.to_bytes()))
        }

        let res: State = State {
            accounts: Arc::new(Mutex::new(accounts)),
            accounts_store: Arc::new(Mutex::new(accounts_store)),
            blocks_store: Arc::new(Mutex::new(block_store)),
            latest_block: Arc::new(Mutex::new(latest_block)),
            pending_transactions: Arc::new(Mutex::new(HashMap::new())),
            network: Arc::new(Mutex::new(network))
        };

        res

    }
}