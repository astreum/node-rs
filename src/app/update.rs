use std::{error::Error, sync::Arc, thread, time::{Instant, SystemTime}};

use opis::Integer;

use crate::relay::{Message, Topic};

use super::App;


impl App {

    pub fn update(&self) -> Result<(), Box<dyn Error>> {

        let relay_clone = Arc::clone(&self.relay);

        let state_clone = Arc::clone(&self.state);

        thread::spawn(move || {

            let mut now = Instant::now();

            loop {

                if now.elapsed().as_secs() > 1 {

                    match state_clone.lock() {

                        Ok(state) => {

                            let current_time = SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap()
                                .as_secs();

                            if current_time > (state.latest_block.time + 3) {

                                let next_block_number = &state.latest_block.number + &Integer::one();

                                let next_block_number_bytes: Vec<u8> = next_block_number.into();

                                let next_block_message = Message::new(&next_block_number_bytes, &Topic::BlockRequest);

                                match relay_clone.lock() {
                
                                    Ok(relay) => {

                                        match relay.random_validator() {

                                            Ok(random_validator) => {

                                                relay.send(&random_validator, &next_block_message)
                                            
                                            },

                                            Err(_) => Ok(()),

                                        }
                    
                                    },
                    
                                    Err(_) => Ok(()),
                    
                                };
                                
                            }

                        },

                        Err(_) => (),

                    }

                }

            }

        });

        Ok(())

    }

}
