use fides::ed25519;
// use neutrondb::Store;
// use opis::Integer;
// use rand::Rng;
// use crate::account::Account;
// use crate::address::Address;
// use crate::app::App;
// use crate::chain::Chain;
// use crate::relay::Relay;
// use crate::relay::message::Message;
// use crate::relay::topic::Topic;
// use crate::transaction::Transaction;
use std::env;
use std::error::Error;
use std::path::Path;
use std::fs;
// mod account;
mod address;
// mod app;
// mod block;
// mod chain;
// mod transaction;
// mod receipt;
// mod relay;
// mod state;

// const CONSENSUS_ADDRESS: Address = Address([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 99, 111, 110, 115, 101, 110, 115, 117, 115]);
// const STELAR_ADDRESS: Address = Address([0, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 99, 111, 110, 115, 101, 110, 115, 117, 115]);

fn main() -> Result<(), Box<dyn Error>> {

    println!(r###"
*      .       *    .               *     .    *          *
.  .        .           *    .     *  .            .
    *   .      *           *               * .       *    .   .
.                     *    .    * .            .         .   .   .

 .vvv.    .vvv.  .vvvvv.  .vvvv.   .vvvv.  .v   v.  .v.   .v.
.v   v.  .v         v     .v   v.  .v      .v   v.  .v v v v.
.vvvvv.   .vv.      v     .vvv.    .vvv.   .v   v.  .v  v  v.
.v   v.      v.     v     .v  v.   .v      .v   v.  .v     v.
.v   v.  .vvv.      v     .v   v.  .vvvv.   .vvv.   .v     v.  .v.

Node v0.0.1

Astreum Foundation 12023 HE.
    "###);

    
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {

        let topic : &str = &args[1];

        match topic {

            "new" => {

                println!("creating account ...");

                let secret_key = ed25519::secret_key();

                let public_key = ed25519::public_key(&secret_key)?;

                let public_key_hex = hex::encode(&public_key);

                let keys_path = Path::new("./keys");

                if !keys_path.exists() {

                    fs::create_dir(keys_path)?;
                    
                }

                let key_path_string = format!("./keys/{}", public_key_hex);

                let key_path = Path::new(&key_path_string);

                fs::write(key_path, &secret_key)?;

                println!("account created for {}", public_key_hex);

            },

            // "account" => {

            //     if args.len() == 4 {

            //         println!("viewing account {}...", &args[3][..]);
            
            //         let chain = Chain::try_from(&args[2][..])?;
            
            //         let address = Address::try_from(&args[3][..])?;
            
            //         let accounts_store_path = format!("./data/{:?}_accounts",chain);
            
            //         let accounts_store: Store<Address, Account> = Store::new(&accounts_store_path)?;
            
            //         let (balance, counter) = match accounts_store.get(&address) {
            
            //             Ok(account) => (account.balance, account.counter),
            
            //             Err(_) => (Integer::zero(), Integer::zero()),
            
            //         };

            //         println!("balance: {}", balance.to_dec());

            //         println!("counter: {}", counter.to_dec());
            
            //         Ok(())
                    
            //     } else {
            
            //         println!("usage is 'account [chain] [address]'!");

            //         Ok(())
                    
            //     }
                
            // },

            // "send" => {

            //     if args.len() == 9 {

            //         let chain = Chain::try_from(&args[4][..])?;
            
            //         let sender = Address::try_from(&args[6][..])?;
            
            //         let accounts_store: Store<Address, Account> = Store::new(
            //             &format!("./data/{:?}_accounts",chain)
            //         )?;
            
            //         let counter = match accounts_store.get(&sender) {
            //             Ok(r) => r.counter,
            //             Err(_) => Integer::zero()
            //         };
            
            //         let recipient = Address::try_from(&args[8][..])?;
            
            //         let value = Integer::from_dec(&args[2][..])?;
            
            //         let secret_key_path_str = format!("./keys/{:?}", sender);
            
            //         let mut tx = Transaction {
            //             chain: chain.clone(),
            //             counter,
            //             recipient,
            //             sender,
            //             value,
            //             data: vec![],
            //             details_hash: [0; 32],
            //             signature: [0; 64],
            //             transaction_hash: [0; 32],
            //         };
            
            //         tx.details_hash = tx.details_hash();
            
            //         let secret_key_path = Path::new(&secret_key_path_str);
                    
            //         let secret_key = fs::read(secret_key_path)?;
            
            //         tx.sign(secret_key[..].try_into()?)?;
            
            //         let tx_bytes: Vec<u8> = tx.into();
                    
            //         let tx_msg = Message::new(&tx_bytes, &Topic::Transaction);
            
            //         let relay = Relay::new(
            //             &rand::thread_rng().gen_range(49152..65535),
            //             &false
            //         )?;

            //         // connect first
                    
            //         relay.broadcast(&tx_msg)?;
            
            //         Ok(())
                    
            //     } else {

            //         println!("usage is 'send [value] on [chain] from [sender] to [recipient]'!");

            //         Ok(())

            //     }

            // },

            // "block" => {

            //     if args.len() == 5 {
            //         // check local storage
            //         // search network
            //         Ok(())

            //     } else {

            //         Err("")?

            //     }

            // },

            // "stake" => app::stake::run(&args),

            // "validate" => {

            //     println!("info: validating");

            //     if args.len() == 4 {

            //         let public_key = Address::try_from(&args[4][..])?;
        
            //         let secret_key_path_str = format!("./keys/{:?}", public_key);
        
            //         let secret_key_path = Path::new(&secret_key_path_str);
                    
            //         let secret_key = fs::read(secret_key_path)?;

            //         println!("info: key found for {:?}", public_key);
        
            //         let chain = Chain::try_from(&args[3][..])?;

            //         let app = App::new(
            //             &chain,
            //             &rand::thread_rng().gen_range(49152..65535),
            //             &true
            //         )?;
                    
            //         app.listen()?;

            //         app.update()?;

            //         app.validate(public_key, secret_key[..].try_into()?)?;
        
            //         Ok(())
        
            //     } else {
        
            //         Err("Usage is: validate [chain] [address]")?
        
            //     }

            // },
            
            // "sync" => {

            //     println!("syncing ...");

            //     if args.len() == 3 {

            //         let chain = Chain::try_from(&args[2][..])?;

            //         let app = App::new(&chain, &55555, &false)?;

            //         app.listen()?;

            //         app.update()?;

            //         loop {}

            //     } else {

            //         println!("usage is 'sync [chain]'!")

            //     };

            //     Ok(())

            // },

            _ =>  help()

        }

    } else {
        
        help()

    }

    Ok(())
    
}

fn help() {

    println!(r###"
Help

- - - + - - - + - - -

new                                                     create a new account

// account [chain] [address]                               view local account information

// send [value] on [chain] from [sender] to [recipient]    send value across addresses     

// block view [chain] [number]                                 shows block information from local & peers

// stake fund [chain] [address] [value]                        create, sign & submit stake funding transaction

// stake withdraw [chain] [address] [value]                    create, sign & submit stake withdrawl transaction

// stake view [chain] [address]                                shows staking information from local & peers 

// validate [chain] [address]                                  create, sign & submit blocks

// sync [chain]                                                get new blocks & update accounts

    "###);

}
