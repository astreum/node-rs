use astro_format::arrays;
use crate::accounts::Account;
use opis::Int;
use std::collections::BTreeMap;
use std::error::Error;

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

}