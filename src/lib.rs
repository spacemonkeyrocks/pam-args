//! PAM Arguments Parser for Rust
//!
//! This crate provides a flexible and type-safe command-line argument parser
//! specifically designed for PAM (Pluggable Authentication Modules) modules in Rust.

// Define the modules
mod error;
pub mod logging;
mod utils;
mod args;
mod config;
pub(crate) mod conversion;
mod storage;
mod tokenizer;
#[cfg(test)]
mod testing;
#[cfg(test)]
mod conversion_tests;
#[cfg(test)]
mod storage_tests;

// Re-export Error and Result types
pub use error::{Error, Result};

// Re-export logging module public API
pub use logging::{LogComponent, LogOperation, LogDestination, LogOptions, LogConfig};
pub use logging::init;

// Re-export core argument types
pub use args::{Flag, KeyValue, AllowedKeyValueFormats};

// Re-export configuration types
pub use config::{ParserConfig, ParserConfigBuilder};

// Re-export storage module public API
pub use storage::{KeyValueStore, DefaultKeyValueStore, NonArgTextStore, KeyValueStoreExt, FromArgValue};
