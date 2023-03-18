use crate::block::Block;
use crate::chain::Chain;
use crate::transaction::Transaction;
use crate::relay::Relay;
use crate::state::State;
use neutrondb::Store;
use opis::Integer;
use std::error::Error;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;
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

        let pending_transactions: Arc<Mutex<BTreeMap<[u8; 32], Transaction>>> = Arc::new(
            Mutex::new(
                BTreeMap::new()
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
            pending_transactions,
            relay,
            state,
        };

        Ok(app)
        
    }

}
