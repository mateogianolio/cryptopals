use cryptopals::*;
use std::io::{self, Read};

#[derive(Debug)]
struct LineResult {
    line: usize,
    repeated_blocks: i32,
}

fn main() {
    let mut bytes: Vec<u8> = Vec::new();
    io::stdin()
        .read_to_end(&mut bytes)
        .expect("Could not read input.");

    // ECB is stateless and deterministic
    // ... same 16 byte plaintext block will always produce the same 16 byte ciphertext ...
    // .. let's look for repeated blocks ...
    let top_result: LineResult = bytes
        .split(|byte| *byte == b'\n')
        .map(|line| hex::decode(&line))
        .enumerate()
        .map(|(line_number, line)| {
            let chunks: Vec<&[u8]> = line.chunks(16).collect();
            let mut repeated_blocks: i32 = 0;
            for i in 0..chunks.len() - 1 {
                let b1 = chunks[i];
                let b2 = chunks[i + 1];

                repeated_blocks += b1.iter().zip(b2).filter(|&(a, b)| a == b).count() as i32;
            }

            LineResult {
                line: line_number,
                repeated_blocks,
            }
        })
        .max_by(|a, b| a.repeated_blocks.partial_cmp(&b.repeated_blocks).unwrap())
        .unwrap();

    println!("{:?}", top_result);
}
