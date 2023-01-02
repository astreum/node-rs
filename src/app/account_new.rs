use fides::ed25519;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {

    println!("creating account ...");

    let private_key = ed25519::secret_key();

    let public_key = ed25519::public_key(&private_key)?;

    let public_key_hex = hex::encode(&public_key);

    let key_path_string = format!("./keys/{}", public_key_hex);

    let key_path = Path::new(&key_path_string);

    fs::write(key_path, &private_key)?;

    println!("account created!");

    Ok(())

}
