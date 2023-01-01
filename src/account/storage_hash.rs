use fides::hash::blake_3;
use fides::merkle_tree::root;
use super::Account;

impl Account {

    pub fn storage_hash(&self) -> [u8; 32] {

        let storage = self.storage
            .iter()
            .map(|x| [blake_3(x.0), blake_3(x.1)].concat())
            .collect::<Vec<_>>();
        
        root(
            blake_3,
            &(storage
                .iter()
                .map(|x| x.as_slice())
                .collect::<Vec<_>>()
            )
        )

    }

}
