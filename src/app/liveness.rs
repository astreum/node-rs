pub fn run() {

   if Instant::now().duration_since(live_time).as_secs() > 30 {

      let ping_envelope = Envelope::new(false, ping_message_bytes.clone());

      let ping_envelope_bytes: Vec<u8> = (&ping_envelope).into();

      if self.peers.is_empty() {

         for seeder in &self.seeders {

            let _ = self.outgoing_socket.send_to(&ping_envelope_bytes, &seeder);
 
         }
     
      } else {

         let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

         let mut removable_peers = Vec::new();

         for (peer_address, peer) in &self.peers {
            
            if (t - peer.timestamp) > 330 {

               removable_peers.push(peer_address.clone());
               
               self.consensus_route.remove(&peer_address);
               
               self.peer_route.remove(&peer_address);

             }

             if (t - peer.timestamp) > 300 {

               let _ = self.outgoing_socket.send_to(&ping_envelope_bytes, &SocketAddr::new(*peer_address, peer.incoming_port));

             }

         }

         for removable in removable_peers {

            self.peers.remove(&removable);

         }

     }

   }
   
}