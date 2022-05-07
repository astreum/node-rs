use crate::accounts::Account;
use crate::blocks::Block;
use crate::Chain;
use crate::state::State;
use astro_format::string;
use neutrondb::Store;
use opis::Int;
use pulsar_network::{Route, Client};
use std::collections::BTreeMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

impl State {

    pub fn new(chain: &Chain) -> Result<Self, Box<dyn Error>> {

        let mut accounts: BTreeMap<[u8;32], [u8;32]> = BTreeMap::new();

        let mut accounts_store = Store::new(&format!("accounts_{:?}", &chain))?;

        let blocks_store = Store::new(&format!("blocks_{:?}", &chain))?;

        let stored_accounts = accounts_store.get_all()?;

        let latest_block = Block::new(&chain);

        match stored_accounts.is_empty() {
            
            true => {

                let nova_address = [0_u8;32];

                let stelar_address = [1_u8;32];
                
                let mut nova_account = Account::new();

                nova_account.storage.insert(
                    stelar_address,
                    Int::one().to_ext_bytes(32).try_into().unwrap()
                );

                accounts_store.put(
                    &string::encode::bytes(&nova_address),
                    &string::encode::bytes(&nova_account.to_bytes())
                )?;

                accounts.insert(nova_address, nova_account.hash());

                let stelar_account = Account::new();
                
                accounts_store.put(
                    &string::encode::bytes(&stelar_address),
                    &string::encode::bytes(&stelar_account.to_bytes())
                )?;

                accounts.insert(stelar_address, stelar_account.hash());


            },

            false => {

                for (address, account) in stored_accounts {

                    let address: [u8;32] = string::decode::bytes(&address)?.try_into().unwrap();

                    let account = Account::from_bytes(&string::decode::bytes(&account)?)?;

                    accounts.insert(address, account.hash());

                }

            }

        };

        let route = match chain {
            Chain::Main => Route::Main,
            Chain::Test => Route::Test
        };

        let seeders = vec![];
        
        let network = Client::new(false, route, seeders);

        Ok(State {
            accounts: Arc::new(Mutex::new(accounts)),
            accounts_store: Arc::new(Mutex::new(accounts_store)),
            blocks_store: Arc::new(Mutex::new(blocks_store)),
            latest_block: Arc::new(Mutex::new(latest_block)),
            pending_transactions: Arc::new(Mutex::new(BTreeMap::new())),
            network: Arc::new(Mutex::new(network))
        })

    }

}