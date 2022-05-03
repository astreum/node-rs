use astro_format::arrays;
use crate::transaction::Transaction;
use fides::ed25519;
use std::error::Error;

#[derive(Copy, Clone, Debug)]
pub struct CancelTransaction {
    pub transaction_hash: [u8; 32],
    pub signature: [u8; 64]
}

impl CancelTransaction {
    
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {

        let details = arrays::decode(bytes)?;

        if details.len() ==2 {
            Ok(CancelTransaction {
                transaction_hash: details[0].try_into()?,
                signature: details[1].try_into()?
            })

        } else {
            Err("Cancel transaction error!")?
        }
        
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        arrays::encode(&[
            &self.transaction_hash,
            &self.signature
        ])
    }

    pub fn verify(&self, tx: &Transaction) -> bool {

        ed25519::verify(&self.transaction_hash, &tx.sender, &self.signature)
        
    }
}