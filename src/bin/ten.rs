use cryptopals::*;
use openssl::symm::{decrypt, Cipher};
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
    let iv: &[u8] = [0u8; 16];
    let decrypted = crypto::decrypt_aes_128_cbc(&bytes, &key, &iv).unwrap();

    println!("{}", str::from_utf8(&decrypted).unwrap());
}
