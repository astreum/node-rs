use std::convert::TryInto;
use astro_notation::{encode, decode};
use fides::{hash, ed25519, chacha20poly1305};
use neutrondb::Store;

pub fn create(password: &str, repeat: &str) {

    if password == repeat {

        let mut app_store: Store = Store::connect("app");

        let priv_key_query: Option<String> = app_store.get("priv_key");

        match priv_key_query {
            
            Some(_) => println!("Wallet ready!"),
            
            None => {

                println!("Creating wallet ...");
                
                let priv_key: [u8; 32] = ed25519::private_key();

                let pub_key: [u8; 32] = ed25519::public_key(&priv_key);

                let pass_key: [u8; 32] = hash(&password.as_bytes().to_vec());

                let encrypted_priv = chacha20poly1305::encrypt(&pass_key, &priv_key.to_vec());

                app_store.put("priv_key", &encode::bytes(&encrypted_priv));

                app_store.put("pub_key", &encode::bytes(&pub_key.to_vec()));
                
                println!("Wallet ready!");

            }

        }

    }

    else {

        println!(r###"
    Passwords do not match!
        "###)

    }

}

pub fn private() {

    let app_store = Store::connect("app");

    let priv_key_query = app_store.get("priv_key");

    match priv_key_query {
        
        Some(r) => println!(r###"
    ENCRYPTED PRIVATE KEY: {}
        "###, r),

        None => println!(r###"
    No wallet found! Please create one with:
        wt create password
            "###)

    }

}

pub fn address() {

    let app_store = Store::connect("app");

    let priv_key_query = app_store.get("pub_key");

    match priv_key_query {
        
        Some(r) => println!(r###"
    address:
    0x
    {}
        "###, r),

        None => println!(r###"
    No wallet found! Please create one with:
        wt create password
            "###)

    }

}

pub fn recover(cipher_text: &str, password: &str) {

    let cipher_message: Vec<u8> = decode::as_bytes(cipher_text);

    let pass_key = hash(&password.as_bytes().to_vec());

    let priv_key = chacha20poly1305::decrypt(&pass_key, &cipher_message);
    
    let pub_key = ed25519::public_key(&priv_key.clone().try_into().unwrap());

    let mut app_store = Store::connect("app");

    app_store.put("priv_key", &encode::bytes(&priv_key));

    app_store.put("pub_key", &encode::bytes(&pub_key.to_vec()));
    
    println!("Wallet ready!");

}