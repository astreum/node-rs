use crate::account::Account;
use crate::address::Address;
use crate::block::Block;
use crate::receipt::Receipt;
use crate::transaction::Transaction;
use fides::ed25519;
use fides::hash::blake_3;
use fides::merkle_tree::root;
use neutrondb::Store;
use opis::Integer;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;

impl Block {

    pub fn create(

        accounts: &BTreeMap<Address, [u8;32]>,
        accounts_store: &Store<Address, Account>,
        latest_block: &Block,
        pending_transactions: &BTreeMap<[u8;32],Transaction>,
        secret_key: &[u8;32],
        validator_address: &Address,
        target_time: &u64

    ) -> Result<(HashMap<Address, Account>, Block), Box<dyn Error>> {

        let mut solar_used = 0;

        let mut changed_accounts: HashMap<Address, Account> = HashMap::new();

        let mut transactions = Vec::new();
        
        let mut receipts: Vec<Receipt> = Vec::new();

        for tx in pending_transactions.iter() {
            
            match tx.1.apply(&accounts_store, &mut changed_accounts) {

                Ok(receipt) => {

                    solar_used += receipt.solar_used;

                    receipts.push(receipt);

                    transactions.push(tx.1.clone());

                },

                _ => (),

            }
            
        }

        let mut validator_account = Account::from_accounts(
            validator_address,
            &changed_accounts,
            &accounts_store
        )?;
        
        let validator_reward = Integer::from_dec("1000000000")?;
        
        validator_account.increase_balance(&validator_reward);

        changed_accounts.insert(*validator_address, validator_account);

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

        let accounts_hashes: Vec<_> = accounts
            .iter()
            .map(|x| {

                let details_hash = match changed_accounts.get(x.0) {

                    Some(account) => account.details_hash(),

                    None => *x.1

                };

                root(
                    blake_3,
                    &[&x.0.0[..], &details_hash[..]]
                )
                
            })
            .collect();

        let accounts_hash = root(
            blake_3,
            &(accounts_hashes
                .iter()
                .map(|x| x.as_slice())
                .collect::<Vec<_>>()
            )
        );

        // calculate delay output 

        let mut new_block = Block {
            accounts_hash,
            chain: latest_block.chain.clone(),
            number: &latest_block.number + &Integer::one(),
            previous_block_hash: latest_block.block_hash,
            receipts_hash,
            signature: [0_u8;64],
            solar_used,
            time: *target_time,
            transactions,
            validator: *validator_address,
            data: "Astreum Foundation Node v0.0.1".as_bytes().into(),
            delay_output: vec![],
            transactions_hash: [0; 32],
            block_hash: [0; 32],
            details_hash: [0; 32],
        };

        new_block.update_details_hash();

        new_block.sign(secret_key);

        Ok((changed_accounts, new_block))
        
    }

}