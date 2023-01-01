use std::{error::Error, path::Path, fs};

use crate::{address::Address, chain::Chain, state::State};




pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    
    println!("Validating ...");

    if args.len() == 4 {

        let validator_address = Address::try_from(&args[4][..])?;

        let private_key_path_str = format!("./keys/{:?}", validator_address);

        let private_key_path = Path::new(&private_key_path_str);
        
        let private_key = fs::read(private_key_path)?;

        let chain = Chain::try_from(&args[3][..])?;

        let state = State::new(false, chain)?;
        
        state.sync();

        state.validate(validator_address, private_key[..].try_into()?);

        Ok(())

    } else {

        Err("Usage is: validate [chain] [address]")?

    }

}