use crate::account::Account;
use crate::address::Address;
use crate::block::Block;
use crate::receipt::Receipt;
use crate::state::State;
use crate::transaction::Transaction;
use fides::hash::blake_3;
use fides::merkle_tree::{root, self};
use neutrondb::Store;
use opis::{Integer};
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use vdf::{WesolowskiVDFParams, VDFParams, VDF};

use super::details_hash;

impl Block {

    pub fn create(
        blocks_store: &Store<Integer, Block>,
        pending_transactions: &BTreeMap<[u8;32],Transaction>,
        public_key: &Address,
        secret_key: &[u8;32],
        state: &mut State,
        target_time: &u64

    ) -> Result<(HashMap<Address, Account>, Block), Box<dyn Error>> {

        let mut solar_used = 0;

        let mut changed_accounts: HashMap<Address, Account> = HashMap::new();

        let mut transactions = Vec::new();
        
        let mut receipts: Vec<Receipt> = Vec::new();

        for tx in pending_transactions.iter() {
            
            match tx.1.apply(&state.accounts_store, &mut changed_accounts) {

                Ok(receipt) => {

                    solar_used += receipt.solar_used;

                    receipts.push(receipt);

                    transactions.push(tx.1.clone());

                },

                _ => (),

            }
            
        }

        let mut validator_account = Account::from_accounts(
            public_key,
            &changed_accounts,
            &state.accounts_store
        )?;
        
        let validator_reward = Integer::from_dec("1000000000")?;
        
        validator_account.increase_balance(&validator_reward);

        changed_accounts.insert(*public_key, validator_account);

        let receipts_hashes: Vec<[u8; 32]> = receipts
            .iter()
            .map(|x| x.receipt_hash())
            .collect();

        let receipts_hash = root(
            blake_3,
            &(receipts_hashes
                .iter()
                .map(|x| x.as_slice())
                .collect::<Vec<_>>()
            )
        );

        let mut accounts = state.accounts.clone();

        // update account details hashes in changed_accounts

        for (address, account) in &changed_accounts {

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

        let vdf = WesolowskiVDFParams(2048).new();

        let challenge = merkle_tree::root(
            blake_3,
            &[
                &public_key.0,
                &state.latest_block.delay_output
            ]
        );

        let int_100: Integer = (&100_u8).into();

        let difficulty = if &state.latest_block.number > &int_100 {

            let past_block_number = &state.latest_block.number - &int_100;

            let past_block = blocks_store.get(&past_block_number)?;

            let block_time_diff = state.latest_block.time - past_block.time;

            state.latest_block.delay_difficulty * (300 / block_time_diff)

        } else {

            1_u64

        };

        let delay_output = vdf
            .solve(&challenge, difficulty)
            .unwrap_or(Err("VDF error!")?);

        let mut new_block = Block {
            accounts_hash,
            chain: state.latest_block.chain.clone(),
            number: &state.latest_block.number + &Integer::one(),
            previous_block_hash: state.latest_block.block_hash,
            receipts_hash,
            signature: [0_u8;64],
            solar_used,
            time: *target_time,
            transactions,
            validator: *public_key,
            data: "Astreum Foundation Node v0.0.1 by Stelar Labs".as_bytes().into(),
            delay_output,
            transactions_hash: [0; 32],
            block_hash: [0; 32],
            details_hash: [0; 32],
            delay_difficulty: difficulty,
        };

        new_block.update_details_hash();

        new_block.sign(secret_key)?;

        // update block hash

        Ok((changed_accounts, new_block))
        
    }

}