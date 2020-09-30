const BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn encode_chunk(chunk: &[u8]) -> Vec<u8> {
    let first = (chunk[0] & 0b11111100) >> 2;

    if chunk.len() == 3 {
        let second = (chunk[0] & 0b00000011) << 4 | (chunk[1] & 0b11110000) >> 4;
        let third = (chunk[1] & 0b00001111) << 2 | (chunk[2] & 0b11000000) >> 6;
        let fourth = chunk[2] & 0b00111111;

        return vec![
            BASE64_TABLE[first as usize],
            BASE64_TABLE[second as usize],
            BASE64_TABLE[third as usize],
            BASE64_TABLE[fourth as usize],
        ];
    } else if chunk.len() == 2 {
        let second = (chunk[0] & 0b00000011) << 4 | (chunk[1] & 0b11110000) >> 4;
        let third = (chunk[1] & 0b00001111) << 2 | (0 & 0b11000000) >> 6;

        return vec![
            BASE64_TABLE[first as usize],
            BASE64_TABLE[second as usize],
            BASE64_TABLE[third as usize],
            b'=',
        ];
    } else {
        let second = (chunk[0] & 0b00000011) << 4 | (0 & 0b11110000) >> 4;

        return vec![
            BASE64_TABLE[first as usize],
            BASE64_TABLE[second as usize],
            b'=',
            b'=',
        ];
    }
}

pub fn encode(bytes: &[u8]) -> Vec<u8> {
    return bytes.chunks(3).map(encode_chunk).flatten().collect();
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
}
