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

 .vvv.    .vvv.  .vvvvv.  .vvvv.   .vvvv.  .v   v.   .vvv.    .vvv.
.v   v.  .v         v     .v   v.  .v      .v   v.  .v   v.  .v
.vvvvv.   .vv.      v     .vvvv.   .vvv.   .v   v.  .v   v.   .vv.
.v   v.      v.     v     .v  v.   .v      .v   v.  .v   v.      v.
.v   v.  .vvv.      v     .v   v.  .vvvv.   .vvv.    .vvv.   .vvv.   .v.

Astreuos Node
node-rs v0.0.1

powered by Stelar Labs

    "###);
    
    let args: Vec<String> = env::args().collect();

    let (password, chain, flag) = parser(args)?;

    let password_key = hash(password.as_bytes());

    let key_path = Path::new("./key.fides");

    let private_key: [u8;32] = match key_path.is_file() {

        true => {
            let encrypted_key = fs::read(key_path)?;
            chacha20poly1305::decrypt(&password_key, &encrypted_key)?.try_into().unwrap()
        },
        
        false => {
            let private_key = ed25519::private_key();
            let encrypted_key = chacha20poly1305::encrypt(&password_key, &private_key)?;
            fs::write(key_path, &encrypted_key)?;
            private_key
        }

    };

    println!("key okay! ...");
    
    let seeders_file = File::open("./seeders.txt")?;

    let mut seeders = Vec::new();
    
    for seeder in BufReader::new(seeders_file).lines() {

        let seeder = seeder?;
        
        let socket: SocketAddr = seeder.parse()?;

        seeders.push(socket)

    }

    let state = State::new(&chain, &flag, seeders)?;

    state.sync();

    match flag {
        Flag::Validate => state.validate(private_key),
        _ => ()
    }

    let listener = TcpListener::bind("127.0.0.1:7878")?;

    println!("serving terminals ...");

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

}

fn parser(args: Vec<String>) -> Result<(String, Chain, Flag), Box<dyn Error>> {

    if args.len() >= 3 {

        let password = args[1].clone();
        
        let chain = match &args[2][..] {
            "main" => Chain::Main,
            "test" => Chain::Test,
            _ => Err("Unknown Chain!")?
        };

        if args.len() >= 4 {
            
            let flag = match &args[3][..] {
                "-b" => Flag::Bootstrap,
                "-v" => Flag::Validate,
                _ => Err("Unknown Flag!")?
            };

            Ok((password, chain, flag))

        } else {
            Ok((password, chain, Flag::Empty))
        }

    } else {
        Err("Enter Password and Chain!")?
    }

}
