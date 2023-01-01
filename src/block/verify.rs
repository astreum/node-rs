use fides::ed25519;
use opis::Integer;
use super::Block;

impl Block {

    pub fn verify(&self) -> bool {

        if self.number == Integer::zero() {

            true

        } else { 

            ed25519::verify(
                &self.details_hash,
                &self.validator.0,
                &self.signature
            )  

        }

    }

}
