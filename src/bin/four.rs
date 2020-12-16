use cryptopals::*;
use std::io::{self, Read};
use std::str;

fn main() {
    let mut bytes: Vec<u8> = Vec::new();
    io::stdin()
        .read_to_end(&mut bytes)
        .expect("Could not read input.");

    for line in bytes.split(|byte| *byte == b'\n') {
        let line = hex::decode(&line);

        for byte in 0u8..=255u8 {
            let decrypted = xor_single_byte(&line, &byte);
            let score = englishness(&decrypted);

            if let Ok(result) = str::from_utf8(&decrypted) {
                if score < 0.7 {
                    continue;
                }

                println!(
                    "{:.2}: {}\t",
                    score,
                    result
                )
            }
        }
    }
}
