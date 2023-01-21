use std::error::Error;

use fides::{ed25519};
use opis::Integer;
use super::Block;

impl Block {

    pub fn verify(&self) ->  Result<bool, Box<dyn Error>> {

        if self.number == Integer::zero() {

            Ok(true)

        } else {

            // verify tx hash

            // verify details hash

            ed25519::verify(&self.details_hash, &self.validator.0, &self.signature)  

        }

    }

}
