use std::error::Error;

use fides::{hash::Blake3Hash, merkle_tree, ed25519};
use opis::Integer;

use crate::{address::Address, chain::Chain};

#[derive(Clone, Debug)]
pub struct Transaction {
    pub chain: Chain,
    pub counter: Integer,
    pub data: Vec<u8>,
    pub recipient: Address,
    pub sender: Address,
    pub signature: [u8; 64],
    pub value: Integer,
}

impl Transaction {

    pub fn new(
        chain: Chain,
        counter: Integer,
        data: Vec<u8>,
        recipient: Address,
        sender: Address,
        value: Integer
    ) -> Self {

        Transaction {
            chain,
            counter,
            data,
            recipient,
            sender,
            signature: [0_u8; 64],
            value
        }
    }

    pub fn hash(&self) -> [u8; 32] {

        let details: [Vec<u8>; 6] = [
            (&self.chain).into(),
            (&self.counter).into(),
            self.data.to_vec(),
            (&self.recipient).into(),
            (&self.sender).into(),
            (&self.value).into(),
        ];

        let root: Blake3Hash = merkle_tree::root_from_owned(&details);

        root.into()

    }

    pub fn sign(&mut self, secret_key: &[u8; 32]) {

        let hash = self.hash();

        let signature = ed25519::sign(&hash, secret_key);

        self.signature = signature

    }
    
}

impl Into<Vec<u8>> for &Transaction {
    fn into(self) -> Vec<u8> {
        astro_format::encode_vec(&[
            (&self.chain).into(),
            (&self.counter).into(),
            self.data.to_vec(),
            (&self.recipient).into(),
            (&self.sender).into(),
            (&self.value).into(),
        ])
    }
}

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
            };
            Ok(tx)
        } else {
            Err("Internal error!")?
        }
    }
}