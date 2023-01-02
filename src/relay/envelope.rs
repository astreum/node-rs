use std::{time::SystemTime, error::Error};

use fides::{hash::blake_3, merkle_tree::root};
use opis::Integer;

use super::Envelope;

impl Envelope {
    
    pub fn new(message: Vec<u8>, ping: bool) -> Envelope {

        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        
        let time_bytes: Vec<u8> = Integer::from(&time).into();

        let ping_byte = if ping { vec![1] } else { vec![0] };

        let mut nonce = 0_u64;

        let mut message_hash = root(
            blake_3,
            &[
                &message,
                &nonce.to_be_bytes(),
                &ping_byte,
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
                    &ping_byte,
                    &time_bytes
                ]
            );

        }

        Envelope {
            message,
            nonce,
            ping,
            time
        }

    }

}

impl Into<Vec<u8>> for &Envelope {
    fn into(self) -> Vec<u8> {
        astro_format::encode(&[
            &self.message,
            &self.nonce.to_be_bytes()[..],
            if self.ping { &[1_u8] } else { &[0_u8] },
            &self.time.to_be_bytes()[..]
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
                message: envelope_fields[0].to_vec(),
                nonce: u64::from_be_bytes(envelope_fields[1].try_into()?),
                ping: if envelope_fields[2] == [1] { true } else { false },
                time: u64::from_be_bytes(envelope_fields[3].try_into()?)
            };

            Ok(result)

        } else {
            Err("Envelope fields error!")?
        }
    }
}