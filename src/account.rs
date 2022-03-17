
use astro_notation::list;
use crate::merkle_tree_hash;
use fides::hash;
use opis::Int;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Account {
    pub hash: [u8; 32],
    pub balance: Int,
    pub counter: Int,
    pub storage: HashMap<[u8; 32], HashMap<[u8; 32], [u8; 32]>>,
    pub storage_hash: [u8; 32]
}

impl Account {

    pub fn new(value: &Int) -> Self {
        
        let mut res = Account {
            hash: [0_u8; 32],
            balance: value.clone(),
            counter: Int::zero(),
            storage: HashMap::new(),
            storage_hash: [0_u8; 32]
        };

        res.hash = res.hash();

        res

    }

    pub fn hash(&self) -> [u8; 32] {
        merkle_tree_hash(vec![
            hash(&self.balance.to_ext_bytes(32)),
            hash(&self.counter.to_ext_bytes(32)),
            self.storage_hash
        ])
    }

    pub fn storage_hash(&self) -> [u8; 32] {

        if self.storage.is_empty() {
            [0_u8; 32]
        } else {

            let mut hashes = self.storage
                .iter()
                .map(|x| (x.0, store_hash(x.1)))
                .collect::<Vec<_>>();

            hashes.sort_by_key(|k| k.0);

            merkle_tree_hash(
                hashes
                .iter()
                .map(|x| merkle_tree_hash(
                    vec![
                        hash(&x.0.to_vec()),
                        x.1
                    ]
                ))
                .collect::<Vec<_>>()
            )
        
        }
    }

    pub fn from_astro(input: &str) -> Self {
        
        let decoded: Vec<Vec<u8>> = list::as_bytes(input);

        let mut res = Account {
            hash: [0_u8; 32],
            balance: Int::from_bytes(&decoded[1]),
            counter: Int::from_bytes(&decoded[2]),
            storage: HashMap::new(),
            storage_hash: [0_u8; 32]
        };

        res.hash = res.hash();

        res.storage_hash = res.storage_hash();

        res

    }
    
    pub fn to_astro(self) -> String {

        list::from_bytes(vec![
            self.balance.to_ext_bytes(32),
            self.counter.to_ext_bytes(32)
        ])
    }

}

fn store_hash(store: &HashMap<[u8; 32], [u8; 32]>) -> [u8; 32] {

    if store.is_empty() {
        [0_u8; 32]
    } else {

        let mut store_vec = store
            .iter()
            .map(|x| (x.0, x.1))
            .collect::<Vec<_>>();

        store_vec.sort_by_key(|k| k.0);

        merkle_tree_hash(
            store_vec
            .iter()
            .map(|x| merkle_tree_hash(
                vec![
                    hash(&x.0.to_vec()),
                    hash(&x.1.to_vec())
                ]
            ))
            .collect::<Vec<_>>()
        )
    
    }

}

pub fn accounts_hash(accounts: &HashMap<[u8; 32], Account>) -> [u8; 32] {
    
    let mut account_hashes = accounts
        .iter()
        .map(|x| (x.0, x.1.hash))
        .collect::<Vec<_>>();

    account_hashes.sort_by_key(|k| k.0);

    merkle_tree_hash(
        account_hashes
        .iter()
        .map(|x| merkle_tree_hash(vec![*x.0, x.1]))
        .collect::<Vec<_>>()
    )

}
