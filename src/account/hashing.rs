
use crate::account::Account;
use crate::merkle_tree_hash;
use fides::hash;
use std::collections::HashMap;

pub fn accounts(accounts: &HashMap<[u8; 32], Account>) -> [u8; 32] {
    
    let mut account_hashes = accounts
        .iter()
        .map(|x| (x.0, x.1.hash))
        .collect::<Vec<_>>();

    account_hashes.sort_by_key(|k| k.0);

    merkle_tree_hash(
        account_hashes
            .iter()
            .map(|x| merkle_tree_hash(vec![hash(&x.0.to_vec()), x.1]))
            .collect::<Vec<_>>()
    )

}