use fides::ed25519;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn run() -> Result<(), Box<dyn Error>> {

    println!("info: creating account");

    let secret_key = ed25519::secret_key();

    let public_key = ed25519::public_key(&secret_key)?;

    let public_key_hex = hex::encode(&public_key);

    let keys_path = Path::new("./keys");

    if !keys_path.exists() {

        fs::create_dir(keys_path)?;
        
    }

    let key_path_string = format!("./keys/{}", public_key_hex);

    let key_path = Path::new(&key_path_string);

    fs::write(key_path, &secret_key)?;

    println!("success: account created for {}", public_key_hex);

    Ok(())

}
