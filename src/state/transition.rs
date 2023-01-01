use std::{error::Error, sync::Arc, collections::HashMap};
use fides::hash::blake_3;
use fides::merkle_tree::root;
use opis::Integer;
use crate::block::Block;
use crate::account::Account;
use crate::address::Address;
use crate::receipt::Receipt;
use super::{State, consensus};

impl State {

    pub fn transition(&self, new_block: &Block) -> Result<(), Box<dyn Error>> {
        
        let accounts_clone = Arc::clone(&self.accounts);
        let mut accounts = accounts_clone.lock().unwrap_or(Err("")?);

        let accounts_store_clone = Arc::clone(&self.accounts_store);
        let mut accounts_store = accounts_store_clone.lock().unwrap_or(Err("")?);

        let latest_block_clone = Arc::clone(&self.latest_block);
        let latest_block = latest_block_clone.lock().unwrap_or(Err("")?);

        let validator_selection = consensus::selection(
            &accounts_store,
            &latest_block,
            &new_block.time
        )?;

        if [
            new_block.previous_block_hash == latest_block.block_hash(),
            new_block.validator == validator_selection
        ]
        .iter()
        .all(|&x| x) {

            let mut changed_accounts: HashMap<Address, Account> = HashMap::new();

            let mut receipts: Vec<Receipt> = Vec::new();

            for tx in &new_block.transactions {

                match tx.apply(&accounts_store, &mut changed_accounts) {
                    
                    Ok(receipt) => receipts.push(receipt),
                    
                    Err(_) => ()
                
                }

            }

            let mut validator_account = Account::from_accounts(
                &new_block.validator,
                &changed_accounts,
                &accounts_store
            )?;

            validator_account.increase_balance(&Integer::from_dec("1000000000000")?);

            changed_accounts.insert(new_block.validator, validator_account);

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

            if [
                receipts_hash == new_block.receipts_hash,
                accounts_hash == new_block.accounts_hash
            ]
            .iter()
            .any(|&x| x == false) {
                
                Err("Internal error!")?
            
            } else {
                
                for (address, account) in changed_accounts {

                    accounts.insert(address, account.details_hash());
                    
                    accounts_store.put(&address, &account)?;

                }

                // change latest_block

                // add block to block store 

                Ok(())
            }
        
        } else {

            Err("Internal error!")?

        }

    }

}
