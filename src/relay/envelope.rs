use std::{time::SystemTime, error::Error};

use fides::{hash::Blake3Hash, merkle_tree::root_from_owned};

use super::Envelope;

impl Envelope {
    
    pub fn new(message: Vec<u8>, ping: bool) -> Envelope {

        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

        let mut nonce = 0_u64;

        let mut details = vec![
            message.clone(),
            nonce.to_be_bytes().to_vec(),
            if ping { vec![1] } else { vec![0] },
            time.to_be_bytes().to_vec()
        ];

        let mut message_hash: Blake3Hash = root_from_owned(&details);
        
        while message_hash.0[0] != 0 {

            nonce += 1;

            details[1] = nonce.to_be_bytes().to_vec();

            message_hash = root_from_owned(&details);

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