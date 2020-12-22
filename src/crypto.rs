use openssl::symm::{decrypt, encrypt, Cipher};

pub fn decrypt_aes_128_ecb(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    decrypt(Cipher::aes_128_ecb(), key, None, &bytes).unwrap()
}

pub fn encrypt_aes_128_ecb(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    encrypt(Cipher::aes_128_ecb(), key, None, &bytes).unwrap()
}

// pub fn encrypt_aes_128_cbc(bytes: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
//     let chunks: Vec<&[u8]> = bytes.chunks(16).collect();
//     let mut ciphers: Vec<&[u8]> = chunks.clone();

//     for (i, chunk) in chunks.iter().enumerate() {
//         let chunk = xor_bytes(
//             &pkcs7(&chunk, 16),
//             if i == 0 { &iv } else { &ciphers[i - 1] },
//         );
//         ciphers[i] = encrypt_aes_128_ecb(&chunk, &key);
//     }

//     ciphers.into_iter().flatten().collect::<Vec<u8>>()
// }

// pub fn decrypt_aes_128_cbc(bytes: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
//     let plaintext = iv.to_vec();
//     for (i, chunk) in bytes.chunks(16).enumerate() {
//         let chunk = decrypt_aes_128_ecb(&bytes, &key);
//         plaintext = xor_bytes(&chunk, &iv);
//     }
// }

pub fn pkcs7(bytes: &[u8], block_size: usize) -> Vec<u8> {
    let pad_byte: u8 = (block_size - bytes.len()) as u8;
    let mut padded_bytes: Vec<u8> = vec![pad_byte; block_size];

    for (i, byte) in bytes.iter().enumerate() {
        padded_bytes[i] = *byte;
    }

    padded_bytes
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
    fn test_pkcs7() {
        let input: Vec<u8> = b"YELLOW SUBMARINE".to_vec();
        let block_size: usize = 20;
        let output: Vec<u8> = b"YELLOW SUBMARINE\x04\x04\x04\x04".to_vec();

        assert_eq!(super::pkcs7(&input, block_size), output);
    }
}
