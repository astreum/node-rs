use crate::accounts::Account;
use crate::blocks::Block;
use crate::state::nova;
use crate::transactions::receipt::Receipt;
use astro_format::string;
use fides::{merkle_root, hash};
use neutrondb::Store;
use opis::Int;
use std::collections::{HashMap, BTreeMap};
use std::error::Error;

impl Block {

    pub fn apply(
        &self,
        accounts: &mut BTreeMap<[u8;32], [u8;32]>,
        accounts_store: &mut Store,
        latest_block: &Block
    ) -> Result<(), Box<dyn Error>> {

        let solar_price = latest_block.next_solar_price();

        let validator = nova::validator_selection(accounts_store, latest_block, &self.time);

        if [
            self.previous_block_hash == latest_block.hash(),
            self.solar_price == solar_price,
            self.validator == validator
        ]
        .iter()
        .all(|&x| x) {

            let mut changed_accounts: HashMap<[u8;32], Account> = HashMap::new();

            let mut receipts: Vec<Receipt> = Vec::new();

            for tx in &self.transactions {

                match tx.apply(&accounts_store, &mut changed_accounts, &solar_price) {
                    
                    Ok(receipt) => receipts.push(receipt),
                    
                    Err(_) => ()
                
                }

            }

            let receipts_hash = merkle_root(
                receipts
                    .iter()
                    .map(|x| x.hash())
                    .collect()
            );

            let accounts_hash = merkle_root(
                accounts
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

            let mut validator_account = Account::from_accounts(&self.validator, &changed_accounts, &accounts_store).unwrap();

            validator_account.balance += Int::from_decimal("1000000000000");

            changed_accounts.insert(self.validator, validator_account);

            if [
                receipts_hash == self.receipts_hash,
                accounts_hash == self.accounts_hash
            ]
            .iter()
            .any(|&x| x == false) {
                
                Err("Internal error!")?
            
            } else {
                
                for (address, account) in changed_accounts {

                    accounts.insert(address, account.hash());

                    let account_key = string::encode::bytes(&address);

                    let account_value = string::encode::bytes(&account.to_bytes());
                    
                    accounts_store.put(&account_key, &account_value).unwrap();

                }

                Ok(())
            }
        
        } else {

            Err("Internal error!")?

        }
        
    }

}
