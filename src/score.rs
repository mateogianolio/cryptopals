const LETTERS: usize = 26;
const FREQUENCY: [f64; LETTERS] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015,  // A-G
    0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749,  // H-N
    0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758,  // O-U
    0.00978, 0.02360, 0.00150, 0.01974, 0.00074                     // V-Z
];

pub fn englishness(bytes: &[u8]) -> f64 {
    let mut counter: [f64; LETTERS] = [0.; LETTERS];
    for byte in bytes.iter() {
        let lowercase_byte = byte.to_ascii_lowercase();
        match lowercase_byte {
            b'a'..=b'z' => counter[(lowercase_byte - b'a') as usize] += 1.,
            _ => ()
        }
    }

    counter
        .iter()
        .enumerate()
        .map(|(i, count)| (FREQUENCY[i] * count / bytes.len() as f64).sqrt())
        .sum()
}