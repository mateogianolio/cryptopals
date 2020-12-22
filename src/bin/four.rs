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

    let top_result: XORResult = bytes
        .split(|byte| *byte == b'\n')
        .map(|line| hex::decode(&line))
        .map(|line| {
            (0u8..=255u8)
                .map(|byte| {
                    let bytes = crypto::xor_single_byte(&line, &byte);
                    let score = englishness(&bytes);

                    XORResult { score, bytes }
                })
                .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
                .unwrap()
        })
        .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
        .unwrap();

    println!("{}", str::from_utf8(&top_result.bytes).unwrap());
}
