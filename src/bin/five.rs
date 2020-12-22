use cryptopals::*;
use std::io::{self, Read};
use std::str;

fn main() {
    let mut bytes: Vec<u8> = Vec::new();
    io::stdin()
        .read_to_end(&mut bytes)
        .expect("Could not read input.");

    for line in bytes.split(|byte| *byte == b'\n') {
        let line = hex::encode(&crypto::xor_repeating_key(&line, b"ICE"));
        println!("{}", str::from_utf8(&line).unwrap());
    }
}
