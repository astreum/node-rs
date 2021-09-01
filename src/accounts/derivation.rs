
use ring::{digest, hmac};

pub fn get(parent_key: Vec<u8>, mut levels: Vec<u32>) -> (Vec<u8>, Vec<u32>) {

    let chain_code = hmac::Key::new(hmac::HMAC_SHA512, &parent_key[32..64]);

    let parent_priv_key: Vec<u8> = parent_key[0..32].to_vec();

    let pop = levels.pop();

    let mut result: (Vec<u8>, Vec<u32>) = (Vec::new(), Vec::new());

    match pop {

        Some(res) => {

            let level_bytes: Vec<u8> = res.to_le_bytes().to_vec();

            let child_key = hmac::sign(&chain_code, &[parent_priv_key, level_bytes].concat());

            let child_key_bytes = child_key.as_ref();

            result = get(child_key_bytes.to_vec(), levels);

        },

        None => {
            result = (parent_key, levels);
        }

    };

    return result

}
