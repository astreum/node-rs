
use std::env;

mod wallets;
mod library;
mod mining;
mod addresses;
mod accounts;
mod network;

fn main() -> std::io::Result<()> {
   
   let startup_art = r###"

   *      .       *    .               *     .    *          *
      .        .           *    .     *  .            .
         *   .      *           *               * .       *    .   .
      .                     *    .    * .            .

       .vvv.    .vvv.  .vvvvv.  .vvvv.   .vvvv.  .v   v.   .vvv.    .vvv.
      .v   v.  .v         v     .v   v.  .v      .v   v.  .v   v.  .v
      .vvvvv.   .vv.      v     .vvvv.   .vvv.   .v   v.  .v   v.   .vv.
      .v   v.      v.     v     .v  v.   .v      .v   v.  .v   v.      v.
      .v   v.  .vvv.      v     .v   v.  .vvvv.   .vvv.    .vvv.   .vvv.   .v.

   Astreuos Node

   version 0.1.0

   Commands:
      create wallet                 Generates a seed phrase and master key
      recover wallet                Recover a wallet through a seed phrase
      remove wallet                 Remove wallet
      show wallet                   View wallet information
      
      accounts                      View all accounts
      new account                   Create a new account
      show account [account]        View account information
         
      new address [account]         Get a new address for a transaction
      show address [address]        View address information

      new transaction [address]     Create, sign and transmit a new transaction
      show transaction [tx_hash]    View transaction information

      sync                          Get the latest blocks and transform accounts
      mine                          Start mining new blocks
      
   "###;

   print!("{}", startup_art);

   let args: Vec<String> = env::args().collect();

   if args.len() < 2 {

      print!(r###"
      
    Command not entered!
      "###);

   } else {
    
      let mut command: String = String::new();

      for arg in &args[1..args.len()] {
         command.push_str(&arg);
      }

      match &command[..] {
         "createwallet" => wallets::create().unwrap(),
         "recoverwallet" => println!("Coming soon ..."),
         "removewallet" => wallets::remove().unwrap(),
         "showwallet" => println!("Coming soon ..."),
         "accounts" => println!("Coming soon ..."),
         "newaccount" => println!("Coming soon ..."),
         "addresses" => println!("Coming soon ..."),
         "newaddress" => println!("Coming soon ..."),
         "newtransaction" => println!("Coming soon ..."),
         "showtransaction" => println!("Coming soon ..."),
         "canceltransaction" => println!("Coming soon ..."),
         "sync" => println!("Coming soon ..."),
         "mine" => mining::start().unwrap(),
         _ => println!("Command not recognized!")
      }

   }

   Ok(())

}
