
use astro_notation::list;
use crate::merkle_tree_hash;
use fides::hash;
use opis::Int;
use std::collections::HashMap;
use std::str;
use std::convert::TryInto;

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
        
        let details_array = list::as_bytes(&input);
        
        let balance: Int = Int::from_bytes(&details_array[0]);

        let counter: Int = Int::from_bytes(&details_array[1]);

        let mut acc: Account = Account {
            hash: [0_u8; 32],
            balance: balance,
            counter: counter,
            storage: HashMap::new(),
            storage_hash: [0_u8; 32]
        };

        if details_array.len() == 3 {

            let stores_array = list::as_bytes(str::from_utf8(&details_array[2]).unwrap());

            for store in stores_array {

                let store_id_and_records = list::as_bytes(str::from_utf8(&store).unwrap());

                let store_id: [u8; 32] = store_id_and_records[0].clone().try_into().unwrap();

                let records = list::as_bytes(str::from_utf8(&store_id_and_records[1]).unwrap());

                let mut store_map: HashMap<[u8; 32], [u8; 32]> = HashMap::new();

                for record in records {

                    let key_and_value = list::as_bytes(str::from_utf8(&record).unwrap());

                    let key: [u8; 32] = key_and_value[0].clone().try_into().unwrap();

                    let value: [u8; 32] = key_and_value[1].clone().try_into().unwrap();

                    store_map.insert(key, value).unwrap();
                
                }

                acc.storage.insert(store_id, store_map).unwrap();

            }

            acc.storage_hash = acc.storage_hash();

        }

        acc.hash = acc.hash();

        acc

    }
    
    pub fn to_astro(&self) -> String {

        let mut balance_counter_storage = vec![
            self.balance.to_bytes(),
            self.counter.to_bytes()
        ];

        let mut ids_and_records: Vec<String> = Vec::new();

        for (id, records) in &self.storage {

            let mut keys_and_values: Vec<String> = Vec::new();

            for (key, value) in records {

                let key_and_value: String = list::from_bytes(vec![key.to_vec(), value.to_vec()]);

                keys_and_values.push(key_and_value)

            }

            let records_str: String = list::from_bytes(
                keys_and_values
                    .iter()
                    .map(|x| x.as_bytes().to_vec())
                    .collect()
                );


            let id_and_records = list::from_bytes(vec![
                id.to_vec(),
                records_str.as_bytes().to_vec()
            ]);

            ids_and_records.push(id_and_records)

        }

        if !self.storage.is_empty() {
            
            let storage_str: String = list::from_bytes(
                ids_and_records
                    .iter()
                    .map(|x| x.as_bytes().to_vec())
                    .collect()
            );

            balance_counter_storage.push(storage_str.as_bytes().to_vec())

        }

        list::from_bytes(balance_counter_storage)

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
        .map(|x| merkle_tree_hash(vec![hash(&x.0.to_vec()), x.1]))
        .collect::<Vec<_>>()
    )

}
