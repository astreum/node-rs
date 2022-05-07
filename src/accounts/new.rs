use crate::accounts::Account;
use opis::Int;
use std::collections::BTreeMap;

impl Account {
    
    pub fn new() -> Self {
        Account {
            balance: Int::zero(),
            counter: Int::zero(),
            storage: BTreeMap::new()
        }
    }
    
}