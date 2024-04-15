use aes::Aes128;
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use block_modes::{BlockMode, Ecb};
use openssl::rsa::Rsa;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Encrypt, RsaPublicKey};
use std::str;


use block_modes::block_padding::Pkcs7;
type Aes128Ecb = Ecb<Aes128, Pkcs7>;

pub fn random_key(length: usize) -> String {
    let key: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    key
}

pub fn base64_encode<T: AsRef<[u8]>>(input: T) -> String {
    general_purpose::STANDARD.encode(input)
}

pub fn base64_decode(plaintext: &str) -> Result<Vec<u8>> {
    let result = general_purpose::STANDARD.decode(plaintext)?;
    Ok(result)
}

pub fn aes_ecb_encrypt(message: &str, key: &str) -> Result<Vec<u8>> {
    let iv = hex_literal::hex!("010001");
    tracing::debug!("[AES-ECB - Encrypt]key: {:?}", key);
    tracing::debug!("[AES-ECB - Encrypt]iv: {:?}", iv);
    tracing::debug!("[AES-ECB - Encrypt]message: {:?}", message);
    let plaintext = message.as_bytes();
    let byte_key = key.as_bytes();

    let cipher = Aes128Ecb::new_from_slices(&byte_key, &iv)?;
    let encrypted_data = cipher.encrypt_vec(plaintext);
    tracing::debug!("[AES-ECB -  Encrypt]data: {:?}", encrypted_data);
    Ok(encrypted_data)
}

pub fn aes_ecb_decrypt(encrypted_data: &[u8], key: &str) -> Result<Vec<u8>> {
    let iv = hex_literal::hex!("010001");
    tracing::debug!("[AES-ECB - Decrypt]key: {:?}", key);
    tracing::debug!("[AES-ECB - Decrypt]iv: {:?}", iv);
    tracing::debug!("[AES-ECB - Decrypt]message: {:?}", encrypted_data);
    let byte_key = key.as_bytes();
    let cipher = Aes128Ecb::new_from_slices(&byte_key, &iv)?;
    let decrypted_data = cipher.decrypt_vec(&encrypted_data).unwrap();
    tracing::info!("[AES-ECB: Decrypt]data: {:?}", encrypted_data);
    Ok(decrypted_data)
}

pub fn rsa_pkcs1_encrypt(message: &str, pem: &str) -> Result<Vec<u8>> {
    if !pem.starts_with("-----BEGIN PUBLIC KEY-----") {
        return Err(anyhow::anyhow!("Invalid PEM format"));
    }

    let mut rng = rand::thread_rng();
    let pub_key = RsaPublicKey::from_public_key_pem(pem)?;

    let data = message.as_bytes();
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])?;
    tracing::debug!("[RSA - Decrypt]pem: {:?}", &pem);
    tracing::debug!("[RSA - Decrypt]data: {:?}", enc_data);
    Ok(enc_data)
}

pub fn rsa_encrypt(message: &str, pem: &str) -> Result<Vec<u8>> {
    if !pem.starts_with("-----BEGIN PUBLIC KEY-----") {
        return Err(anyhow::anyhow!("Invalid PEM format"));
    }

    let rsa = Rsa::public_key_from_pem(pem.as_bytes())?;
    let mut encr_key = vec![0u8; rsa.size() as usize];
    let data = message.as_bytes();
    let encr_len = rsa.public_encrypt(&data, &mut encr_key, openssl::rsa::Padding::PKCS1)?;
    Ok(encr_key[..encr_len].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        let message = "Hello world!";
        let output = base64_encode(message);
        assert_eq!(output, "SGVsbG8gd29ybGQh");
    }

    #[test]
    fn test_base64_decode() {
        let message = "SGVsbG8gd29ybGQh";
        let output = base64_decode(message).unwrap();
        assert_eq!(output, "Hello world!".as_bytes());
        assert_eq!(str::from_utf8(&output).unwrap(), "Hello world!");
    }

    #[test]
    fn test_aes_ecb_encode() {
        let message = "Hello world!";
        let key = "4krtPt4BBRNN59bT";
        let output = aes_ecb_encrypt(message, key).unwrap();
        assert_eq!(
            output,
            [6, 206, 181, 21, 161, 246, 110, 126, 31, 1, 246, 208, 219, 0, 151, 35]
        );
    }

    #[test]
    fn test_aes_ecb_decode() {
        let message = [
            6, 206, 181, 21, 161, 246, 110, 126, 31, 1, 246, 208, 219, 0, 151, 35,
        ];
        let key = "4krtPt4BBRNN59bT";
        let output = aes_ecb_decrypt(&message, key).unwrap();
        assert_eq!(output, "Hello world!".as_bytes());
        assert_eq!(str::from_utf8(&output).unwrap(), "Hello world!");
    }

    #[test]
    fn test_aes_ecb_base64_encode() {
        let message = "Hello world!";
        let key = "4krtPt4BBRNN59bT";
        let encrypted = aes_ecb_encrypt(message, key).unwrap();
        let output = base64_encode(encrypted);
        assert_eq!(output, "Bs61FaH2bn4fAfbQ2wCXIw==");
    }

    #[test]
    fn test_aes_ecb_base64_decode() {
        let message = "Bs61FaH2bn4fAfbQ2wCXIw==";
        let key = "4krtPt4BBRNN59bT";
        let decrypted = base64_decode(message).unwrap();
        let output = aes_ecb_decrypt(&decrypted, key).unwrap();
        assert_eq!(output, "Hello world!".as_bytes());
        assert_eq!(str::from_utf8(&output).unwrap(), "Hello world!");
    }
}
