use cryptopals::*;
use std::io::{self, Read};
use std::str;

fn main() {
    let mut bytes: Vec<u8> = Vec::new();
    io::stdin()
        .read_to_end(&mut bytes)
        .expect("Could not read input.");

    let bytes = hex::decode(&bytes);
    let bytes = base64::encode(&bytes);

    println!("{}", str::from_utf8(&bytes).unwrap());
}
