
use std::io;

pub fn get() -> String {

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let result: String = input.trim().to_owned();

    return result

}