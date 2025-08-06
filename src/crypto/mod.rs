//! Cryptographic operations for secure data handling
//!
//! This module provides AES-256-GCM encryption and decryption
//! with proper key validation and error handling.

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose, Engine as _};

use crate::errors::{Result, TokenProviderError};

pub mod constants;

/// AES-GCM key size in bytes (256 bits)
pub const KEY_SIZE: usize = 32;
/// AES-GCM nonce/IV size in bytes (96 bits)
pub const NONCE_SIZE: usize = 12;

/// Validates and decodes a base64-encoded key
pub fn decode_and_validate_key(base64_key: &str) -> Result<[u8; KEY_SIZE]> {
    let decoded = general_purpose::STANDARD
        .decode(base64_key)
        .map_err(TokenProviderError::InvalidBase64)?;

    if decoded.len() != KEY_SIZE {
        return Err(TokenProviderError::InvalidKeyLength {
            actual: decoded.len(),
        });
    }

    let mut key = [0u8; KEY_SIZE];
    key.copy_from_slice(&decoded);
    Ok(key)
}

/// Validates and decodes a base64-encoded nonce/IV
pub fn decode_and_validate_nonce(base64_nonce: &str) -> Result<[u8; NONCE_SIZE]> {
    let decoded = general_purpose::STANDARD
        .decode(base64_nonce)
        .map_err(TokenProviderError::InvalidBase64)?;

    if decoded.len() != NONCE_SIZE {
        return Err(TokenProviderError::InvalidIvLength {
            actual: decoded.len(),
        });
    }

    let mut nonce = [0u8; NONCE_SIZE];
    nonce.copy_from_slice(&decoded);
    Ok(nonce)
}

/// Encrypts data using AES-256-GCM
///
/// # Arguments
/// * `data` - The plaintext data to encrypt
/// * `key` - 32-byte encryption key
/// * `nonce` - 12-byte nonce/IV
///
/// # Returns
/// * `Ok(Vec<u8>)` - The encrypted ciphertext
/// * `Err(TokenProviderError)` - If encryption fails
pub fn encrypt_data(
    data: &[u8],
    key: &[u8; KEY_SIZE],
    nonce: &[u8; NONCE_SIZE],
) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);

    cipher
        .encrypt(nonce, data)
        .map_err(|e| TokenProviderError::CryptoError(e.to_string()))
}

/// Decrypts data using AES-256-GCM
///
/// # Arguments
/// * `ciphertext` - The encrypted data
/// * `key` - 32-byte decryption key
/// * `nonce` - 12-byte nonce/IV used for encryption
///
/// # Returns
/// * `Ok(Vec<u8>)` - The decrypted plaintext
/// * `Err(TokenProviderError)` - If decryption or authentication fails
pub fn decrypt_data(
    ciphertext: &[u8],
    key: &[u8; KEY_SIZE],
    nonce: &[u8; NONCE_SIZE],
) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| TokenProviderError::CryptoError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = [1u8; 32];
        let nonce = [2u8; 12];
        let plaintext = b"Hello, World!";

        let ciphertext = encrypt_data(plaintext, &key, &nonce).unwrap();
        let decrypted = decrypt_data(&ciphertext, &key, &nonce).unwrap();

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_invalid_key_length() {
        let short_key = "dGVzdA=="; // "test" in base64 (4 bytes)
        let result = decode_and_validate_key(short_key);

        assert!(matches!(
            result,
            Err(TokenProviderError::InvalidKeyLength { actual: 4 })
        ));
    }

    #[test]
    fn test_invalid_nonce_length() {
        let short_nonce = "dGVzdA=="; // "test" in base64 (4 bytes)
        let result = decode_and_validate_nonce(short_nonce);

        assert!(matches!(
            result,
            Err(TokenProviderError::InvalidIvLength { actual: 4 })
        ));
    }

    #[test]
    fn test_invalid_base64() {
        let invalid_base64 = "not_base64!@#";
        let result = decode_and_validate_key(invalid_base64);

        assert!(matches!(result, Err(TokenProviderError::InvalidBase64(_))));
    }

    #[test]
    fn test_authentication_failure() {
        let key = [1u8; 32];
        let nonce = [2u8; 12];
        let plaintext = b"Hello, World!";

        let mut ciphertext = encrypt_data(plaintext, &key, &nonce).unwrap();

        // Tamper with the ciphertext
        if let Some(byte) = ciphertext.get_mut(0) {
            *byte = byte.wrapping_add(1);
        }

        let result = decrypt_data(&ciphertext, &key, &nonce);
        assert!(matches!(result, Err(TokenProviderError::CryptoError(_))));
    }
}
