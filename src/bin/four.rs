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

    let mut results: Vec<(f64, Vec<u8>)> = lines
        .iter()
        .map(|line| {
            let mut results: Vec<(f64, Vec<u8>)> = (0u8..=255u8)
                .map(|byte| {
                    let decrypted = xor_single_byte(&line, &byte);
                    let score = score::englishness(&decrypted);
                    (score, decrypted)
                })
                .collect();

            results.sort_by(|(a, _), (b, _)| b.partial_cmp(&a).unwrap());
            results.first().cloned().unwrap()
        })
        .collect();

    results.sort_by(|(a, _), (b, _)| b.partial_cmp(&a).unwrap());
    results.iter().take(5).for_each(|(score, decrypted)| {
        println!(
            "{:.2}: {}\t",
            score,
            str::from_utf8(&decrypted).unwrap_or("Could not parse string.")
        )
    });
}
