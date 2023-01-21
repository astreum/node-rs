use std::error::Error;
use super::Topic;

impl Into<Vec<u8>> for Topic {
    fn into(self) -> Vec<u8> {
        match self {
            Topic::Block => vec![1],
            Topic::BlockRequest => vec![2],
            Topic::Routing => vec![3],
            Topic::Transaction => vec![4],
        }
    }
}


impl TryFrom<&[u8]> for Topic {
    type Error = Box<dyn Error>;

    fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {
        match value {
            [1] => Ok(Topic::Block),
            [2] => Ok(Topic::BlockRequest),
            [3] => Ok(Topic::Routing),
            [4] => Ok(Topic::Transaction),
            _ => Err("Topic value error!")?
        }
    }
}