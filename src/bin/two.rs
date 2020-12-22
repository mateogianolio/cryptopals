use cryptopals::*;
use std::io::{self, Read};
use std::str;

fn main() {
    let mut bytes: Vec<u8> = Vec::new();
    io::stdin()
        .read_to_end(&mut bytes)
        .expect("Could not read input.");

    let lines: Vec<Vec<u8>> = bytes
        .split(|byte| *byte == b'\n')
        .map(|line| hex::decode(&line))
        .collect();

    let (a, b) = (&lines[0], &lines[1]);

    println!(
        "{}",
        str::from_utf8(&hex::encode(&crypto::xor_bytes(&a, &b))).unwrap()
    );
}
