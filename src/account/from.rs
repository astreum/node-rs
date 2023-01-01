use std::{collections::BTreeMap, error::Error};

use opis::Integer;

use super::Account;

impl TryFrom<&[u8]> for Account {

    fn try_from(arg: &[u8]) -> Result<Self, Box<dyn Error>> {

        let account_details = astro_format::decode(arg)?;

        if account_details.len() == 3 {

            let decoded_storage = astro_format::decode(account_details[2])?;

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
                balance: Integer::from(account_details[0]),
                counter: Integer::from(account_details[1]),
                details_hash: [0_u8; 32],
                storage: storage,
                storage_hash: [0_u8; 32]
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
    type Error = Box<dyn Error>;
    fn try_from(value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        Account::try_from(&value[..])
    }
}
