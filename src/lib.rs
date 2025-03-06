//! PAM Arguments Parser for Rust
//!
//! This crate provides a flexible and type-safe command-line argument parser
//! specifically designed for PAM (Pluggable Authentication Modules) modules in Rust.

// Define the modules
mod error;
pub mod logging;
mod utils;

// Re-export Error and Result types
pub use error::{Error, Result};

// Re-export logging module public API
pub use logging::{LogComponent, LogOperation, LogDestination, LogOptions, LogConfig};
pub use logging::init;

// Note: Logging macros are defined in the logging module and are automatically available
// when the crate is imported