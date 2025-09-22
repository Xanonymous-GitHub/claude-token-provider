use rpassword::prompt_password;
use std::io::{self, Write};

use crate::crypto::{decode_and_validate_key, decode_and_validate_nonce, KEY_SIZE, NONCE_SIZE};
use crate::{Result, TokenProviderError};

/// Validates base64 input format before attempting decode
pub fn validate_base64_format(input: &str) -> Result<()> {
    // Check for valid base64 characters
    let valid_chars = input
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=');

    if !valid_chars {
        return Err(TokenProviderError::InvalidBase64(
            base64::DecodeError::InvalidByte(0, 0), // Placeholder error
        ));
    }

    // Check padding
    let padding_count = input.chars().rev().take_while(|&c| c == '=').count();
    if padding_count > 2 {
        return Err(TokenProviderError::InvalidBase64(
            base64::DecodeError::InvalidPadding,
        ));
    }

    Ok(())
}

/// Enhanced key input with format validation
pub fn get_secret_key() -> Result<[u8; KEY_SIZE]> {
    loop {
        let key_input = prompt_password("Enter AES-256-GCM Secret Key (Base64): ")
            .map_err(|e| TokenProviderError::IoError(io::Error::new(io::ErrorKind::Other, e)))?;

        // Pre-validate format
        if let Err(_) = validate_base64_format(&key_input) {
            eprintln!("Error: Invalid Base64 format");
            eprintln!("Please ensure your input contains only valid Base64 characters (A-Z, a-z, 0-9, +, /, =)\n");
            continue;
        }

        match decode_and_validate_key(&key_input) {
            Ok(key) => return Ok(key),
            Err(e) => {
                eprintln!("Error: {}", e.user_message());
                if !e.is_recoverable() {
                    return Err(e);
                }
                eprintln!("Expected: 44 Base64 characters (32 bytes when decoded)\n");
            }
        }
    }
}

/// Prompts user for the AES-GCM IV/Nonce
pub fn get_nonce() -> Result<[u8; NONCE_SIZE]> {
    print!("Enter AES-256-GCM IV/Nonce (Base64): ");
    io::stdout().flush().map_err(TokenProviderError::IoError)?;

    loop {
        let mut nonce_input = String::new();
        io::stdin()
            .read_line(&mut nonce_input)
            .map_err(TokenProviderError::IoError)?;

        let nonce_input = nonce_input.trim();

        // Pre-validate format
        if let Err(_) = validate_base64_format(nonce_input) {
            eprintln!("Error: Invalid Base64 format");
            eprintln!("Please ensure your input contains only valid Base64 characters (A-Z, a-z, 0-9, +, /, =)");
            print!("Please try again: ");
            io::stdout().flush().map_err(TokenProviderError::IoError)?;
            continue;
        }

        match decode_and_validate_nonce(nonce_input) {
            Ok(nonce) => return Ok(nonce),
            Err(e) => {
                eprintln!("Error: {}", e.user_message());
                if !e.is_recoverable() {
                    return Err(e);
                }
                eprintln!("Expected: 16 Base64 characters (12 bytes when decoded)");
                print!("Please try again: ");
                io::stdout().flush().map_err(TokenProviderError::IoError)?;
            }
        }
    }
}

pub const APP_TOKEN: &str = env!("APP_TOKEN");
/// Displays application banner and instructions
pub fn display_banner() {
    println!("      Claude Token Provider    ");
    println!("   Secure Configuration Manager");
    println!("=================================\n");

    println!("This tool will decrypt and apply configuration settings.");
    println!("You will need to provide:");
    println!("  1. Secret Key (32 bytes, Base64-encoded)");
    println!("  2. IV/Nonce (12 bytes, Base64-encoded)");
    println!();
    println!("Copyright (c) Xanonymous\n");
    println!("Build: {}", APP_TOKEN);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_base64_format() {
        // Valid base64
        assert!(validate_base64_format("SGVsbG8gV29ybGQ=").is_ok());
        assert!(validate_base64_format("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=").is_ok());

        // Invalid characters
        assert!(validate_base64_format("Hello@World!").is_err());
        assert!(validate_base64_format("SGVsbG8gV29ybGQ===").is_err()); // Too much padding
    }
}
