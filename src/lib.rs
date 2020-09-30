pub mod base64;
pub mod hex;
pub mod score;

pub fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b).map(|(a, b)| a ^ b).collect()
}

pub fn xor_single_byte(a: &[u8], byte: &u8) -> Vec<u8> {
    xor_bytes(a, &vec![*byte; a.len()])
}

#[cfg(test)]
mod tests {}
