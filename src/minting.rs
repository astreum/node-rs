
use crate::addresses;
use crate::network::Network;
use std::time::Duration;

use std::error::Error;
use std::thread;
use std::net::UdpSocket;

use stellar_notation::{
    value_decode
};

pub fn run() -> Result<(), Box<dyn Error>> {

    print!(r###"

    Minting started ...
    "###);

    let store = neutrondb::store("app")?;

    let master_key_query = store.get("master_key")?;

    match master_key_query {
        
        Some(res) => {

            let master_key: Vec<u8> = value_decode::as_bytes(&res)?;

            let mining_address = addresses::get(master_key, vec![0,0]);

            print!(r###"

    Minting Address: {}
            "###, mining_address);

    //         // syncronize with the network

            let network = Network::new()?;

            thread::spawn(move || {
                
                for received in network.receiver {
                    println!("Got: {:?}", received);
                }
                
            });

            let socket = UdpSocket::bind("127.0.0.1:34254").unwrap();

        },

        None => {
            print!(r###"

    Wallet not created!
            "###);

        }

    }

    Ok(())
}
