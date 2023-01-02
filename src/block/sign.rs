use std::error::Error;

use fides::ed25519;
use super::Block;

impl Block {

    pub fn sign(&mut self, secret_key: &[u8; 32]) -> Result<(), Box<dyn Error>> {

        self.signature = ed25519::sign(&self.details_hash, secret_key)?;

        Ok(())

    }
    
}