mod account;
mod block;
mod bootstrap;
mod slots;
mod state;
mod sync;
mod transaction;
mod transform;
mod validate;
mod wallet;
use state::State;
use std::env;
use neutrondb::Store;
use astro_notation::decode;
use fides::{ hash, chacha20poly1305 };
use std::convert::TryInto;

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
         "accounts all" => (),
         "accounts one" => (),
         "tx new" => (),
         "tx cancel" => (),
         "nova stakes" => (),
         "nova stake" => (),
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
               println!("Please provide an password & chain ...");
               help()
            }
         },

         "bootstrap chain" => {

            let state: State = State::current(&args[3]);

            state.bootstrap();

            loop {}

         },
         _ => help()
      }

   } else {
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

      Wallet ...................................................................................................

      wallet new [password]                                                          create a new wallet
      wallet key                                                                     view encrypted key
      wallet address                                                                 view address
      wallet recover [encrypted key] [password]                                      recover your wallet

      Accounts ...................................................................................................

      accounts all                                                                   view all accounts
      accounts one [address]                                                         view one account

      Transaction ...............................................................................................

      tx new [password] [chain] [recipient] [amount] [solar limit] [solar price]     create & send a transaction
      tx cancel [password] [tx hash]                                                 send cancel tx message

      Nova .......................................................................................................

      nova stakes                                                                    view all stakes
      nova stake [address]                                                           view address stake
      nova validate [password] [chain]                                               validate the blockchain
    
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