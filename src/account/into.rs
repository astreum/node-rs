use super::Account;

impl Into<Vec<u8>> for Account {

    fn into(self) -> Vec<u8> {

        let storage_bytes: Vec<Vec<u8>> = self.storage
            .into_iter()
            .map(|x| astro_format::encode(&[&x.0, &x.1]))
            .collect();
        
        astro_format::encode_vec(&[
            self.balance.into(),
            self.counter.into(),
            astro_format::encode_vec(&storage_bytes)
        ])

    }
    
}
