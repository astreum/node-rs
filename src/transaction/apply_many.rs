
use crate::account::Account;
use crate::transaction::{apply_one, Receipt, Transaction};
use opis::Int;
use std::collections::HashMap;

pub fn run(
    mut accounts: HashMap<[u8; 32], Account>,
    txs: Vec<Transaction>,
    current_solar_price: &Int
) -> (HashMap<[u8; 32], Account>, Vec<Transaction>, Vec<Receipt>) {

    let mut updated_accs: HashMap<[u8; 32], Account> = HashMap::new();

    let mut applied_txs: Vec<Transaction> = Vec::new();

    let mut receipts: Vec<Receipt> = Vec::new();

    for tx in txs {

        let (updated, receipt) = apply_one::run(&accounts, &tx, &current_solar_price);

        match receipt {

            Some(r) => {

                receipts.push(r);

                applied_txs.push(tx);

                for (address, account) in updated{
                    updated_accs.insert(address, account.clone());
                    accounts.insert(address, account);
                }

            },

            None => ()
        }
    }

    (updated_accs, applied_txs, receipts)

}