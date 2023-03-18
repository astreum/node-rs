use std::{time::SystemTime, error::Error};

use fides::{hash::blake_3, merkle_tree::root};
use opis::Integer;

#[derive(Clone, Debug)]
pub struct Envelope {
    pub encrypted: bool,
    pub message: Vec<u8>,
    nonce: u64,
    time: u64
}

impl Envelope {
    
    pub fn new(encrypted: bool, message: Vec<u8>) -> Envelope {

        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        
        let time_bytes: Vec<u8> = Integer::from(&time).into();

        let encrypted_byte = if encrypted { vec![1] } else { vec![0] };

        let mut nonce = 0_u64;

        let mut message_hash = root(
            blake_3,
            &[
                &message,
                &nonce.to_be_bytes(),
                &encrypted_byte,
                &time_bytes
            ]
        );
        
        while message_hash[0] != 0 {

            nonce += 1;

            message_hash = root(
                blake_3,
                &[
                    &message,
                    &nonce.to_be_bytes(),
                    &encrypted_byte,
                    &time_bytes
                ]
            );

        }

        Envelope {
            encrypted,
            message,
            nonce,
            time
        }

    }

}

impl Into<Vec<u8>> for &Envelope {

    fn into(self) -> Vec<u8> {

        astro_format::encode(&[
            if self.encrypted { &[1_u8] } else { &[0_u8] },
            &self.message,
            &self.nonce.to_be_bytes()[..],
            &self.time.to_be_bytes()[..],
        ])

    }

}

impl TryFrom<&[u8]> for Envelope {

    type Error = Box<dyn Error>;

    fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {

        let envelope_fields = astro_format::decode(value)?;

        if envelope_fields.len() == 4 {

            // verify difficulty and time
            
            let result = Envelope {
                encrypted: if envelope_fields[0] == [1] { true } else { false },
                message: envelope_fields[1].to_vec(),
                nonce: u64::from_be_bytes(envelope_fields[2].try_into()?),
                time: u64::from_be_bytes(envelope_fields[3].try_into()?),
            };

            Ok(result)

        } else {
            Err("Envelope fields error!")?
        }
    }
}