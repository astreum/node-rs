use std::collections::{BTreeMap, HashMap};

use fides::{merkle_root, hash};
use opis::Int;

use crate::{transaction::{Transaction, receipt::Receipt, apply::ApplyTxArg}, state::nova, accounts::Accounts, account::Account};

use super::Block;

impl Block {

    pub fn create(
        accounts: &Accounts,
        latest_block: &Block,
        pending: &BTreeMap<[u8;32],Transaction>,
        public_key: &[u8;32]
    ) -> Block {

        let solar_price = nova::solar(&latest_block.solar_price, &latest_block.solar_used);

        let mut solar_used = Int::zero();

        let mut changed: HashMap<[u8;32], Account> = HashMap::new();

        let mut transactions = Vec::new();
        
        let mut receipts: Vec<Receipt> = Vec::new();

        for tx in pending.iter() {

            let apply_tx_arg = ApplyTxArg {
                accounts_store: &accounts.store,
                latest_block,
                solar_price: &solar_price
            };

            match tx.1.apply(apply_tx_arg) {

                Ok(apply_tx_res) => {

                    for (address, account) in apply_tx_res.accounts {

                        changed.insert(address, account);
                    }

                    solar_used += apply_tx_res.receipt.solar_used.clone();

                    receipts.push(apply_tx_res.receipt);

                    transactions.push(tx.1.clone());

                },

                _ => ()
            }
            
        }

        let validator = Account::from_accounts(public_key, &changed, &accounts.store).unwrap();

        let receipts_hash = merkle_root(receipts.iter().map(|x| x.hash()).collect());

        let accounts_hash = merkle_root(
            accounts.details
                .iter()
                .map(|x| {

                    let details_hash = match changed.get(x.0) {
                        Some(account) => account.hash(),
                        None => *x.1
                    };

                    let address_hash = hash(x.0);

                    merkle_root(vec![address_hash, details_hash])
                    
                })
                .collect()
        );

        let block = Block {
            accounts_hash,
            chain: latest_block.chain.clone(),
            number: &latest_block.number + &Int::one(),
            previous_block_hash: latest_block.hash(),
            receipts_hash: receipts_hash,
            signature: [0_u8;64],
            solar_price,
            solar_used,
            time: Int::zero(),
            transactions,
            validator: public_key.clone()
        };

        block
        
    }

}