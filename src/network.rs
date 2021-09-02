
use std::sync::mpsc;
use std::net::UdpSocket;
use std::error::Error;

pub fn listener(sender: mpsc::Sender<Vec<u8>>) -> Result<(), Box<dyn Error>> {

    loop {

        let mut socket = UdpSocket::bind("127.0.0.1:34254")?;

        let mut buf = [0; 10];

        let (amt, src) = socket.recv_from(&mut buf)?;

        sender.send(buf.to_vec())?;

    }

}

// pub fn send() {}

// pub fn ping() {}