# Claude Token Provider

A secure, self-deleting configuration management tool built in Rust, demonstrating advanced cryptographic
concepts, memory-safe systems programming, and production-quality error handling.

## ⚠️ **CRITICAL WARNING**

**🔴 THIS APPLICATION WILL DELETE ITSELF AFTER EXECUTION 🔴**

This is an intentional security feature that may trigger antivirus software and security systems.

**DO NOT USE ON PRODUCTION SYSTEMS OR IMPORTANT MACHINES**

### Core Technologies

- **🔐 Cryptography**: AES-256-GCM authenticated encryption with proper key validation
- **🛡️ Error Handling**: Custom error types with comprehensive propagation using `thiserror`
- **🧠 Memory Safety**: Safe handling of cryptographic material and sensitive data
- **📋 JSON Processing**: Complex data manipulation with serde and deep merging algorithms
- **⚙️ Systems Programming**: File I/O, cross-platform path handling, and executable self-modification
- **🧪 Testing**: Comprehensive unit, integration, and security testing strategies

### Software Engineering Principles

- **Modular Architecture**: Clean separation of concerns across domain-specific modules
- **Functional Programming**: Immutability by default, iterator patterns, and Result/Option usage
- **Production Quality**: Professional-grade error messages, input validation, and user experience
- **Security-First Design**: Defense in depth, fail-safe defaults, and comprehensive validation

## 🏗️ Architecture

```
src/
├── main.rs              # Application orchestration and workflow
├── lib.rs               # Library root with public API
├── crypto/              # Cryptographic operations
│   ├── mod.rs           # AES-256-GCM encrypt/decrypt with validation
│   └── constants.rs     # Pre-encrypted configuration data
├── config/              # Configuration management
│   ├── types.rs         # Serde data structures and defaults
│   ├── merger.rs        # Deep JSON merging algorithms
│   ├── file_ops.rs      # Cross-platform file I/O operations
│   └── mod.rs           # Module exports
├── errors.rs            # Custom error types with user-friendly messages
├── input.rs             # Secure user input handling and validation
└── self_deletion.rs     # Self-deletion logic with platform considerations
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.90.0 or later
- Isolated testing environment (VM or container recommended)
- Basic understanding of cryptographic principles

### Building and Running

```bash
# Clone and build
git clone <repository>
cd claude-token-provider
cargo build --release

# Run the application
./target/release/claude-token-provider
```

### Demo Credentials

For testing, use these demo credentials when prompted:

- **Secret Key**: `AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=`
- **IV/Nonce**: `AAAAAAAAAAAAAAAA`

⚠️ **These are zero-byte demo keys for learning only!**

## 🔄 Application Workflow

The application follows a secure 4-step process:

1. **🔐 Credential Collection**: Securely prompt for AES-256-GCM key and nonce
2. **🔓 Decryption**: Decrypt hardcoded configuration using authenticated encryption
3. **📋 Configuration Merge**: Deep merge with existing config at `~/.config/my_app/config.json`
4. **🗑️ Self-Deletion**: Remove executable to prevent unauthorized reuse

### What It Does

- Decrypts embedded configuration data using AES-256-GCM
- Creates configuration directory if it doesn't exist
- Performs intelligent deep merge with existing configuration
- Handles corrupted or invalid JSON files gracefully
- Provides detailed error messages for debugging
- Attempts secure self-deletion after completion

## 🛡️ Security Features

### Cryptographic Security

- **AES-256-GCM**: NIST-approved authenticated encryption
- **256-bit Keys**: Exceeds current security recommendations
- **Authentication**: Prevents tampering and ensures data integrity
- **Input Validation**: Comprehensive Base64 and length validation

### Memory Safety

- **Rust Ownership**: Prevents buffer overflows and memory corruption
- **No Use-After-Free**: Borrow checker eliminates entire vulnerability classes
- **RAII**: Automatic resource cleanup prevents leaks

### Operational Security

- **Self-Deletion**: Prevents executable reuse and reduces attack surface
- **Secure Input**: Password-style input prevents credential display
- **Error Handling**: No information leakage through error messages
- **Fail-Safe Defaults**: Secure behavior when operations fail

### Input Security

- **Format Validation**: Base64 character validation before decoding
- **Length Enforcement**: Exact key (32 bytes) and nonce (12 bytes) requirements
- **Error Recovery**: User-friendly retry mechanism for invalid inputs

## 🧪 Development Commands

### Building and Testing

```bash
# Development build with debug info
cargo build

# Optimized release build
cargo build --release

# Run comprehensive test suite
cargo test

# Run with detailed output
cargo test -- --nocapture

# Run specific test module
cargo test crypto::tests

# Integration tests
cargo test --test integration_test
```

### Code Quality

```bash
# Format code to Rust standards
cargo fmt

# Run comprehensive linter
cargo clippy --all-targets --all-features

# Check without building
cargo check

# Generate documentation
cargo doc --open
```

### Analysis and Debugging

```bash
# Run with backtraces for debugging
RUST_BACKTRACE=1 cargo run

# Verbose compiler output
cargo build -v

# Dependency tree analysis  
cargo tree

# Security audit of dependencies
cargo audit
```

## 📊 Configuration Details

### Default Configuration Structure

```json
{
  "config_version": 2,
  "settings": {
    "retries": 5,
    "timeout_ms": 5000
  },
  "features": {
    "beta_feature_x": true
  }
}
```

### Deep Merge Behavior

- **Objects**: Recursively merged, new keys added, existing keys updated
- **Arrays**: Completely replaced (no element-wise merging)
- **Primitives**: Replaced with new values
- **Null Values**: Explicit null overwrites existing values

### File Locations

- **Config Directory**: `~/.config/my_app/` (cross-platform)
- **Config File**: `config.json` (pretty-printed JSON)
- **Backup**: Corrupted files are automatically replaced

## ⚠️ Security Limitations & Considerations

### Intentional Limitations

- **Demo Keys**: Uses zero-byte keys for reproducible examples
- **Key Management**: No secure key derivation or generation mechanisms
- **Nonce Reuse**: No uniqueness verification (single-use tool design mitigates this)
- **Memory Clearing**: Sensitive data not explicitly zeroed from memory

### Platform Considerations

- **Linux/macOS**: Self-deletion generally succeeds
- **Windows**: May be blocked by file locks or antivirus software
- **Antivirus**: Self-deletion triggers may cause false positives
- **Permissions**: Requires write access to home directory

### Threat Model

**✅ Protects Against:**

- Configuration data confidentiality (encryption at rest)
- Data tampering (authenticated encryption)
- Casual executable reuse (self-deletion)
- Configuration corruption (validation and graceful recovery)

**❌ Does NOT Protect Against:**

- Memory dumps or debugging (decrypted data exists in memory)
- Side-channel attacks (no constant-time operations)
- Key compromise (relies on user-provided keys)
- Advanced persistent threats (no anti-tampering measures)

## 📚 Learning Outcomes

By studying and running this project, you'll gain hands-on experience with:

### Rust Programming

- Advanced error handling patterns with custom error types
- Memory-safe cryptographic programming
- Functional programming techniques (iterators, combinators)
- Module system organization and visibility control
- Testing strategies for systems programming

### Cryptographic Engineering

- Authenticated encryption (AEAD) implementation patterns
- Secure input validation and sanitization
- Key material handling best practices
- Cryptographic error handling without information leakage

### Systems Programming

- Cross-platform file system operations
- Process self-modification techniques
- Resource management with RAII
- Platform-specific behavior handling

### Software Engineering

- Production-quality error messages and user experience
- Comprehensive testing strategies (unit, integration, security)
- Documentation-driven development
- Security-conscious software design

## 🔧 Customization for Learning

### Extending Cryptographic Features

```rust
// Example: Add key derivation
use argon2::Argon2;

fn derive_key_from_password(password: &str, salt: &[u8]) -> [u8; 32] {
    // Implementation
}
```

### Adding Security Measures

```rust
// Example: Memory zeroing
use zeroize::Zeroize;

fn secure_decrypt(ciphertext: &[u8], key: &mut [u8; 32]) -> Result<Vec<u8>> {
    let result = decrypt_data(ciphertext, key, nonce);
    key.zeroize(); // Clear sensitive data
    result
}
```

## 📄 License

MIT License - See LICENSE file for details.

---