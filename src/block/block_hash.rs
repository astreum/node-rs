use fides::hash::blake_3;
use fides::merkle_tree::root;
use super::Block;

impl Block {

    pub fn block_hash(&self) -> [u8; 32] {

        let details_hash_bytes: Vec<u8> = self.details_hash.into();

        let signature_bytes: Vec<u8> = self.signature.into();

        root(
            blake_3,
            &[
                &details_hash_bytes,
                &signature_bytes
            ]
        )

    }

}
