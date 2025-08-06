//! Self-deletion functionality
//!
//! WARNING: This module implements potentially dangerous functionality.
//! Self-deleting executables can trigger antivirus software and may be
//! blocked by security systems. This is intended for educational purposes only.

use crate::{Result, TokenProviderError};
use self_replace::self_delete;
use std::env;

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
    let exe_path = env::current_exe().map_err(TokenProviderError::IoError)?;

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
