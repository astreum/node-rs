use std::{error::Error, collections::BTreeMap};

use opis::Integer;
use fides::merkle_tree;
use fides::hash::Blake3Hash;
use fides::hash::blake_3;

#[derive(Clone,Debug)]
pub struct Account {
    pub balance: Integer,
    pub counter: Integer,
    pub storage: BTreeMap<[u8; 32], [u8; 32]>,
    pub storage_hash: [u8; 32]
}

impl Account {

    pub fn new() -> Account {
        Account {
            balance: Integer::zero(),
            counter: Integer::zero(),
            storage: BTreeMap::new(),
            storage_hash: [0_u8; 32]
        }
    }

    pub fn increase_balance(&mut self, amount: &Integer) {

        self.balance += amount 

    }

    pub fn remove_balance(&mut self, amount:&Integer) -> Result<(), Box<dyn Error>> {

        if &self.balance >= amount {

            self.balance -= amount;

            Ok(())

        } else {

            Err("Not enough balance!")?

        }
    }

    pub fn update_counter(&mut self) {

        self.counter += Integer::one()

    }

    pub fn hash(&self) -> [u8;32] {

        let details: [Vec<u8>; 3] = [
            (&self.balance).into(),
            (&self.counter).into(),
            self.storage_hash.into()
        ];

        let root: Blake3Hash = merkle_tree::root_from_owned(&details);

        root.into()
        
    }

    pub fn storage_hash(&self) -> [u8;32] {

        let storage: Vec<Vec<u8>> = self.storage
            .iter()
            .map(|x| [blake_3(x.0), blake_3(x.1)].concat())
            .collect();
        
        let root: Blake3Hash = merkle_tree::root_from_owned(&storage);

        root.into()

    }

    pub fn update_storage_hash(&mut self) {

        self.storage_hash = self.storage_hash()

    }

}

impl Into<Vec<u8>> for Account {

    fn into(self) -> Vec<u8> {

        let storage_bytes: Vec<Vec<u8>> = self.storage
            .into_iter()
            .map(|x| astro_format::encode(&[&x.0, &x.1]))
            .collect();
        
        astro_format::encode_vec(&[
            self.balance.into(),
            self.counter.into(),
            astro_format::encode_vec(&storage_bytes)
        ])

    }
    
}

impl TryFrom<&[u8]> for Account {
    fn try_from(arg: &[u8]) -> Result<Self, Box<dyn Error>> {    
        let decoded_account = astro_format::decode(arg)?;
        if decoded_account.len() == 3 {
            let decoded_storage = astro_format::decode(decoded_account[2])?;
            let mut storage = BTreeMap::new();         
            for i in decoded_storage {              
                let decoded_kv = astro_format::decode(i)?;
                if decoded_kv.len() == 2 {
                    storage.insert(
                        decoded_kv[0].try_into()?,
                        decoded_kv[1].try_into()?
                    );
                }
            }
            let mut result = Account {
                balance: Integer::from(decoded_account[0]),
                counter: Integer::from(decoded_account[1]),
                storage: storage,
                storage_hash: [0_u8;32]
            };
            result.update_storage_hash();
            Ok(result)
        } else {
            Err("Internal error!")?
        }
    }

    type Error = Box<dyn Error>;

}

impl TryFrom<Vec<u8>> for Account {
    type Error = Result<Self, Box<dyn Error>>;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Account::try_from(value)
    }
}