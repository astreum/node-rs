use astro_format::arrays;
use fides::{hash, merkle_root};
use opis::Int;
use std::collections::BTreeMap;
use std::error::Error;

#[derive(Debug)]
pub struct Account {
    pub balance: Int,
    pub counter: Int,
    pub storage: BTreeMap<[u8;32], [u8;32]>
}

impl Account {

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {

        let details = arrays::decode(bytes)?;

        if details.len() == 3 {

            let storage = {

                let mut k_vs = BTreeMap::new();

                let enc_k_vs = arrays::decode(details[2])?;

                for enc_k_v in enc_k_vs {

                    let dec_k_v = arrays::decode(enc_k_v)?;

                    k_vs.insert(dec_k_v[0].try_into().unwrap(), dec_k_v[1].try_into().unwrap());
                    
                }

                k_vs

            };

            Ok(Account{
                balance: Int::from_bytes(details[0]),
                counter: Int::from_bytes(details[1]),
                storage: storage
            })

        } else {
            Err("Internal error!")?
        }
    }
    
    pub fn hash(&self) -> [u8;32] {
        merkle_root(vec![
            hash(&self.balance.to_bytes()),
            hash(&self.counter.to_bytes()),
            self.storage_hash()
        ])
    }

    pub fn new() -> Self {
        Account {
            balance: Int::zero(),
            counter: Int::zero(),
            storage: BTreeMap::new()
        }
    }
    
    pub fn storage_hash(&self) -> [u8;32] {
        merkle_root(
            self.storage
                .iter()
                .map(|x| {
                    let joined = [hash(x.0), hash(x.1)].concat();
                    hash(&joined[..])
                })
                .collect()
        )
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {

        let encoded_key_values = self.storage
            .iter()
            .map(|(k, v)| arrays::encode(&[k, v]))
            .collect::<Vec<_>>();

        let mut storage_bytes: Vec<&[u8]> = Vec::new();

        for i in 0..encoded_key_values.len() {
            storage_bytes.push(&encoded_key_values[i]);
        }

        arrays::encode(&[
            &self.balance.to_bytes(),
            &self.counter.to_bytes(),
            &arrays::encode(&storage_bytes)
        ])

    }

}