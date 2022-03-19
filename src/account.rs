
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

                let store_array = list::as_bytes(str::from_utf8(&store).unwrap());

                let store_id: [u8; 32] = store_array[0].clone().try_into().unwrap();

                let records = list::as_bytes(str::from_utf8(&store_array[1]).unwrap());
                
                let mut store_map: HashMap<[u8; 32], [u8; 32]> = HashMap::new();

                for record in records {

                    let key_and_value = list::as_bytes(str::from_utf8(&record).unwrap());

                    let key: [u8; 32] = key_and_value[0].clone().try_into().unwrap();

                    let value: [u8; 32] = key_and_value[1].clone().try_into().unwrap();

                    store_map.insert(key, value);
                
                }

                acc.storage.insert(store_id, store_map);

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

        let mut stores: Vec<String> = Vec::new();

        for store in &self.storage {

            let mut keys_and_values: Vec<String> = Vec::new();

            for record in store.1 {

                let key_and_value: String = list::from_bytes(vec![record.0.to_vec(), record.1.to_vec()]);

                println!(" * key_and_value: {:?}", key_and_value);

                keys_and_values.push(key_and_value)

            }

            let records_str: String = list::from_bytes(
                keys_and_values
                    .iter()
                    .map(|x| x.as_bytes().to_vec())
                    .collect()
                );

            println!(" * records_str: {:?}", records_str);


            let id_and_records = list::from_bytes(vec![
                store.0.to_vec(),
                records_str.as_bytes().to_vec()
            ]);

            stores.push(id_and_records)

        }

        if !stores.is_empty() {
            
            let stores_str: String = list::from_bytes(
                stores
                    .iter()
                    .map(|x| x.as_bytes().to_vec())
                    .collect()
            );

            balance_counter_storage.push(stores_str.as_bytes().to_vec())

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
