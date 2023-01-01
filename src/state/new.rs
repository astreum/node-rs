use std::{error::Error, collections::{BTreeMap, HashMap}, fs::File, io::{BufReader, BufRead}, net::SocketAddr, sync::{Arc, Mutex}};

use neutrondb::Store;
use opis::Integer;

use crate::{chain::Chain, address::Address, account::Account, block::Block, STELAR_ADDRESS, CONSENSUS_ADDRESS, relay::Relay};

use super::State;

impl State {

    pub fn new(bootstrap: bool, chain: Chain) -> Result<Self, Box<dyn Error>> {

        let mut accounts: BTreeMap<Address, [u8;32]> = BTreeMap::new();

        let mut accounts_store: Store<Address, Account> = Store::new(
            &format!("./data/{:?}_accounts",chain)
        )?;

        let blocks_store: Store<Integer, Block> = Store::new(
            &format!("blocks_{:?}", &chain)
        )?;

        // let stored_accounts = accounts_store.get_all()?;

        let stored_accounts: HashMap<Address, Account> = HashMap::new();

        let latest_block = Block::new(&chain);

        match stored_accounts.is_empty() {
            
            true => {
                
                let mut consensus_account = Account::new();

                consensus_account.storage.insert(
                    (&STELAR_ADDRESS).into(),
                    Integer::one().to_ext_bytes(32).try_into().unwrap()
                );

                accounts_store.put(&CONSENSUS_ADDRESS, &consensus_account)?;

                accounts.insert(CONSENSUS_ADDRESS, consensus_account.details_hash());

                let stelar_account = Account::new();
                
                accounts_store.put(&STELAR_ADDRESS, &stelar_account)?;

                accounts.insert(STELAR_ADDRESS, stelar_account.details_hash());

            },

            false => {

                for (address, account) in stored_accounts {

                    accounts.insert(address, account.details_hash());
                    
                }

            }

        };

        let seeders_file = File::open("./seeders.txt")?;

        let mut seeders = Vec::new();
        
        for seeder in BufReader::new(seeders_file).lines() {

            let seeder = seeder?;
            
            let socket: SocketAddr = seeder.parse()?;

            seeders.push(socket)

        }
        
        let relay = Relay::new(chain, false, seeders, true)?;

        Ok(State {
            accounts: Arc::new(Mutex::new(accounts)),
            accounts_store: Arc::new(Mutex::new(accounts_store)),
            blocks_store: Arc::new(Mutex::new(blocks_store)),
            latest_block: Arc::new(Mutex::new(latest_block)),
            pending_transactions: Arc::new(Mutex::new(BTreeMap::new())),
            relay: Arc::new(Mutex::new(relay))
        })

    }

}