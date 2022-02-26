use astro_notation::{encode, decode, list};
use fides::{asymmetric, hash};
use neutrondb::Store;
use pulsar_network::{Message, MessageKind, Network, Peer, Routes};
use std::collections::HashMap;
use std::convert::TryInto;
use std::str;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::error::Error;

fn merkle_tree_hash(_input: &Vec<Vec<u8>>) -> [u8;32] {
    [0_u8;32]
}

struct State {
    accounts: HashMap<[u8; 32], [u8; 32]>,
    accounts_store: Store,
    current_block_hash: [u8; 32]
}

impl State {

    pub fn apply_block(&mut self, block: Block) -> Result<(), Box<dyn Error>> {

        let result = true;

        for tx in block.transactions {
            match self.apply_transaction(tx) {
                Ok(_) => (),
                Err(_) => {
                    result = false;
                    break
                }
            }
        }

        if result {
            Ok(())
        } else {
            Err("State transition failed!")?
        }

    } 

    pub fn apply_transaction(&mut self, mut transaction: Transaction) -> Result<(), Box<dyn Error>> {
        
        // get sender account

        // check sender and tx counter are same

        // check sender balance is enough for solar and value transfer

        // get or create (costs Solar) recipient account

        // update sender and recipient accounts

        Ok(())

    }

    pub fn create_block(&mut self, transactions: Vec<Transaction>) -> Block {

        for tx in transactions {
            self.apply_transaction(tx);
        }

        Block {
            accounts_hash: [0_u8; 32],
            previous_block_hash: [0_u8; 32],
            receipts_hash: [0_u8; 32],
            solar_limit: [0_u8; 32],
            solar_price: [0_u8; 32],
            solar_used: [0_u8; 32],
            time: [0_u8; 32],
            transactions_hash: [0_u8; 32],
            transactions: transactions
        }

    }

}

struct Account {
    balance: [u8; 32],
    counter: [u8; 32],
    storage: [u8; 32]
}

impl Account {

    pub fn hash(self) -> [u8; 32] {
        merkle_tree_hash(vec![self.balance.to_vec(), self.counter.to_vec(), self.storage.to_vec()])
    }
    
    pub fn from_astro(input: &str) -> Self {
        
        let decoded = list::as_bytes(input);

        Account {
            balance: decoded[0].try_into().unwrap(),
            counter: decoded[1].try_into().unwrap(),
            storage: decoded[2].try_into().unwrap()
        }

    }
    
    pub fn to_astro(self) -> String {
        list::from_bytes(vec![self.balance.to_vec(), self.counter.to_vec(), self.storage.to_vec()])
    }

}

struct Block {
    accounts_hash: [u8; 32],
    previous_block_hash: [u8; 32],
    receipts_hash: [u8; 32],
    solar_limit: [u8; 32],
    solar_price: [u8; 32],
    solar_used: [u8; 32],
    time: [u8; 32],
    transactions_hash: [u8; 32],
    transactions: Vec<Transaction>
}

impl Block {

    pub fn from_bytes(input: &Vec<u8>) -> Result<Block, Box<dyn Error>> {
        
        if input.len() < 256 {
            Err("Unsupported block format!")?
        } else if input.len() == 256 {

            let block = Block {
                accounts_hash: input[..32].try_into().unwrap(),
                previous_block_hash: input[32..64].try_into().unwrap(),
                receipts_hash: input[64..96].try_into().unwrap(),
                solar_limit: input[96..128].try_into().unwrap(),
                solar_price: input[128..160].try_into().unwrap(),
                solar_used: input[160..192].try_into().unwrap(),
                time: input[192..224].try_into().unwrap(),
                transactions_hash: input[224..256].try_into().unwrap(),
                transactions: Vec::new()
            };

            if block.transactions_hash == hash(&Vec::new()) {
                Ok(block)
            } else {
                Err("Invalid transactions hash!")?
            }

        // } else if (input.len() - 256) % 5 == 0 {

        //     let block = Block {
        //         accounts_hash: input[..32].try_into().unwrap(),
        //         previous_block_hash: input[..32].try_into().unwrap(),
        //         receipts_hash: input[..32].try_into().unwrap(),
        //         solar_limit: input[..32].try_into().unwrap(),
        //         solar_price: input[..32].try_into().unwrap(),
        //         solar_used: input[..32].try_into().unwrap(),
        //         time: input[..32].try_into().unwrap(),
        //         transactions_hash: input[..32].try_into().unwrap(),
        //         transactions: Vec::new()
        //     };

        } else {
            Err("Unsupported block format!")?
        }

    }

    pub fn to_bytes(self) -> Vec<u8> {
        vec![
            self.accounts_hash.to_vec(),
            self.previous_block_hash.to_vec(),
            self.receipts_hash.to_vec(),
            self.solar_limit.to_vec(),
            self.solar_price.to_vec(),
            self.solar_used.to_vec(),
            self.time.to_vec(),
            self.transactions_hash.to_vec(),
        ].concat()
    }
}

struct Transaction {
    counter: [u8; 32],
    recipient: [u8; 32],
    sender: [u8; 32],
    solar_limit: [u8; 32],
    solar_price: [u8; 32],
    value: [u8; 32]
}

impl Transaction {
    
    pub fn verify(input: &str) -> Self {

        let decoded: Vec<Vec<u8>> = list::as_bytes(input);

        let signature = decoded[5];

        // verify tx

        let sender = [0_u8; 32];

        Transaction {
            counter: decoded[0].try_into().unwrap(),
            recipient: decoded[1].try_into().unwrap(),
            sender: sender,
            solar_limit: decoded[2].try_into().unwrap(),
            solar_price: decoded[3].try_into().unwrap(),
            value: decoded[4].try_into().unwrap()
        }

    }

}

pub fn blocks(password: &str, routes: &str ) {

    let mut app_store: Store = Store::connect("app");

    let mut accounts_store: Store = Store::connect("accounts");

    let mut block_store: Store = Store::connect("blocks");

    let mut transaction_store: Store = Store::connect("transactions");

    // let mut active_transactions

    let mut current_block: Block;

    let mut current_block_hash: [u8; 32];

    let mut current_validator: [u8;32];

    let mut pending_transactions: HashMap<[u8; 32], Transaction>;

    // Hashmap<previous block hash, next block hash>
    let mut block_relationship: HashMap<[u8; 32], [u8; 32]>;

    let mut accounts: HashMap<[u8; 32], [u8; 32]>;

    let network_route: Routes = match routes {
        "main validation" => Routes::MainValidation,
        "test validation" => Routes::TestValidation,
        _ => panic!("{} is not a supported route!", routes)    
    };

    let network = Network::config(network_route);

    let messages = network.messages();

    loop {

        for (message, peer) in messages {

            match message.kind {
                
                MessageKind::Block => {

                    let decoded: Vec<Vec<u8>> = list::as_bytes(&message.body);

                    if decoded[1] == current_block_hash.to_vec() {

                        let transactions_hash = merkle_tree_hash(&decoded[9..].to_vec());

                        if decoded[7] == transactions_hash.to_vec() {
                            
                            let block_hash = merkle_tree_hash(&decoded[..8].to_vec());
                            
                            // verify block signature

                            if true {

                                // verify transaction order

                                for tx in &decoded[9..] {

                                    let tx_str: &str = str::from_utf8(tx).unwrap();

                                    let transaction: Transaction = Transaction::verify(tx_str);
                                    
                                }

                            }

                        }

                    }

                },

                MessageKind::CancelTransaction => (),

                MessageKind::NextBlock => {

                    let next_block: [u8; 32] = decode::as_bytes(&message.body).try_into().unwrap();

                    match block_relationship.get(&next_block) {
                        Some(r) => {

                            // get blocks 
                            // get transactions
                            // format and send to asker

                        },
                        None => ()
                    }


                },

                MessageKind::Transaction => {

                    let body_signature: Vec<Vec<u8>> = list::as_bytes(&message.body);

                    let transaction_hash = merkle_tree_hash(&body_signature);

                    // verify tx signature!!!

                    match pending_transactions.get(&transaction_hash) {
                        Some(r) => (),
                        None => {

                            let body_str = str::from_utf8(&body_signature[0]).unwrap();

                            let transaction = Transaction::from_message(body_str);

                            pending_transactions.insert(transaction_hash, transaction);

                            network.broadcast(message)

                        }
                    }
                }

            }

        } 

    }

}