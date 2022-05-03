use crate::transaction::Transaction;
use opis::Int;

impl Transaction {

    pub fn new() -> Self {
        Transaction {
            chain: Int::zero(),
            counter: Int::zero(),
            recipient: [0_u8; 32],
            sender: [0_u8; 32],
            signature: [0_u8; 64],
            solar_limit: Int::zero(),
            solar_price: Int::zero(),
            value: Int::zero()
        }
    }

}