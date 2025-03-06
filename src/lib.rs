//! PAM Arguments Parser for Rust
//!
//! This crate provides a flexible and type-safe command-line argument parser
//! specifically designed for PAM (Pluggable Authentication Modules) modules in Rust.

// Define the modules
mod error;
mod utils;

// Re-export Error and Result types
pub use error::{Error, Result};