use std::net::{IpAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run(incoming_queue_clone: Arc<Mutex<Vec<(Vec<u8>, IpAddr)>>>, incoming_socket: UdpSocket) {

    thread::spawn(move || {

        let mut envelope_buffer = [0; 32000];

        loop {

            match incoming_socket.recv_from(&mut envelope_buffer) {

                Ok((envelope_buffer_length, sender_socket_address)) => {

                    let envelope_buffer = &mut envelope_buffer[..envelope_buffer_length];

                    match incoming_queue_clone.lock() {

                        Ok(mut incoming_queue) => {

                            incoming_queue.push((envelope_buffer.to_vec(), sender_socket_address.ip()))

                        },

                        Err(_) => (),

                    }

                },

                Err(_) => (),

            }
            
        }

    });

}
