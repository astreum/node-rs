use crate::merkle_tree_hash;
use astro_notation::list;
use std::convert::TryInto;
use opis::Int;
use fides::hash;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Account {
    pub index: usize,
    pub balance: Int,
    pub counter: Int,
    pub storage: HashMap<u8, HashMap<Vec<u8>, Vec<u8>>>
}

impl Account {

    pub fn hash(self) -> [u8; 32] {
        merkle_tree_hash(vec![
            hash(&self.balance.clone().to_ext_bytes(32)),
            hash(&self.counter.clone().to_ext_bytes(32)),
            self.storage_hash()
        ])
    }

    pub fn storage_hash(&self) -> [u8; 32] {
        [0_u8; 32]
    }
    
    pub fn from_astro(input: &str) -> Self {
        
        let decoded: Vec<Vec<u8>> = list::as_bytes(input);

        Account {
            index: usize::from_be_bytes(decoded[0][..].try_into().unwrap()),
            balance: Int::from_bytes(&decoded[1]),
            counter: Int::from_bytes(&decoded[2]),
            storage: HashMap::new()
        }
    }
    
    pub fn to_astro(self) -> String {

        list::from_bytes(vec![
            self.index.to_be_bytes().to_vec(),
            self.balance.to_ext_bytes(32),
            self.counter.to_ext_bytes(32)
        ])
    }
}
