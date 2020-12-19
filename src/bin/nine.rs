use cryptopals::*;
use std::io::{self, Read};

fn main() {
    let mut bytes: Vec<u8> = Vec::new();
    io::stdin()
        .read_to_end(&mut bytes)
        .expect("Could not read input.");
    
    // ignore newlines
    for line in bytes.split(|byte| *byte == b'\n') {
        let padded_line = pkcs7(&line, 20);
        println!("{:?}", padded_line);
    }
}
