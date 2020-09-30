use std::io::{self, Read};
use std::str;

fn main() {
    let bytes = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_vec();

    println!("bytes (base16): {:?}", bytes);

    // Convert hex input to bytes
    let bytes: Vec<u8> = cryptopals::hex::decode(&bytes);

    println!("bytes (base10): {:?}", bytes);

    let bytes: Vec<u8> = cryptopals::base64::encode(&bytes);

    println!("bytes (base64): {:?}", bytes);

    println!("str (base64): {}", str::from_utf8(&bytes).unwrap());
}
