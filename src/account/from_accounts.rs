use std::collections::HashMap;

use astro_format::string;
use neutrondb::Store;

use super::Account;


impl Account {

    pub fn from_accounts(address: &[u8;32], accounts: &HashMap<[u8;32], Account>, store: &Store) -> Option<Account> {

        match accounts.get(address) {
    
            Some(account) => Some(account.clone()),
    
            None => {
    
                let address_string = string::encode::bytes(address);
    
                match store.get(&address_string) {
    
                    Some(encoded) => {
    
                        let bytes = string::decode::bytes(&encoded).unwrap();
    
                        let account = Account::from_bytes(&bytes).unwrap();
    
                        Some(account)
                    },
    
                    None => None
                    
                }
            }
        }
    }
    
}