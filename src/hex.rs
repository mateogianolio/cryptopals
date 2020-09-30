const HEX_TABLE: &[u8; 16] = b"0123456789abcdef";

fn decode_byte(byte: &u8) -> u8 {
    match byte {
        b'A'..=b'F' => byte - b'A' + 10,
        b'a'..=b'f' => byte - b'a' + 10,
        b'0'..=b'9' => byte - b'0',
        _ => panic!("Invalid hex char"),
    }
}

fn decode_chunk(chunk: &[u8]) -> u8 {
    decode_byte(&chunk[0]) * 16 + decode_byte(&chunk[1])
}

pub fn decode(bytes: &[u8]) -> Vec<u8> {
    bytes
        .chunks(2)
        .map(decode_chunk)
        .collect()
}

fn encode_chunk(chunk: &[u8]) -> Vec<u8> {
    vec![
        HEX_TABLE[((chunk[0] >> 4) & 0b00001111) as usize],
        HEX_TABLE[(chunk[0] & 0b00001111) as usize],
    ]
}

pub fn encode(bytes: &[u8]) -> Vec<u8> {
    bytes
        .chunks(1)
        .map(encode_chunk)
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_decode_hex() {
        let input: Vec<u8> = b"000102030405060708090a0b0c0d0e0f".to_vec();
        let output: Vec<u8> = vec![0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8];

        assert_eq!(super::decode(&input), output);
    }

    #[test]
    fn test_encode_hex() {
        let input: Vec<u8> = vec![0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8];
        let output: Vec<u8> = b"000102030405060708090a0b0c0d0e0f".to_vec();

        assert_eq!(super::encode(&input), output);
    }
}
