use cryptopals::*;
use std::io::{self, Read};
use std::str;

fn main() {
    let mut bytes: Vec<u8> = Vec::new();
    io::stdin()
        .read_to_end(&mut bytes)
        .expect("Could not read input.");

    // ignore newlines
    let bytes: Vec<u8> = bytes
        .iter()
        .cloned()
        .filter(|byte| *byte != b'\n')
        .collect();

    // decode
    let bytes = base64::decode(&bytes);
    let key: &[u8] = b"YELLOW SUBMARINE";

    println!(
        "{}",
        str::from_utf8(&crypto::decrypt_aes_128_ecb(&bytes, &key)).unwrap()
    );
}
