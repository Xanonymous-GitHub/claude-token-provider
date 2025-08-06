//! Utility to generate encrypted constants for the main application
//!
//! This utility encrypts the JSON configuration data and outputs the
//! ciphertext in a format that can be embedded in the constants.rs file.

use claude_token_provider::crypto::{constants::ORIGINAL_JSON, encrypt_data};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Claude Token Provider - Constant Generator ===\n");

    let key = br#"00000000000000000000000000000000"#;
    let nonce = br#"000000000000"#;

    // Encrypt the original JSON
    println!("Original JSON to encrypt:");
    println!("{}", ORIGINAL_JSON);
    println!();

    let json_bytes = ORIGINAL_JSON.as_bytes();
    let encrypted = encrypt_data(json_bytes, &key, &nonce)?;

    println!("Encrypted ciphertext ({} bytes):", encrypted.len());
    println!();

    // Generate the Rust array format
    println!("Copy the following into src/crypto/constants.rs:");
    println!();
    println!("pub const ENCRYPTED_CONFIG: &[u8] = &[");

    // Format as Rust byte array with proper line wrapping
    for (i, byte) in encrypted.iter().enumerate() {
        if i % 12 == 0 {
            print!("    ");
        }
        print!("{:#04x},", byte);

        if (i + 1) % 12 == 0 {
            println!();
        } else {
            print!(" ");
        }
    }

    if encrypted.len() % 12 != 0 {
        println!();
    }

    println!("];");
    println!();

    // Verify the encryption worked by attempting to decrypt
    use claude_token_provider::crypto::decrypt_data;

    println!("Verification: Attempting to decrypt...");
    let decrypted = decrypt_data(&encrypted, &key, &nonce)?;
    let decrypted_text = String::from_utf8(decrypted)?;

    if decrypted_text == ORIGINAL_JSON {
        println!("✓ Encryption/decryption verified successfully!");
    } else {
        println!("❌ Verification failed - decrypted text doesn't match original!");
        println!("Decrypted: {}", decrypted_text);
    }

    println!();
    Ok(())
}
