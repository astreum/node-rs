
use crate::account::Account;
use crate::address::Address;
use crate::block::Block;
use crate::chain::Chain;
use crate::transaction::Transaction;
use fides::x25519;
use neutrondb::Store;
use opis::Integer;
use rand::Rng;
use std::error::Error;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use std::net::{UdpSocket, IpAddr, SocketAddr};
use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use super::App;
use super::peer::Peer;
use super::ping::Ping;
use super::route::Route;

impl App {

   pub fn new(
      account_address: Address,
      account_key: [u8;32],
      chain: Chain,
      incoming_port: u16,
      validator: bool
   ) -> Result<Self, Box<dyn Error>> {

      let accounts: BTreeMap<Address, [u8;32]> = BTreeMap::new();

      let accounts_store: Store<Address, Account> = Store::new(&format!("./data/{:?}_accounts",chain))?;

      let blocks_store: Store<Integer, Block> = Store::new(&format!("./data/{:?}_blocks", &chain))?;

      let consensus_route = Route::new();

      let incoming_queue = Arc::new(Mutex::new(Vec::new()));

      let incoming_address = format!("127.0.0.1:{}", &incoming_port);

      let incoming_socket = UdpSocket::bind(incoming_address)?;

      let mut latest_block_file = File::open("latest_block")?;

      let mut latest_block_bytes = Vec::new();
    
      latest_block_file.read_to_end(&mut latest_block_bytes)?;

      let latest_block = Block::try_from(latest_block_bytes)?;

      let outgoing_port: u16 = rand::thread_rng().gen_range(49152..65535);

      let outgoing_address = format!("127.0.0.1:{}", outgoing_port);

      let outgoing_socket = UdpSocket::bind(outgoing_address)?;

      let peer_route = Route::new();

      let peers: HashMap<IpAddr, Peer> = HashMap::new();

      let pending_transactions: BTreeMap<[u8; 32], Transaction> = BTreeMap::new();

      

      let relay_key = x25519::secret_key();

      let relay_address_bytes = x25519::public_key(&relay_key);

      let relay_address = Address(relay_address_bytes);

      let ping = Ping::new(&chain, &incoming_port, &relay_address, &validator);

      let seeders_file = File::open("./seeders.txt")?;

      let mut seeders = Vec::new();
   
      for seeder in BufReader::new(seeders_file).lines() {

         let seeder = seeder?;
         
         let socket: SocketAddr = seeder.parse()?;

         seeders.push(socket)

      }

      let app = App {
         accounts,
         accounts_store,
         blocks_store,
         chain,
         consensus_route,
         incoming_queue,
         incoming_socket,
         latest_block,
         outgoing_socket,
         peer_route,
         peers,
         pending_transactions,
         ping,
         seeders,
         validator,
         account_address,
         account_key,
         relay_address,
         relay_key,
      };

      Ok(app)

   }

}
