use astro_format::arrays;
use crate::transaction::Transaction;

impl Transaction {

    pub fn to_bytes(&self) -> Vec<u8> {  
        arrays::encode(&[
            &self.chain.to_bytes(),
            &self.counter.to_bytes(),
            &self.recipient,
            &self.sender,
            &self.signature,
            &self.solar_limit.to_bytes(),
            &self.solar_price.to_bytes(),
            &self.value.to_bytes()
        ])
    }
    
}
