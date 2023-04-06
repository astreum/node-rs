use std::error::Error;

#[derive(Clone, Debug)]
pub enum Topic {
    Object,
    ObjectRequest,
    Ping,
    Route,
    RouteRequest,
    State,
    StateRequest,
    Transaction,
}

impl Into<Vec<u8>> for Topic {

    fn into(self) -> Vec<u8> {

        match self {
            Topic::Object => vec![0],
            Topic::ObjectRequest => vec![1],
            Topic::Ping => vec![2],
            Topic::Route => vec![3],
            Topic::RouteRequest => vec![4],
            Topic::State => vec![5],
            Topic::StateRequest => vec![6],
            Topic::Transaction => vec![7],
        }

    }

}


impl TryFrom<&[u8]> for Topic {

    type Error = Box<dyn Error>;

    fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {

        match value {
            [0] => Ok(Topic::Object),
            [1] => Ok(Topic::ObjectRequest),
            [2] => Ok(Topic::Ping),
            [3] => Ok(Topic::Route),
            [4] => Ok(Topic::RouteRequest),
            [5] => Ok(Topic::State),
            [6] => Ok(Topic::StateRequest),
            [7] => Ok(Topic::Transaction),
            _ => Err("topic decoding error!")?
        }

    }

}
