use astro_format::arrays;
use crate::transactions::Transaction;
use opis::Int;
use std::error::Error;

impl Transaction {

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {

        let details = arrays::decode(bytes)?;

        if details.len() == 8 {
            
            let tx = Transaction {
                chain: Int::from_bytes(details[0]),
                counter: Int::from_bytes(details[1]),
                recipient: details[2].clone().try_into().unwrap_or(Err("Recipient error!")?),
                sender: details[3].clone().try_into().unwrap_or(Err("Sender error!")?),
                signature: details[4].clone().try_into().unwrap_or(Err("Signature error!")?),
                solar_limit: Int::from_bytes(details[5]),
                solar_price: Int::from_bytes(details[6]),
                value: Int::from_bytes(details[7])
            };

            Ok(tx)

        } else {

            Err("Parameters error!")?
            
        }

    }

}
