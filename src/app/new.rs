use std::{error::Error, collections::BTreeMap, fs::File, io::{BufReader, BufRead}, net::SocketAddr, sync::{Arc, Mutex}};

use neutrondb::Store;
use opis::Integer;

use crate::{block::Block, chain::Chain, relay::Relay, state::State};

use super::App;

impl App {

    pub fn new(chain: Chain) -> Result<Self, Box<dyn Error>> {

        let seeders_file = File::open("./seeders.txt")?;

        let mut seeders = Vec::new();
        
        for seeder in BufReader::new(seeders_file).lines() {

            let seeder = seeder?;
            
            let socket: SocketAddr = seeder.parse()?;

            seeders.push(socket)

        }
        
        let relay = Arc::new(Mutex::new(Relay::new(chain.clone(), false, seeders, true)?));

        let blocks_store: Arc<Mutex<Store<Integer, Block>>> = Arc::new(Mutex::new(Store::new(
            &format!("./data/{:?}_blocks", &chain)
        )?));

        let state = Arc::new(Mutex::new(State::new(chain)?));

        let app = App {
            state,
            relay,
            blocks_store,
            pending_transactions: Arc::new(Mutex::new(BTreeMap::new()))
        };

        Ok(app)
        
    }

}