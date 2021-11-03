
use std::sync::mpsc;
use std::error::Error;
use std::thread;
use std::net::UdpSocket;

use neutrondb::store;

pub struct Network {
    pub routes: Vec<(String, String)>,
    pub receiver: mpsc::Receiver<Vec<u8>>,
}

impl Network {

    pub fn new() -> Result<Network, Box<dyn Error>> {

        let (sender, receiver) = mpsc::channel();

        let mut network = Network { 
            receiver: receiver,
            routes: vec![]
        };

        let routes_store = store("routes")?;

        let routes = routes_store.get_all()?;

        match routes {

            Some(res) => network.routes = res,

            None => network.routes = vec![("node_id".to_string(), "node_add".to_string())]

        }

        thread::spawn(move || {
            
            loop {

                let socket = UdpSocket::bind("127.0.0.1:44444").unwrap();
        
                let mut buf = [0; 3];
        
                let (amt, src) = socket.recv_from(&mut buf).unwrap();
        
                sender.send(buf.to_vec()).unwrap();
        
            }
            
        });

        Ok(network)

    }

}