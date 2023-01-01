use std::error::Error;

use neutrondb::Store;

use crate::{chain::Chain, address::Address, account::Account};


pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {

    if args.len() == 5 {

        println!("Account View");

        let chain = Chain::try_from(&args[3][..])?;

        let address = Address::try_from(&args[4][..])?;

        let accounts_store_path = format!("./data/{:?}",chain);

        let accounts_store: Store<Address, Account> = Store::new(&accounts_store_path)?;

        println!("Checking local storage ...");

        let account = accounts_store.get(&address)?;
        // check local storage
        println!("Searching network ...");
        // search network

        Ok(())
        
    } else {

        Err("Arg error!")?
        
    }

}
