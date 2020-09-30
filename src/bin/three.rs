use std::io::{self, Read};
use std::str;

fn main() {
    let mut bytes: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut bytes).expect("Could not read input.");

    let lhs: Vec<u8> = cryptopals::hex::decode(&bytes);

    // try lhs XOR all ASCII bytes and calculate score
    for byte in 0u8..=255u8 {
        let decrypted: Vec<u8> = cryptopals::xor_single_byte(&lhs, &byte);
        let score = cryptopals::score::englishness(&decrypted);

        println!("(score: {}): {}", score, str::from_utf8(&decrypted).unwrap());
    }
}
