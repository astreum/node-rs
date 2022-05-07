use crate::transactions::Transaction;
use fides::ed25519;

impl Transaction {
    
    pub fn verify(&self) -> bool {

        ed25519::verify(&self.body_hash(), &self.sender, &self.signature)
        
    }

}
