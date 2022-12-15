use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use std::{vec, thread};
use std::{env, path::Path, fs, error::Error, time::SystemTime};

use fides::ed25519;
use neutrondb::Store;
use opis::Integer;

use crate::account::Account;
use crate::address::Address;
use crate::chain::Chain;
use crate::transaction::Transaction;
use crate::relay::{Relay, Topic};
mod account;
mod address;
mod chain;
mod transaction;
mod relay;

const CONSENSUS_ADDRESS: Address = Address([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 99, 111, 110, 115, 101, 110, 115, 117, 115]);

fn main() -> Result<(), Box<dyn Error>> {

    println!(r###"
    *      .       *    .               *     .    *          *
    .  .        .           *    .     *  .            .
        *   .      *           *               * .       *    .   .
        .                     *    .    * .            .         .   .   .

     .vvv.    .vvv.  .vvvvv.  .vvvv.   .vvvv.  .v   v.  .v.     .v.
    .v   v.  .v         v     .v   v.  .v      .v   v.  .v v   v v.
    .vvvvv.   .vv.      v     .vvvv.   .vvv.   .v   v.  .v  v v  v.
    .v   v.      v.     v     .v  v.   .v      .v   v.  .v   v   v.
    .v   v.  .vvv.      v     .v   v.  .vvvv.   .vvv.   .v       v.  .v.
    
    Node v0.0.1
    "###);

    
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {

        let topic : &str = &args[1];

        match topic {
            "account" => {
                if args.len() >= 3 {
                    let command : &str = &args[2];
                    match command {
                        "new" => {
                            println!(
                                "[{:?}] : creating account ...",
                                SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis()
                            );
                            let private_key = ed25519::private_key();
                            let public_key = ed25519::public_key(&private_key);
                            let public_key_hex = hex::encode(&public_key);
                            let key_path_string = format!("./keys/{}.fides", public_key_hex);
                            let key_path = Path::new(&key_path_string);
                            fs::write(key_path, &private_key)?;
                            println!(
                                "[{:?}] : account {} created!",
                                SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis(),
                                public_key_hex
                            );
                        },
                        "view" => {
                            if args.len() == 5 {
                                println!("Account View");
                                println!("- - - + - - - + - - -");
                                let chain = Chain::try_from(&args[3][..])?;
                                let address = Address::try_from(&args[4][..])?;
                                let accounts_store: Store<Address, Account> = Store::new(
                                    &format!("./data/{:?}",chain)
                                )?;
                                println!("Checking local storage ...");

                                let account = accounts_store.get(&address)?;
                                // check local storage
                                println!("Searching network ...");
                                // search network
                            }
                        }
                        _ => Err("")?
                    }
                } else {
                    Err("")?
                }
            },
            "transaction" => {

                if args.len() == 7 {

                    let chain = Chain::try_from(&args[3][..])?;

                    let sender = Address::try_from(&args[4][..])?;

                    let accounts_store: Store<Address, Account> = Store::new(
                        &format!("./data/{:?}_accounts",chain)
                    )?;

                    let counter = match accounts_store.get(&sender) {
                        Ok(r) => r.counter,
                        Err(_) => Integer::zero()
                    };

                    let recipient = Address::try_from(&args[5][..])?;

                    let value = Integer::from_dec(&args[6][..])?;

                    let secret_key_path_str = format!("./keys/{:?}.fides", sender);

                    let mut tx = Transaction::new(chain, counter, vec![], recipient, sender, value );

                    let secret_key_path = Path::new(&secret_key_path_str);
                    
                    let secret_key = fs::read(secret_key_path)?;

                    tx.sign(secret_key[..].try_into()?);

                    // submit transaction
                }
            },

            "block" => {
                if args.len() == 5 {
                    // check local storage
                    // search network
                }
            },

            "stake" => {

                if args.len() >= 3 {

                    let command : &str = &args[2];

                    match command {

                        "fund" => {

                            if args.len() == 6 {

                                let chain = Chain::try_from(&args[3][..])?;

                                let sender = Address::try_from(&args[4][..])?;

                                let accounts_store: Store<Address, Account> = Store::new(
                                    &format!("./data/{:?}_accounts",chain)
                                )?;

                                let counter = match accounts_store.get(&sender) {
                                    Ok(r) => r.counter,
                                    Err(_) => Integer::zero()
                                };

                                let value = Integer::from_dec(&args[5][..])?;

                                let recipient = CONSENSUS_ADDRESS;

                                let secret_key_path_str = format!("./keys/{:?}.fides", &sender);

                                let mut tx = Transaction::new(
                                    chain,
                                    counter,
                                    vec![],
                                    recipient,
                                    sender,
                                    value
                                );

                                let secret_key_path = Path::new(&secret_key_path_str);
                                
                                let secret_key = fs::read(secret_key_path)?;

                                tx.sign(secret_key[..].try_into()?);

                                // submit tx

                            }
                        },

                        "withdraw" => {

                            if args.len() == 6 {

                                let chain = Chain::try_from(&args[3][..])?;

                                let sender = Address::try_from(&args[4][..])?;

                                let accounts_store: Store<Address, Account> = Store::new(
                                    &format!("./data/{:?}_accounts",chain)
                                )?;

                                let counter = match accounts_store.get(&sender) {
                                    Ok(r) => r.counter,
                                    Err(_) => Integer::zero()
                                };

                                let data = Integer::from_dec(&args[5][..])?.into();

                                let value = Integer::zero();

                                let recipient = CONSENSUS_ADDRESS;

                                let secret_key_path_str = format!("./keys/{:?}.fides", &sender);

                                let mut tx = Transaction::new(chain, counter, data, recipient, sender, value);

                                let secret_key_path = Path::new(&secret_key_path_str);
                                
                                let secret_key = fs::read(secret_key_path)?;

                                tx.sign(secret_key[..].try_into()?);

                                // submit tx

                            }
                            
                        },

                        "view" => {
                            if args.len() == 5 {
                            }
                        },

                        _ => Err("")?

                    }

                }

            },

            "validate" => {

                println!("Validating ...");

                if args.len() == 4 {

                    let chain = Chain::try_from(&args[3][..])?;

                    let validator = Address::try_from(&args[4][..])?;

                    let mut accounts_store: Store<Address, Account> = Store::new(
                        &format!("./data/{:?}_accounts",chain)
                    )?;

                    let mut blocks_store: Store<Integer, Account> = Store::new(
                        &format!("./data/{:?}_blocks",chain)
                    )?;

                    let seeders_file = File::open(".seeders.txt")?;

                    let seeders = BufReader::new(seeders_file)
                        .lines()
                        .into_iter()
                        .map(|x| (x.unwrap()).parse().unwrap())
                        .collect();

                    let client = Relay::new(chain, false, seeders, true)?;

                    let messages = client.messages()?;

                    // syncing thread
                    thread::spawn( move || {

                        for (message, sender) in messages {

                            match message.topic {
                                
                                Topic::Block => {
                                    
                                },

                                Topic::BlockRequest => {


                                },

                                Topic::Transaction => {

                                },

                                _ => ()

                            }

                        }

                    });

                    thread::spawn( move || {

                        let mut now = Instant::now();

                        loop {

                            if now.elapsed().as_secs() > 1 {

                                // check if validator

                                // if latest block is too old request new

                            }

                            now = Instant::now()
                            
                        }

                    });
                
                } else {

                    println!("Usage is: validate [chain] [address]")

                }

            },
            "sync" => {
                if args.len() == 3 {
                    println!(
                        "[{:?}] : syncing blockchain ...",
                        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis()
                    );
                    loop { }
                }
            },
            _ =>  ()
        }
    } else {
        println!(r###"

    Usage

    - - - + - - - + - - -

    account new                                                 create & store a new private key

    account view [chain] [address]                              shows account information from local & peers 

    transaction new [chain] [address] [recipient] [value]       create, sign & submit a transaction     

    block view [chain] [number]                                 shows block information from local & peers

    stake fund [chain] [address] [value]                        create, sign & submit stake funding transaction

    stake withdraw [chain] [address] [value]                    create, sign & submit stake withdrawl transaction

    stake view [chain] [address]                                shows staking information from local & peers 

    validate [chain] [address]                                  create, sign & submit blocks

    sync [chain]                                                get new blocks & update accounts

        "###);

    }
    
    println!(r###"
    Copyright 12022 HE Astreum Foundation
    
    "###);
    
    Ok(())
    
}
