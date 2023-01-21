use rand::Rng;

use crate::address::Address;
use crate::app::App;
use crate::chain::Chain;
use std::env;
use std::error::Error;
use std::path::Path;
use std::fs;
mod account;
mod address;
mod app;
mod block;
mod chain;
mod transaction;
mod receipt;
mod relay;
mod state;

const CONSENSUS_ADDRESS: Address = Address([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 99, 111, 110, 115, 101, 110, 115, 117, 115]);

const STELAR_ADDRESS: Address = Address([0, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 99, 111, 110, 115, 101, 110, 115, 117, 115]);

fn main() -> Result<(), Box<dyn Error>> {

    println!(r###"
    *      .       *    .               *     .    *          *
    .  .        .           *    .     *  .            .
        *   .      *           *               * .       *    .   .
        .                     *    .    * .            .         .   .   .

     .vvv.    .vvv.  .vvvvv.  .vvvv.   .vvvv.  .v   v.  .v.     .v.
    .v   v.  .v         v     .v   v.  .v      .v   v.  .v v   v v.
    .vvvvv.   .vv.      v     .vvv.    .vvv.   .v   v.  .v  v v  v.
    .v   v.      v.     v     .v  v.   .v      .v   v.  .v   v   v.
    .v   v.  .vvv.      v     .v   v.  .vvvv.   .vvv.   .v       v.  .v.
    
    Node v0.0.1

    Copyright © Astreum Foundation Established 12023 HE
    
    "###);

    
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {

        let topic : &str = &args[1];

        match topic {

            "account" => {

                if args.len() >= 3 {

                    let command : &str = &args[2];

                    match command {

                        "new" => app::account_new::run(),
                        
                        "view" => app::account_view::run(&args),

                        _ => Err("")?

                    }

                } else {

                    Err("")?

                }
                
            },

            "transaction" => app::transaction::run(&args),

            // "block" => {

            //     if args.len() == 5 {
            //         // check local storage
            //         // search network
            //         Ok(())

            //     } else {

            //         Err("")?

            //     }

            // },

            "stake" => app::stake::run(&args),

            "validate" => {

                println!("info: validating");

                if args.len() == 4 {

                    let public_key = Address::try_from(&args[4][..])?;
        
                    let secret_key_path_str = format!("./keys/{:?}", public_key);
        
                    let secret_key_path = Path::new(&secret_key_path_str);
                    
                    let secret_key = fs::read(secret_key_path)?;

                    println!("info: key found for {:?}", public_key);
        
                    let chain = Chain::try_from(&args[3][..])?;

                    let app = App::new(
                        &chain,
                        &rand::thread_rng().gen_range(49152..65535),
                        &true
                    )?;
                    
                    app.listen()?;

                    app.update()?;

                    app.validate(public_key, secret_key[..].try_into()?)?;
        
                    Ok(())
        
                } else {
        
                    Err("Usage is: validate [chain] [address]")?
        
                }

            },
            
            // "sync" => {

            //     println!("Syncing ...");

            //     let chain = Chain::try_from(&args[3][..])?;

            //     let app = App::new(chain)?;
                    
            //     app.listen()?;

            //     app.update()?;

            //     Ok(())

            // },

            _ =>  Err("")?

        }

    } else {

        println!(r###"

    Usage

    - - - + - - - + - - -

    account new                                                 create & store a new private key

    account view [chain] [address]                              shows account information in local storage 

    transaction new [chain] [address] [recipient] [value]       create, sign & submit a transaction     

    block view [chain] [number]                                 shows block information from local & peers

    stake fund [chain] [address] [value]                        create, sign & submit stake funding transaction

    stake withdraw [chain] [address] [value]                    create, sign & submit stake withdrawl transaction

    stake view [chain] [address]                                shows staking information from local & peers 

    validate [chain] [address]                                  create, sign & submit blocks

    sync [chain]                                                get new blocks & update accounts

        "###);
        
        Ok(())

    }
    
}
