
use stellar_notation::{
    StellarObject,
    byte_decode,
    value_get,
    list_get
};

pub struct State {
    accounts: Vec<StellarObject>,
    account: Vec<StellarObject>,
    code: Vec<StellarObject>,
    stores: Vec<StellarObject>
}

pub struct Tx {
    sender: String,
    to: String,
    amount: u128
}

pub fn transform(mut state: State, txs: Vec<Tx>) -> (State, Vec<u8>) {

    let mut receipts = Vec::new();

    for tx in txs {

        let sender_account_hash = list_get::as_str(&state.accounts, &tx.sender).unwrap();

        let sender_account = list_get::as_bytes(&state.account, &sender_account_hash).unwrap();
        
        let sender_account_data = byte_decode::list(&sender_account);

        let sender_counter = list_get::as_uint64(&sender_account_data, "counter").unwrap();

        let sender_balance = list_get::as_uint128(&sender_account_data, "balance").unwrap();



    }

    return (state, receipts)

}