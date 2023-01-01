use rand::Rng;
use super::Relay;
use std::error::Error;
use std::net::IpAddr;
use std::sync::Arc;

impl Relay {

    pub fn random_validator(&self) -> Result<IpAddr, Box<dyn Error>> {
        
        let consensus_route_clone = Arc::clone(&self.consensus_route);

        let consensus_route_clone_lock = consensus_route_clone.lock();

        match consensus_route_clone_lock {

            Ok(consensus_route) => {

                let mut rng = rand::thread_rng();

                let random_bucket = rng.gen_range(0..consensus_route.len());

                let mut i = 0;

                let mut res: Result<IpAddr, Box<dyn Error>> = Err("Internal error!")?;

                for (_,bucket) in consensus_route.iter() {

                    if i == random_bucket {

                        let random_peer = rng.gen_range(0..bucket.len() as u8);

                        let validator = bucket.get(&random_peer).unwrap_or(Err("Internal error!")?);

                        res = Ok(validator.ip())

                    } else {

                        i += 1

                    }
                    
                }

                res

            },

            Err(_) => Err("Internal error!")?,

        }
        
    }

}
