use std::str;

fn main() {
    let bytes = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_vec();
    let bytes = cryptopals::hex::decode(&bytes);
    let bytes = cryptopals::base64::encode(&bytes);

    println!("{}", str::from_utf8(&bytes).unwrap());
}
