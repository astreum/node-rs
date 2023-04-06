use fides::{merkle_tree, hash::blake_3};
use opis::Integer;

#[derive(Clone, Debug)]
pub struct Receipt {
    pub solar_used: u64,
    pub status: Status
}

impl Receipt {

    pub fn hash(&self) -> [u8; 32] {

        let solar_used_bytes: Vec<u8> = Integer::from(&self.solar_used).into();

        let status_bytes: Vec<u8> = (&self.status).into();
        
        merkle_tree::root(blake_3, &[&solar_used_bytes, &status_bytes])
        
    }

    pub fn new() -> Self {

        Receipt {
            solar_used: 0,
            status: Status::BalanceError
        }

    }
}

#[derive(Clone, Debug)]
pub enum Status {
    Accepted,
    BalanceError,
    SolarError
}

impl Into<Vec<u8>> for &Status {

    fn into(self) -> Vec<u8> {
        match self {
            Status::Accepted => vec![1_u8],
            Status::BalanceError => vec![2_u8],
            Status::SolarError => vec![3_u8]
        }
    }

}
