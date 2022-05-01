use astro_format::string;
use crate::account::Account;
use crate::Chain;
use neutrondb::Store;
use opis::Int;
use std::collections::BTreeMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

pub struct State {
    pub accounts: Arc<Mutex<BTreeMap<[u8;32], [u8;32]>>>,
    pub accounts_store: Arc<Mutex<Store>>
}

impl State {

    pub fn new(chain: &Chain) -> Result<Self, Box<dyn Error>> {

        let mut accounts: BTreeMap<[u8;32], [u8;32]> = BTreeMap::new();

        let mut accounts_store = Store::new(&format!("accounts_{:?}", &chain))?;

        let stored_accounts = accounts_store.get_all()?;

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

        Ok(State {
            accounts: Arc::new(Mutex::new(accounts)),
            accounts_store: Arc::new(Mutex::new(accounts_store))
        })

    }

}
