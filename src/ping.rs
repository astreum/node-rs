use crate::{chain::Chain, address::Address};

#[derive(Clone, Debug)]
pub struct Ping {
    pub chain: Chain,
    pub incoming_port: u16,
    pub public_address: Address,
    pub validator: bool
}

impl Ping {

    pub fn new(
        chain: &Chain,
        incoming_port: &u16,
        public_address: &Address,
        validator: &bool
    ) -> Self {
        Ping {
            chain: chain.clone(),
            incoming_port: incoming_port.clone(),
            public_address: public_address.clone(),
            validator: validator.clone()
        }
    }
}

impl Into<Vec<u8>> for &Ping {

    fn into(self) -> Vec<u8> {

        let chain_bytes: Vec<u8> = (&self.chain).into();

        astro_format::encode(&[
            &chain_bytes,
            &self.incoming_port.to_be_bytes()[..],
            &self.public_address.0[..],
            if self.validator { &[1_u8] } else { &[0_u8] }
        ])

    }

}

impl TryFrom<&[u8]> for Ping {

    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {

        let ping_fields = astro_format::decode(value)?;
        
        if ping_fields.len() == 4 {

            let chain: Chain = Chain::try_from(ping_fields[0])?;

            let validator = match ping_fields[3] {
                [0] => false,
                [1] => true,
                _ => Err("Validator details error!")?
            };

            Ok(Ping{
                chain,
                incoming_port: u16::from_be_bytes(ping_fields[1].try_into()?),
                public_address: ping_fields[2].try_into()?,
                validator,
            })
            
        } else {

            Err("Ping fields error!")?

        }

    }

}
