
use std::env;

mod library;
mod wallets;

fn main() -> std::io::Result<()> {
   
   let startup = r###"

   *      .       *    .               *     .    *          *
      .        .           *    .     *  .            .
         *   .      *           *               * .       *    .   .
      .                     *    .    * .            .

       .vvv.    .vvv.  .vvvvv.  .vvvv.   .vvvv.  .v   v.   .vvv.    .vvv.
      .v   v.  .v         v     .v   v.  .v      .v   v.  .v   v.  .v
      .vvvvv.   .vv.      v     .vvvv.   .vvv.   .v   v.  .v   v.   .vv.
      .v   v.      v.     v     .v  v.   .v      .v   v.  .v   v.      v.
      .v   v.  .vvv.      v     .v   v.  .vvvv.   .vvv.    .vvv.   .vvv.   .v.

   Astreuos Terminal

   version 0.1.0

   "###;

   let help = r###"

   Commands:

      Wallet ...........................................................................

      create wallet                 generates a seed phrase and master key
      recover wallet                recover a wallet through a seed phrase
      remove wallet                 remove master key
      show wallet                   view wallet information
      
      Accounts .........................................................................

      accounts                      view all accounts
      new account                   create a new account
      show account [account]        view account information

      Address ..........................................................................
         
      new address [account]         get a new address for a transaction
      show address [address]        view address information

      Transactions .....................................................................

      new transaction [account]     craft, sign and send a new transaction
      show transaction [tx_hash]    view transaction information
      cancel transaction [tx_hash]  remove a transaction from the tx pool

      Blockchain .......................................................................

      sync                          get the latest blocks
      mint                          validate the blockchain by minting new blocks

      Nova .............................................................................

      stake                         add quanta to the treasury and start minting
      withdraw                      remove quanta from the treasury

      pools                         view staking pools
      add to pool                   add quanta into a staking pool  
      withdraw from pool            remove quanta from a staking pool

      Nebula ...........................................................................

      get                           get an object
      store                         store an object
      
      serve                         start file server
      
   "###;

   print!("{}", startup);

   let args: Vec<String> = env::args().collect();

   if args.len() < 2 {

      print!(r###"
      
   Command not entered!
      "###);

      print!(r###"
      
   Usage:
      astreuos-terminal [command] [argument]
      "###);

      print!("{}", help);

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
         // "mint" => minting::run()?,
         // "storage" => storage::run()?,
         "help" => print!("{}", help),
         _ => {
            print!(r###"
      Command not recognized!
            "###)}
      }

   }

   Ok(())

}
