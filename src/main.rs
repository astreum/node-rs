
use std::env;
mod wallet;

fn main() {

   header();

   let mut args: Vec<String> = Vec::new();

   for arg in env::args() {
      args.push(arg)
   }

   let main_arg: &str = &format!("{} {}", &args[1], &args[2]);

   match main_arg {
      "wt create" => wallet::create(&args[3]),
      "wt private" => wallet::private(),
      "wt public" => wallet::public(),
      "wt recover" => wallet::recover(&args[3], &args[4]),
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

   Astreuos Node

   version 0.2.0
   
   "###)
   
}

fn help() {

   println!(r###"

   Usage:
      rust-astreuos [command] [arguments]

   Commands:

      Wallet ..........................................................................................
  
      wt create [password]                                                   generates your private key
      wt private                                                             view encrypted private key
      wt public                                                              view public key
      wt recover [encrypted private key] [password]                          recover a wallet
  
      Transactions .........................................................................................
  
      tx new [pass] [receipient] [amount] [solar price] [solar limit]        create, sign & send tx message
      tx cancel [pass] [tx hash]                                             send cancel tx message
  
      Nova ............................................................................................
  
      nv add [amount]                                                        add to stake balance
      nv stake                                                               check stake balance
  
 
   "###)

}
