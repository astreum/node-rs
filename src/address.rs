use std::error::Error;

#[derive(Copy,Clone,Debug)]
#[derive(Ord,PartialEq, PartialOrd,Eq)]
#[derive(Hash)]
pub struct Address(pub [u8; 32]);

impl TryFrom<&str> for Address {
    fn try_from(arg: &str) -> Result<Self, Box<dyn Error>> {
        let address_bytes = hex::decode(arg)?;
        let address_array: [u8; 32] = address_bytes[..].try_into()?;
        Ok(Address(address_array))
    }
    type Error = Box<dyn Error>;
}

impl TryFrom<String> for Address {
    type Error = Box<dyn Error>;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        (&value[..]).try_into()
    }
}

impl TryFrom<&[u8]> for Address {
    type Error = Box<dyn Error>;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let address_array: [u8; 32] = value.try_into()?;
        Ok(Address(address_array))
    }
}

impl TryFrom<Vec<u8>> for Address {
    type Error = Box<dyn Error>;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let address_array: [u8; 32] = value[..].try_into()?;
        Ok(Address(address_array))
    }
}

impl Into<Vec<u8>> for &Address {
    fn into(self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl Into<Vec<u8>> for Address {
    fn into(self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl Into<[u8; 32]> for &Address {
    fn into(self) -> [u8; 32] {
        self.0
    }
}