// create new blocks and new solar

pub fn run() {

   let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

   if current_time > self.latest_block.time && Instant::now().duration_since(validation_time).as_secs() > 1 {

      let mut changed_accounts: HashMap<Address, Account> = HashMap::new();

      let total_time_delta = current_time - &self.latest_block.time;

      let misses = if total_time_delta > 3 {

         let time_delta = total_time_delta - 1;

         time_delta / 3

      } else {
         
         0

      };

      let mut random: [u8;32] = self.latest_block.delay_output.clone().try_into().unwrap();

      for _ in 0..misses {
         random = blake_3(&random);
      };

      let consensus_account = self.accounts_store.get(&CONSENSUS_ADDRESS).unwrap();

      let total_stake = consensus_account.storage
         .iter()
         .fold(Integer::zero(), |acc, x| acc + x.1[..].into());

      let mut random_int = Integer::from(&random[..]);

      random_int = random_int.modulo(&total_stake).unwrap();

      let mut current_validator: Option<Address> = None;

      for x in consensus_account.storage {

         random_int = random_int - x.1[..].into();

         if random_int <= Integer::zero() {

            current_validator = Some(Address(x.0[..].try_into().unwrap()));
            
            break;
         
         }

      }

      if current_validator != None && &self.account_address == &current_validator.unwrap() {

         let mut solar_used = 0;

         let mut transactions: Vec<Transaction> = Vec::new();

         let mut receipts: Vec<Receipt> = Vec::new();

         validation_time = Instant::now();

         for (_, tx) in &self.pending_transactions {

            if Instant::now().duration_since(validation_time).as_millis() < 1000 {

               match tx.apply(&self.accounts_store, &mut changed_accounts) {
                  
                  Ok(receipt) => {

                     solar_used += receipt.solar_used;

                     transactions.push(tx.clone());

                     receipts.push(receipt);
                  
                  },
               
                  Err(_) => (),

               }
            
            } else {

               break;

            }

         }

         let mut validator_account = Account::from_accounts(&self.account_address, &changed_accounts, &self.accounts_store).unwrap();

         let validator_reward = Integer::from_dec("1000000000").unwrap();

         validator_account.increase_balance(&validator_reward);

         changed_accounts.insert(self.account_address, validator_account);

         let receipts_hashes: Vec<[u8; 32]> = receipts
            .iter()
            .map(|x| x.hash())
            .collect();

         let receipts_hash = merkle_tree::root(
            blake_3,
            &(receipts_hashes
               .iter()
               .map(|x| x.as_slice())
               .collect::<Vec<_>>()
            )
         );

         let accounts_clone = self.accounts.clone();

         // update changed_accounts details hash 

         for (address, account) in &changed_accounts {
            
            self.accounts.insert(*address, account.details_hash);
      
         }

         let accounts_hashes: Vec<_> = accounts_clone
            .iter()
            .map(|x| merkle_tree::root(blake_3, &[&x.0.0[..], x.1]))
            .collect();
         
         let accounts_hash = merkle_tree::root(
            blake_3,
            &(accounts_hashes
               .iter()
               .map(|x| x.as_slice())
               .collect::<Vec<_>>()
            )
         );

         let vdf = WesolowskiVDFParams(2048).new();

         let challenge = merkle_tree::root(blake_3, &[&self.account_address.0, &self.latest_block.delay_output]);

         let int_100: Integer = (&100_u8).into();

         let difficulty = if &self.latest_block.number > &int_100 {

            let past_block_number = &self.latest_block.number - &int_100;

            let past_block = self.blocks_store.get(&past_block_number).unwrap();

            let block_time_diff = self.latest_block.time - past_block.time;

            self.latest_block.delay_difficulty * (300 / block_time_diff)
   
         } else {
   
            1_u64
   
         };

         let delay_output = vdf.solve(&challenge, difficulty).unwrap_or(Err("VDF error!").unwrap());

         let mut new_block = Block {
            accounts_hash,
            chain: self.chain.clone(),
            number: next_block_number.clone(),
            previous_block_hash: self.latest_block.block_hash,
            receipts_hash,
            signature: [0_u8;64],
            solar_used,
            time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            transactions,
            validator: self.account_address,
            data: "Astreum Foundation Node v0.0.1".as_bytes().into(),
            delay_output,
            transactions_hash: [0; 32],
            block_hash: [0; 32],
            details_hash: [0; 32],
            delay_difficulty: difficulty,
         };
         
         new_block.update_details_hash();
         
         new_block.sign(&self.account_key).unwrap();
         
         new_block.update_block_hash();

         let new_block_bytes: Vec<u8> = new_block.into();

         let new_block_message = Message::new(&new_block_bytes, &Topic::Block);

         let new_block_envelope = Envelope::new(false, (&new_block_message).into());

         let new_block_envelope_bytes: Vec<u8> = (&new_block_envelope).into();

         let samples = self.peer_route.samples();

         for sample in samples {

            match self.peers.get(&sample) {
               
               Some(peer) => {

                  let sample_socket_address = SocketAddr::new(sample, peer.incoming_port);

                  let _ = self.outgoing_socket.send_to(&new_block_envelope_bytes, sample_socket_address);

               },
               
               None => (),
            
            }

         }

      }

      validation_time = Instant::now();

   }
         
}
