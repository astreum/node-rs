use fides::{merkle_root, hash};
use opis::Int;

#[derive(Clone, Debug)]
pub struct Receipt {
    pub solar_used: Int,
    pub status: Status
}

impl Receipt {

    pub fn hash(&self) -> [u8;32] {
        merkle_root(vec![
            hash(&self.solar_used.to_bytes()),
            hash(&self.status.to_bytes())
        ])
    }

    pub fn new() -> Self {
        Receipt { solar_used: Int::zero(), status: Status::BalanceError }
    }
}

#[derive(Clone, Debug)]
pub enum Status {
    Accepted,
    BalanceError,
    SolarError
}

impl Status {

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Status::Accepted => vec![1_u8],
            Status::BalanceError => vec![2_u8],
            Status::SolarError => vec![3_u8]
        }
    }

}