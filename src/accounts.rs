use neutrondb::Store;
use std::collections::BTreeMap;

pub struct Accounts {
    pub details: BTreeMap<[u8;32], [u8;32]>,
    pub store: Store
}
