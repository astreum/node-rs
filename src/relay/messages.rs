use crate::relay::Message;
use crate::relay::Relay;
use std::error::Error;
use std::net::IpAddr;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

impl Relay {
    
    pub fn messages(&self) -> Result<Receiver<(Message, IpAddr)>, Box<dyn Error>> {

        let (sender, receiver): (Sender<(Message, IpAddr)>, Receiver<(Message, IpAddr)>) = channel();

        self.incoming(sender);

        self.outgoing();

        self.listen();

        self.liveness();

        Ok(receiver)

    }

}