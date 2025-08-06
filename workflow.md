# Claude Token Provider Implementation Workflow

## Executive Summary

This workflow guides the implementation of a secure, self-deleting configuration management CLI tool in Rust. The project demonstrates advanced Rust concepts including cryptography, error handling, JSON processing, and systems programming.

**Educational Objectives:**
- Master Rust error handling with custom error types
- Implement production-quality cryptographic operations
- Learn memory-safe systems programming
- Practice functional programming patterns in Rust
- Understand self-modifying program concepts

**Timeline Estimate:** 3-5 days for experienced Rust developers, 1-2 weeks for learning purposes

---

## Phase 1: Project Foundation & Setup

**Duration:** 0.5-1 day  
**Complexity:** Simple  
**Prerequisites:** Rust 1.88.0, basic cargo knowledge

### Tasks

#### 1.1 Project Structure Setup
```bash
# Validate Rust version
rustc --version  # Should be 1.88.0

# Initialize project structure
cargo init --name claude-token-provider
mkdir -p src/{crypto,config,errors}
```

#### 1.2 Cargo.toml Configuration
**File:** `Cargo.toml`
```toml
[package]
name = "claude-token-provider"
version = "0.1.0"
edition = "2021"
rust-version = "1.88.0"

[dependencies]
# Cryptography
aes-gcm = "0.10"

# JSON manipulation
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Base64 encoding/decoding
base64 = "0.22"

# Secure password input
rpassword = "7.3"

# Self-deletion functionality
self_replace = "1.5"

# Error handling
thiserror = "1.0"

# Optional utilities
dirs = "5.0"  # For cross-platform home directory
```

#### 1.3 Module Structure Setup
**File:** `src/lib.rs`
```rust
//! Claude Token Provider - Secure Configuration Management Tool
//! 
//! This library provides functionality for managing encrypted configuration
//! data with automatic self-deletion capabilities.

pub mod crypto;
pub mod config;
pub mod errors;

pub use errors::{Result, TokenProviderError};
```

### Acceptance Criteria
- [ ] Cargo.toml properly configured with all dependencies
- [ ] Module structure established
- [ ] Project compiles without errors (`cargo check`)
- [ ] Documentation comments added to public interfaces

### Educational Focus
- **Cargo.toml:** Dependency management, feature flags, edition specification
- **Module System:** Rust's module structure and visibility rules
- **Documentation:** Rust documentation conventions and best practices

---

## Phase 2: Error Handling Foundation

**Duration:** 0.5 day  
**Complexity:** Moderate  
**Learning Focus:** Custom error types, error propagation, thiserror

### Tasks

#### 2.1 Custom Error Type Definition
**File:** `src/errors.rs`
```rust
use thiserror::Error;

/// Custom error type for the Token Provider application
#[derive(Error, Debug)]
pub enum TokenProviderError {
    #[error("Invalid base64 input: {0}")]
    InvalidBase64(#[from] base64::DecodeError),
    
    #[error("Invalid key length: expected 32 bytes, got {actual}")]
    InvalidKeyLength { actual: usize },
    
    #[error("Invalid IV length: expected 12 bytes, got {actual}")]
    InvalidIvLength { actual: usize },
    
    #[error("Cryptographic operation failed: {0}")]
    CryptoError(String),
    
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("I/O operation failed: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Self-deletion failed: {0}")]
    SelfDeletionError(String),
}

/// Type alias for Result with our custom error type
pub type Result<T> = std::result::Result<T, TokenProviderError>;
```

#### 2.2 Error Display Implementation
Extend the error type with helpful context and user-friendly messages:

```rust
impl TokenProviderError {
    /// Returns whether the error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            TokenProviderError::InvalidBase64(_) |
            TokenProviderError::InvalidKeyLength { .. } |
            TokenProviderError::InvalidIvLength { .. } => true,
            _ => false,
        }
    }
    
    /// Returns a user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            TokenProviderError::InvalidBase64(_) => 
                "Please check your base64 input format".to_string(),
            TokenProviderError::InvalidKeyLength { .. } => 
                "Secret key must be exactly 32 bytes when decoded".to_string(),
            TokenProviderError::InvalidIvLength { .. } => 
                "IV/Nonce must be exactly 12 bytes when decoded".to_string(),
            TokenProviderError::CryptoError(_) => 
                "Key or IV mismatch, or data corruption detected".to_string(),
            _ => self.to_string(),
        }
    }
}
```

### Acceptance Criteria
- [ ] All error variants properly defined with descriptive messages
- [ ] Error conversion traits implemented (From trait)
- [ ] User-friendly error messages provided
- [ ] Error categorization (recoverable vs non-recoverable)
- [ ] Comprehensive unit tests for error handling

### Educational Focus
- **Thiserror:** Automatic trait derivation for error types
- **Error Propagation:** Using `?` operator and Result types
- **Error Design:** Creating meaningful error hierarchies
- **User Experience:** Balancing technical accuracy with usability

---

## Phase 3: Cryptographic Foundation

**Duration:** 1-1.5 days  
**Complexity:** High  
**Learning Focus:** AES-GCM encryption, memory safety, key handling

### Tasks

#### 3.1 Cryptographic Module Structure
**File:** `src/crypto/mod.rs`
```rust
//! Cryptographic operations for secure data handling
//! 
//! This module provides AES-256-GCM encryption and decryption
//! with proper key validation and error handling.

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::{Engine as _, engine::general_purpose};

use crate::errors::{Result, TokenProviderError};

/// AES-GCM key size in bytes (256 bits)
pub const KEY_SIZE: usize = 32;
/// AES-GCM nonce/IV size in bytes (96 bits)
pub const NONCE_SIZE: usize = 12;
```

#### 3.2 Key and IV Validation
```rust
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
```

#### 3.3 Encryption Implementation
```rust
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
pub fn encrypt_data(data: &[u8], key: &[u8; KEY_SIZE], nonce: &[u8; NONCE_SIZE]) -> Result<Vec<u8>> {
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
pub fn decrypt_data(ciphertext: &[u8], key: &[u8; KEY_SIZE], nonce: &[u8; NONCE_SIZE]) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce);
    
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| TokenProviderError::CryptoError(e.to_string()))
}
```

#### 3.4 Hardcoded Ciphertext Generation
Create a build-time utility to generate the encrypted JSON:

**File:** `src/crypto/constants.rs`
```rust
/// Pre-encrypted configuration data
/// 
/// This ciphertext was generated by encrypting the following JSON:
/// ```json
/// {
///   "config_version": 2,
///   "settings": {
///     "retries": 5,
///     "timeout_ms": 5000
///   },
///   "features": {
///     "beta_feature_x": true
///   }
/// }
/// ```
/// 
/// Note: In a real application, this would be generated at build time
/// or through a secure deployment process.
pub const ENCRYPTED_CONFIG: &[u8] = &[
    // TODO: Generate this during build process
    // For now, this will be populated after implementing the encryption logic
];

/// Original JSON for reference (remove in production)
pub const ORIGINAL_JSON: &str = r#"{
  "config_version": 2,
  "settings": {
    "retries": 5,
    "timeout_ms": 5000
  },
  "features": {
    "beta_feature_x": true
  }
}"#;
```

### Acceptance Criteria
- [ ] AES-256-GCM encryption/decryption functions implemented
- [ ] Key and nonce validation with proper error handling
- [ ] Memory-safe byte array operations
- [ ] Comprehensive unit tests with known test vectors
- [ ] Build-time ciphertext generation (build.rs or utility script)
- [ ] Clear documentation of cryptographic choices

### Educational Focus
- **AEAD Cryptography:** Understanding authenticated encryption
- **Memory Safety:** Safe handling of cryptographic material
- **Error Handling:** Cryptographic error patterns
- **Security Best Practices:** Key validation and secure defaults

---

## Phase 4: JSON Processing & File Operations

**Duration:** 1 day  
**Complexity:** Moderate  
**Learning Focus:** Serde, deep merging, file I/O, path handling

### Tasks

#### 4.1 Configuration Types
**File:** `src/config/types.rs`
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Application configuration structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub config_version: u32,
    pub settings: Settings,
    pub features: HashMap<String, bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub retries: u32,
    pub timeout_ms: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            config_version: 2,
            settings: Settings {
                retries: 5,
                timeout_ms: 5000,
            },
            features: {
                let mut features = HashMap::new();
                features.insert("beta_feature_x".to_string(), true);
                features
            },
        }
    }
}
```

#### 4.2 Deep JSON Merging
**File:** `src/config/merger.rs`
```rust
use serde_json::{Value, Map};
use crate::Result;

/// Performs deep merge of JSON values
/// 
/// The `new_value` takes precedence over `existing_value` for conflicts.
/// Objects are merged recursively, arrays and primitives are replaced.
pub fn deep_merge_json(existing: &mut Value, new: Value) -> Result<()> {
    match (existing, new) {
        (Value::Object(existing_map), Value::Object(new_map)) => {
            merge_objects(existing_map, new_map)?;
        }
        (existing, new) => {
            // Replace existing value with new value
            *existing = new;
        }
    }
    Ok(())
}

/// Recursively merges two JSON objects
fn merge_objects(existing: &mut Map<String, Value>, new: Map<String, Value>) -> Result<()> {
    for (key, new_value) in new {
        match existing.get_mut(&key) {
            Some(existing_value) => {
                // Recursively merge if both are objects
                deep_merge_json(existing_value, new_value)?;
            }
            None => {
                // Insert new key-value pair
                existing.insert(key, new_value);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deep_merge_nested_objects() {
        let mut existing = json!({
            "a": {
                "b": 1,
                "c": 2
            },
            "d": 3
        });
        
        let new = json!({
            "a": {
                "b": 10,
                "e": 5
            },
            "f": 6
        });
        
        deep_merge_json(&mut existing, new).unwrap();
        
        let expected = json!({
            "a": {
                "b": 10,
                "c": 2,
                "e": 5
            },
            "d": 3,
            "f": 6
        });
        
        assert_eq!(existing, expected);
    }
}
```

#### 4.3 File Operations
**File:** `src/config/file_ops.rs`
```rust
use std::path::{Path, PathBuf};
use std::fs;
use dirs;
use serde_json::{Value, to_string_pretty};

use crate::{Result, TokenProviderError};
use super::merger::deep_merge_json;

/// Default configuration directory and file paths
const CONFIG_DIR: &str = ".config/my_app";
const CONFIG_FILE: &str = "config.json";

/// Gets the target configuration file path
pub fn get_config_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| TokenProviderError::IoError(
            std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found")
        ))?;
    
    Ok(home_dir.join(CONFIG_DIR).join(CONFIG_FILE))
}

/// Ensures the configuration directory exists
pub fn ensure_config_dir(config_path: &Path) -> Result<()> {
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

/// Reads existing configuration file if it exists and is valid JSON
pub fn read_existing_config(config_path: &Path) -> Result<Option<Value>> {
    if !config_path.exists() {
        return Ok(None);
    }
    
    let content = fs::read_to_string(config_path)?;
    
    // Try to parse as JSON
    match serde_json::from_str::<Value>(&content) {
        Ok(json) => Ok(Some(json)),
        Err(_) => {
            // File exists but is not valid JSON - will be overwritten
            eprintln!("Warning: Existing config file is not valid JSON and will be replaced");
            Ok(None)
        }
    }
}

/// Writes configuration to file with pretty formatting
pub fn write_config(config_path: &Path, config: &Value) -> Result<()> {
    let pretty_json = to_string_pretty(config)?;
    fs::write(config_path, pretty_json)?;
    Ok(())
}

/// Handles the complete file operation workflow
pub fn apply_config_update(new_config: Value) -> Result<()> {
    let config_path = get_config_path()?;
    
    // Ensure directory exists
    ensure_config_dir(&config_path)?;
    
    // Read existing configuration
    let mut final_config = match read_existing_config(&config_path)? {
        Some(mut existing) => {
            // Deep merge new config into existing
            deep_merge_json(&mut existing, new_config)?;
            existing
        }
        None => {
            // No existing config or invalid JSON - use new config directly
            new_config
        }
    };
    
    // Write the final configuration
    write_config(&config_path, &final_config)?;
    
    println!("Configuration successfully updated at: {}", config_path.display());
    Ok(())
}
```

### Acceptance Criteria
- [ ] Serde configuration types properly defined
- [ ] Deep JSON merging algorithm implemented and tested
- [ ] Cross-platform file path handling
- [ ] Proper directory creation and permissions
- [ ] Graceful handling of corrupted configuration files
- [ ] Comprehensive unit and integration tests

### Educational Focus
- **Serde:** Serialization/deserialization patterns
- **Collections:** HashMap and Vec manipulation
- **Path Handling:** Cross-platform file system operations
- **Error Recovery:** Handling corrupted data gracefully

---

## Phase 5: User Interface & Input Handling

**Duration:** 0.5 day  
**Complexity:** Simple  
**Learning Focus:** CLI interaction, secure input, validation

### Tasks

#### 5.1 User Input Module
**File:** `src/input.rs`
```rust
use std::io::{self, Write};
use rpassword::prompt_password;
use base64::{Engine as _, engine::general_purpose};

use crate::{Result, TokenProviderError};
use crate::crypto::{decode_and_validate_key, decode_and_validate_nonce, KEY_SIZE, NONCE_SIZE};

/// Prompts user for the AES-256-GCM secret key securely
pub fn get_secret_key() -> Result<[u8; KEY_SIZE]> {
    loop {
        let key_input = prompt_password("Enter AES-256-GCM Secret Key (Base64): ")
            .map_err(|e| TokenProviderError::IoError(
                io::Error::new(io::ErrorKind::Other, e)
            ))?;
        
        match decode_and_validate_key(&key_input) {
            Ok(key) => return Ok(key),
            Err(e) => {
                eprintln!("Error: {}", e.user_message());
                if !e.is_recoverable() {
                    return Err(e);
                }
                eprintln!("Please try again.\n");
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
        
        match decode_and_validate_nonce(nonce_input) {
            Ok(nonce) => return Ok(nonce),
            Err(e) => {
                eprintln!("Error: {}", e.user_message());
                if !e.is_recoverable() {
                    return Err(e);
                }
                print!("Please try again: ");
                io::stdout().flush().map_err(TokenProviderError::IoError)?;
            }
        }
    }
}

/// Displays application banner and instructions
pub fn display_banner() {
    println!("=================================");
    println!("   Claude Token Provider v0.1.0");
    println!("   Secure Configuration Manager");
    println!("=================================\n");
    
    println!("This tool will decrypt and apply configuration settings.");
    println!("You will need to provide:");
    println!("  1. Secret Key (32 bytes, Base64-encoded)");
    println!("  2. IV/Nonce (12 bytes, Base64-encoded)");
    println!();
    println!("⚠️  WARNING: This application will self-delete after completion!\n");
}
```

#### 5.2 Input Validation Enhancement
```rust
/// Validates base64 input format before attempting decode
pub fn validate_base64_format(input: &str) -> Result<()> {
    // Check for valid base64 characters
    let valid_chars = input.chars().all(|c| {
        c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '='
    });
    
    if !valid_chars {
        return Err(TokenProviderError::InvalidBase64(
            base64::DecodeError::InvalidByte(0, 0) // Placeholder error
        ));
    }
    
    // Check padding
    let padding_count = input.chars().rev().take_while(|&c| c == '=').count();
    if padding_count > 2 {
        return Err(TokenProviderError::InvalidBase64(
            base64::DecodeError::InvalidPadding
        ));
    }
    
    Ok(())
}

/// Enhanced key input with format validation
pub fn get_secret_key_validated() -> Result<[u8; KEY_SIZE]> {
    loop {
        let key_input = prompt_password("Enter AES-256-GCM Secret Key (Base64): ")
            .map_err(|e| TokenProviderError::IoError(
                io::Error::new(io::ErrorKind::Other, e)
            ))?;
        
        // Pre-validate format
        if let Err(e) = validate_base64_format(&key_input) {
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
```

### Acceptance Criteria
- [ ] Secure password input without terminal echo
- [ ] Base64 format validation before decoding
- [ ] User-friendly error messages and recovery
- [ ] Clear input prompts and instructions
- [ ] Input validation prevents common errors

### Educational Focus
- **CLI Design:** User experience considerations for terminal applications
- **Security:** Secure input handling practices
- **Error Recovery:** Interactive error handling patterns
- **Validation:** Input sanitization and validation strategies

---

## Phase 6: Integration & Self-Deletion

**Duration:** 1 day  
**Complexity:** High  
**Learning Focus:** Application orchestration, self-modification, error handling

### Tasks

#### 6.1 Self-Deletion Module
**File:** `src/self_deletion.rs`
```rust
//! Self-deletion functionality
//! 
//! WARNING: This module implements potentially dangerous functionality.
//! Self-deleting executables can trigger antivirus software and may be
//! blocked by security systems. This is intended for educational purposes only.

use std::env;
use self_replace::self_delete;
use crate::{Result, TokenProviderError};

/// Attempts to delete the current executable
/// 
/// # Safety
/// This function is inherently platform-dependent and may fail on some systems:
/// - Windows: May fail if the executable is locked or in use
/// - Unix-like: Generally succeeds as the file can be unlinked while running
/// - Some antivirus software may block this operation
/// 
/// # Returns
/// * `Ok(())` if deletion succeeds or is not needed
/// * `Err(TokenProviderError)` if deletion fails
pub fn perform_self_deletion() -> Result<()> {
    // Get the current executable path for logging
    let exe_path = env::current_exe()
        .map_err(TokenProviderError::IoError)?;
    
    println!("Attempting to delete executable: {}", exe_path.display());
    
    match self_delete() {
        Ok(()) => {
            println!("✓ Executable successfully deleted");
            Ok(())
        }
        Err(e) => {
            let error_msg = format!("Failed to delete executable: {}", e);
            eprintln!("⚠️  Warning: {}", error_msg);
            
            // Log additional context
            eprintln!("   This may be due to:");
            eprintln!("   - File permissions");
            eprintln!("   - Antivirus software blocking the operation");
            eprintln!("   - Platform-specific restrictions");
            eprintln!("   - The executable being in use by another process");
            
            // Return error but don't panic - application completed its main task
            Err(TokenProviderError::SelfDeletionError(error_msg))
        }
    }
}

/// Displays self-deletion warning
pub fn display_self_deletion_warning() {
    println!("📋 SELF-DELETION NOTICE:");
    println!("   This application will attempt to delete itself after completion.");
    println!("   This is a security feature to prevent unauthorized reuse.");
    println!("   If deletion fails, please manually remove the executable.\n");
}

/// Confirms self-deletion with user (optional safety measure)
pub fn confirm_self_deletion() -> Result<bool> {
    use std::io::{self, Write};
    
    print!("Proceed with self-deletion? [y/N]: ");
    io::stdout().flush().map_err(TokenProviderError::IoError)?;
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(TokenProviderError::IoError)?;
    
    let input = input.trim().to_lowercase();
    Ok(input == "y" || input == "yes")
}
```

#### 6.2 Main Application Logic
**File:** `src/main.rs`
```rust
//! Claude Token Provider - Main Application
//! 
//! A secure, self-deleting configuration management tool demonstrating
//! advanced Rust concepts including cryptography, error handling, and
//! systems programming.

use claude_token_provider::{
    crypto::{decrypt_data, constants::ENCRYPTED_CONFIG},
    config::file_ops::apply_config_update,
    input::{display_banner, get_secret_key, get_nonce},
    self_deletion::{display_self_deletion_warning, perform_self_deletion},
    Result, TokenProviderError,
};

fn main() -> Result<()> {
    // Display application banner and warnings
    display_banner();
    display_self_deletion_warning();
    
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
            "No encrypted configuration data found".to_string()
        ));
    }
    
    let decrypted_bytes = decrypt_data(ENCRYPTED_CONFIG, &key, &nonce)?;
    
    // Step 3: Parse JSON
    println!("Step 3: Parsing configuration JSON");
    let decrypted_text = String::from_utf8(decrypted_bytes)
        .map_err(|e| TokenProviderError::CryptoError(
            format!("Decrypted data is not valid UTF-8: {}", e)
        ))?;
    
    let config_json: serde_json::Value = serde_json::from_str(&decrypted_text)
        .map_err(TokenProviderError::JsonError)?;
    
    // Step 4: Apply configuration
    println!("Step 4: Applying configuration to file system");
    apply_config_update(config_json)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_application_components() {
        // This would be a comprehensive integration test
        // For now, just ensure the main components compile
        assert!(true);
    }
}
```

#### 6.3 Build-time Configuration Generation
**File:** `build.rs`
```rust
//! Build script to generate encrypted configuration data
//! 
//! This script encrypts the default JSON configuration and embeds
//! it as a constant in the compiled binary.

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // For educational purposes, we'll use a known key/nonce
    // In production, these would be generated securely
    let key = [0u8; 32]; // Demo key - all zeros
    let nonce = [0u8; 12]; // Demo nonce - all zeros
    
    let json_config = r#"{
  "config_version": 2,
  "settings": {
    "retries": 5,
    "timeout_ms": 5000
  },
  "features": {
    "beta_feature_x": true
  }
}"#;
    
    // This is a placeholder - in a real build script, you would:
    // 1. Import the aes-gcm crate
    // 2. Encrypt the JSON with proper key/nonce
    // 3. Generate the constant declaration
    // 4. Write it to a file that gets included
    
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/crypto/constants.rs");
    
    // For now, just print instructions
    println!("cargo:warning=Build script placeholder - implement encryption logic");
}
```

### Acceptance Criteria
- [ ] Main application workflow implemented
- [ ] Self-deletion functionality with proper error handling
- [ ] Build-time configuration encryption
- [ ] Graceful error handling throughout the application
- [ ] Platform-specific considerations documented
- [ ] Security warnings and user notifications

### Educational Focus
- **Application Architecture:** Orchestrating complex workflows
- **Error Propagation:** Handling errors across multiple subsystems
- **Build Scripts:** Compile-time code generation
- **Security Considerations:** Self-modifying programs and their implications

---

## Phase 7: Testing & Validation

**Duration:** 1 day  
**Complexity:** Moderate  
**Learning Focus:** Unit testing, integration testing, security testing

### Tasks

#### 7.1 Cryptographic Tests
**File:** `src/crypto/tests.rs`
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{encrypt_data, decrypt_data, decode_and_validate_key, decode_and_validate_nonce};
    
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
        
        assert!(matches!(result, Err(TokenProviderError::InvalidKeyLength { actual: 4 })));
    }
    
    #[test]
    fn test_invalid_nonce_length() {
        let short_nonce = "dGVzdA=="; // "test" in base64 (4 bytes)
        let result = decode_and_validate_nonce(short_nonce);
        
        assert!(matches!(result, Err(TokenProviderError::InvalidIvLength { actual: 4 })));
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
```

#### 7.2 JSON Processing Tests
**File:** `src/config/tests.rs`
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::NamedTempFile;
    use std::io::Write;
    
    #[test]
    fn test_deep_merge_complex() {
        let mut base = json!({
            "config_version": 1,
            "settings": {
                "retries": 3,
                "timeout_ms": 1000
            },
            "features": {
                "feature_a": true
            }
        });
        
        let update = json!({
            "config_version": 2,
            "settings": {
                "timeout_ms": 5000,
                "new_setting": "value"
            },
            "features": {
                "feature_b": true
            },
            "new_section": {
                "key": "value"
            }
        });
        
        deep_merge_json(&mut base, update).unwrap();
        
        let expected = json!({
            "config_version": 2,
            "settings": {
                "retries": 3,
                "timeout_ms": 5000,
                "new_setting": "value"
            },
            "features": {
                "feature_a": true,
                "feature_b": true
            },
            "new_section": {
                "key": "value"
            }
        });
        
        assert_eq!(base, expected);
    }
    
    #[test]
    fn test_file_operations() {
        // Test with temporary file
        let mut temp_file = NamedTempFile::new().unwrap();
        let existing_config = json!({"version": 1, "data": "test"});
        
        writeln!(temp_file, "{}", serde_json::to_string_pretty(&existing_config).unwrap()).unwrap();
        
        let read_config = read_existing_config(temp_file.path()).unwrap();
        assert_eq!(read_config, Some(existing_config));
    }
    
    #[test]
    fn test_corrupted_json_handling() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "not valid json").unwrap();
        
        let result = read_existing_config(temp_file.path()).unwrap();
        assert_eq!(result, None); // Should return None for invalid JSON
    }
}
```

#### 7.3 Integration Tests
**File:** `tests/integration_test.rs`
```rust
use claude_token_provider::*;
use tempfile::TempDir;
use std::fs;

#[test]
fn test_full_workflow_simulation() {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");
    
    // Simulate the workflow without user input and self-deletion
    // This would test the core logic components
    
    // 1. Test encryption/decryption
    let test_json = r#"{"test": "data"}"#;
    let key = [1u8; 32];
    let nonce = [2u8; 12];
    
    let encrypted = crypto::encrypt_data(test_json.as_bytes(), &key, &nonce).unwrap();
    let decrypted = crypto::decrypt_data(&encrypted, &key, &nonce).unwrap();
    
    assert_eq!(test_json.as_bytes(), decrypted.as_slice());
    
    // 2. Test JSON processing
    let json_value: serde_json::Value = serde_json::from_str(&String::from_utf8(decrypted).unwrap()).unwrap();
    
    // 3. Test file operations
    config::file_ops::write_config(&config_path, &json_value).unwrap();
    
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("test"));
}

#[test]
fn test_error_handling_chain() {
    // Test that errors propagate correctly through the entire chain
    use base64::{Engine as _, engine::general_purpose};
    
    // Test invalid base64
    let invalid_key = "invalid_base64!@#";
    let result = crypto::decode_and_validate_key(invalid_key);
    assert!(result.is_err());
    
    // Test short key
    let short_key = general_purpose::STANDARD.encode(b"short");
    let result = crypto::decode_and_validate_key(&short_key);
    assert!(matches!(result, Err(TokenProviderError::InvalidKeyLength { .. })));
}
```

### Acceptance Criteria
- [ ] Unit tests for all major components (>80% coverage)
- [ ] Integration tests for end-to-end workflows
- [ ] Error handling tests for all error variants
- [ ] Security tests (tampered data, invalid inputs)
- [ ] Performance tests for cryptographic operations
- [ ] Platform compatibility tests

### Educational Focus
- **Testing Strategies:** Unit vs integration vs security testing
- **Test Organization:** Rust testing conventions and best practices
- **Mocking:** Testing components in isolation
- **Property-based Testing:** Advanced testing techniques

---

## Phase 8: Documentation & Security Review

**Duration:** 0.5 day  
**Complexity:** Simple  
**Learning Focus:** Documentation, security analysis, deployment considerations

### Tasks

#### 8.1 Code Documentation
Ensure comprehensive documentation throughout:

```rust
//! # Claude Token Provider
//! 
//! A secure, educational tool demonstrating advanced Rust concepts including:
//! 
//! - **Cryptography**: AES-256-GCM authenticated encryption
//! - **Error Handling**: Custom error types with `thiserror`
//! - **JSON Processing**: Serde serialization with deep merging
//! - **File I/O**: Cross-platform file system operations
//! - **Self-Modification**: Executable self-deletion patterns
//! 
//! ## Security Considerations
//! 
//! This application is designed for educational purposes and demonstrates
//! several security concepts:
//! 
//! - Authenticated encryption prevents data tampering
//! - Secure input handling for cryptographic material
//! - Memory-safe byte operations
//! - Self-deletion for operational security
//! 
//! ## Platform Support
//! 
//! - **Linux**: Fully supported
//! - **macOS**: Fully supported  
//! - **Windows**: Self-deletion may be blocked by antivirus
//! 
//! ## Usage Warning
//! 
//! ⚠️ **This tool will delete itself after execution!**
//! 
//! This is a security feature but may trigger antivirus software.
//! Only use in controlled environments for educational purposes.

#[cfg(doctest)]
doc_comment::doctest!("../README.md");
```

#### 8.2 README.md
**File:** `README.md`
```markdown
# Claude Token Provider

A secure, self-deleting configuration management tool built in Rust for educational purposes.

## ⚠️ Important Warning

**This application will delete itself after execution.** This is an intentional security feature that may trigger antivirus software. Only use in controlled educational environments.

## Educational Objectives

This project demonstrates advanced Rust concepts:

- **Cryptography**: AES-256-GCM authenticated encryption
- **Error Handling**: Custom error types with comprehensive propagation
- **Memory Safety**: Safe handling of cryptographic material
- **JSON Processing**: Complex data manipulation with serde
- **Systems Programming**: File I/O and self-modification
- **Testing**: Comprehensive unit and integration testing

## Prerequisites

- Rust 1.88.0 or later
- Understanding of cryptographic principles (recommended)
- Isolated testing environment (required)

## Building

\```bash
git clone <repository>
cd claude-token-provider
cargo build --release
\```

## Usage

\```bash
# Run the application
./target/release/claude-token-provider

# You will be prompted for:
# 1. Secret Key (32 bytes, Base64-encoded)
# 2. IV/Nonce (12 bytes, Base64-encoded)
\```

### Test Credentials

For educational testing, you can use these demo credentials:

- **Key**: `AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=` (32 zero bytes)
- **IV**: `AAAAAAAAAAAAAAAA` (12 zero bytes)

## Architecture

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library root
├── crypto/              # Cryptographic operations
│   ├── mod.rs           # Encryption/decryption
│   └── constants.rs     # Hardcoded ciphertext
├── config/              # Configuration management
│   ├── types.rs         # Data structures
│   ├── merger.rs        # JSON deep merging
│   └── file_ops.rs      # File I/O operations
├── errors.rs            # Error types
├── input.rs             # User input handling
└── self_deletion.rs     # Self-deletion logic
```

## Security Features

- **Authenticated Encryption**: AES-256-GCM prevents tampering
- **Input Validation**: Comprehensive validation of user inputs
- **Memory Safety**: Rust's ownership system prevents buffer overflows
- **Self-Deletion**: Removes executable after completion

## Testing

\```bash
# Run all tests
cargo test

# Run with coverage
cargo test --coverage

# Run integration tests
cargo test --test integration_test
\```

## Educational Use Only

This software is designed exclusively for educational purposes to demonstrate:

- Secure cryptographic patterns in Rust
- Production-quality error handling
- Memory-safe systems programming
- Self-modifying program concepts

**Do not use in production environments.**

## License

MIT License - see LICENSE file for details.
```

#### 8.3 Security Analysis Document
**File:** `SECURITY.md`
```markdown
# Security Analysis

## Overview

This document analyzes the security aspects of the Claude Token Provider for educational purposes.

## Threat Model

### Assumptions
- User has legitimate access to decryption credentials
- Execution environment is trusted
- Configuration data confidentiality is required
- Executable reuse should be prevented

### Threats Addressed
1. **Data Confidentiality**: AES-256-GCM encryption
2. **Data Integrity**: Authentication tag verification
3. **Replay Attacks**: Single-use executable design
4. **Configuration Tampering**: Deep validation and merging

### Threats NOT Addressed
1. **Memory Dumps**: Decrypted data exists in memory
2. **Key Management**: Keys provided by user (not generated)
3. **Side-Channel Attacks**: No timing attack mitigations
4. **Endpoint Security**: No anti-debugging measures

## Cryptographic Analysis

### Encryption: AES-256-GCM
- **Algorithm**: NIST-approved, industry standard
- **Key Size**: 256 bits (exceeds current recommendations)
- **Mode**: Galois/Counter Mode provides both confidentiality and authenticity
- **Nonce**: 96-bit nonce prevents replay attacks

### Key Management
- **Strength**: Relies on user-provided entropy
- **Storage**: Keys exist only in memory during execution
- **Transmission**: No network transmission
- **Lifecycle**: Keys discarded after single use

## Implementation Security

### Memory Safety
- **Language**: Rust provides memory safety guarantees
- **Buffer Overflows**: Prevented by ownership system
- **Use-After-Free**: Prevented by borrow checker
- **Double-Free**: Prevented by RAII

### Input Validation
- **Base64 Validation**: Format checking before decode
- **Length Validation**: Exact length requirements enforced
- **Error Handling**: No information leakage through errors

### Self-Deletion
- **Purpose**: Prevents unauthorized reuse
- **Limitations**: May fail due to file locks or permissions
- **Detection**: May trigger antivirus software
- **Platform Dependency**: Success varies by operating system

## Recommendations for Educational Use

### Do:
- Use in isolated, controlled environments
- Test with non-sensitive data
- Study the cryptographic implementations
- Analyze error handling patterns
- Experiment with different input combinations

### Don't:
- Use with real production data
- Deploy in production environments
- Bypass or disable security features
- Use on systems with important data

## Known Limitations

1. **Key Management**: No secure key derivation or generation
2. **Nonce Reuse**: No nonce uniqueness verification
3. **Memory Clearing**: Sensitive data not explicitly zeroed
4. **Timing Attacks**: No constant-time operations
5. **Side Channels**: No protection against power/EM analysis

## Future Enhancements (Educational)

1. Implement secure key derivation (PBKDF2/Argon2)
2. Add memory zeroization for sensitive data
3. Implement constant-time comparison functions
4. Add nonce uniqueness tracking
5. Include side-channel attack mitigations

## Conclusion

This implementation demonstrates secure coding practices for educational purposes while intentionally simplifying certain aspects (like key management) to focus on core concepts. It should not be used as a template for production security implementations.
```

### Acceptance Criteria
- [ ] Comprehensive code documentation (rustdoc)
- [ ] Complete README with usage instructions
- [ ] Security analysis document
- [ ] Build and deployment instructions
- [ ] Educational objectives clearly stated
- [ ] Limitations and warnings prominently displayed

### Educational Focus
- **Documentation:** Effective technical communication
- **Security Analysis:** Threat modeling and risk assessment
- **Responsible Disclosure:** Ethical considerations in security education

---

## Success Metrics & Validation

### Functional Requirements ✅
- [ ] Application encrypts/decrypts JSON configuration correctly
- [ ] User input validation prevents common errors
- [ ] JSON deep merging preserves existing configuration
- [ ] File operations handle all edge cases (missing directories, corrupted files)
- [ ] Self-deletion attempts to remove executable (with graceful failure handling)

### Educational Requirements ✅
- [ ] Code demonstrates advanced Rust patterns (error handling, memory safety, functional programming)
- [ ] Comprehensive testing suite (unit, integration, security tests)
- [ ] Security best practices documented and implemented
- [ ] Clear separation of concerns and modular architecture
- [ ] Production-quality error handling and user experience

### Technical Requirements ✅
- [ ] Uses Rust 2021 edition with specified dependencies
- [ ] Follows functional programming style with immutability by default
- [ ] Implements robust error handling with custom error types
- [ ] Provides cross-platform compatibility
- [ ] Includes comprehensive documentation and security analysis

### Performance Targets 🎯
- **Compilation Time**: < 2 minutes for full build
- **Execution Time**: < 5 seconds for typical configuration
- **Memory Usage**: < 10MB peak memory usage
- **Binary Size**: < 5MB release binary

### Quality Metrics 📊
- **Test Coverage**: > 80% line coverage
- **Documentation Coverage**: 100% of public APIs documented
- **Security Review**: No high-severity security issues
- **Code Quality**: Passes all linting rules (clippy)

---

## Risk Assessment & Mitigation

### Technical Risks 🔴
| Risk | Probability | Impact | Mitigation |
|------|-------------|---------|------------|
| Self-deletion blocked by AV | High | Medium | Document limitation, provide manual cleanup |
| Platform compatibility issues | Medium | Medium | Test on multiple platforms, provide fallbacks |
| Cryptographic implementation bugs | Low | High | Use established libraries, comprehensive testing |
| File permission errors | Medium | Low | Graceful error handling, user guidance |

### Educational Risks 📚
| Risk | Probability | Impact | Mitigation |
|------|-------------|---------|------------|
| Misuse in production | Medium | High | Clear warnings, educational-only licensing |
| Security misconceptions | Low | Medium | Comprehensive security documentation |
| Incomplete understanding | Medium | Low | Progressive complexity, detailed explanations |

### Operational Risks ⚙️
| Risk | Probability | Impact | Mitigation |
|------|-------------|---------|------------|
| Data loss from self-deletion | Low | Medium | Clear warnings, backup recommendations |
| Antivirus false positives | High | Low | Documentation, whitelisting instructions |
| Build environment issues | Medium | Low | Detailed setup instructions, version pinning |

---

## Conclusion

This workflow provides a comprehensive guide to implementing a production-quality Rust application that demonstrates advanced concepts in cryptography, error handling, and systems programming. The educational focus ensures that learners gain practical experience with real-world software development challenges while maintaining security awareness and best practices.

The progressive phase structure allows for incremental learning and validation, while the comprehensive testing and documentation requirements ensure that the final product serves as an excellent educational resource for advanced Rust programming concepts.

**Remember: This is an educational tool designed for learning purposes in controlled environments. The self-deletion functionality and cryptographic components require careful handling and should never be used in production systems without extensive additional security measures.**