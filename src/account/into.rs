use super::Account;

impl Into<Vec<u8>> for Account {

    fn into(self) -> Vec<u8> {

        let balance_bytes: Vec<u8> = self.balance.into();

        let counter_bytes: Vec<u8> = self.counter.into();

        let storage_bytes: Vec<Vec<u8>> = self.storage
            .into_iter()
            .map(|x| astro_format::encode(&[&x.0, &x.1]))
            .collect();
        
        astro_format::encode(&[
            &balance_bytes,
            &counter_bytes,
            &astro_format::encode(&(
                storage_bytes.iter().map(|x| x.as_slice()).collect::<Vec<_>>()
            ))
        ])

    }
    
}
