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

    let mut results: Vec<(usize, f64)> = Vec::new();

    // guess key sizes in range [2,40]
    for keysize in 2..=40 {
        let b1 = &bytes[0..keysize];
        let b2 = &bytes[keysize..keysize * 2];
        let b3 = &bytes[keysize * 2..keysize * 3];
        let b4 = &bytes[keysize * 3..keysize * 4];

        let distance =
            hamming_distance(&b1, &b2) + hamming_distance(&b2, &b3) + hamming_distance(&b3, &b4);
        let normalized_distance = distance as f64 / (keysize as f64 * 3.0);

        results.push((keysize, normalized_distance));
    }

    results.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    // test breaking xor with top 5 results
    for (keysize, _) in &results[0..5] {
        let mut key: Vec<u8> = Vec::new();

        // we want to transpose
        // [1, 2, 3, 4, 5]
        // [6, 7, 8, 9, 10]
        // [...]
        // to
        // [1, 6, ...]
        // [2, 7, ...]
        // [...]
        let blocks: Vec<&[u8]> = bytes.chunks_exact(*keysize).collect();
        for i in 0..*keysize {
            let transposed_block: Vec<u8> = blocks.iter().map(|block| block[i]).collect();

            // solve single-char xor using technique from before
            let mut results: Vec<(f64, u8)> = (0u8..=255u8)
                .map(|byte| {
                    let decrypted = xor_single_byte(&transposed_block, &byte);
                    let score = englishness(&decrypted);
                    (score, byte)
                })
                .collect();

            results.sort_by(|(a, _), (b, _)| b.partial_cmp(&a).unwrap());

            key.push(results[0].1);
        }

        // ok... now we have a key
        println!("ATTEMPT WITH {}", str::from_utf8(&key).unwrap());
        println!("-----");

        // attempt decrypting text
        let decrypted = xor_repeating_key(&bytes, &key);
        println!("DECRYPTED TEXT {:?}", str::from_utf8(&decrypted).unwrap());
        println!("-----");
    }
}
