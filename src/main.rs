const STELAR_ADDRESS: [u8;32] = [1_u8;32];
const NOVA_ADDRESS: [u8;32] = [0_u8;32];
mod accounts;
mod blocks;
mod server;
mod state;
mod transactions;
use accounts::Account;
use astro_format::string;
use fides::{hash, chacha20poly1305, ed25519};
use server::{Request, Response};
use state::State;
use std::fs::File;
use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpListener, SocketAddr};
use std::sync::Arc;
use std::{env, fs, fmt};
use std::error::Error;
use std::path::Path;

pub enum Chain { Main, Test }

impl Chain {

    pub fn from_string(arg: &str) -> Result<Self, Box<dyn Error>>{

        match arg {
            "main" => Ok(Chain::Main),
            "test" => Ok(Chain::Test),
            _ => Err("Unknown chain option!")?
        }

    }
}

impl fmt::Debug for Chain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Chain::Main => write!(f, "main"),
            Chain::Test => write!(f, "test"),
        }
    }
}

#[derive(Debug)]
pub enum Flag { Bootstrap, Empty, Validate }

fn main() -> Result<(), Box<dyn Error>> {

    println!(r###"

*      .       *    .               *     .    *          *
.  .        .           *    .     *  .            .
	*   .      *           *               * .       *    .   .
	.                     *    .    * .            .         .   .   .

 .vvv.    .vvv.  .vvvvv.  .vvvv.   .vvvv.  .v   v.  .v.     .v.
.v   v.  .v         v     .v   v.  .v      .v   v.  .v v   v v.
.vvvvv.   .vv.      v     .vvvv.   .vvv.   .v   v.  .v  v v  v.
.v   v.      v.     v     .v  v.   .v      .v   v.  .v   v   v.
.v   v.  .vvv.      v     .v   v.  .vvvv.   .vvv.   .v       v.  .v.

Astreum Node

node-rs v0.0.1

    "###);
    
    let args: Vec<String> = env::args().collect();

    let command: &str = &args[1];

    match command {

        "new" => {

            if args.len() == 3 {

                let private_key = ed25519::private_key();

                let public_key = ed25519::public_key(&private_key);

                let public_key_hex = string::encode::bytes(&public_key);

                let key_path_string = format!("./accounts/{}.fides", public_key_hex);

                let key_path = Path::new(&key_path_string);

                let password_key = hash(args[2].as_bytes());

                let encrypted_key = chacha20poly1305::encrypt(&password_key, &private_key)?;
            
                fs::write(key_path, &encrypted_key)?;

                println!("Account created!");

                Ok(())

            } else {

                Err("Please provide a password!")?

            }

        },
        
        "serve" => {

            if args.len() == 2 {
                
                let chain = Chain::from_string(&args[2])?;

                let state = State::new(false, &chain)?;
                
                state.sync();

                let listener = TcpListener::bind("127.0.0.1:7878")?;
			
                for stream in listener.incoming() {

                    let mut stream = stream?;

                    let mut buffer = [0; 1024];

                    stream.read(&mut buffer)?;

                    let request = Request::from(buffer);

                    if request.method == Some("GET".to_string()) && request.path == Some("/accounts".to_string()) {

                        let accounts_store_clone = Arc::clone(&state.accounts_store);

                        let accounts_store = accounts_store_clone.lock().unwrap();

                        let mut contents: String = String::from("{");

                        for (address, account) in accounts_store.get_all().unwrap() {

                            let account_bytes = string::decode::bytes(&account)?;

                            let account = Account::from_bytes(&account_bytes)?;

                            if contents != String::from("{") {
                                contents.push(',');
                            }

                            let s: String = format!("\"{}\":\"{}\"",
                                address,
                                account.balance.to_decimal()
                            );

                            contents.push_str(&s)
                        
                        }

                        contents.push('}');

                        let mut response = Response::new();

                        response.body = contents;

                        stream.write(&response.into_bytes())?;
                        
                        stream.flush()?;

                    }

                }

                Ok(())

            } else {

                Err("Please provide the chain!")?

            }

        },
        
        "validate" => {

            if args.len() == 5 {

                let address = &args[3];

                let key_path_string = format!("./accounts/{}.fides", address);

                let key_path = Path::new(&key_path_string);

                let private_key: [u8;32] = match key_path.is_file() {

                    true => {

                        let password_key = hash(args[4].as_bytes());

                        let encrypted_key = fs::read(key_path)?;

                        chacha20poly1305::decrypt(&password_key, &encrypted_key)?.try_into().unwrap()

                    },

                    false => Err("Private key not found!")?

                };

                let chain = Chain::from_string(&args[2])?;

                let state = State::new(false, &chain)?;
                
                state.sync();

                state.validate(private_key);

                loop {}

            } else {

                Err("Please provide the chain, address and password!")?

            }
            
        },
        
        "bootstrap" => {

            if args.len() == 3 {

                let chain = Chain::from_string(&args[2])?;

                let state = State::new(true, &chain)?;
                
                state.sync();

                loop {}

            } else {

                Err("Please provide the chain!")?

            }

        },

        _ => Err("Command not supported!")?
    
    }

}
