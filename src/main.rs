
use std::env;
// mod validate;
mod wallet;

fn main() {

   header();

   let mut args: Vec<String> = Vec::new();

   for arg in env::args() {
      args.push(arg)
   }

   let main_arg: &str = &format!("{} {}", &args[1], &args[2]);

   match main_arg {
      "wt create" => wallet::create(&args[3], &args[4]),
      "wt private" => wallet::private(),
      "wt address" => wallet::address(),
      "wt recover" => wallet::recover(&args[3], &args[4]),
      "tx new" => (),
      "tx cancel" => (),
      "nv add" => (),
      "nv stake" => (),
      // "nv validate" => validate::blocks(&args[3], format!("{} {}", &args[4], &args[5])),    
      _ => help()
   }

}

fn header() {

   println!(r###"
   
      *      .       *    .               *     .    *          *
      .        .           *    .     *  .            .
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

   Usage:
      rust-astreuos [command] [arguments]

   Commands:

      Wallet ................................................................................................
  
      wt create [password] [repeat password]                                 generates your key and address
      wt key                                                                 view encrypted key
      wt address                                                             view address
      wt recover [encrypted key] [password]                                  recover your wallet
  
      Transactions ..........................................................................................
  
      tx new [password] [receipient] [amount] [solar limit] [solar price]    create, sign & send tx message
      tx cancel [password] [tx hash]                                         send cancel tx message
  
      Nova ..................................................................................................
  
      nv add [amount]                                                        add to stake balance
      nv stake                                                               check stake balance
      nv validate [chain] [password]                                                 create new blocks
    
 
   "###)

}
