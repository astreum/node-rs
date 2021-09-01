
use std::error::Error;

use crate::addresses;

use stellar_notation::value_get;

pub fn start() -> Result<(), Box<dyn Error>> {

    print!(r###"

    Mining started ...
    "###);

    let mut store = neutrondb::store("app")?;

    let master_key_query = store.get("master_key")?;

    match master_key_query {
        
        Some(res) => {

            let master_key: Vec<u8> = value_get::as_bytes(res)?;

            let mining_address = addresses::get(master_key, vec![0,0]);

            print!(r###"

    Mining Address: {}
            "###, mining_address);


        },

        None => {
            print!(r###"

    Wallet not created!
            "###);

        }

    }

    Ok(())
}