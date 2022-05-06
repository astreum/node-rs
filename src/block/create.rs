use std::collections::{BTreeMap, HashMap};

use fides::{merkle_root, hash, ed25519};
use opis::Int;

use crate::account::Account;
use crate::accounts::Accounts;
use crate::transaction::receipt::Receipt;
use crate::state::nova;
use crate::transaction::Transaction;

use super::Block;

impl Block {

    pub fn create(
        accounts: &Accounts,
        latest_block: &Block,
        pending_transactions: &BTreeMap<[u8;32],Transaction>,
        private_key: &[u8;32],
        public_key: &[u8;32],
        target_time: &u64
    ) -> (HashMap<[u8;32], Account>, Block) {

        let solar_price = nova::next_solar_price(&latest_block);

        let mut solar_used = Int::zero();

        let mut changed_accounts: HashMap<[u8;32], Account> = HashMap::new();

        let mut transactions = Vec::new();
        
        let mut receipts: Vec<Receipt> = Vec::new();

        for tx in pending_transactions.iter() {
            
            match tx.1.apply(&accounts.store, &mut changed_accounts, &solar_price) {

                Ok(receipt) => {

                    solar_used += receipt.solar_used.clone();

                    receipts.push(receipt);

                    transactions.push(tx.1.clone());

                },

                _ => ()
            }
            
        }

        let validator = Account::from_accounts(public_key, &changed_accounts, &accounts.store);

        let receipts_hash = merkle_root(receipts.iter().map(|x| x.hash()).collect());

        let accounts_hash = merkle_root(
            accounts.details
                .iter()
                .map(|x| {

                    let details_hash = match changed_accounts.get(x.0) {
                        Some(account) => account.hash(),
                        None => *x.1
                    };

                    let address_hash = hash(x.0);

                    merkle_root(vec![address_hash, details_hash])
                    
                })
                .collect()
        );

        let mut block = Block {
            accounts_hash,
            chain: latest_block.chain.clone(),
            number: &latest_block.number + &Int::one(),
            previous_block_hash: latest_block.hash(),
            receipts_hash,
            signature: [0_u8;64],
            solar_price,
            solar_used,
            time: Int::from_bytes(&target_time.to_be_bytes()),
            transactions,
            validator: public_key.clone()
        };

        block.signature = ed25519::sign(&block.hash(), &private_key);

        (changed_accounts, block)
        
    }

}