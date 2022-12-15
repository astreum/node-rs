use crate::relay::Relay;
use std::sync::Arc;
use std::thread;

impl Relay {

    pub fn outgoing(&self) {

        let outgoing_queue_clone = Arc::clone(&self.outgoing_queue);

        let outgoing_socket_clone = Arc::clone(&self.outgoing_socket);

        thread::spawn(move || {

            match outgoing_socket_clone.lock() {

                Ok(outgoing_socket) => {

                    loop {
                        
                        match outgoing_queue_clone.lock() {

                            Ok(mut outgoing_queue) => {

                                match outgoing_queue.pop() {

                                    Some((outgoing_message, outgoing_address)) => {
                                        
                                        let _ = outgoing_socket.send_to(&outgoing_message, &outgoing_address);

                                    },

                                    None => ()

                                }

                            },
                            
                            Err(_) => ()

                        }

                    }

                },

                Err(_) => ()

            }

        });

    }

}
