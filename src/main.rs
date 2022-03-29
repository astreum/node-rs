mod account;
mod block;
mod nova;
mod state;
mod transaction;
mod interface;
use state::State;
use neutrondb::Store;
use astro_notation::{encode, decode};
use std::convert::TryInto;
use opis::Int;
use transaction::Transaction;
use fides::{hash, ed25519, chacha20poly1305};
use std::sync::Arc;
use pulsar_network::{Message, MessageKind};
use interface::Interface;

const NOVA_ADDRESS: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 110, 111, 118, 97];
const NOVA_STAKE_STORE_ID: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 115, 116, 97, 107, 101];
const NOVA_SLOTS_STORE_ID: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 115, 108, 111, 116, 115];
const FIRST_ADDRESS: [u8; 32] = [174, 0, 137, 41, 53, 190, 116, 104, 223, 140, 157, 66, 71, 7, 92, 205, 3, 187, 103, 166, 148, 21, 127, 172, 150, 249, 144, 128, 60, 212, 48, 235];

const HEADER: &str = r###"
*      .       *    .               *     .    *          *
.  .        .           *    .     *  .            .
	*   .      *           *               * .       *    .   .
	.                     *    .    * .            .         .   .   .

Rust Astreuos v 0.0.1
"###;

const WELCOME_HEADER: &str = r###"
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

fn main() {

	let mut ui: Interface = Interface::new();

	let mut context: Context = Context::Welcome;

	let mut app_store: Store = Store::connect("app");

	let chain = match app_store.get("chain"){
		Some(r) => r,
		None => "test".to_string()
	};

	let validation = match app_store.get("validation") {
		Some(r) => r,
		None => encode::bool(&false)
	};

	let state: State = State::current(&chain);

	let mut private_key: [u8; 32] = [0_u8; 32];

	let mut public_key: [u8; 32] = [0_u8; 32];

	let mut input_cache: Vec<String> = Vec::new();

	let welcome: Vec<&str> = vec!["Welcome,", "", "+ login", "+ new", "+ recover"];

	let home: Vec<&str> = vec!["Home,", "", "+ Wallet", "+ Accounts", "+ Transact", "+ Nova", "+ Settings"];

	ui.header = WELCOME_HEADER.to_string();
	ui.udpate(&welcome);
	ui.refresh();

    loop {

		let prompt_str = match context { Context::Welcome => "type here: > ", _ => "> " };
		
		let input: &str = &prompt(prompt_str);
		
		if input == "exit" {

			break;
		
		} else if input == "home" && private_key != [0_u8; 32] {
			
			context = Context::Home;
			
			ui.udpate(&home);

		} else {
			
			match context {

				Context::Welcome => {
					
					match input {
						
						"login" => {
							context = Context::Login;
							ui.lines = vec!["Enter password,".to_string()]
						},
						
						"new" => {
							context = Context::New;
							ui.lines = vec!["Enter password,".to_string()]
						},
						// "recover" => { context = Context::Home; body = "Home" },
						
						"bootstrap" => {
							state.bootstrap();
							ui.lines = vec!["Bootstrapping,".to_string()]
						}
						
						_ => ui.lines.push("Type login, new or recover!".to_string())
					
					}
				},
				
				Context::Login => {

					let private_key_cipher = app_store.get("priv_key").expect("Could not find private key!");

					let validation = app_store.get("validation").expect("could not find validation settings!");

					let cipher_buf: Vec<u8> = decode::as_bytes(&private_key_cipher);

					let pass_key = hash(&input.as_bytes().to_vec());

					let priv_key = chacha20poly1305::decrypt(&pass_key, &cipher_buf);

					private_key = priv_key.try_into().unwrap();

					let pub_key = app_store.get("pub_key").unwrap();

					public_key = decode::as_bytes(&pub_key).try_into().unwrap();

					context = Context::Home;

					ui.header = HEADER.to_string();

					ui.udpate(&home);

					state.sync();
					
					if decode::as_bool(&validation) {
						state.validate(private_key)
					}

				},

				Context::New => {

					match app_store.get("priv_key") {
        
						Some(_) => {

							context = Context::Login;

							ui.lines = vec!["Private Key found! Enter password again,".to_string()]

						},
					
						None => {
						
							private_key = ed25519::private_key();
							public_key = ed25519::public_key(&private_key);
						
							let pass_key: [u8; 32] = hash(&input.as_bytes().to_vec());
						
							let encrypted_priv = chacha20poly1305::encrypt(&pass_key, &private_key.to_vec());
						
							app_store.put("priv_key", &encode::bytes(&encrypted_priv));
							app_store.put("pub_key", &encode::bytes(&public_key.to_vec()));
							app_store.put("chain", "test");
							app_store.put("validation", &encode::bool(&false));
							
							context = Context::Home;

							ui.header = HEADER.to_string();

							ui.udpate(&home);

							state.sync()

						}
					}
				},

				Context::Home => {

					match input {
						
						"wallet" => {
							
							let private_key_cipher = app_store.get("priv_key").unwrap();
							
							// let pub_key = app_store.get("pub_key").unwrap();

							let pub_key: String = format!("{:?}",public_key);
							
							let update = vec![
								"Wallet,","",
								"encrypted key",
								&private_key_cipher[..61],
								&private_key_cipher[61..], "",
								"address",
								&pub_key
							];

							ui.udpate(&update);

						},

						"accounts" => {
							
							let accs_clone = Arc::clone(&state.accounts);
							
							let accs = accs_clone.lock().unwrap();
							
							ui.udpate(&vec!["Accounts,", ""]);
							
							accs
								.iter()
								.for_each(|x| 
									ui.lines.push(
										format!(
											"{} {} Quarks",
											encode::bytes(&x.0.to_vec()),
											x.1.balance.to_decimal()
										)
									)
								)
						},
						
						"transact" => {
							
							context = Context::Transact;
							
							ui.udpate(&vec!["Transact,", "", "+ new", "+ cancel"])

						},
						
						"nova" => {

							let accs_clone = Arc::clone(&state.accounts);

							let accs = accs_clone.lock().unwrap();

							let acc = accs.get(&NOVA_ADDRESS).unwrap();
							
							let nova_stake_store = acc.storage.get(&NOVA_STAKE_STORE_ID).unwrap();

							ui.udpate(&vec!["Nova Stakes,", ""]);

							nova_stake_store
								.iter()
								.for_each(|x| 
									ui.lines.push(
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
							ui.lines = vec![
								"Settings,".to_string(), "".to_string(),
								format!("+ chain: {}", chain),
								format!("+ validation: {}", decode::as_bool(&validation)), "".to_string(),
								"Type network or validation to change settings!".to_string()
							];
						},

						_ => ui.lines = vec!["Type wallet, accounts, transact, nova or settings!".to_string()]

					}

				},

				Context::Transact => {
					match input {
						"new" => {
							ui.lines = vec![
								"New Transaction,".to_string(),
								"".to_string(),
								"enter recipient address:".to_string()
							];						
						},
						"cancel" => (),
						_ => ui.lines = vec!["Type new or cancel!".to_string()]
					}
				},

				Context::NewTransaction => {

					if input_cache.len() == 4 {
						
						if input == "send" {

							ui.lines.pop();
							
							ui.lines.push("Sending . . .".to_string());
							
							ui.refresh();

							let accs_clone = Arc::clone(&state.accounts);

							let accs = accs_clone.lock().unwrap();

							let acc = accs.get(&public_key).unwrap();

							let chain_borrow: &str = &chain;

							let chain_id = match chain_borrow {
								"main" => 1,
								"test" => 2,
								_ => panic!("internal error")
							};

							let mut tx = Transaction {
								chain: chain_id,
								counter: acc.counter.clone(),
								hash: [0_u8; 32],
								recipient: decode::as_bytes(&input_cache[0]).try_into().unwrap(),
								sender: public_key,
								signature: [0_u8; 64],
								solar_limit: u32::from_str_radix(&input_cache[2], 10).unwrap(),
								solar_price: Int::from_decimal(&input_cache[3]),
								value: Int::from_decimal(&input_cache[1])
							};
							
							tx.signature = ed25519::sign(&tx.body_hash(), &private_key, &public_key);

							let new_tx_message: Message = Message::new(MessageKind::Transaction, tx.to_bytes());

							let net_clone = Arc::clone(&state.network);

							let net = net_clone.lock().unwrap();

							net.broadcast(new_tx_message);

							ui.lines.pop();

							ui.lines.push("Sent!".to_string())

						}

					} else {

						input_cache.push(input.to_string());

						match input_cache.len() {
							1 => {
								ui.lines.pop();
								ui.lines.push(format!("Recipient: {}", input_cache[0]));
								ui.lines.push("Enter Amount:".to_string())
							},

							2 => {
								ui.lines.pop();
								ui.lines.push(format!("Amount: {}", input_cache[1]));
								ui.lines.push("Enter Solar Limit:".to_string())
							},

							3 => {
								ui.lines.pop();
								ui.lines.push(format!("Solar Limit: {}", input_cache[2]));
								ui.lines.push("Enter Solar Price:".to_string())
							},

							4 => {
								ui.lines.pop();
								ui.lines.push(format!("Solar Price: {}", input_cache[3]));
								ui.lines.push("Type send to proceed.".to_string())
							}

							_ => ui.lines = vec!["Error!".to_string()]
						}
					}

				},

				Context::CancelTransaction => (),

				Context::Settings => {

					match input {
						"network" => {

							let chain_borrow: &str = &chain;

							match chain_borrow {
								"main" => app_store.put("network", "test"),
								"test" => app_store.put("network", "main"),
								_ => ()
							}
						},
						"validation" => {
							match decode::as_bool(&validation) {
								false => app_store.put("validation", &encode::bool(&true)),
								true => app_store.put("validation", &encode::bool(&false))
							}
						},
						_ => ()
					}

					context = Context::Settings;

					ui.lines = vec![
						"Settings,".to_string(), "".to_string(),
						"Please Restart the App!".to_string()
					];

				}
			}
		}
		ui.refresh();
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