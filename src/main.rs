
use std::{env, error::Error};
use crate::address::Address;
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

const BURN_ADDRESS: Address = Address([0;32]);

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
    "###);

    
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {

        let topic : &str = &args[1];

        match topic {

            "account" => {

                if args.len() >= 3 {

                    let command : &str = &args[2];

                    match command {

                        "new" => app::account_new::run(&args)?,
                        
                        "view" => app::account_view::run(&args)?,

                        _ => Err("")?

                    }

                } else {

                    Err("")?

                }
                
            },

            "transaction" => app::transaction::run(&args)?,

            "block" => {
                if args.len() == 5 {
                    // check local storage
                    // search network
                }
            },

            "stake" => app::stake::run(&args)?,

            "validate" => app::validate::run(&args)?,
            
            "sync" => app::sync::run(&args)?,

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
