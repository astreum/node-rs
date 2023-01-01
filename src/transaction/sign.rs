use fides::ed25519;
use super::Transaction;

impl Transaction {

    pub fn sign(&mut self, secret_key: &[u8; 32]) {

        let signature = ed25519::sign(&self.details_hash, secret_key);

        self.signature = signature

    }

}
