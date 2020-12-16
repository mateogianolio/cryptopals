const BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

// a:  0b 00xxxxxx
// b:  0b 00xxyyyy
// c:  0b 00yyyyzz
// d:  0b 00zzzzzz
fn encode_chunk(chunk: &[u8]) -> Vec<u8> {
    let a = (chunk[0] & 0b11111100) >> 2;

    if chunk.len() == 3 {
        let b = (chunk[0] & 0b00000011) << 4 | (chunk[1] & 0b11110000) >> 4;
        let c = (chunk[1] & 0b00001111) << 2 | (chunk[2] & 0b11000000) >> 6;
        let d = chunk[2] & 0b00111111;

        return vec![
            BASE64_TABLE[a as usize],
            BASE64_TABLE[b as usize],
            BASE64_TABLE[c as usize],
            BASE64_TABLE[d as usize],
        ];
    } else if chunk.len() == 2 {
        let b = (chunk[0] & 0b00000011) << 4 | (chunk[1] & 0b11110000) >> 4;
        let c = (chunk[1] & 0b00001111) << 2;

        return vec![
            BASE64_TABLE[a as usize],
            BASE64_TABLE[b as usize],
            BASE64_TABLE[c as usize],
            b'=',
        ];
    } else {
        let b = (chunk[0] & 0b00000011) << 4;

        return vec![
            BASE64_TABLE[a as usize],
            BASE64_TABLE[b as usize],
            b'=',
            b'=',
        ];
    }
}

// x:  0b aaaaaabb
// y:  0b bbbbcccc
// z:  0b ccdddddd
fn decode_chunk(chunk: &[u8]) -> Vec<u8> {
    let chunk: Vec<u8> = chunk
        .iter()
        .cloned()
        .filter(|byte| *byte != b'=')
        .map(|byte| BASE64_TABLE.iter().position(|&base64_byte| base64_byte == byte).unwrap() as u8)
        .collect();

    if chunk.len() == 2 {
        let x = (chunk[0] & 0b00111111) << 2 | (chunk[1] & 0b00110000) >> 4;
        return vec![x];
    } else if chunk.len() == 3 {
        let x = (chunk[0] & 0b00111111) << 2 | (chunk[1] & 0b00110000) >> 4;
        let y = (chunk[1] & 0b00001111) << 4 | (chunk[2] & 0b00111100) >> 2;
        return vec![x, y];
    } else {
        let x = (chunk[0] & 0b00111111) << 2 | (chunk[1] & 0b00110000) >> 4;
        let y = (chunk[1] & 0b00001111) << 4 | (chunk[2] & 0b00111100) >> 2;
        let z = (chunk[2] & 0b00000011) << 6 | (chunk[3] & 0b00111111);
        return vec![x, y, z];
    }
}

pub fn encode(bytes: &[u8]) -> Vec<u8> {
    return bytes.chunks(3).map(encode_chunk).flatten().collect();
}

pub fn decode(bytes: &[u8]) -> Vec<u8> {
    return bytes.chunks(4).map(decode_chunk).flatten().collect();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode_base64() {
        let input: Vec<u8> = vec![
            0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
        ];
        let output: Vec<u8> = b"AAECAwQFBgcICQoLDA0ODw==".to_vec();

        assert_eq!(super::encode(&input), output);
    }

    #[test]
    fn test_decode_base64() {
        let input: Vec<u8> = b"AAECAwQFBgcICQoLDA0ODw==".to_vec();
        let output: Vec<u8> = vec![
            0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
        ];

        assert_eq!(super::decode(&input), output);
    }
}
