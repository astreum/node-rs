use std::{error::Error, path::Path, fs};

use neutrondb::Store;
use opis::Integer;

use crate::{chain::Chain, address::Address, account::Account, CONSENSUS_ADDRESS, transaction::Transaction};


pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {

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

                    let secret_key_path_str = format!("./keys/{:?}", &sender);

                    let mut tx = Transaction {
                        chain,
                        counter,
                        recipient,
                        sender,
                        value,
                        data: vec![],
                        details_hash: [0; 32],
                        signature: [0; 64],
                    };

                    let secret_key_path = Path::new(&secret_key_path_str);
                    
                    let secret_key = fs::read(secret_key_path)?;

                    tx.sign(secret_key[..].try_into()?);

                    // submit tx

                    Ok(())

                } else {

                    Err("")?

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

                    let secret_key_path_str = format!("./keys/{:?}", &sender);

                    let mut tx = Transaction {
                        chain,
                        counter,
                        data,
                        recipient,
                        sender,
                        value,
                        details_hash: [0; 32],
                        signature: [0; 64],
                    };

                    let secret_key_path = Path::new(&secret_key_path_str);
                    
                    let secret_key = fs::read(secret_key_path)?;

                    tx.sign(secret_key[..].try_into()?);

                    // submit tx

                    Ok(())

                } else {

                    Err("")?

                }
                
            },

            "view" => {
                if args.len() == 5 {
                }

                Ok(())
                
            },

            _ => Err("")?

        }

    } else {
        
        Err("")?

    }

}
