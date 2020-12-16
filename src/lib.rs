pub mod base64;
pub mod hex;

const LETTERS: usize = 27;
const FREQUENCY: [f64; LETTERS] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, // A-G
    0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, // H-N
    0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, // O-U
    0.00978, 0.02360, 0.00150, 0.01974, 0.00074, // V-Z
    0.13000 // space
];

pub fn englishness(bytes: &[u8]) -> f64 {
    let mut counter: [f64; LETTERS] = [0.; LETTERS];
    for byte in bytes.iter() {
        let lowercase_byte = byte.to_ascii_lowercase();
        if let b' ' = lowercase_byte {
            counter[26] += 1.;
        } else if let b'a'..=b'z' = lowercase_byte {
            counter[(lowercase_byte - b'a') as usize] += 1.;
        }
    }

    counter
        .iter()
        .enumerate()
        .map(|(i, count)| (FREQUENCY[i] * count / bytes.len() as f64).sqrt())
        .sum()
}

pub fn distance(a: &[u8], b: &[u8]) -> i32 {
    let mut differing_bits: i32 = 0;
    for mut byte in xor_bytes(a, b) {
        while byte != 0 {
            differing_bits += (byte & 1) as i32;
            byte >>= 1;
        }
    }

    differing_bits
}

pub fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b).map(|(a, b)| a ^ b).collect()
}

pub fn xor_single_byte(a: &[u8], byte: &u8) -> Vec<u8> {
    xor_bytes(a, &vec![*byte; a.len()])
}

pub fn xor_repeating_key(a: &[u8], key: &[u8]) -> Vec<u8> {
    let b: Vec<u8> = key.iter().cloned().cycle().take(a.len()).collect();

    xor_bytes(a, &b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_distance() {
        let a: Vec<u8> = b"this is a test".to_vec();
        let b: Vec<u8> = b"wokka wokka!!!".to_vec();

        let output: i32 = 37;

        assert_eq!(super::distance(&a, &b), output)
    }
}
