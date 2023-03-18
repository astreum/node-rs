use std::net::IpAddr;

use rand::Rng;

#[derive(Clone, Debug)]
pub struct Bucket(pub Vec<IpAddr>);

impl Bucket {

    pub fn new() -> Bucket {

        Bucket(vec![])

    }

    pub fn add(&mut self, ip_address: IpAddr) {

        self.0.push(ip_address);
        
    }

    pub fn remove(&mut self, ip_address: &IpAddr) {

        self.0.iter().filter(|&x| x != ip_address);

    }

    pub fn membership(&self, ip_address: &IpAddr) -> bool {

        self.0.contains(ip_address)

    }

    pub fn size(&self) -> usize {

        self.0.len()

    }

    pub fn sample(&self) -> Option<IpAddr> {

        if self.0.is_empty() {

            None

        } else {

            let mut rng = rand::thread_rng();

            let i = rng.gen_range(0..self.0.len());

            Some(self.0[i])

        }
        
    }
    
}
