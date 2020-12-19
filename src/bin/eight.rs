use cryptopals::*;
use std::io::{self, Read};
// use std::str;
// use openssl::symm::{decrypt, Cipher};

fn main() {
    let mut bytes: Vec<u8> = Vec::new();
    io::stdin()
        .read_to_end(&mut bytes)
        .expect("Could not read input.");
    
    // let key: &[u8] = b"YELLOW SUBMARINE";
    // let cipher = Cipher::aes_128_ecb();

    // stateless and deterministic
    // same 16 byte plaintext block will always produce the same 16 byte ciphertext
    let mut results: Vec<(usize, i32)> = Vec::new();

    // ignore newlines
    for (i, line) in bytes.split(|byte| *byte == b'\n').enumerate() {
        let line = hex::decode(&line);
        let chunks: Vec<&[u8]> = line.chunks(16).collect();
        let mut repeated_blocks: i32 = 0;
        
        for i in 0..chunks.len() - 1 {
            let b1 = chunks[i];
            let b2 = chunks[i + 1];

            repeated_blocks += b1.iter().zip(b2).filter(|&(a, b)| a == b).count() as i32;
        }

        results.push((i, repeated_blocks))
    }

    results.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    let (i, repeated_blocks) = results[0];
    println!("line {} has {} repeated blocks", i, repeated_blocks);
}
