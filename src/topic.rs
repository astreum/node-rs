use std::error::Error;

#[derive(Clone, Debug)]
pub enum Topic {
    Ping,
    RouteRequest,
    Route,
    BlockRequest,
    Block,
    Transaction,
}

impl Into<Vec<u8>> for Topic {

    fn into(self) -> Vec<u8> {

        match self {
            Topic::Ping => vec![1],
            Topic::RouteRequest => vec![2],
            Topic::Route => vec![3],
            Topic::BlockRequest => vec![4],
            Topic::Block => vec![5],
            Topic::Transaction => vec![6],
        }

    }

}


impl TryFrom<&[u8]> for Topic {

    type Error = Box<dyn Error>;

    fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {

        match value {
            [1] => Ok(Topic::Ping),
            [2] => Ok(Topic::RouteRequest),
            [3] => Ok(Topic::Route),
            [4] => Ok(Topic::BlockRequest),
            [5] => Ok(Topic::Block),
            [6] => Ok(Topic::Transaction),
            _ => Err("topic decoding error!")?
        }

    }

}
