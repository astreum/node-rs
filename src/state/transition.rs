use std::error::Error;

use fides::{merkle_tree, hash::blake_3};

use crate::{block::Block, receipt::Receipt, account::Account};

use super::State;

impl State {

    pub fn transition(&mut self, block: &Block) -> Result<(), Box<dyn Error>> {

        match self.validator(&block.time) {

            Ok((validator, mut changed_accounts)) => {

                if [
                    block.previous_block_hash == self.latest_block.block_hash,
                    block.validator == validator
                ]
                .iter()
                .all(|&x| x) {

                    let mut receipts: Vec<Receipt> = Vec::new();

                    for tx in &block.transactions {

                        match tx.apply(&self.accounts_store, &mut changed_accounts) {
                            
                            Ok(receipt) => receipts.push(receipt),
                            
                            Err(_) => ()
                        
                        }

                    }

                    let mut validator_account = Account::from_accounts(
                        &block.validator,
                        &changed_accounts,
                        &self.accounts_store
                    )?;

                    validator_account.increase_balance(&((&1000000000000_u64).into()));

                    changed_accounts.insert(block.validator, validator_account);

                    let receipts_hashes: Vec<[u8;32]> = receipts
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

                    let accounts_hashes: Vec<_> = self.accounts
                        .iter()
                        .map(|x| {

                            let details_hash = match changed_accounts.get(x.0) {

                                Some(account) => account.details_hash(),

                                None => *x.1

                            };

                            merkle_tree::root(
                                blake_3,
                                &[&x.0.0[..], &details_hash[..]]
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

                    if [
                        receipts_hash == block.receipts_hash,
                        accounts_hash == block.accounts_hash
                    ]
                    .iter()
                    .any(|&x| x == false) {
                        
                        Err("Internal error!")?
                    
                    } else {
                        
                        for (address, account) in changed_accounts {

                            self.accounts.insert(address, account.details_hash());
                            
                            self.accounts_store.put(&address, &account)?;

                        }

                        // change latest_block

                        // add block to block store 

                        Ok(())

                    }
                
                } else {

                    Err("Internal error!")?

                }

            },

            Err(_) => todo!(),

        }

    }
    
}