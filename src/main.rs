mod account;
mod block;
mod nova;
mod state;
mod transaction;
mod wallet;
use state::State;
use std::env;
use neutrondb::Store;
use astro_notation::{encode, decode};
use fides::{ hash, chacha20poly1305 };
use std::convert::TryInto;
use account::Account;
use opis::Int;
use block::Block;

const NOVA_ADDRESS: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 110, 111, 118, 97];
const NOVA_STAKE_STORE_ID: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 115, 116, 97, 107, 101];
const NOVA_SLOTS_STORE_ID: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 115, 108, 111, 116, 115];
const FIRST_ADDRESS: [u8; 32] = [174, 0, 137, 41, 53, 190, 116, 104, 223, 140, 157, 66, 71, 7, 92, 205, 3, 187, 103, 166, 148, 21, 127, 172, 150, 249, 144, 128, 60, 212, 48, 235];

fn main() {

   header();

   let mut args: Vec<String> = Vec::new();

   for arg in env::args() {
      args.push(arg)
   }

   if args.len() > 3 {

      let command: &str = &format!("{} {}", args[1], args[2]);

      match command {

         "wallet create" => wallet::create(args),
         "wallet key" => wallet::key(),
         "wallet address" => wallet::address(),
         "wallet recover" => wallet::recover(args),
         "sync blockchain" => {

            if args.len() == 4 {

               let state: State = State::current(&args[3]);

               state.sync();

               loop {}

            } else {

               println!("Please check syntax!");
               println!("sync blockchain [chain id]")

            }

         }
         "accounts all" => {

            if args.len() == 4 {
               
               let accounts_store: Store = Store::connect(&format!("accounts_{}", &args[3]));

               match accounts_store.get_all() {

                  Some(r) => {
                     
                     println!("Accounts . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .");

                     println!("");

                     for (address, details) in r {

                        let acc: Account = Account::from_astro(&details);

                        println!("{}   {} quarks", address, acc.balance.to_decimal())

                     }
                  },

                  None => {
                     println!("NeutronDB Store for {} accounts is empty! Sync Blockchain first!", args[3]);
                     println!("sync blockchain [chain id]")
                  }
               }

            } else {
               println!("Please check syntax!");
               println!("accounts all [chain id]")
            }
         },

         "accounts one" => {

            if args.len() == 5 {

               let accounts_store: Store = Store::connect(&format!("accounts_{}", &args[3]));

               match accounts_store.get(&args[4]) {
                  
                  Some(r) => {

                     let acc = Account::from_astro(&r);

                     println!("Account . . . . . . . . . . . . . . . . . .");
                     
                     println!("");

                     println!("{} quarks", acc.balance.to_decimal());

                     println!("{} transactions", acc.counter.to_decimal());

                  },

                  None => println!("Account not found!")
               }
            } else {
               println!("Please check syntax!");
               println!("accounts one [chain id] [address]")
            }
         },

         "tx suggest" => {

            if args.len() == 5 {

               let chain_id: &str = &args[3];

               match chain_id {
                  "main" => 1,
                  "test" => 2,
                  _ => panic!("{} is not a supported chain id!", chain_id)
               };

               let blocks_store: Store = Store::connect(&format!("blocks_{}", chain_id));

               let latest_block_str = blocks_store.get("latest_block").unwrap();

               let latest_block = Block::from_bytes(&decode::as_bytes(&latest_block_str)).unwrap();

               let suggested_price = latest_block.solar_price + Int::from_decimal("20");

               let accounts_store: Store = Store::connect(&format!("accounts_{}", chain_id));

               match accounts_store.get(&args[4]) {
                  
                  Some(_) => {
                     println!("1000 solar limit");
                     println!("{} quarks solar price", suggested_price.to_decimal())
                  },

                  None => {
                     println!("1001000 solar limit");
                     println!("{} quarks solar price", suggested_price.to_decimal())
                  }
               }
            } else {
               println!("Please check syntax!");
               println!("tx suggest [chain id] [recipient]")
            }

         },
         "tx new" => {

            if args.len() == 9 {

            } else {
               println!("Please check syntax!");
               println!("tx suggest [chain id] [recipient]")
            }

         },
         "tx cancel" => (),
         "nova stakes" => {

            
            if args.len() == 4 {
               
               let accounts_store: Store = Store::connect(&format!("accounts_{}", &args[3]));

               match accounts_store.get(&encode::bytes(&NOVA_ADDRESS.to_vec())) {

                  Some(r) => {

                     let acc = Account::from_astro(&r);

                     let nova_stake_store = acc.storage.get(&NOVA_STAKE_STORE_ID).unwrap();

                     println!("Stakes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .");

                     println!("");

                     for (address, stake) in nova_stake_store {

                        print!("{}    {} quarks", encode::bytes(&address.to_vec()), Int::from_bytes(&stake.to_vec()).to_decimal());

                     }

                     println!("");

                  },

                  None => {
                     println!("Please sync blockchain!");
                     println!("sync blockchain [chain id]")
                  }
               }
            } else {
               println!("Please check syntax!");
               println!("nova stakes [chain id]")
            }
         },

         "nova validate" => {
            
            if args.len() == 5 {

               let app_store = Store::connect("app");

               let priv_key_query = app_store.get("priv_key");

               match priv_key_query {
        
                  Some(r) => {

                     let cipher_message: Vec<u8> = decode::as_bytes(&r);

                     let pass_key = hash(&args[3].as_bytes().to_vec());

                     let priv_key = chacha20poly1305::decrypt(&pass_key, &cipher_message);

                     let state: State = State::current(&args[4]);

                     state.sync();

                     state.validate(priv_key.try_into().unwrap());

                     loop {}

                  },

                  None => {
                     println!("Account not found!");
                     help()
                 }

               }

            } else {
               println!("Please check syntax!");
               println!("nova stakes [chain id]")
            }
         },

         "bootstrap chain" => {

            let state: State = State::current(&args[3]);

            state.bootstrap();

            loop {}

         },
         _ => {
            println!("Please check syntax!");
            help()
         }
      }

   } else {
      println!("Please check syntax!");
      help()
   }
   

}

fn header() {
   
   println!(r###"
   
   *      .       *    .               *     .    *          *
    .  .        .           *    .     *  .            .
         *   .      *           *               * .       *    .   .
      .                     *    .    * .            .         .   .   .

 .vvv.    .vvv.  .vvvvv.  .vvvv.   .vvvv.  .v   v.   .vvv.    .vvv.
.v   v.  .v         v     .v   v.  .v      .v   v.  .v   v.  .v
.vvvvv.   .vv.      v     .vvvv.   .vvv.   .v   v.  .v   v.   .vv.
.v   v.      v.     v     .v  v.   .v      .v   v.  .v   v.      v.
.v   v.  .vvv.      v     .v   v.  .vvvv.   .vvv.    .vvv.   .vvv.   .v.

Rust Astreuos

version 1.0.0
    "###)
}

fn help() {
   
   println!(r###"
Commands:

   Wallet . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

   wallet new [password]                                                          create a new wallet
   wallet key                                                                     view encrypted key
   wallet address                                                                 view address
   wallet recover [encrypted key] [password]                                      recover your wallet

   Syncronization . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

   sync blockchain [chain id]                                                     get the latest blocks

   Accounts . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

   accounts all [chain id]                                                        view all accounts
   accounts one [chain id] [address]                                              view one account

   Transactions . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

   tx suggest [chain id][recipient]                                               suggests solar limit and price
   tx new [password] [chain id] [recipient] [amount] [solar limit] [solar price]  create and send a transaction
   tx cancel [chain id] [password] [tx hash]                                      send cancel tx message

   Nova . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

   nova stakes [chain id]                                                          view all stakes
   nova validate [chain id] [password]                                             validate the blockchain
    "###)
}

fn merkle_tree_hash(mut hashes: Vec<[u8;32]>) -> [u8; 32] {

   if hashes.len() == 0 {
      [0_u8; 32]
   } else {

      if hashes.len() % 2 != 0 { hashes.push([0_u8; 32]) };

      while hashes.len() > 1 {

         let mut cache: Vec<[u8; 32]> = Vec::new();

         let mut intermediate: Vec<[u8; 32]> = Vec::new();

         for h in &hashes {
            
            intermediate.push(*h);
            
            if intermediate.len() == 2 {
                  
                  cache.push(hash(&[
                     intermediate[0].to_vec(),
                     intermediate[1].to_vec()
                  ].concat()));

                  intermediate.clear()

            }

         }

         hashes = cache
      
      };

      hashes[0]

   }

}