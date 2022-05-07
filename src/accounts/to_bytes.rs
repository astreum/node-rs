use astro_format::arrays;
use crate::accounts::Account;

impl Account {
    
    pub fn to_bytes(&self) -> Vec<u8> {

        let encoded_key_values = self.storage
            .iter()
            .map(|(k, v)| arrays::encode(&[k, v]))
            .collect::<Vec<_>>();

        let mut storage_bytes: Vec<&[u8]> = Vec::new();

        for i in 0..encoded_key_values.len() {
            storage_bytes.push(&encoded_key_values[i]);
        }

        arrays::encode(&[
            &self.balance.to_bytes(),
            &self.counter.to_bytes(),
            &arrays::encode(&storage_bytes)
        ])

    }

}