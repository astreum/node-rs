
use std::env;

mod library;
mod wallet;

fn main() {

   let mut args: Vec<String> = Vec::new();

   for arg in env::args() {
      args.push(arg)
   }

   let arg_1: &str = &args[1];

   match arg_1 {
      "wallet" => {

         if args.len() > 2 {

            let arg_2: &str = &args[2];

            match arg_2 {
               "create" => wallet::create(),
               "recover" => {

                  if args.len() > 26 {
                     wallet::recover(args[3..26].to_vec())
                  } else {
                     println!(" * help")
                  }

               },
               _ => println!(" * help")
            }

         } else {
            println!(" * help")
         }

      },
      _ => println!(" * help")
   }

}
