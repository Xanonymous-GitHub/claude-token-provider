You are a Senior Principal Rust Engineer specializing in secure, high-performance systems programming. Your work reflects a deep understanding of computer science fundamentals, cryptographic principles, and modern software architecture.

Your task is to implement a small, single-purpose command-line application in Rust. Adhere strictly to the following specifications.

1. Core Project Specifications
Rust Edition: 2021

Rust Version: 1.88.0

Paradigms: Prioritize functional-style data manipulation (iterators, Option/Result), immutability by default, and robust error handling. The code must be clean, well-commented, and production-quality.

Dependencies: Generate a complete Cargo.toml file. Use the following high-quality, modern crates:

aes-gcm for cryptography.

serde and serde_json for JSON manipulation.

base64 for decoding user input.

rpassword for securely reading the secret key from stdin.

self_replace for handling the self-deletion of the executable.

thiserror for creating idiomatic, descriptive error types.

2. Program Logic and Flow
The application will execute the following steps in order:

Embed Encrypted Data:

Take the following plaintext JSON data:

JSON

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
In your generated code, you must programmatically encrypt this JSON string using AES-256-GCM.

The key for this encryption will be derived from the user-supplied secret at runtime.

The nonce (IV) for this encryption will also be supplied by the user at runtime.

Hard-code the resulting ciphertext into the Rust source as a &[u8] slice.

Acquire User Input:

Prompt the user to enter the AES-256-GCM Secret Key. Use the rpassword::prompt_password function to read this from stdin without echoing the characters to the terminal.

Prompt the user to enter the AES-256-GCM IV (Nonce). This can be read normally from stdin.

Both the key and IV are expected to be Base64-encoded strings (standard encoding with padding). Your code must decode them into byte arrays. The key must be exactly 32 bytes and the IV must be exactly 12 bytes after decoding.

Decrypt and Verify Data:

Use the decoded user-supplied key and IV to decrypt the hard-coded ciphertext blob.

The aes-gcm crate will automatically verify the authentication tag. If decryption or verification fails, the program must exit with a clear error message indicating a key/IV mismatch or data corruption.

Parse and Apply JSON Configuration:

Upon successful decryption, parse the resulting plaintext bytes into a serde_json::Value.

Define a hard-coded target file path. Use a safe, platform-agnostic path, for example: ~/.config/my_app/config.json.

File Logic:

If the target directory (~/.config/my_app/) does not exist, create it.

If the config.json file does not exist, create it and write the decrypted JSON content to it, pretty-printed.

If config.json exists and contains valid JSON, perform a deep merge. The decrypted JSON's keys and values should be recursively added to or overwritten in the existing JSON object.

If config.json exists but is not valid JSON (e.g., it's corrupted or not a JSON file), completely overwrite it with the new, pretty-printed JSON content.

Self-Deletion:

After the file operation is successfully completed, the program must delete its own executable from the filesystem.

Use the self_replace::self_delete() function for this purpose. Handle any potential errors from this operation gracefully, printing a message to stderr if deletion fails. Acknowledge in a code comment the inherent risks and platform-dependencies of this action.

3. Error Handling
Implement a custom enum for your application's errors using thiserror. Create distinct error variants for:

Invalid Base64 input.

Incorrect key or IV length.

Cryptographic decryption/authentication failure.

JSON parsing errors.

All forms of I/O errors (file read, file write, directory creation).

Self-deletion failure.

The main function must return a Result<(), Error>, and any error that propagates up to main should be printed to stderr in a user-friendly format.

