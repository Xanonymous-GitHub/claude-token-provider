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

impl TokenProviderError {
    /// Returns whether the error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            TokenProviderError::InvalidBase64(_)
            | TokenProviderError::InvalidKeyLength { .. }
            | TokenProviderError::InvalidIvLength { .. } => true,
            _ => false,
        }
    }

    /// Returns a user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            TokenProviderError::InvalidBase64(_) => {
                "Please check your base64 input format".to_string()
            }
            TokenProviderError::InvalidKeyLength { .. } => {
                "Secret key must be exactly 32 bytes when decoded".to_string()
            }
            TokenProviderError::InvalidIvLength { .. } => {
                "IV/Nonce must be exactly 12 bytes when decoded".to_string()
            }
            TokenProviderError::CryptoError(_) => {
                "Key or IV mismatch, or data corruption detected".to_string()
            }
            _ => self.to_string(),
        }
    }
}
