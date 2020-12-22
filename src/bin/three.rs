use cryptopals::*;
use std::io::{self, Read};
use std::str;

struct XORResult {
    score: f64,
    bytes: Vec<u8>,
}

fn main() {
    let mut bytes: Vec<u8> = Vec::new();
    io::stdin()
        .read_to_end(&mut bytes)
        .expect("Could not read input.");

    let bytes = hex::decode(&bytes);
    let top_result: XORResult = (0u8..=255u8)
        .map(|byte| {
            let bytes = crypto::xor_single_byte(&bytes, &byte);
            let score = englishness(&bytes);

            XORResult { score, bytes }
        })
        .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
        .unwrap();

    println!("{}", str::from_utf8(&top_result.bytes).unwrap());
}
