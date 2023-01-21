use std::error::Error;

use neutrondb::Store;

use crate::{chain::Chain, address::Address, account::Account};


pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {

    if args.len() == 5 {

        println!("info: viewing account");

        let chain = Chain::try_from(&args[3][..])?;

        let address = Address::try_from(&args[4][..])?;

        let accounts_store_path = format!("./data/{:?}_accounts",chain);

        let accounts_store: Store<Address, Account> = Store::new(&accounts_store_path)?;

        match accounts_store.get(&address) {

            Ok(account) => println!(
                "success: {} solar and {} transactions",
                account.balance.to_dec(),
                account.counter.to_dec()
            ),

            Err(_) => println!("success: 0 solar and 0 transactions"),

        };

        Ok(())
        
    } else {

        Err("Arg error!")?
        
    }

}
