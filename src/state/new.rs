use std::{error::Error, collections::{BTreeMap, HashMap}};

use neutrondb::Store;
use opis::Integer;

use crate::{chain::Chain, address::Address, account::Account, block::Block, STELAR_ADDRESS, CONSENSUS_ADDRESS, relay::Relay};

use super::State;

impl State {

    pub fn new(chain: Chain) -> Result<Self, Box<dyn Error>> {

        let mut accounts: BTreeMap<Address, [u8;32]> = BTreeMap::new();

        let mut accounts_store: Store<Address, Account> = Store::new(
            &format!("./data/{:?}_accounts",chain)
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

        Ok(State {
            accounts,
            accounts_store,
            latest_block,
        })

    }

}