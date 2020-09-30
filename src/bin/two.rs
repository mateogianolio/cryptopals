use std::str;

fn main() {
    let lhs = b"1c0111001f010100061a024b53535009181c".to_vec();
    let lhs = cryptopals::hex::decode(&lhs);

    let rhs = b"686974207468652062756c6c277320657965".to_vec();
    let rhs = cryptopals::hex::decode(&rhs);

    let xor = cryptopals::xor_bytes(&lhs, &rhs);
    let xor = cryptopals::hex::encode(&xor);

    println!("{}", str::from_utf8(&xor).unwrap());
}
