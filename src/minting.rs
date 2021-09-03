
use crate::addresses;
use crate::network;
use std::thread;
use std::sync::mpsc;
use std::error::Error;
use stellar_notation::{
    StellarObject,
    StellarValue,
    byte_decode,
    list_get,
    value_get
};

struct Block {
    number: u64
}

impl Block {

    fn from_bytes(bytes: Vec<u8>) -> Block {

        let block_objects: Vec<StellarObject> = byte_decode::list(&bytes);

        let mut block: Block = Block {
            number: list_get::as_uint64(block_objects, "number").unwrap()
        };

        return block;

    }
}

pub fn start() -> Result<(), Box<dyn Error>> {

    print!(r###"

    Minting started ...
    "###);

    let mut store = neutrondb::store("app")?;

    let master_key_query = store.get("master_key")?;

    match master_key_query {
        
        Some(res) => {

            let master_key: Vec<u8> = value_get::as_bytes(res)?;

            let mining_address = addresses::get(master_key, vec![0,0]);

            print!(r###"

    Minting Address: {}
            "###, mining_address);

            sync()?;

            // get latest block

            // start listening for new blocks

            let (sender, receiver) = mpsc::channel();

            thread::spawn(move || network::listener(sender).unwrap());

            for received in receiver {

                let decoded_message: StellarObject = byte_decode::object(&received)?;

                match &decoded_message.0[..] {
                    "new_block" => {

                        match decoded_message.1 {
                            StellarValue::Bytes(_) => {

                            },
                            _ => ()
                        }

                    },
                    "new_transaction" => (),
                    _ => ()
                }
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