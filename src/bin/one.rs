use cryptopals::*;
use std::io::{self, Read};
use std::str;

fn main() {
    let mut bytes: Vec<u8> = Vec::new();
    io::stdin()
        .read_to_end(&mut bytes)
        .expect("Could not read input.");

    println!(
        "{}",
        str::from_utf8(&hex::decode(&base64::decode(&bytes))).unwrap()
    );
}
