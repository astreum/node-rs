use crate::relay::Relay;
use std::sync::Arc;
use std::thread;

impl Relay {

    pub fn incoming(&self) {

        let incoming_queue_clone = Arc::clone(&self.incoming_queue);

        let incoming_socket_clone = Arc::clone(&self.incoming_socket);

        thread::spawn(move || {

            match incoming_socket_clone.lock() {

                Ok(incoming_socket) => {

                    let mut buffer = [0; 32000];

                    loop {

                        match incoming_socket.recv_from(&mut buffer) {

                            Ok((data_length, source)) => {

                                let buffer = &mut buffer[..data_length];

                                match incoming_queue_clone.lock() {

                                    Ok(mut incoming_queue) => incoming_queue.push((buffer.to_vec(), source.ip())),

                                    Err(_) => ()
                                
                                }

                            },

                            Err(_) => (),

                        }

                    }

                },

                Err(_) => ()

            }

        });

    }

}
