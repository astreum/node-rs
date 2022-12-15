use std::{error::Error, fmt};

#[derive(Clone)]
pub enum Chain { Main, Test }

impl fmt::Debug for Chain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Chain::Main => write!(f, "main"),
            Chain::Test => write!(f, "test"),
        }
    }
}

impl TryFrom<&str> for Chain {
    type Error = Box<dyn Error>;
    fn try_from(value: &str) -> Result<Self, Box<dyn Error>> {
        match value {
            "main" => Ok(Chain::Main),
            "test" => Ok(Chain::Test),
            _ => Err("Unknown chain option!")?
        }
    }
}

impl TryFrom<String> for Chain {
    type Error = Box<dyn Error>;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Chain::try_from(&value[..])
    }
}

impl TryFrom<&[u8]> for Chain {
    type Error = Box<dyn Error>;
    fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {
        match value {
            [1] => Ok(Chain::Main),
            [0] => Ok(Chain::Test),
            _ => Err("Unknown chain option!")?
        }
    }
}

impl Into<Vec<u8>> for &Chain {
    fn into(self) -> Vec<u8> {
        match self {
            Chain::Main => vec![1],
            Chain::Test => vec![0]
        }
    }
}
