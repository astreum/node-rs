use std::{error::Error, net::UdpSocket, sync::{Arc, Mutex}, collections::{HashMap, BTreeMap}};

use fides::x25519;
use neutrondb::Store;
use opis::Integer;
use rand::Rng;

use crate::{ping::Ping, address::Address, block::Block, route::Route, chain::Chain};
mod decode;
mod incoming;
mod validate;
mod mint;

pub fn run(
	account_address: Address,
	account_key: [u8;32],
	chain: Chain,
	incoming_port: u16,
	validator: bool,
) -> Result<(), Box<dyn Error>> {

	let incoming_address = format!("127.0.0.1:{}", &incoming_port);

   let incoming_socket = UdpSocket::bind(incoming_address)?;

	let incoming_queue = Arc::new(Mutex::new(Vec::new()));

	incoming::run(incoming_queue.clone(), incoming_socket);

	let peers = Arc::new(Mutex::new(HashMap::new()));

	let relay_key = x25519::secret_key();

	let relay_address_bytes = x25519::public_key(&relay_key);

	let relay_address = Address(relay_address_bytes);

	let ping = Ping::new(&chain, &incoming_port, &relay_address, &validator);

	let blocks: Store<Integer, Block> = Store::new(&format!("./data/{:?}_blocks", &chain))?;

	let blocks_store = Arc::new(Mutex::new(blocks));

	let consensus_route = Arc::new(Mutex::new(Route::new()));

	let outgoing_port: u16 = rand::thread_rng().gen_range(49152..65535);

	let outgoing_address = format!("127.0.0.1:{}", outgoing_port);

	let outgoing_socket = UdpSocket::bind(outgoing_address)?;

	let peer_route = Arc::new(Mutex::new(Route::new()));

	let pending_transactions = Arc::new(Mutex::new(BTreeMap::new()));

	let proposed_blocks = Arc::new(Mutex::new(HashMap::new()));

	decode::run(
		blocks_store.clone(),
		consensus_route.clone(),
		incoming_queue.clone(),
		outgoing_socket,
		peer_route.clone(),
		peers.clone(),
		pending_transactions.clone(),
		ping,
		proposed_blocks.clone(),
		relay_key
	);

	// outgoing

	// liveness

	validate::run(proposed_blocks.clone());

	if validator {

		mint::run()

	}

	loop { }

}









// use std::{net::{UdpSocket, IpAddr, SocketAddr}, error::Error, collections::{HashMap, BTreeMap}, time::{SystemTime, Instant}, str::FromStr, fs::File, io::{Read, Write}};

// use fides::{decrypt, x25519, encrypt, hash::blake_3, merkle_tree};
// use neutrondb::Store;
// use opis::Integer;
// use rand::Rng;
// use vdf::{WesolowskiVDFParams, VDFParams, VDF};

// use crate::{envelope::Envelope, peer::Peer, message::Message, topic::Topic, ping::Ping, address::Address, chain::Chain, route::Route, block::Block, account::Account, receipt::Receipt, transaction::Transaction, CONSENSUS_ADDRESS};

// pub fn run(
// 	account_address: Address,
// 	account_key: [u8;32],
// 	chain: Chain,
// 	incoming_port: u16,
// 	validator: bool
// ) -> Result<(), Box<dyn Error>> {

//    println!("application running ...");

//    let incoming_address = format!("127.0.0.1:{}", &incoming_port);

//    let incoming_socket = UdpSocket::bind(incoming_address)?;

// 	let relay_key = x25519::secret_key();

// 	let relay_address_bytes = x25519::public_key(&relay_key);

// 	let relay_address = Address(relay_address_bytes);

// 	let mut peers: HashMap<IpAddr, Peer> = HashMap::new();

// 	let ping = Ping::new(&chain, &incoming_port, &relay_address, &validator);

// 	let ping_message_bytes: Vec<u8> = (&ping).into();

// 	let mut peer_route = Route::new();

// 	let mut consensus_route = Route::new();

// 	let outgoing_port: u16 = rand::thread_rng().gen_range(49152..65535);

// 	let outgoing_address = format!("127.0.0.1:{}", outgoing_port);

// 	let outgoing_socket = UdpSocket::bind(outgoing_address)?;

// 	let mut latest_block_file = File::open("latest_block")?;

// 	let mut latest_block_bytes = Vec::new();
	
// 	latest_block_file.read_to_end(&mut latest_block_bytes)?;

// 	let mut latest_block = Block::try_from(latest_block_bytes)?;

// 	let mut accounts: BTreeMap<Address, [u8;32]> = BTreeMap::new();

// 	let mut accounts_store: Store<Address, Account> = Store::new(&format!("./data/{:?}_accounts",chain))?;

// 	let mut blocks_store: Store<Integer, Block> = Store::new(&format!("./data/{:?}_blocks", &chain))?;

// 	let mut pending_transactions: BTreeMap<[u8; 32], Transaction> = BTreeMap::new();

// 	let mut sync_time = Instant::now();

// 	let live_time = Instant::now();

// 	let mut validation_time = Instant::now();

//    let mut buffer = [0; 32000];
   
//    loop {

// 	  match incoming_socket.recv_from(&mut buffer) {

// 		 Ok((data_length, sender_socket_address)) => {

// 			let buffer = &mut buffer[..data_length];

// 			match Envelope::try_from(&buffer[..]) {

// 				Ok(envelope) => {

// 					let plain_message_option = if envelope.encrypted {
   
// 						match peers.get(&sender_socket_address.ip()) {
							
// 							Some(peer) => {

// 								match astro_format::decode(&envelope.message) {

// 									Ok(cipher_fields) => {

// 										if cipher_fields.len() == 2 {

// 											let cipher_nonce_res = cipher_fields[0].try_into();

// 											match cipher_nonce_res {

// 												Ok(cipher_nonce) => {

// 													match decrypt(&peer.shared_key, cipher_nonce, cipher_fields[1]) {
				
// 														Ok(decrypted) => Some(decrypted),
					
// 														Err(_) => None,
					
// 													}

// 												},

// 												Err(_) => None,

// 											}

// 										} else {
			
// 											None
			
// 										}

// 									},

// 									Err(_) => None,
								
// 								}

// 							},

// 							None => None,

// 						}

// 					} else {

// 						Some(envelope.message)
		 
// 					};

// 					match plain_message_option {
   
// 						Some(plain_message) => {

// 							match Message::try_from(&plain_message[..]) {

// 								Ok(message) => {

// 									match message.topic {

// 										Topic::Ping => {

// 											match Ping::try_from(&message.body[..]) {

// 												Ok(ping) => {

// 													let peer = Peer {
// 														incoming_port: ping.incoming_port,
// 														shared_key: x25519::shared_key(&ping.public_address.0, &relay_key),
// 														timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
// 													};

// 													match peers.get(&sender_socket_address.ip()) {

// 														Some(_) => (),

// 														None => {

// 															let response_envelope = Envelope::new(false, ping_message_bytes.clone());
															
// 															let response_bytes: Vec<u8> = (&response_envelope).into();
															
// 															let recipient_socket = SocketAddr::new(sender_socket_address.ip(), ping.incoming_port);

// 															let _ = outgoing_socket.send_to(&response_bytes, &recipient_socket);

// 														}

// 													}

// 													peers.insert(sender_socket_address.ip(), peer);

// 													let peer_public_key_str = ping.public_address.0
// 														.iter()
// 														.fold(String::new(), |acc, x| format!("{}{:08b}", acc, x));

// 													let _ = peer_route.add(&sender_socket_address.ip(), &peer_public_key_str);

// 													if ping.validator {
														
// 														let _ = consensus_route.add(&sender_socket_address.ip(), &peer_public_key_str);

// 													}


// 												},

// 												Err(_) => (),

// 											}

// 										},

// 										Topic::RouteRequest => {

// 											let samples = consensus_route.samples();

// 											let mut sample_socket_addresses_bytes = Vec::new();

// 											for sample in samples {

// 												match peers.get(&sample) {

// 													Some(peer) => {

// 														let sample_socket_address = SocketAddr::new(sample, peer.incoming_port);

// 														let sample_socket_address_str = sample_socket_address.to_string();

// 														let sample_socket_address_bytes = sample_socket_address_str.into_bytes();

// 														sample_socket_addresses_bytes.push(sample_socket_address_bytes);

// 													},

// 													None => (),
// 												}
												
// 											}

// 											let sample_socket_addresses_bytes_slices: Vec<&[u8]> = sample_socket_addresses_bytes
// 												.iter()
// 												.map(|x| x.as_slice())
// 												.collect();

// 											let encoded_samples = astro_format::encode(&sample_socket_addresses_bytes_slices);

// 											let route_message = Message::new(&encoded_samples, &Topic::Route);

// 											match peers.get(&sender_socket_address.ip()) {

// 												Some(peer) => {

// 													let route_message_bytes: Vec<u8> = (&route_message).into();

// 													match encrypt(&peer.shared_key, &route_message_bytes) {

// 														Ok((nonce, cipher)) => {

// 															let encrypted_message = astro_format::encode(&[&nonce[..], &cipher[..]][..]);

// 															let route_envelope = Envelope::new(true, encrypted_message);

// 															let envelope_bytes: Vec<u8> = (&route_envelope).into();

// 															let _ = outgoing_socket.send_to(&envelope_bytes, &SocketAddr::new(sender_socket_address.ip(), peer.incoming_port));

// 														},
														
// 														Err(_) => (),
													
// 													}
												
// 												},

// 												None => (),

// 											}
										
// 										},

// 										Topic::Route => {

// 											match astro_format::decode(&message.body) {

// 												Ok(sample_bytes) => {

// 													let mut sample_socket_addresses = vec![];

// 													for sample in sample_bytes {

// 														match String::from_utf8(sample.to_vec()) {

// 															Ok(sample_socket_address_str) => {

// 																match SocketAddr::from_str(&sample_socket_address_str) {

// 																	Ok(sample_socket_address) => sample_socket_addresses.push(sample_socket_address),

// 																	Err(_) => (),

// 																}

// 															},

// 															Err(_) => (),

// 														}
														
// 													}

// 													let ping_envelope = Envelope::new(false, ping_message_bytes.clone());

// 													let ping_envelope_bytes: Vec<u8> = (&ping_envelope).into();
													
// 													for sample_socket_address in sample_socket_addresses {

// 														let _ = outgoing_socket.send_to(&ping_envelope_bytes, &sample_socket_address);

// 													}

// 												},

// 												Err(_) => (),

// 											}

// 										},

// 										Topic::BlockRequest => {

// 											match peers.get(&sender_socket_address.ip()) {

// 												Some(peer) => {

// 													let block_number: Integer = message.body.into();

// 													match blocks_store.get(&block_number) {
														
// 														Ok(block) => {

// 															let block_bytes: Vec<u8> = block.into();

// 															let block_message = Message::new(&block_bytes, &Topic::Block);

// 															let block_envelope = Envelope::new(false, (&block_message).into());

// 															let block_envelope_bytes: Vec<u8> = (&block_envelope).into();

// 															let peer_socket_address = SocketAddr::new(sender_socket_address.ip(), peer.incoming_port);

// 															let _ = outgoing_socket.send_to(&block_envelope_bytes, &peer_socket_address);

// 														},
													
// 														Err(_) => (),

// 													}

// 												},
											  
// 												None => (),
										 
// 											}

// 										},

// 										Topic::Block => {

// 											match Block::try_from(&message.body[..]) {

// 												Ok(block) => {

// 													if block.previous_block_hash == latest_block.block_hash && block.time > latest_block.time {

// 														let mut changed_accounts: HashMap<Address, Account> = HashMap::new();

// 														let block_time_delta = block.time - &latest_block.time;

// 														let misses = if block_time_delta > 3 {

// 															let time_delta = block_time_delta - 1;

// 															time_delta / 3
												
// 														} else {
															
// 															0
												
// 														};

// 														let mut random: [u8;32] = latest_block.delay_output.clone().try_into().unwrap();

// 														for _ in 0..misses {
// 															random = blake_3(&random);
// 														};

// 														let consensus_account = accounts_store.get(&CONSENSUS_ADDRESS).unwrap();

// 														let total_stake = consensus_account.storage
// 															.iter()
// 															.fold(Integer::zero(), |acc, x| acc + x.1[..].into());

// 														let mut random_int = Integer::from(&random[..]);

// 														random_int = random_int.modulo(&total_stake).unwrap();

// 														let mut current_validator: Option<Address> = None;

// 														for x in consensus_account.storage {

// 															random_int = random_int - x.1[..].into();

// 															if random_int <= Integer::zero() {

// 																current_validator = Some(Address(x.0[..].try_into().unwrap()));
																
// 																break;
															
// 															}

// 														}
											
// 														if current_validator != None && block.validator == current_validator.unwrap() {

// 															let mut solar_used = 0;

// 															let mut transactions: Vec<Transaction> = Vec::new();
											
// 															let mut receipts: Vec<Receipt> = Vec::new();

// 															validation_time = Instant::now();
											
// 															for tx in &block.transactions {
											
// 																match tx.apply(&accounts_store, &mut changed_accounts) {
																	
// 																	Ok(receipt) => {
										
// 																		solar_used += receipt.solar_used;
										
// 																		transactions.push(tx.clone());
										
// 																		receipts.push(receipt);
																	
// 																	},
																
// 																	Err(_) => (),
										
// 																}
											
// 															}
											
// 															let mut validator_account = Account::from_accounts(&block.validator, &changed_accounts, &accounts_store).unwrap();
											
// 															let validator_reward = Integer::from_dec("1000000000").unwrap();
											
// 															validator_account.increase_balance(&validator_reward);
											
// 															changed_accounts.insert(block.validator, validator_account);
											
// 															let receipts_hashes: Vec<[u8; 32]> = receipts
// 																.iter()
// 																.map(|x| x.hash())
// 																.collect();
											
// 															let receipts_hash = merkle_tree::root(
// 																blake_3,
// 																&(receipts_hashes
// 																	.iter()
// 																	.map(|x| x.as_slice())
// 																	.collect::<Vec<_>>()
// 																)
// 															);
											
// 															let accounts_clone = accounts.clone();
											
// 															// update changed_accounts details hash 
											
// 															for (address, account) in &changed_accounts {
																
// 																accounts.insert(*address, account.details_hash);
														
// 															}
											
// 															let accounts_hashes: Vec<_> = accounts_clone
// 																.iter()
// 																.map(|x| merkle_tree::root(blake_3, &[&x.0.0[..], x.1]))
// 																.collect();
															
// 															let accounts_hash = merkle_tree::root(
// 																blake_3,
// 																&(accounts_hashes
// 																	.iter()
// 																	.map(|x| x.as_slice())
// 																	.collect::<Vec<_>>()
// 																)
// 															);

// 															let vdf = WesolowskiVDFParams(2048).new();
											
// 															let challenge = merkle_tree::root(blake_3, &[&block.validator.0, &latest_block.delay_output]);
											
// 															let int_100: Integer = (&100_u8).into();

// 															let difficulty = if &latest_block.number > &int_100 {

// 																let past_block_number = &latest_block.number - &int_100;
												
// 																let past_block = blocks_store.get(&past_block_number).unwrap();
												
// 																let block_time_diff = latest_block.time - past_block.time;
												
// 																latest_block.delay_difficulty * (300 / block_time_diff)
													
// 															} else {
													
// 																1_u64
													
// 															};

// 															match vdf.verify(&challenge, difficulty, &block.delay_output) {

// 																Ok(_) => {

// 																	if block.accounts_hash == accounts_hash {

// 																		if block.receipts_hash == receipts_hash {

// 																			if block.solar_used == solar_used {

// 																				let mut file = File::create("latest_block").unwrap();

// 																				file.write_all(&message.body).unwrap();

// 																				latest_block = block;

																				

// 																			}

// 																		}

// 																	}

// 																},

// 																Err(_) => (),

// 														  }

// 														}  

// 													}
											  
// 											  },

// 											  _ => (),

// 										  }

// 										},

// 										Topic::Transaction => {

// 											match Transaction::try_from(&message.body[..]) {
												
// 												Ok(tx) => {

// 													match pending_transactions.get(&tx.transaction_hash) {
														
// 														Some(_) => (),
														
// 														None => {

// 															pending_transactions.insert(tx.transaction_hash, tx);

// 															// pass to other validators

// 														},
// 													}
// 												},
// 												Err(_) => (),
// 											}
// 										}
// 									}
// 								},
// 								Err(_) => (),
// 							}
// 						},
// 						None => (),
// 					};

// 				},
// 				Err(_) => todo!(),

// 			}

// 		 },

// 		 Err(_) => (),

// 	  }

//    }

// }
