use crate::blocks::Block;
use fides::ed25519;
use opis::Int;

impl Block {

    pub fn verify(&self) -> bool {

        if self.number == Int::zero() {

            true
        
        } else {
            
            ed25519::verify(&self.body_hash(), &self.validator, &self.signature)
        
        }
    }

}