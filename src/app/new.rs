use std::{error::Error, collections::BTreeMap, sync::{Arc, Mutex}};

use neutrondb::Store;
use opis::Integer;

use crate::{block::Block, chain::Chain, relay::Relay, state::State};

use super::App;

impl App {

    pub fn new(chain: &Chain, incoming_port: &u16, validator: &bool) -> Result<Self, Box<dyn Error>> {

        let blocks_store: Arc<Mutex<Store<Integer, Block>>> = Arc::new(
            Mutex::new(
                Store::new(
                    &format!("./data/{:?}_blocks", &chain)
                )?
            )
        );
        
        let relay = Arc::new(
            Mutex::new(
                Relay::new(
                    incoming_port,
                    validator
                )?
            )
        );

        let state = Arc::new(
            Mutex::new(
                State::new(chain)?
            )
        );

        let app = App {
            blocks_store,
            pending_transactions: Arc::new(
                Mutex::new(
                    BTreeMap::new()
                )
            ),
            relay,
            state,
        };

        Ok(app)
        
    }

}
