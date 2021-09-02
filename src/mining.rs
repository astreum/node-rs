
use crate::addresses;
use crate::network;
use std::thread;
use std::sync::mpsc;
use std::error::Error;
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

            sync()?;

            // get latest block

            // start listening for new blocks

            let (sender, receiver) = mpsc::channel();

            thread::spawn(move || network::listener(sender).unwrap());

            for received in receiver {

                // if block number == next block
                    // validate block

                // if block number > 
            }

        },

        None => {
            print!(r###"

    Wallet not created!
            "###);

        }

    }

    Ok(())
}

pub fn sync() -> Result<(), Box<dyn Error>> {
    Ok(())
}