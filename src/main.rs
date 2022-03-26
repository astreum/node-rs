mod account;
mod block;
mod nova;
mod state;
mod transaction;
use state::State;
use neutrondb::Store;
use astro_notation::{encode, decode};
use std::convert::TryInto;
use account::Account;
use opis::Int;
use block::Block;
use transaction::Transaction;
use fides::{hash, ed25519, chacha20poly1305};
use std::sync::Arc;
use pulsar_network::{Message, MessageKind};
use std::io;

const NOVA_ADDRESS: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 110, 111, 118, 97];
const NOVA_STAKE_STORE_ID: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 115, 116, 97, 107, 101];
const NOVA_SLOTS_STORE_ID: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 115, 108, 111, 116, 115];
const FIRST_ADDRESS: [u8; 32] = [174, 0, 137, 41, 53, 190, 116, 104, 223, 140, 157, 66, 71, 7, 92, 205, 3, 187, 103, 166, 148, 21, 127, 172, 150, 249, 144, 128, 60, 212, 48, 235];

const HEADER: &str = r###"
*      .       *    .               *     .    *          *
.  .        .           *    .     *  .            .
	*   .      *           *               * .       *    .   .
	.                     *    .    * .            .         .   .   .

Rust Astreuos version 0.0.1
"###;

const STARTUP: &str = r###"
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

version 0.0.1
"###;

const HOME: &str = r###"
Home,

+ Wallet
+ Accounts
+ Transact
+ Nova
+ Settings
"###;

use std::io::Write;

enum Context {
	Welcome,
	Login,
	Home,
	New,
	Transact,
	NewTransaction,
	CancelTransaction,
	Settings
}
 
fn prompt(name:&str) -> String {
	let mut line = String::new();
	print!("{}", name);
	std::io::stdout().flush().unwrap();
	std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
	return line.trim().to_string();

}

struct Frame {
	pub lines: Vec<String>
}

impl Frame {
	
	pub fn welcome() -> Self {
		
		Frame {
			lines: vec![String::from(r###"
Welcome,

+ login
+ new
+ recover
			"###)]
		}

	}

	pub fn render(&self, private_key: &[u8; 32]) {
		
		print!("\x1Bc");

		if private_key == &[0_u8; 32] {
			println!("{}", STARTUP)
		} else {
			println!("{}", HEADER)
		}

		for line in &self.lines {
			println!("{}", line)
		}

    	println!("- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -")

	}

}

fn main() {

	let mut context: Context = Context::Welcome;

	let mut app_store: Store = Store::connect("app");

	let chain: String = match app_store.get("chain") {
		Some(r) => r,
		None => "test".to_string()
	};

	let chain_borrow: &str = &chain;

	let chain_id: u8 = match chain_borrow {
		"main" => 1,
		"test" => 2,
		_ => panic!("Chain not supported!")
	};

	let state: State = State::current(&chain);

	let validation: bool = true;

	let mut private_key: [u8; 32] = [0_u8; 32];

	let mut master_public_key: [u8; 32] = [0_u8; 32];

	let mut input_cache: Vec<String> = Vec::new();

    let mut frame: Frame = Frame::welcome();

	frame.render(&private_key);

    loop {

		let prompt_str = match context {
			Context::Welcome => "type here: > ",
			_ => "> "
		};
		
		let input: &str = &prompt(prompt_str);
		
		if input == "exit" {
			
			break;
		
		} else if input == "home" && private_key != [0_u8; 32] {

			context = Context::Home;

			frame.lines = vec![HOME.to_string()]
		
		} else {
			
			match context {
				
				Context::Welcome => {
					
					match input {
						
						"login" => {
							context = Context::Login;
							frame.lines = vec!["Enter password,".to_string()]
						},
						
						"new" => {
							context = Context::New;
							frame.lines = vec!["Enter password,".to_string()]
						},
						
						// "recover" => { context = Context::Home; body = "Home" },

						"bootstrap" => {
							state.bootstrap();
							frame.lines = vec!["Bootstrapping,".to_string()]
						}
						
						_ => frame.lines = vec!["Type login, new or recover!".to_string()]
					
					}
				
				},
				
				Context::Login => {

					let private_key_cipher = app_store.get("priv_key").expect("Could not find private key!");

					let cipher_buf: Vec<u8> = decode::as_bytes(&private_key_cipher);

					let pass_key = hash(&input.as_bytes().to_vec());

					let priv_key = chacha20poly1305::decrypt(&pass_key, &cipher_buf);

					private_key = priv_key.try_into().unwrap();

					if validation {
						state.validate(private_key)
					}

					let pub_key = app_store.get("pub_key").unwrap();

					master_public_key = decode::as_bytes(&pub_key).try_into().unwrap();

					context = Context::Home;

					frame.lines = vec![HOME.to_string()];

					state.sync()

				},

				Context::New => {

					match app_store.get("priv_key") {
        
						Some(_) => {

							context = Context::Login;

							frame.lines = vec!["Private Key found! Enter password again,".to_string()]

						},
					
						None => {
						
							private_key = ed25519::private_key();
							
							master_public_key = ed25519::public_key(&private_key);
						
							let pass_key: [u8; 32] = hash(&input.as_bytes().to_vec());
						
							let encrypted_priv = chacha20poly1305::encrypt(&pass_key, &private_key.to_vec());
						
							app_store.put("priv_key", &encode::bytes(&encrypted_priv));
						
							app_store.put("pub_key", &encode::bytes(&master_public_key.to_vec()));

							app_store.put("chain", "test");

							app_store.put("validate", "no");
							
							context = Context::Home;
							
							frame.lines = vec![HOME.to_string()];

							state.sync()

						}
					}
				},

				Context::Home => {

					match input {
						
						"wallet" => {
							let private_key_cipher = app_store.get("priv_key").unwrap();
							let public_key = app_store.get("pub_key").unwrap();
							frame.lines = vec![
								"Wallet,".to_string(), "".to_string(),
								"Encrypted Key".to_string(),
								private_key_cipher[..61].to_string(),
								private_key_cipher[61..].to_string(), "".to_string(),
								"Address".to_string(),
								public_key
							]
						},

						"accounts" => {
							let accs_clone = Arc::clone(&state.accounts);
							let accs = accs_clone.lock().unwrap();
							frame.lines = vec!["Accounts,".to_string(), "".to_string()];
							accs
								.iter()
								.for_each(|x| 
									frame.lines.push(
										format!(
											"{} {} quarks",
											encode::bytes(&x.0.to_vec()),
											x.1.balance.to_decimal()
										)
									)
								)
						},
						
						"transact" => {
							context = Context::Transact;
							frame.lines = vec![
								"Transact,".to_string(),
								"".to_string(),
								"+ new".to_string(),
								"+ cancel".to_string()
							];
						},
						
						"nova" => {

							let accs_clone = Arc::clone(&state.accounts);

							let accs = accs_clone.lock().unwrap();

							let acc = accs.get(&NOVA_ADDRESS).unwrap();
							
							let nova_stake_store = acc.storage.get(&NOVA_STAKE_STORE_ID).unwrap();

							frame.lines = vec!["Nova Stakes,".to_string(), "".to_string()];

							nova_stake_store
								.iter()
								.for_each(|x| 
									frame.lines.push(
										format!(
											"{} {} quarks",
											encode::bytes(&x.0.to_vec()),
											Int::from_bytes(&x.1.to_vec()).to_decimal()
										)
									)
								)

						},

						"settings" => {
							context = Context::Settings;
							frame.lines = vec![
								"Settings,".to_string(), "".to_string(),
								format!("+ network: {}", chain),
								format!("+ validation: {}", validation), "".to_string(),
								"Type network or validation to change settings!".to_string()
							];
						},

						_ => frame.lines = vec!["Type wallet, accounts, transact, nova or settings!".to_string()]

					}

				},

				Context::Transact => {
					match input {
						"new" => {
							frame.lines = vec![
								"New Transaction,".to_string(),
								"".to_string(),
								"enter recipient address:".to_string()
							];						
						},
						"cancel" => (),
						_ => frame.lines = vec!["Type new or cancel!".to_string()]
					}
				},

				Context::NewTransaction => {

					if input_cache.len() == 4 {
						
						if input == "send" {

							frame.lines.pop();
							
							frame.lines.push("Sending . . .".to_string());
							
							frame.render(&private_key);

							let accs_clone = Arc::clone(&state.accounts);

							let accs = accs_clone.lock().unwrap();

							let acc = accs.get(&master_public_key).unwrap();

							let mut tx = Transaction {
								chain: chain_id,
								counter: acc.counter.clone(),
								hash: [0_u8; 32],
								recipient: decode::as_bytes(&input_cache[0]).try_into().unwrap(),
								sender: master_public_key,
								signature: [0_u8; 64],
								solar_limit: u32::from_str_radix(&input_cache[2], 10).unwrap(),
								solar_price: Int::from_decimal(&input_cache[3]),
								value: Int::from_decimal(&input_cache[1])
							};
							
							tx.signature = ed25519::sign(&tx.body_hash(), &private_key, &master_public_key);

							let new_tx_message: Message = Message::new(MessageKind::Transaction, tx.to_bytes());

							let net_clone = Arc::clone(&state.network);

							let net = net_clone.lock().unwrap();

							net.broadcast(new_tx_message);

							frame.lines.pop();

							frame.lines.push("Sent!".to_string())

						}

					} else {

						input_cache.push(input.to_string());

						match input_cache.len() {
							1 => {
								frame.lines.pop();
								frame.lines.push(format!("Recipient: {}", input_cache[0]));
								frame.lines.push("Enter Amount:".to_string())
							},

							2 => {
								frame.lines.pop();
								frame.lines.push(format!("Amount: {}", input_cache[1]));
								frame.lines.push("Enter Solar Limit:".to_string())
							},

							3 => {
								frame.lines.pop();
								frame.lines.push(format!("Solar Limit: {}", input_cache[2]));
								frame.lines.push("Enter Solar Price:".to_string())
							},

							4 => {
								frame.lines.pop();
								frame.lines.push(format!("Solar Price: {}", input_cache[3]));
								frame.lines.push("Type send to proceed.".to_string())
							}

							_ => frame.lines = vec!["Error!".to_string()]
						}
					}

				},

				Context::CancelTransaction => (),

				Context::Settings => {

					match input {
						"network" => (),
						"validation" => (),
						_ => ()
					}
					
				},

				_ => {

					context = Context::Home;
					
					frame.lines = vec![HOME.to_string()]
				
				}
			
			}
		
		}

		frame.render(&private_key);
	
	}

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