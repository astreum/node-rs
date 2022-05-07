mod from_accounts;
mod from_bytes;
mod hash;
mod new;
mod storage_hash;
mod to_bytes;
use opis::Int;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Account {
    pub balance: Int,
    pub counter: Int,
    pub storage: BTreeMap<[u8;32], [u8;32]>
}
