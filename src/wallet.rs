use std::convert::TryInto;
use astro_notation::{encode, decode};
use fides::{hash, ed25519, chacha20poly1305};
use neutrondb::Store;

pub fn create(args: Vec<String>) {

    if args.len() == 4 {

        let mut app_store: Store = Store::connect("app");

        match app_store.get("priv_key") {
        
            Some(_) => println!("Account ready!"),
        
            None => {
            
                let priv_key: [u8; 32] = ed25519::private_key();
            
                let pub_key: [u8; 32] = ed25519::public_key(&priv_key);
            
                let pass_key: [u8; 32] = hash(&args[3].as_bytes().to_vec());
            
                let encrypted_priv = chacha20poly1305::encrypt(&pass_key, &priv_key.to_vec());
            
                app_store.put("priv_key", &encode::bytes(&encrypted_priv));
            
                app_store.put("pub_key", &encode::bytes(&pub_key.to_vec()));
                
                println!(r###"
Account ready!

Encrypted Key:
{}
{}

Address: {}
                "###,
                &encode::bytes(&encrypted_priv)[..61],
                &encode::bytes(&encrypted_priv)[61..],
                &encode::bytes(&pub_key.to_vec()))
            }
        }
    } else {
        println!("Please provide a password ...");
        help()
    }
}

pub fn key() {

    let app_store = Store::connect("app");

    let priv_key_query = app_store.get("priv_key");

    match priv_key_query {
        
        Some(r) => println!(r###"
Encrypted Key:
{}
{}
        "###,
        r[..61].to_string(),
        r[61..].to_string()),

        None => {
            println!("Account not found!");
            help()
        }

    }

}

pub fn address() {

    let app_store = Store::connect("app");

    let priv_key_query = app_store.get("pub_key");

    match priv_key_query {
        
        Some(r) => println!(r###"
Address: {}
        "###, r),

        None => {
            println!("Account not found!");
            help()
        }

    }

}

pub fn recover(args: Vec<String>) {

    if args.len() == 5 {

        let cipher_message: Vec<u8> = decode::as_bytes(&args[3]);

        let pass_key = hash(&args[4].as_bytes().to_vec());

        let priv_key = chacha20poly1305::decrypt(&pass_key, &cipher_message);
        
        let pub_key = ed25519::public_key(&priv_key.clone().try_into().unwrap());

        let mut app_store = Store::connect("app");

        app_store.put("priv_key", &encode::bytes(&priv_key));

        app_store.put("pub_key", &encode::bytes(&pub_key.to_vec()));
        
        println!("Account ready!");

     } else {
        println!("Please provide an encrypted key & password ...");
        help()
     }

    

}