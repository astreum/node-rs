
use opis::Int;
use std::collections::HashMap;
use std::convert::TryInto;

pub fn current_validator(
    nova_stake_store: &HashMap<[u8; 32], [u8; 32]>,
    missed: u64,
    latest_block_hash: [u8; 32]
)
-> ([u8; 32], HashMap<[u8; 32], [u8; 32]>) {

    let mut latest_block_hash_int: Int = Int::from_bytes(&latest_block_hash.to_vec());

    let mut addresses: Vec<(Int, Int)> = nova_stake_store
        .iter()
        .map(|x| (Int::from_bytes(&x.0.to_vec()), Int::from_bytes(&x.1.to_vec())))
        .collect();

    for _i in 0..missed {

        latest_block_hash_int = latest_block_hash_int.clone().lfsr();

        addresses.sort_by_key(|k| &k.0 ^ &latest_block_hash_int);

        let leader_slots = &addresses[0].1 - &Int::one();

        addresses[0].1 = leader_slots;

        addresses.retain(|x| x.1 > Int::zero())

    }

    addresses.sort_by_key(|k| &k.0 ^ &latest_block_hash_int);

    let current_validator: [u8; 32] = addresses[0].0.to_ext_bytes(32).try_into().unwrap();

    addresses[0].1 = &addresses[0].1 - &Int::one();

    addresses.retain(|x| x.1 > Int::zero());

    let mut new_nova_stake_store: HashMap<[u8; 32], [u8; 32]> = HashMap::new();

    for i in addresses {

        let key = i.0.to_ext_bytes(32).try_into().unwrap();

        let value = i.1.to_ext_bytes(32).try_into().unwrap();

        new_nova_stake_store.insert(key, value);
    }

    (current_validator, new_nova_stake_store)


}