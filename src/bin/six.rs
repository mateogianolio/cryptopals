use cryptopals::*;
use std::io::{self, Read};
use std::str;

struct KeyResult {
    size: usize,
    score: f64,
}

struct XORResult {
    byte: u8,
    score: f64,
}

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

    let bytes = base64::decode(&bytes);

    // guess key sizes in range [2,40]
    let mut results: Vec<KeyResult> = (2..=40)
        .map(|size| {
            let b1 = &bytes[0..size];
            let b2 = &bytes[size..size * 2];
            let b3 = &bytes[size * 2..size * 3];
            let b4 = &bytes[size * 3..size * 4];

            let distance = hamming_distance(&b1, &b2)
                + hamming_distance(&b2, &b3)
                + hamming_distance(&b3, &b4);
            let score = distance as f64 / (size as f64 * 3.0);

            KeyResult { size, score }
        })
        .collect();

    // sort by score
    results.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

    // test breaking xor with top 5 results
    for result in &results[0..5] {
        let mut key_candidate: Vec<u8> = Vec::new();

        // we want to transpose
        // [1, 2, 3, 4, 5]
        // [6, 7, 8, 9, 10]
        // [...]
        // to
        // [1, 6, ...]
        // [2, 7, ...]
        // [...]
        let blocks: Vec<&[u8]> = bytes.chunks_exact(result.size).collect();
        for i in 0..result.size {
            let transposed_block: Vec<u8> = blocks.iter().map(|block| block[i]).collect();

            // solve single-char xor using similar technique as before
            let top_result: XORResult = (0u8..=255u8)
                .map(|byte| XORResult {
                    score: englishness(&crypto::xor_single_byte(&transposed_block, &byte)),
                    byte,
                })
                .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
                .unwrap();

            key_candidate.push(top_result.byte);
        }

        // ok... now we have a key candidate
        println!("ATTEMPT WITH {}", str::from_utf8(&key_candidate).unwrap());

        // attempt decrypting text
        println!(
            "DECRYPTED TEXT {:?}",
            str::from_utf8(&crypto::xor_repeating_key(&bytes, &key_candidate)).unwrap()
        );

        println!("-----");
    }
}
