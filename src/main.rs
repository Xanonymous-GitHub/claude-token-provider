//! Claude Token Provider - Main Application
//!
//! A secure, self-deleting configuration management tool demonstrating
//! advanced Rust concepts including cryptography, error handling, and
//! systems programming.

use claude_token_provider::{
    config::file_ops::apply_config_update,
    crypto::{constants::ENCRYPTED_CONFIG, decrypt_data},
    input::{display_banner, get_nonce, get_secret_key},
    self_deletion::perform_self_deletion,
    Result, TokenProviderError,
};

fn main() -> Result<()> {
    // Display application banner and warnings
    display_banner();

    // Execute main application logic
    match run_application() {
        Ok(()) => {
            println!("\n✓ Configuration successfully applied!");
        }
        Err(e) => {
            eprintln!("\n❌ Application failed: {}", e.user_message());
            eprintln!("Technical details: {}", e);

            // Still attempt self-deletion even on failure
            if let Err(deletion_error) = perform_self_deletion() {
                eprintln!("Additional error during cleanup: {}", deletion_error);
                return Err(e); // Return original error
            }

            return Err(e);
        }
    }

    // Attempt self-deletion
    match perform_self_deletion() {
        Ok(()) => {
            println!("🗑️  Executable successfully removed");
            Ok(())
        }
        Err(deletion_error) => {
            eprintln!("⚠️  Configuration applied but cleanup failed");
            Err(deletion_error)
        }
    }
}

/// Main application workflow
fn run_application() -> Result<()> {
    println!("🔐 Starting secure configuration update...\n");

    // Step 1: Get user inputs
    println!("Step 1: Acquiring decryption credentials");
    let key = get_secret_key()?;
    let nonce = get_nonce()?;

    // Step 2: Decrypt hardcoded configuration
    println!("\nStep 2: Decrypting configuration data");
    if ENCRYPTED_CONFIG.is_empty() {
        return Err(TokenProviderError::CryptoError(
            "No encrypted configuration data found. Please run Phase 8 to generate encrypted constants.".to_string()
        ));
    }

    let decrypted_bytes = decrypt_data(ENCRYPTED_CONFIG, &key, &nonce)?;

    // Step 3: Parse JSON
    println!("Step 3: Parsing configuration JSON");
    let decrypted_text = String::from_utf8(decrypted_bytes).map_err(|e| {
        TokenProviderError::CryptoError(format!("Decrypted data is not valid UTF-8: {}", e))
    })?;

    let config_json: serde_json::Value =
        serde_json::from_str(&decrypted_text).map_err(TokenProviderError::JsonError)?;

    // Step 4: Apply configuration
    println!("Step 4: Applying configuration to file system");
    apply_config_update(config_json)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_application_components() {
        // This would be a comprehensive integration test
        // For now, just ensure the main components compile
        assert!(true);
    }
}
