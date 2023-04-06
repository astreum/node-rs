// check validity of blocks

use std::{sync::{Arc, Mutex}, collections::HashMap, time::Instant, thread};

use crate::block::Block;

pub fn run(
   proposed_blocks: Arc<Mutex<HashMap<[u8;32], Block>>>
) {

   thread::spawn(move || {

      let mut validation_time = Instant::now();

      loop {

         if Instant::now().duration_since(validation_time).as_secs() < 2 {

            validation_time = Instant::now()
            
         }
          
      }
   
   });

}

// match Block::try_from(&message.body[..]) {
         
//    Ok(block) => {

//       if block.previous_block_hash == latest_block.block_hash && block.time > latest_block.time {

//          let mut changed_accounts: HashMap<Address, Account> = HashMap::new();

//          let block_time_delta = block.time - &latest_block.time;

//          let misses = if block_time_delta > 3 {

//             let time_delta = block_time_delta - 1;

//             time_delta / 3
   
//          } else {
            
//             0
   
//          };

//          let mut random: [u8;32] = latest_block.delay_output.clone().try_into().unwrap();

//          for _ in 0..misses {
//             random = blake_3(&random);
//          };

//          let consensus_account = accounts_store.get(&CONSENSUS_ADDRESS).unwrap();

//          let total_stake = consensus_account.storage
//             .iter()
//             .fold(Integer::zero(), |acc, x| acc + x.1[..].into());

//          let mut random_int = Integer::from(&random[..]);

//          random_int = random_int.modulo(&total_stake).unwrap();

//          let mut current_validator: Option<Address> = None;

//          for x in consensus_account.storage {

//             random_int = random_int - x.1[..].into();

//             if random_int <= Integer::zero() {

//                current_validator = Some(Address(x.0[..].try_into().unwrap()));
               
//                break;
            
//             }

//          }

//          if current_validator != None && block.validator == current_validator.unwrap() {

//             let mut solar_used = 0;

//             let mut transactions: Vec<Transaction> = Vec::new();

//             let mut receipts: Vec<Receipt> = Vec::new();

//             validation_time = Instant::now();

//             for tx in &block.transactions {

//                match tx.apply(&accounts_store, &mut changed_accounts) {
                  
//                   Ok(receipt) => {

//                      solar_used += receipt.solar_used;

//                      transactions.push(tx.clone());

//                      receipts.push(receipt);
                  
//                   },
               
//                   Err(_) => (),

//                }

//             }

//             let mut validator_account = Account::from_accounts(&block.validator, &changed_accounts, &accounts_store).unwrap();

//             let validator_reward = Integer::from_dec("1000000000").unwrap();

//             validator_account.increase_balance(&validator_reward);

//             changed_accounts.insert(block.validator, validator_account);

//             let receipts_hashes: Vec<[u8; 32]> = receipts
//                .iter()
//                .map(|x| x.hash())
//                .collect();

//             let receipts_hash = merkle_tree::root(
//                blake_3,
//                &(receipts_hashes
//                   .iter()
//                   .map(|x| x.as_slice())
//                   .collect::<Vec<_>>()
//                )
//             );

//             let accounts_clone = accounts.clone();

//             // update changed_accounts details hash 

//             for (address, account) in &changed_accounts {
               
//                accounts.insert(*address, account.details_hash);
         
//             }

//             let accounts_hashes: Vec<_> = accounts_clone
//                .iter()
//                .map(|x| merkle_tree::root(blake_3, &[&x.0.0[..], x.1]))
//                .collect();
            
//             let accounts_hash = merkle_tree::root(
//                blake_3,
//                &(accounts_hashes
//                   .iter()
//                   .map(|x| x.as_slice())
//                   .collect::<Vec<_>>()
//                )
//             );

//             let vdf = WesolowskiVDFParams(2048).new();

//             let challenge = merkle_tree::root(blake_3, &[&block.validator.0, &latest_block.delay_output]);

//             let int_100: Integer = (&100_u8).into();

//             let difficulty = if &latest_block.number > &int_100 {

//                let past_block_number = &latest_block.number - &int_100;
   
//                let past_block = blocks_store.get(&past_block_number).unwrap();
   
//                let block_time_diff = latest_block.time - past_block.time;
   
//                latest_block.delay_difficulty * (300 / block_time_diff)
      
//             } else {
      
//                1_u64
      
//             };

//             match vdf.verify(&challenge, difficulty, &block.delay_output) {

//                Ok(_) => {

//                   if block.accounts_hash == accounts_hash {

//                      if block.receipts_hash == receipts_hash {

//                         if block.solar_used == solar_used {

//                            let mut file = File::create("latest_block").unwrap();

//                            file.write_all(&message.body).unwrap();

//                            latest_block = block;

                           

//                         }

//                      }

//                   }

//                },

//                Err(_) => (),

//             }

//          }  

//       }
   
//    },

//    _ => (),

// }
