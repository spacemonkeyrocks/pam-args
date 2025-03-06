//! Core argument types for the pam-args library.
//!
//! This module defines the fundamental argument types that represent command-line
//! arguments in PAM modules. These types are central to the public API and are
//! directly used by library consumers to define their argument structure.

use std::any::Any;
use std::fmt;
use crate::error::{Error, Result};

/// Represents a flag (boolean) command-line argument
#[derive(Debug, Clone)]
pub struct Flag {
    /// The name of the flag
    name: String,
    
    /// Description of the flag for help text
    description: String,
    
    /// List of arguments that this flag depends on
    dependencies: Vec<String>,
    
    /// List of arguments that this flag conflicts with
    exclusions: Vec<String>,
}

/// Represents a key-value pair command-line argument
pub struct KeyValue {
    /// The name of the key-value pair
    name: String,
    
    /// Description of the key-value pair for help text
    description: String,
    
    /// Whether this key-value pair is required
    required: bool,
    
    /// List of arguments that this key-value pair depends on
    dependencies: Vec<String>,
    
    /// List of arguments that this key-value pair conflicts with
    exclusions: Vec<String>,
    
    /// Allowed formats for this key-value pair
    allowed_formats: Vec<AllowedKeyValueFormats>,
    
    /// Allowed values for this key-value pair (if restricted)
    allowed_values: Option<Vec<String>>,
    
    /// Whether this key-value pair has a type converter
    has_type_converter: bool,
}

// Manual implementation of Debug for KeyValue
impl fmt::Debug for KeyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyValue")
            .field("name", &self.name)
            .field("description", &self.description)
            .field("required", &self.required)
            .field("dependencies", &self.dependencies)
            .field("exclusions", &self.exclusions)
            .field("allowed_formats", &self.allowed_formats)
            .field("allowed_values", &self.allowed_values)
            .field("has_type_converter", &self.has_type_converter)
            .finish()
    }
}

// Manual implementation of Clone for KeyValue
impl Clone for KeyValue {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            description: self.description.clone(),
            required: self.required,
            dependencies: self.dependencies.clone(),
            exclusions: self.exclusions.clone(),
            allowed_formats: self.allowed_formats.clone(),
            allowed_values: self.allowed_values.clone(),
            has_type_converter: self.has_type_converter,
        }
    }
}

/// Represents allowed formats for key-value pairs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AllowedKeyValueFormats {
    /// KEY=VALUE format (e.g., "USER=admin")
    KeyValue,
    
    /// KEY format without value (e.g., "DEBUG")
    KeyOnly,
    
    /// KEY= format with empty value (e.g., "EMPTY=")
    KeyEquals,
    
    /// Convenience type for all formats
    KeyAll,
}

impl Flag {
    /// Creates a new flag with the given name and description
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the flag
    /// * `description` - The description of the flag for help text
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::Flag;
    ///
    /// let debug_flag = Flag::new("DEBUG", "Enable debug mode");
    /// ```
    pub fn new<S1, S2>(name: S1, description: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            name: name.into(),
            description: description.into(),
            dependencies: Vec::new(),
            exclusions: Vec::new(),
        }
    }
    
    /// Adds a dependency to this flag
    ///
    /// The flag will only be considered if the dependency is present.
    ///
    /// # Arguments
    ///
    /// * `dependency` - The name of the argument that this flag depends on
    ///
    /// # Returns
    ///
    /// The flag with the dependency added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::Flag;
    ///
    /// let flag = Flag::new("VERBOSE", "Enable verbose output")
    ///     .depends_on("DEBUG");
    /// ```
    pub fn depends_on<S: Into<String>>(mut self, dependency: S) -> Self {
        self.dependencies.push(dependency.into());
        self
    }
    
    /// Adds an exclusion to this flag
    ///
    /// The flag will be rejected if the excluded argument is present.
    ///
    /// # Arguments
    ///
    /// * `exclusion` - The name of the argument that this flag excludes
    ///
    /// # Returns
    ///
    /// The flag with the exclusion added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::Flag;
    ///
    /// let flag = Flag::new("DEBUG", "Enable debug mode")
    ///     .excludes("QUIET");
    /// ```
    pub fn excludes<S: Into<String>>(mut self, exclusion: S) -> Self {
        self.exclusions.push(exclusion.into());
        self
    }
    
    /// Returns the name of this flag
    ///
    /// # Returns
    ///
    /// The name of the flag
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Returns the description of this flag
    ///
    /// # Returns
    ///
    /// The description of the flag
    pub fn description(&self) -> &str {
        &self.description
    }
    
    /// Returns the dependencies of this flag
    ///
    /// # Returns
    ///
    /// A slice of the dependencies
    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }
    
    /// Returns the exclusions of this flag
    ///
    /// # Returns
    ///
    /// A slice of the exclusions
    pub fn exclusions(&self) -> &[String] {
        &self.exclusions
    }
    
    /// Returns whether this flag has a binding
    ///
    /// # Returns
    ///
    /// true if the flag has a binding, false otherwise
    pub fn has_binding(&self) -> bool {
        // For simplicity, we're not implementing bindings in this version
        false
    }
}

impl KeyValue {
    /// Creates a new key-value pair with the given name and description
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the key-value pair
    /// * `description` - The description of the key-value pair for help text
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::KeyValue;
    ///
    /// let user_kv = KeyValue::new("USER", "Username for authentication");
    /// ```
    pub fn new<S1, S2>(name: S1, description: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            name: name.into(),
            description: description.into(),
            required: false,
            dependencies: Vec::new(),
            exclusions: Vec::new(),
            allowed_formats: vec![AllowedKeyValueFormats::KeyValue],
            allowed_values: None,
            has_type_converter: false,
        }
    }
    
    /// Sets this key-value pair as required
    ///
    /// # Returns
    ///
    /// The key-value pair with the required flag set
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::KeyValue;
    ///
    /// let kv = KeyValue::new("USER", "Username for authentication")
    ///     .required();
    /// ```
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    
    /// Adds a dependency to this key-value pair
    ///
    /// The key-value pair will only be considered if the dependency is present.
    ///
    /// # Arguments
    ///
    /// * `dependency` - The name of the argument that this key-value pair depends on
    ///
    /// # Returns
    ///
    /// The key-value pair with the dependency added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::KeyValue;
    ///
    /// let kv = KeyValue::new("PORT", "Port number")
    ///     .depends_on("HOST");
    /// ```
    pub fn depends_on<S: Into<String>>(mut self, dependency: S) -> Self {
        self.dependencies.push(dependency.into());
        self
    }
    
    /// Adds an exclusion to this key-value pair
    ///
    /// The key-value pair will be rejected if the excluded argument is present.
    ///
    /// # Arguments
    ///
    /// * `exclusion` - The name of the argument that this key-value pair excludes
    ///
    /// # Returns
    ///
    /// The key-value pair with the exclusion added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::KeyValue;
    ///
    /// let kv = KeyValue::new("HOST", "Host name")
    ///     .excludes("LOCAL");
    /// ```
    pub fn excludes<S: Into<String>>(mut self, exclusion: S) -> Self {
        self.exclusions.push(exclusion.into());
        self
    }
    
    /// Sets the allowed formats for this key-value pair
    ///
    /// # Arguments
    ///
    /// * `formats` - Slice of allowed formats
    ///
    /// # Returns
    ///
    /// The key-value pair with the allowed formats set
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::{KeyValue, AllowedKeyValueFormats};
    ///
    /// let kv = KeyValue::new("DEBUG", "Enable debug mode")
    ///     .allowed_formats(&[
    ///         AllowedKeyValueFormats::KeyOnly,
    ///         AllowedKeyValueFormats::KeyValue,
    ///     ]);
    /// ```
    pub fn allowed_formats(mut self, formats: &[AllowedKeyValueFormats]) -> Self {
        self.allowed_formats = formats.to_vec();
        self
    }
    
    /// Sets the allowed values for this key-value pair
    ///
    /// # Arguments
    ///
    /// * `values` - Slice of allowed values
    ///
    /// # Returns
    ///
    /// The key-value pair with the allowed values set
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::KeyValue;
    ///
    /// let kv = KeyValue::new("ALIGN", "Text alignment")
    ///     .allowed_values(&["LEFT", "CENTER", "RIGHT"]);
    /// ```
    pub fn allowed_values<S: AsRef<str>>(mut self, values: &[S]) -> Self {
        self.allowed_values = Some(values.iter().map(|s| s.as_ref().to_string()).collect());
        self
    }
    
    /// Sets the type converter function for this key-value pair
    ///
    /// # Arguments
    ///
    /// * `converter` - The function to use for type conversion
    ///
    /// # Returns
    ///
    /// The key-value pair with the type converter set
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::KeyValue;
    /// use std::str::FromStr;
    ///
    /// let kv = KeyValue::new("WIDTH", "Width in pixels")
    ///     .type_converter(i32::from_str);
    /// ```
    pub fn type_converter<T, E>(mut self, _converter: fn(&str) -> std::result::Result<T, E>) -> Self
    where
        T: 'static + std::any::Any,
        E: std::fmt::Display,
    {
        // For simplicity, we're just setting a flag to indicate that a type converter was set
        self.has_type_converter = true;
        self
    }
    
    /// Returns the name of this key-value pair
    ///
    /// # Returns
    ///
    /// The name of the key-value pair
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Returns the description of this key-value pair
    ///
    /// # Returns
    ///
    /// The description of the key-value pair
    pub fn description(&self) -> &str {
        &self.description
    }
    
    /// Returns whether this key-value pair is required
    ///
    /// # Returns
    ///
    /// true if the key-value pair is required, false otherwise
    pub fn is_required(&self) -> bool {
        self.required
    }
    
    /// Returns the dependencies of this key-value pair
    ///
    /// # Returns
    ///
    /// A slice of the dependencies
    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }
    
    /// Returns the exclusions of this key-value pair
    ///
    /// # Returns
    ///
    /// A slice of the exclusions
    pub fn exclusions(&self) -> &[String] {
        &self.exclusions
    }
    
    /// Returns the allowed formats for this key-value pair
    ///
    /// # Returns
    ///
    /// A slice of the allowed formats
    pub fn get_allowed_formats(&self) -> &[AllowedKeyValueFormats] {
        &self.allowed_formats
    }
    
    /// Returns the allowed values for this key-value pair
    ///
    /// # Returns
    ///
    /// An optional slice of the allowed values
    pub fn get_allowed_values(&self) -> Option<&[String]> {
        self.allowed_values.as_deref()
    }
    
    /// Checks if a value is allowed for this key-value pair
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check
    ///
    /// # Returns
    ///
    /// true if the value is allowed, false otherwise
    pub fn is_value_allowed(&self, value: &str) -> bool {
        match &self.allowed_values {
            Some(values) => values.iter().any(|v| v == value),
            None => true,
        }
    }
    
    /// Returns whether this key-value pair has a type converter
    ///
    /// # Returns
    ///
    /// true if the key-value pair has a type converter, false otherwise
    pub fn has_type_converter(&self) -> bool {
        self.has_type_converter
    }
    
    /// Returns whether this key-value pair has a binding
    ///
    /// # Returns
    ///
    /// true if the key-value pair has a binding, false otherwise
    pub fn has_binding(&self) -> bool {
        // For simplicity, we're not implementing bindings in this version
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_flag_creation() {
        let flag = Flag::new("DEBUG", "Enable debug mode");
        assert_eq!(flag.name(), "DEBUG");
        assert_eq!(flag.description(), "Enable debug mode");
        assert!(flag.dependencies().is_empty());
        assert!(flag.exclusions().is_empty());
        assert!(!flag.has_binding());
    }

    #[test]
    fn test_flag_dependencies() {
        let flag = Flag::new("VERBOSE", "Verbose output")
            .depends_on("DEBUG");
        
        assert_eq!(flag.dependencies().len(), 1);
        assert_eq!(flag.dependencies()[0], "DEBUG");
    }

    #[test]
    fn test_flag_exclusions() {
        let flag = Flag::new("DEBUG", "Debug mode")
            .excludes("QUIET");
        
        assert_eq!(flag.exclusions().len(), 1);
        assert_eq!(flag.exclusions()[0], "QUIET");
    }

    #[test]
    fn test_keyvalue_creation() {
        let kv = KeyValue::new("USER", "Username for authentication");
        assert_eq!(kv.name(), "USER");
        assert_eq!(kv.description(), "Username for authentication");
        assert!(!kv.is_required());
        assert!(kv.dependencies().is_empty());
        assert!(kv.exclusions().is_empty());
        assert_eq!(kv.get_allowed_formats().len(), 1);
        assert_eq!(kv.get_allowed_formats()[0], AllowedKeyValueFormats::KeyValue);
        assert!(kv.get_allowed_values().is_none());
        assert!(!kv.has_type_converter());
        assert!(!kv.has_binding());
    }

    #[test]
    fn test_keyvalue_required() {
        let kv = KeyValue::new("USER", "Username")
            .required();
        
        assert!(kv.is_required());
    }

    #[test]
    fn test_keyvalue_dependencies() {
        let kv = KeyValue::new("PORT", "Port number")
            .depends_on("HOST");
        
        assert_eq!(kv.dependencies().len(), 1);
        assert_eq!(kv.dependencies()[0], "HOST");
    }

    #[test]
    fn test_keyvalue_exclusions() {
        let kv = KeyValue::new("HOST", "Host name")
            .excludes("LOCAL");
        
        assert_eq!(kv.exclusions().len(), 1);
        assert_eq!(kv.exclusions()[0], "LOCAL");
    }

    #[test]
    fn test_keyvalue_allowed_formats() {
        let kv = KeyValue::new("DEBUG", "Debug mode")
            .allowed_formats(&[
                AllowedKeyValueFormats::KeyOnly,
                AllowedKeyValueFormats::KeyValue,
            ]);
        
        assert_eq!(kv.get_allowed_formats().len(), 2);
        assert_eq!(kv.get_allowed_formats()[0], AllowedKeyValueFormats::KeyOnly);
        assert_eq!(kv.get_allowed_formats()[1], AllowedKeyValueFormats::KeyValue);
    }

    #[test]
    fn test_keyvalue_allowed_values() {
        let kv = KeyValue::new("ALIGN", "Text alignment")
            .allowed_values(&["LEFT", "CENTER", "RIGHT"]);
        
        assert!(kv.get_allowed_values().is_some());
        assert_eq!(kv.get_allowed_values().unwrap().len(), 3);
        assert_eq!(kv.get_allowed_values().unwrap()[0], "LEFT");
        assert_eq!(kv.get_allowed_values().unwrap()[1], "CENTER");
        assert_eq!(kv.get_allowed_values().unwrap()[2], "RIGHT");
        
        assert!(kv.is_value_allowed("LEFT"));
        assert!(kv.is_value_allowed("CENTER"));
        assert!(kv.is_value_allowed("RIGHT"));
        assert!(!kv.is_value_allowed("BOTTOM"));
    }
}