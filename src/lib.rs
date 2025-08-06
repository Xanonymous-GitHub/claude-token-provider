//! Claude Token Provider - Secure Configuration Management Tool
//!
//! This library provides functionality for managing encrypted configuration
//! data with automatic self-deletion capabilities.

pub mod config;
pub mod crypto;
pub mod errors;
pub mod input;
pub mod self_deletion;

pub use errors::{Result, TokenProviderError};
