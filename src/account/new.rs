use opis::Integer;
use std::collections::BTreeMap;
use super::Account;

impl Account {

    pub fn new() -> Account {

        Account {
            balance: Integer::zero(),
            counter: Integer::zero(),
            storage: BTreeMap::new(),
            storage_hash: [0_u8; 32],
            details_hash: [0_u8; 32],
        }

    }

}

