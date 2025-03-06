//! Storage and access module for the pam-args library.
//!
//! This module provides a flexible and efficient way to store and retrieve key-value pairs
//! parsed from arguments. It implements a trait-based abstraction layer for storage operations,
//! allowing for different storage backends while maintaining a consistent API.

use std::collections::HashMap;
use std::str::FromStr;
use crate::error::{Error, Result};

/// Trait defining the interface for key-value storage
pub trait KeyValueStore {
    /// Adds a key-value pair to the store
    ///
    /// # Arguments
    ///
    /// * `key` - The key to add
    /// * `value` - The value to associate with the key, or None for key-only entries
    fn add(&mut self, key: &str, value: Option<&str>);
    
    /// Retrieves a value from the store
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// The value associated with the key, or None if not found or key-only
    fn get(&self, key: &str) -> Option<&str>;
    
    /// Checks if a key exists in the store
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check
    ///
    /// # Returns
    ///
    /// true if the key exists, false otherwise
    fn has_key(&self, key: &str) -> bool;
    
    /// Returns all keys in the store
    ///
    /// # Returns
    ///
    /// A vector of key references
    fn keys(&self) -> Vec<&str>;
    
    /// Returns the number of entries in the store
    ///
    /// # Returns
    ///
    /// The number of entries
    fn len(&self) -> usize;
    
    /// Checks if the store is empty
    ///
    /// # Returns
    ///
    /// true if the store is empty, false otherwise
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Default implementation of KeyValueStore using HashMap
#[derive(Debug, Clone)]
pub struct DefaultKeyValueStore {
    /// Storage for key-value pairs
    store: HashMap<String, Option<String>>,
    
    /// Whether keys are case-sensitive
    case_sensitive: bool,
}

impl DefaultKeyValueStore {
    /// Creates a new, empty key-value store
    ///
    /// # Arguments
    ///
    /// * `case_sensitive` - Whether keys are case-sensitive
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create a case-sensitive store
    /// let store = DefaultKeyValueStore::new(true);
    /// ```
    pub fn new(case_sensitive: bool) -> Self {
        Self {
            store: HashMap::new(),
            case_sensitive,
        }
    }
    
    /// Sets whether keys are case-sensitive
    ///
    /// # Arguments
    ///
    /// * `case_sensitive` - Whether keys are case-sensitive
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use pam_args::storage::DefaultKeyValueStore;
    ///
    /// let mut store = DefaultKeyValueStore::new(true);
    /// store.set_case_sensitive(false);
    /// ```
    pub fn set_case_sensitive(&mut self, case_sensitive: bool) {
        self.case_sensitive = case_sensitive;
    }
    
    /// Normalizes a key according to case sensitivity settings
    ///
    /// # Arguments
    ///
    /// * `key` - The key to normalize
    ///
    /// # Returns
    ///
    /// The normalized key
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use pam_args::storage::DefaultKeyValueStore;
    ///
    /// let store = DefaultKeyValueStore::new(false);
    /// assert_eq!(store.normalize_key("DEBUG"), "debug");
    /// ```
    pub fn normalize_key(&self, key: &str) -> String {
        if self.case_sensitive {
            key.to_string()
        } else {
            key.to_lowercase()
        }
    }
    
    /// Gets a value with type conversion
    ///
    /// # Type Parameters
    ///
    /// * `T` - The target type
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// The converted value, or None if not found or conversion failed
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use pam_args::storage::DefaultKeyValueStore;
    ///
    /// let mut store = DefaultKeyValueStore::new(true);
    /// store.add("WIDTH", Some("80"));
    ///
    /// assert_eq!(store.value_of::<i32>("WIDTH"), Some(80));
    /// ```
    pub fn value_of<T>(&self, key: &str) -> Option<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.get(key).and_then(|value| value.parse::<T>().ok())
    }
    
    /// Clears the store
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use pam_args::storage::DefaultKeyValueStore;
    ///
    /// let mut store = DefaultKeyValueStore::new(true);
    /// store.add("KEY", Some("VALUE"));
    /// store.clear();
    /// assert!(store.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.store.clear();
    }
}

impl KeyValueStore for DefaultKeyValueStore {
    fn add(&mut self, key: &str, value: Option<&str>) {
        let normalized_key = self.normalize_key(key);
        let owned_value = value.map(String::from);
        self.store.insert(normalized_key, owned_value);
    }
    
    fn get(&self, key: &str) -> Option<&str> {
        let normalized_key = self.normalize_key(key);
        self.store.get(&normalized_key).and_then(|opt| opt.as_deref())
    }
    
    fn has_key(&self, key: &str) -> bool {
        let normalized_key = self.normalize_key(key);
        self.store.contains_key(&normalized_key)
    }
    
    fn keys(&self) -> Vec<&str> {
        self.store.keys().map(|s| s.as_str()).collect()
    }
    
    fn len(&self) -> usize {
        self.store.len()
    }
}

/// Storage for non-argument text
#[derive(Debug, Clone, Default)]
pub struct NonArgTextStore {
    /// Vector of non-argument text
    text: Vec<String>,
}

impl NonArgTextStore {
    /// Creates a new, empty non-argument text store
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use pam_args::storage::NonArgTextStore;
    ///
    /// let store = NonArgTextStore::new();
    /// ```
    pub fn new() -> Self {
        Self { text: Vec::new() }
    }
    
    /// Adds a non-argument text string to the store
    ///
    /// # Arguments
    ///
    /// * `text` - The text to add
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use pam_args::storage::NonArgTextStore;
    ///
    /// let mut store = NonArgTextStore::new();
    /// store.add("This is non-argument text");
    /// ```
    pub fn add<S: Into<String>>(&mut self, text: S) {
        self.text.push(text.into());
    }
    
    /// Adds multiple non-argument text strings to the store
    ///
    /// # Arguments
    ///
    /// * `texts` - The texts to add
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use pam_args::storage::NonArgTextStore;
    ///
    /// let mut store = NonArgTextStore::new();
    /// store.add_multiple(vec!["Text 1", "Text 2"]);
    /// ```
    pub fn add_multiple<S, I>(&mut self, texts: I)
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        for text in texts {
            self.text.push(text.into());
        }
    }
    
    /// Retrieves all non-argument text
    ///
    /// # Returns
    ///
    /// A slice of all stored text strings
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use pam_args::storage::NonArgTextStore;
    ///
    /// let mut store = NonArgTextStore::new();
    /// store.add("Text");
    ///
    /// assert_eq!(store.texts(), &["Text"]);
    /// ```
    pub fn texts(&self) -> &[String] {
        &self.text
    }
    
    /// Returns the number of text entries
    ///
    /// # Returns
    ///
    /// The number of entries
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use pam_args::storage::NonArgTextStore;
    ///
    /// let mut store = NonArgTextStore::new();
    /// store.add("Text");
    ///
    /// assert_eq!(store.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.text.len()
    }
    
    /// Checks if the store is empty
    ///
    /// # Returns
    ///
    /// true if the store is empty, false otherwise
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use pam_args::storage::NonArgTextStore;
    ///
    /// let store = NonArgTextStore::new();
    /// assert!(store.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
    
    /// Clears the store
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use pam_args::storage::NonArgTextStore;
    ///
    /// let mut store = NonArgTextStore::new();
    /// store.add("Text");
    /// store.clear();
    /// assert!(store.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.text.clear();
    }
}

/// Type conversion extension for key-value stores
pub trait KeyValueStoreExt: KeyValueStore {
    /// Gets a value with type conversion
    ///
    /// # Type Parameters
    ///
    /// * `T` - The target type
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// The converted value, or None if not found or conversion failed
    fn value_of<T>(&self, key: &str) -> Option<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.get(key).and_then(|value| value.parse::<T>().ok())
    }
}

// Implement the extension trait for any type that implements KeyValueStore
impl<T: KeyValueStore> KeyValueStoreExt for T {}

/// A trait for converting string arguments to specific types
pub trait FromArgValue: Sized {
    /// Converts a string argument value to this type
    ///
    /// # Arguments
    ///
    /// * `value` - The string value to convert
    ///
    /// # Returns
    ///
    /// The converted value or an error
    fn from_arg_value(value: &str) -> Result<Self>;
}

// Implement FromArgValue for common types
impl FromArgValue for String {
    fn from_arg_value(value: &str) -> Result<Self> {
        Ok(value.to_string())
    }
}

impl FromArgValue for i32 {
    fn from_arg_value(value: &str) -> Result<Self> {
        value.parse::<i32>().map_err(|_| {
            Error::InvalidIntValue(value.to_string())
        })
    }
}

impl FromArgValue for bool {
    fn from_arg_value(value: &str) -> Result<Self> {
        match value.to_lowercase().as_str() {
            "true" | "yes" | "1" | "on" => Ok(true),
            "false" | "no" | "0" | "off" => Ok(false),
            _ => Err(Error::InvalidBoolValue(value.to_string())),
        }
    }
}

impl FromArgValue for char {
    fn from_arg_value(value: &str) -> Result<Self> {
        let mut chars = value.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => Ok(c),
            _ => Err(Error::InvalidInput(format!(
                "Expected a single character, got: '{}'", value
            ))),
        }
    }
}

// Implement FromArgValue for Option<T> types
impl<T: FromArgValue> FromArgValue for Option<T> {
    fn from_arg_value(value: &str) -> Result<Self> {
        if value.is_empty() {
            Ok(None)
        } else {
            T::from_arg_value(value).map(Some)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_store_creation() {
        let store = DefaultKeyValueStore::new(true);
        assert!(store.is_empty());
    }

    #[test]
    fn test_default_store_add_get() {
        let mut store = DefaultKeyValueStore::new(true);
        store.add("KEY", Some("value"));
        assert_eq!(store.get("KEY"), Some("value"));
    }

    #[test]
    fn test_default_store_case_sensitivity() {
        // Case-sensitive store
        let mut store = DefaultKeyValueStore::new(true);
        store.add("KEY", Some("value"));
        assert_eq!(store.get("KEY"), Some("value"));
        assert_eq!(store.get("key"), None);

        // Case-insensitive store
        let mut store = DefaultKeyValueStore::new(false);
        store.add("KEY", Some("value"));
        assert_eq!(store.get("KEY"), Some("value"));
        assert_eq!(store.get("key"), Some("value"));
    }

    #[test]
    fn test_default_store_key_only() {
        let mut store = DefaultKeyValueStore::new(true);
        store.add("DEBUG", None);
        assert_eq!(store.get("DEBUG"), None);
        assert!(store.has_key("DEBUG"));
    }

    #[test]
    fn test_default_store_key_equals() {
        let mut store = DefaultKeyValueStore::new(true);
        store.add("EMPTY", Some(""));
        assert_eq!(store.get("EMPTY"), Some(""));
        assert!(store.has_key("EMPTY"));
    }

    #[test]
    fn test_default_store_keys() {
        let mut store = DefaultKeyValueStore::new(true);
        store.add("KEY1", Some("value1"));
        store.add("KEY2", Some("value2"));
        
        let keys = store.keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"KEY1"));
        assert!(keys.contains(&"KEY2"));
    }

    #[test]
    fn test_default_store_value_of() {
        let mut store = DefaultKeyValueStore::new(true);
        store.add("PORT", Some("8080"));
        
        assert_eq!(store.value_of::<i32>("PORT"), Some(8080));
        assert_eq!(store.value_of::<String>("PORT"), Some("8080".to_string()));
        assert_eq!(store.value_of::<i32>("UNKNOWN"), None);
    }

    #[test]
    fn test_default_store_clear() {
        let mut store = DefaultKeyValueStore::new(true);
        store.add("KEY", Some("value"));
        assert!(!store.is_empty());
        
        store.clear();
        assert!(store.is_empty());
    }

    #[test]
    fn test_non_arg_text_store() {
        let mut store = NonArgTextStore::new();
        assert!(store.is_empty());
        
        store.add("Text 1");
        assert_eq!(store.len(), 1);
        assert_eq!(store.texts(), &["Text 1"]);
        
        store.add_multiple(vec!["Text 2", "Text 3"]);
        assert_eq!(store.len(), 3);
        assert_eq!(store.texts(), &["Text 1", "Text 2", "Text 3"]);
        
        store.clear();
        assert!(store.is_empty());
    }

    #[test]
    fn test_from_arg_value_string() {
        let result = String::from_arg_value("test");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn test_from_arg_value_i32() {
        let result = i32::from_arg_value("123");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 123);
        
        let result = i32::from_arg_value("abc");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::InvalidIntValue(_)));
    }

    #[test]
    fn test_from_arg_value_bool() {
        // Test true values
        for value in &["true", "yes", "1", "on"] {
            let result = bool::from_arg_value(value);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), true);
        }
        
        // Test false values
        for value in &["false", "no", "0", "off"] {
            let result = bool::from_arg_value(value);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), false);
        }
        
        // Test invalid value
        let result = bool::from_arg_value("invalid");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::InvalidBoolValue(_)));
    }

    #[test]
    fn test_from_arg_value_char() {
        let result = char::from_arg_value("a");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 'a');
        
        let result = char::from_arg_value("abc");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::InvalidInput(_)));
    }

    #[test]
    fn test_from_arg_value_option() {
        let result = Option::<i32>::from_arg_value("123");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(123));
        
        let result = Option::<i32>::from_arg_value("");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_key_value_store_ext() {
        let mut store = DefaultKeyValueStore::new(true);
        store.add("PORT", Some("8080"));
        
        // Use the extension trait method
        let port: Option<i32> = KeyValueStoreExt::value_of(&store, "PORT");
        assert_eq!(port, Some(8080));
    }
}