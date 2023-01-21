use neutrondb::Store;
use opis::Integer;
use rand::Rng;
use std::error::Error;
use std::fs;
use std::path::Path;
use crate::chain::Chain;
use crate::address::Address;
use crate::account::Account;
use crate::relay::Message;
use crate::relay::Topic;
use crate::relay::Relay;
use crate::transaction::Transaction;

pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {

    if args.len() == 7 {

        let chain = Chain::try_from(&args[3][..])?;

        let sender = Address::try_from(&args[4][..])?;

        let accounts_store: Store<Address, Account> = Store::new(
            &format!("./data/{:?}_accounts",chain)
        )?;

        let counter = match accounts_store.get(&sender) {
            Ok(r) => r.counter,
            Err(_) => Integer::zero()
        };

        let recipient = Address::try_from(&args[5][..])?;

        let value = Integer::from_dec(&args[6][..])?;

        let secret_key_path_str = format!("./keys/{:?}", sender);

        let mut tx = Transaction {
            chain: chain.clone(),
            counter,
            recipient,
            sender,
            value,
            data: vec![],
            details_hash: [0; 32],
            signature: [0; 64],
        };

        tx.details_hash = tx.details_hash();

        let secret_key_path = Path::new(&secret_key_path_str);
        
        let secret_key = fs::read(secret_key_path)?;

        tx.sign(secret_key[..].try_into()?)?;

        let tx_bytes: Vec<u8> = tx.into();
        
        let tx_msg = Message::new(&tx_bytes, &Topic::Transaction);

        let relay = Relay::new(
            &rand::thread_rng().gen_range(49152..65535),
            &false
        )?;
        
        relay.broadcast(&tx_msg)?;

        Ok(())
        
    } else {
        Err("Arg error!")?
    }

}
