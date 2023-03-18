use std::{collections::HashMap, net::IpAddr, error::Error};

use super::bucket::Bucket;

use rand::Rng;


#[derive(Clone, Debug)]
pub struct Route(pub HashMap<String, Bucket>);

impl Route {

    pub fn new() -> Route {

        Route(HashMap::new())
    
    }

    pub fn bucket(&self, id: &str) -> Result<String, Box<dyn Error>> {

        if id.len() == 256 {

            let mut result = String::new();

            for i in 1..=256 {

                let bucket_key = &id[0..i];

                match self.0.get(bucket_key) {

                    Some(bucket) => {

                        if bucket.size() < 256 {

                            result = bucket_key.to_string();

                            break;

                        }

                    },

                    None => {

                        result = bucket_key.to_string();

                        break;
                    
                    },

                }
            
            }

            Ok(result)

        } else {

            Err("internal error!")?

        }

    }

    pub fn add(&mut self, ip_address: &IpAddr, public_key: &str) -> Result<(), Box<dyn Error>> {

        let bucket_key = self.bucket(public_key)?;

        match self.0.get_mut(&bucket_key) {

            Some(bucket) => {
                
                bucket.add(*ip_address)
            
            },

            None => {

                let mut bucket = Bucket::new();

                bucket.add(*ip_address);

                self.0.insert(bucket_key, bucket);

            }

        };

        Ok(())

    }

    pub fn remove(&mut self, ip_address: &IpAddr) {
        
        for (_, bucket) in &mut self.0 {

            bucket.remove(ip_address)
            
        }

    }

    pub fn samples(&self) -> Vec<IpAddr> {

        let mut result = Vec::new();

        for (_,bucket) in &self.0 {

            let sample_option = bucket.sample();

            match sample_option {

                Some(sample) => result.push(sample),

                None => (),

            }

        }

        result

    }

    pub fn sample(&self) -> Option<IpAddr> {

        let samples = self.samples();

        if samples.is_empty() {

            None

        } else {

            let mut rng = rand::thread_rng();

            let i = rng.gen_range(0..samples.len());

            Some(samples[i])

        }

    }

}
