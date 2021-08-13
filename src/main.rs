use std::env::args;

fn main() {

    let miner_address: String = args().nth(1).unwrap();
    
    println!("miner_address: {:?}", miner_address);

}
