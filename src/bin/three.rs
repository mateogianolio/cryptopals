use std::str;

fn main() {
    let lhs = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_vec();
    let lhs = cryptopals::hex::decode(&lhs);

    // try lhs XOR all ASCII bytes and calculate score
    for byte in 0u8..=255u8 {
        let decrypted: Vec<u8> = cryptopals::xor_single_byte(&lhs, &byte);
        let score = cryptopals::score::englishness(&decrypted);

        println!(
            "(score: {}): {}",
            score,
            str::from_utf8(&decrypted).unwrap()
        );
    }
}
