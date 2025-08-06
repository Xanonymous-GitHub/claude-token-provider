# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## ⚠️ Project Security Warning

**This is a self-deleting executable designed for educational purposes.** The application will attempt to delete itself after successful execution. Only run in isolated environments (VMs or containers) and never on production systems.

## Project Overview

`claude-token-provider` is a sophisticated secure configuration management tool built in Rust that demonstrates advanced cryptographic programming, memory-safe systems programming, and production-quality error handling. The project serves as a comprehensive educational resource for learning advanced Rust concepts through a real-world security application.

**Key Features:**
- AES-256-GCM authenticated encryption for configuration data
- Self-deletion functionality for operational security
- Deep JSON merging with existing configuration files
- Comprehensive error handling with custom error types
- Secure user input handling with validation
- Cross-platform file system operations
- Modular architecture with clean separation of concerns
- Production-quality code structure with extensive testing
- Real encrypted configuration data (including API tokens)
- Educational utilities and examples for learning

**Educational Focus:** This project demonstrates enterprise-level Rust programming patterns including cryptographic implementation, memory safety, functional programming style, comprehensive testing strategies, and production-quality software engineering practices.

## Architecture Overview

### Core Modules
- **`src/main.rs`**: Application orchestration and main workflow
- **`src/lib.rs`**: Library root with public API exports
- **`src/crypto/`**: AES-256-GCM encryption/decryption with key validation
  - **`mod.rs`**: Core cryptographic operations and validation functions
  - **`constants.rs`**: Pre-encrypted configuration data (real encrypted API tokens)
- **`src/config/`**: Configuration management, JSON deep merging, file I/O
  - **`types.rs`**: Serde data structures and configuration types
  - **`merger.rs`**: Deep JSON merging algorithms with comprehensive tests
  - **`file_ops.rs`**: Cross-platform file I/O operations and path handling
  - **`mod.rs`**: Module exports and public API
- **`src/errors.rs`**: Custom error types with user-friendly messages
- **`src/errors/`**: Extended error handling modules (if needed)
- **`src/input.rs`**: Secure user input handling and Base64 validation
- **`src/self_deletion.rs`**: Platform-aware executable self-deletion

### Educational Resources
- **`examples/`**: Learning utilities and code generation tools
  - **`generate_constants.rs`**: Utility to generate encrypted configuration constants
- **`PRD.md`**: Product Requirements Document with detailed specifications
- **`workflow.md`**: Comprehensive implementation workflow and educational guide
- **`README.md`**: Detailed project documentation and usage instructions

### Key Dependencies
- `aes-gcm`: Cryptographic operations (AES-256-GCM)
- `serde`/`serde_json`: JSON serialization and manipulation
- `base64`: Base64 encoding/decoding for user inputs
- `rpassword`: Secure password input without terminal echo
- `self-replace`: Cross-platform executable self-deletion
- `thiserror`: Custom error type generation
- `dirs`: Cross-platform home directory detection
- `tempfile`: Testing utilities for file operations

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

### Educational Utilities
```bash
# Generate encrypted constants for embedding in binary
cargo run --example generate_constants

# View the original JSON configuration (debug builds only)
# This shows what gets encrypted and embedded in the binary

# Build and analyze the encrypted configuration flow
cargo build && ./target/debug/claude-token-provider
```

### Testing and Validation
```bash
# Run comprehensive test suite (covers all modules)
cargo test

# Run tests with detailed output
cargo test -- --nocapture

# Run specific test modules
cargo test crypto::tests         # Cryptographic operations
cargo test config::tests         # Configuration management
cargo test config::merger::tests # JSON deep merging
cargo test config::file_ops      # File operations

# Test individual modules
cargo test --lib crypto
cargo test --lib config
cargo test --lib errors

# Integration tests (if present)
cargo test --test integration_test

# Test with backtraces for debugging
RUST_BACKTRACE=1 cargo test

# Test with full debug output
RUST_BACKTRACE=full cargo test
```

### Code Quality and Analysis
```bash
# Format code to Rust standards
cargo fmt

# Run Clippy linter with all checks
cargo clippy --all-targets --all-features

# Check compilation without building
cargo check

# Check individual modules
cargo check --lib
cargo check --examples

# Generate and open documentation
cargo doc --open

# Generate documentation with private items
cargo doc --document-private-items --open

# Dependency security audit
cargo audit
```

### Development Analysis
```bash
# Analyze dependency tree
cargo tree

# Verbose build output with timing
cargo build -v --timings

# Check for unused dependencies
cargo machete

# Binary size analysis
cargo bloat --release

# Profile compilation time
cargo build --timings

# Check for security vulnerabilities in dependencies
cargo audit
```

## Demo Usage (Educational Only)

This project includes demo credentials for educational testing:

**Demo Credentials:**
- **Secret Key**: `AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=` (32 zero bytes, Base64)
- **IV/Nonce**: `AAAAAAAAAAAAAAAA` (12 zero bytes, Base64)

**Configuration Target:** `~/.config/my_app/config.json`

**⚠️ These are zero-byte demo keys for educational purposes only!**

### What the Application Does

1. **Prompts for Credentials**: Securely requests the AES-256-GCM key and nonce from the user
2. **Decrypts Configuration**: Uses authenticated encryption to decrypt the embedded configuration data
3. **Parses JSON**: Converts the decrypted bytes into a JSON configuration object
4. **Merges Configuration**: Performs deep JSON merging with any existing configuration file
5. **Self-Deletes**: Attempts to remove the executable after successful completion

### Real Configuration Data

The embedded configuration includes real encrypted data (including API tokens) for educational purposes. This demonstrates:
- Real-world cryptographic implementation
- Secure handling of sensitive configuration data
- Production-quality encryption practices
- Authentication and integrity verification

**Note:** The encrypted configuration contains actual API tokens for demonstration purposes - this showcases how sensitive data would be handled in real applications.

## Security Considerations

### What This Tool Demonstrates
- Authenticated encryption with AES-256-GCM (NIST-approved standard)
- Secure input validation and comprehensive error handling
- Memory-safe cryptographic programming with Rust's ownership system
- Cross-platform file system security and safe path handling
- Secure application lifecycle management with self-deletion
- Real-world handling of encrypted sensitive data (API tokens)
- Production-quality security practices and validation

### Educational Security Features
- **AEAD Cryptography**: Uses authenticated encryption to prevent tampering
- **Input Sanitization**: Comprehensive validation of Base64 inputs and key/nonce lengths
- **Error Handling**: Security-conscious error messages that don't leak information
- **Memory Safety**: Rust's ownership system prevents buffer overflows and use-after-free
- **Secure Defaults**: Fails securely when operations cannot be completed safely

### Educational Limitations
- Uses demo keys for reproducible examples (zero-byte keys)
- No key derivation functions (PBKDF2/Argon2) implemented
- Sensitive data not explicitly cleared from memory using zeroization
- Self-deletion may be blocked by antivirus software or file permissions
- No protection against memory dumps or debugging during execution

### Safe Usage Guidelines
- **Required**: Run only in isolated environments (VMs/containers)
- **Study Focus**: Implementation patterns, security concepts, and code quality
- **Real Data**: Contains encrypted API tokens for educational demonstration
- **Never**: Deploy or use in production systems or with production credentials
- **Testing**: Use only with the provided demo credentials or test data

## Code Organization Patterns

### Error Handling Strategy
- Custom error types using `thiserror` for idiomatic Rust errors and automatic trait derivation
- Comprehensive error propagation with `Result<T>` types throughout all modules
- User-friendly error messages separate from technical details (dual message system)
- Error recovery patterns for interactive user input with retry mechanisms
- Recoverable vs non-recoverable error classification for user experience

### Cryptographic Patterns
- Authenticated encryption (AEAD) using AES-256-GCM with proper key/nonce validation
- Input validation before cryptographic operations (Base64 format, length checking)
- Memory-safe byte array handling with fixed-size arrays and proper bounds checking
- Proper error handling for cryptographic failures without information leakage
- Real encrypted configuration data embedded in binary constants

### Configuration Management
- Deep JSON merging algorithms with recursive object handling
- Graceful handling of corrupted configuration files (overwrite vs recovery)
- Cross-platform path handling with the `dirs` crate and proper directory creation
- Atomic file operations with proper error recovery and rollback capabilities
- Serde-based type-safe configuration structures with defaults

### Module Architecture
- **Separation of Concerns**: Each module has a single, well-defined responsibility
- **Clean APIs**: Public interfaces are minimal and focused
- **Testing Integration**: Tests are co-located with implementation code
- **Documentation**: Comprehensive rustdoc documentation for all public APIs

## Testing Strategy

### Comprehensive Unit Tests
- **Cryptographic Tests**: Round-trip encryption/decryption, authentication failure detection
- **Input Validation**: Base64 format validation, key/nonce length validation, edge cases
- **Error Handling**: All error variants tested, error message accuracy, recovery paths
- **JSON Merging**: Complex nested object merging, array handling, primitive replacement
- **File Operations**: Directory creation, file overwriting, permission handling, corruption recovery

### Module-Specific Testing
- **crypto::tests**: Cryptographic operations, key validation, tampered data detection
- **config::merger::tests**: Deep JSON merging algorithms with complex test cases
- **config::file_ops**: File system operations, cross-platform path handling
- **input validation**: Base64 decoding, user input sanitization, interactive retry logic

### Integration Testing
- End-to-end workflow simulation with temporary directories and files
- File system operation testing across different platforms
- Error propagation chain validation from input to output
- Cross-platform compatibility testing (Linux, macOS, Windows considerations)
- Real encrypted data decryption and configuration application

### Security Testing
- Tampered ciphertext detection (authentication tag verification)
- Invalid input handling (malformed Base64, incorrect lengths, invalid characters)
- Authentication failure scenarios (wrong keys, corrupted data)
- Boundary condition testing (empty inputs, maximum lengths, edge cases)
- Memory safety validation (no buffer overflows, proper bounds checking)

### Educational Testing Examples
- Demonstrates proper test organization and structure
- Shows comprehensive error condition coverage
- Includes performance and security test patterns
- Provides examples of testing cryptographic code safely

## Important Reminders

1. **Self-Deletion**: This application will delete itself after execution (may be blocked by antivirus)
2. **Educational Only**: Comprehensive learning tool, not intended for production use
3. **Demo Keys**: Uses predictable zero-byte keys for reproducible learning examples
4. **Real Encrypted Data**: Contains actual encrypted API tokens for educational demonstration
5. **Isolated Environment**: Always run in VMs or containers due to self-deletion behavior
6. **Security Focus**: Study the production-quality security patterns and implementations
7. **Comprehensive Testing**: Extensive test suite demonstrates proper testing strategies
8. **Documentation**: Review README.md, workflow.md, and PRD.md for complete context

## Learning Outcomes

By studying this project, developers will gain practical experience with:

### Advanced Rust Programming
- **Memory Safety**: Understanding ownership, borrowing, and lifetime management in cryptographic contexts
- **Error Handling**: Custom error types, comprehensive propagation, and user-friendly error design
- **Functional Programming**: Iterator patterns, Option/Result chaining, and immutability by default
- **Module System**: Clean separation of concerns, public API design, and documentation practices
- **Testing**: Unit, integration, and security testing strategies for systems programming

### Cryptographic Engineering
- **AEAD Implementation**: Authenticated encryption with AES-256-GCM using industry-standard libraries
- **Key Management**: Secure handling, validation, and lifecycle management of cryptographic material
- **Input Validation**: Comprehensive sanitization and validation of user-provided cryptographic inputs
- **Security Best Practices**: Defense in depth, secure defaults, and information leakage prevention

### Systems Programming
- **File Operations**: Cross-platform path handling, atomic operations, and error recovery
- **Self-Modification**: Understanding and implementing executable self-deletion patterns
- **User Interface**: Secure input handling, interactive error recovery, and user experience design
- **Production Quality**: Professional-grade error messages, logging, and operational considerations

### Software Engineering Excellence
- **Architecture**: Modular design, clean APIs, and maintainable code organization
- **Documentation**: Comprehensive technical documentation and educational resources
- **Testing**: Professional testing strategies covering functionality, security, and edge cases
- **Build Systems**: Advanced Cargo usage, examples, utilities, and development workflows