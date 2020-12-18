use cryptopals::*;
use std::io::{self, Read};
use std::str;
use openssl::symm::{decrypt, Cipher};

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
    let cipher = Cipher::aes_128_ecb();
    let iv: &[u8] = &[0, 16];
    let decrypted = decrypt(cipher, key, Some(iv), &bytes).unwrap();

    println!("{}", str::from_utf8(&decrypted).unwrap());
}
