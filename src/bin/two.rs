use std::io::{self, Read};
use std::str;

fn main() {
    let mut bytes: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut bytes).expect("Could not read input.");

    let mut iter = bytes.split(|byte| *byte == b'\n');

    let lhs = cryptopals::hex::decode(&iter.next().unwrap());
    let rhs = cryptopals::hex::decode(&iter.next().unwrap());

    let xor = cryptopals::xor_bytes(&lhs, &rhs);
    let xor = cryptopals::hex::encode(&xor);

    println!("bytes (xor): {:?}", xor);
    println!("str (xor): {}", str::from_utf8(&xor).unwrap());
}
