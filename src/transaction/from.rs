use std::error::Error;

use super::Transaction;

impl TryFrom<&[u8]> for Transaction {

    type Error = Box<dyn Error>;

    fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {

        let tx_decoded = astro_format::decode(value)?;

        if tx_decoded.len() == 6 {

            let tx = Transaction {
                chain: tx_decoded[0].try_into()?,
                counter: tx_decoded[1].try_into()?,
                data: tx_decoded[2].try_into()?,
                recipient: tx_decoded[3].try_into()?,
                sender: tx_decoded[4].try_into()?,
                signature: tx_decoded[5].try_into()?,
                value: tx_decoded[6].try_into()?,
                details_hash: [0; 32],
            };

            Ok(tx)

        } else {

            Err("Internal error!")?

        }

    }

}
