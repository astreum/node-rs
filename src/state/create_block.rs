use std::{error::Error, collections::{BTreeMap, HashMap}, thread};

use fides::{merkle_tree, hash::blake_3};
use neutrondb::Store;
use opis::Integer;
use vdf::{WesolowskiVDFParams, VDFParams, VDF};

use crate::{block::Block, address::Address, transaction::Transaction, receipt::Receipt, account::Account};

use super::State;

impl State {

    pub fn create_block(
        &mut self,
        blocks_store: &Store<Integer, Block>,
        changed_accounts: &mut HashMap<Address, Account>,
        pending_transactions: &BTreeMap<[u8;32],Transaction>,
        public_key: &Address,
        secret_key: &[u8; 32],
        target_time: &u64
    ) -> Result<Block, Box<dyn Error>> {

        let delay_difficulty = self.delay_difficulty(blocks_store)?;

        let challenge = merkle_tree::root(
            blake_3,
            &[
                &public_key.0,
                &self.latest_block.delay_output
            ]
        );

        let delay_output = thread::spawn(move || {

            let vdf = WesolowskiVDFParams(2048).new();

            vdf.solve(&challenge, delay_difficulty)

        });

        let mut transactions = Vec::new();
        
        let mut receipts: Vec<Receipt> = Vec::new();

        let mut solar_used = 0;

        for tx in pending_transactions.iter() {
            
            match tx.1.apply(&self.accounts_store, changed_accounts) {

                Ok(receipt) => {

                    solar_used += receipt.solar_used;

                    receipts.push(receipt);

                    transactions.push(tx.1.clone());

                },

                Err(_) => (),

            }
            
        }

        let mut validator_account = Account::from_accounts(
            public_key,
            &changed_accounts,
            &self.accounts_store
        )?;
        
        validator_account.increase_balance(&(&1000000000000_u64).into());

        changed_accounts.insert(*public_key, validator_account);

        let receipts_hashes: Vec<[u8; 32]> = receipts
            .iter()
            .map(|x| x.receipt_hash())
            .collect();

        let receipts_hash = merkle_tree::root(
            blake_3,
            &(receipts_hashes
                .iter()
                .map(|x| x.as_slice())
                .collect::<Vec<_>>()
            )
        );

        let mut accounts = self.accounts.clone();

        // update account details hashes in changed_accounts

        for (address, account) in changed_accounts {

            accounts.insert(*address, account.details_hash);

        }

        let accounts_hashes: Vec<_> = accounts
            .iter()
            .map(|x| {
                merkle_tree::root(
                    blake_3,
                    &[&x.0.0[..], x.1]
                )
            })
            .collect();

        let accounts_hash = merkle_tree::root(
            blake_3,
            &(accounts_hashes
                .iter()
                .map(|x| x.as_slice())
                .collect::<Vec<_>>()
            )
        );

        // calculate transactions hash

        let mut new_block = Block {
            accounts_hash,
            chain: self.latest_block.chain.clone(),
            number: &self.latest_block.number + &Integer::one(),
            previous_block_hash: self.latest_block.block_hash,
            receipts_hash,
            signature: [0_u8;64],
            solar_used,
            time: *target_time,
            transactions,
            validator: *public_key,
            data: "Astreum Foundation Node v0.0.1 by Stelar Labs".as_bytes().into(),
            delay_output: delay_output.join().unwrap_or(Err("VDF error!")?).unwrap_or(Err("VDF error!")?),
            transactions_hash: [0; 32],
            block_hash: [0; 32],
            details_hash: [0; 32],
            delay_difficulty,
        };

        new_block.update_details_hash();

        new_block.sign(secret_key)?;

        new_block.update_block_hash();

        Ok(new_block)

    }
    
}