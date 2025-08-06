# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## ⚠️ Project Security Warning

**This is a self-deleting executable designed for educational purposes.** The application will attempt to delete itself after successful execution. Only run in isolated environments (VMs or containers) and never on production systems.

## Project Overview

`claude-token-provider` is a sophisticated secure configuration management tool built in Rust that demonstrates advanced cryptographic programming, memory-safe systems programming, and production-quality error handling. The project serves as an educational resource for learning advanced Rust concepts through a real-world security application.

**Key Features:**
- AES-256-GCM authenticated encryption for configuration data
- Self-deletion functionality for operational security
- Deep JSON merging with existing configuration files
- Comprehensive error handling with custom error types
- Secure user input handling with validation
- Cross-platform file system operations
- Modular architecture with clean separation of concerns

**Educational Focus:** This project demonstrates enterprise-level Rust programming patterns including cryptographic implementation, memory safety, functional programming style, and comprehensive testing strategies.

## Architecture Overview

### Core Modules
- **`src/main.rs`**: Application orchestration and main workflow
- **`src/lib.rs`**: Library root with public API exports
- **`src/crypto/`**: AES-256-GCM encryption/decryption with key validation
- **`src/config/`**: Configuration management, JSON deep merging, file I/O
- **`src/errors.rs`**: Custom error types with user-friendly messages
- **`src/input.rs`**: Secure user input handling and Base64 validation
- **`src/self_deletion.rs`**: Platform-aware executable self-deletion

### Key Dependencies
- `aes-gcm`: Cryptographic operations (AES-256-GCM)
- `serde`/`serde_json`: JSON serialization and manipulation
- `base64`: Base64 encoding/decoding for user inputs
- `rpassword`: Secure password input without terminal echo
- `self-replace`: Cross-platform executable self-deletion
- `thiserror`: Custom error type generation
- `dirs`: Cross-platform home directory detection

## Development Commands

### Build and Run
```bash
# Development build with debug symbols
cargo build

# Optimized release build
cargo build --release

# ⚠️ WARNING: This will attempt self-deletion after completion
cargo run

# Run with demo credentials (see Usage section below)
# Key: AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=
# Nonce: AAAAAAAAAAAAAAAA
```

### Testing and Validation
```bash
# Run comprehensive test suite
cargo test

# Run tests with detailed output
cargo test -- --nocapture

# Run specific test modules
cargo test crypto::tests
cargo test config::tests

# Integration tests
cargo test --test integration_test

# Test with backtraces for debugging
RUST_BACKTRACE=1 cargo test
```

### Code Quality and Analysis
```bash
# Format code to Rust standards
cargo fmt

# Run Clippy linter with all checks
cargo clippy --all-targets --all-features

# Check compilation without building
cargo check

# Generate and open documentation
cargo doc --open

# Dependency security audit
cargo audit
```

### Development Analysis
```bash
# Analyze dependency tree
cargo tree

# Verbose build output
cargo build -v

# Check for unused dependencies
cargo machete

# Binary size analysis
cargo bloat --release
```

## Demo Usage (Educational Only)

This project includes demo credentials for educational testing:

**Demo Credentials:**
- **Secret Key**: `AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=` (32 zero bytes, Base64)
- **IV/Nonce**: `AAAAAAAAAAAAAAAA` (12 zero bytes, Base64)

**Configuration Target:** `~/.config/my_app/config.json`

**⚠️ These are zero-byte demo keys for educational purposes only!**

## Security Considerations

### What This Tool Demonstrates
- Authenticated encryption with AES-256-GCM
- Secure input validation and error handling
- Memory-safe cryptographic programming
- Cross-platform file system security
- Secure application lifecycle management

### Educational Limitations
- Uses demo keys for reproducible examples
- No key derivation or secure key generation
- Sensitive data not explicitly cleared from memory
- Self-deletion may be blocked by antivirus software

### Safe Usage Guidelines
- Run only in isolated environments (VMs/containers)
- Use with non-sensitive test data only
- Study the implementation patterns and security concepts
- Never deploy or use in production systems

## Code Organization Patterns

### Error Handling Strategy
- Custom error types using `thiserror` for idiomatic Rust errors
- Comprehensive error propagation with `Result<T>` types
- User-friendly error messages separate from technical details
- Error recovery patterns for interactive user input

### Cryptographic Patterns
- Authenticated encryption (AEAD) using AES-256-GCM
- Input validation before cryptographic operations
- Memory-safe byte array handling
- Proper error handling for cryptographic failures

### Configuration Management
- Deep JSON merging algorithms
- Graceful handling of corrupted configuration files
- Cross-platform path handling with the `dirs` crate
- Atomic file operations with proper error recovery

## Testing Strategy

### Unit Tests
- Cryptographic round-trip tests
- Input validation edge cases
- Error condition testing
- JSON merging algorithm verification

### Integration Tests
- End-to-end workflow simulation
- File system operation testing
- Error propagation chain validation
- Cross-platform compatibility testing

### Security Tests
- Tampered data detection
- Invalid input handling
- Authentication failure scenarios
- Boundary condition testing

## Important Reminders

1. **Self-Deletion**: This application will delete itself after execution
2. **Educational Only**: Not intended for production use
3. **Demo Keys**: Uses predictable keys for learning purposes
4. **Isolated Environment**: Always run in VMs or containers
5. **Security Focus**: Study the security patterns and implementations