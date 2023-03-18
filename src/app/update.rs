use std::{error::Error, sync::Arc, thread, time::{Instant, SystemTime, Duration}};

use opis::Integer;

use crate::relay::{message::Message, topic::Topic};

use super::App;


impl App {

    pub fn update(&self) -> Result<(), Box<dyn Error>> {

        let relay_clone = Arc::clone(&self.relay);

        let state_clone = Arc::clone(&self.state);

        thread::spawn(move || {

            let delay = Duration::from_millis(100);

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

                                println!("currently at block #{:?}, updating ...", &state.latest_block.number);

                                let next_block_number = &state.latest_block.number + &Integer::one();

                                let next_block_number_bytes: Vec<u8> = next_block_number.into();

                                let next_block_message = Message::new(
                                    &next_block_number_bytes,
                                    &Topic::BlockRequest
                                );

                                match relay_clone.lock() {
                
                                    Ok(relay) => {

                                        let consensus_route_clone = relay.consensus_route.clone();

                                        match consensus_route_clone.lock() {

                                            Ok(consensus_route) => {

                                                match consensus_route.sample() {

                                                    Some(sample) => {

                                                        let _ = relay.send(&sample, &next_block_message);

                                                    },

                                                    None => (),
                                                
                                                }
                                            
                                            },

                                            Err(_) => todo!(),

                                        };
                    
                                    },
                    
                                    Err(_) => (),
                    
                                };
                                
                            }

                        },

                        Err(_) => (),

                    }

                    now = Instant::now()

                } else {
                    
                    thread::sleep(delay)
                    
                }

            }

        });

        Ok(())

    }

}
