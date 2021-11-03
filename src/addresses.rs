
use std::fmt::Write;

use crate::accounts::derivation;

use ring::{
    rand,
    signature::{self, KeyPair},
};

use sha3::{Digest, Sha3_256};


pub fn get(master_key: Vec<u8>, levels: Vec<u32>) -> String {

    let address_key = derivation::get(master_key, levels);

    let address_priv = &address_key.0[0..32];

    let rng = rand::SystemRandom::new();
    
    let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng).unwrap();

    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).unwrap();

    let sig = key_pair.sign(address_priv);

    let mut hasher = Sha3_256::new();

    hasher.update(sig.as_ref());

    let result = hasher.finalize();

    let mut address = String::from("0X");

    for byte in result {
        write!(&mut address, "{:X}", byte).expect("Unable to write");
    };

    return address

}