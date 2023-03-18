#[derive(Clone, Debug)]
pub struct Ping {
    pub incoming_port: u16,
    pub public_key: [u8; 32],
    pub validator: bool
}

impl Into<Vec<u8>> for &Ping {

    fn into(self) -> Vec<u8> {

        astro_format::encode(&[
            &self.incoming_port.to_be_bytes()[..],
            &self.public_key[..],
            if self.validator { &[1_u8] } else { &[0_u8] }
        ])

    }

}

impl TryFrom<&[u8]> for Ping {

    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {

        let ping_fields = astro_format::decode(value)?;
        
        if ping_fields.len() == 3 {

            Ok(Ping{
                incoming_port: u16::from_be_bytes(ping_fields[0].try_into()?),
                public_key: ping_fields[1].try_into()?,
                validator: if ping_fields[2] == [1] { true } else { false }
            })
            
        } else {

            Err("Ping fields error!")?

        }

    }

}
