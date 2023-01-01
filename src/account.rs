mod balance;
mod details_hash;
mod from;
mod into;
mod new;
mod storage_hash;
use std::collections::HashMap;
use std::{error::Error, collections::BTreeMap};

use neutrondb::Store;
use opis::Integer;

use crate::address::Address;

#[derive(Clone,Debug)]
pub struct Account {
    pub balance: Integer,
    pub counter: Integer,
    pub details_hash: [u8; 32],
    pub storage: BTreeMap<Vec<u8>, Vec<u8>>,
    pub storage_hash: [u8; 32],
}

impl Account {

    pub fn from_accounts(

        address: &Address,
        changed_accounts: &HashMap<Address, Account>,
        accounts_store: &Store<Address, Account>
    
    ) -> Result<Account, Box<dyn Error>>
    
    {

        match changed_accounts.get(address) {
    
            Some(account) => Ok(account.clone()),
    
            None => {
                
                accounts_store.get(address)

            },

        }

    }

    

    

    pub fn increase_counter(&mut self) {

        self.counter += Integer::one()

    }

    pub fn update_storage_hash(&mut self) {

        self.storage_hash = self.storage_hash()

    }

}



