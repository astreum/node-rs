use crate::merkle_tree_hash;
use astro_notation::list;
use std::convert::TryInto;
use opis::Int;
use fides::hash;

#[derive(Copy, Clone, Debug)]
pub struct Account {
    pub address: [u8; 32],
    pub balance: [u8; 32],
    pub counter: [u8; 32],
    pub index: usize,
    pub storage: [u8; 32]
}

impl Account {

    pub fn hash(self) -> [u8; 32] {
        merkle_tree_hash(vec![
            hash(&self.balance.to_vec()),
            hash(&self.counter.to_vec()),
            hash(&self.storage.to_vec())
        ])
    }
    
    pub fn from_astro(input: &str) -> Self {
        
        let decoded = list::as_bytes(input);

        Account {
            address: decoded[0].try_into().unwrap(),
            balance: decoded[1].try_into().unwrap(),
            counter: decoded[2].try_into().unwrap(),
            index: usize::from_be_bytes(decoded[3][..].try_into().unwrap()),
            storage: decoded[4].try_into().unwrap()
        }

    }
    
    pub fn to_astro(self) -> String {

        list::from_bytes(vec![
            self.address.to_vec(),
            self.balance.to_vec(),
            self.counter.to_vec(),
            self.index.to_be_bytes().to_vec(),
            self.storage.to_vec()
        ])
    }

    pub fn add_balance(&mut self, value: [u8; 32]) {

        let bal: Int = Int::from_bytes(&self.balance.to_vec()) + Int::from_bytes(&value.to_vec());

        self.balance = bal.to_ext_bytes(32).try_into().unwrap();

    }

    pub fn remove_balance(&mut self, value: Int) {

        let bal: Int = Int::from_bytes(&self.balance.to_vec()) - value;

        self.balance = bal.to_ext_bytes(32).try_into().unwrap();

    }

}