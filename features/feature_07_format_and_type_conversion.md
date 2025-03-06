# Feature 7: Format & Type Conversion

## Module Type
**Conversion**: This component provides type conversion infrastructure for handling different argument formats and converting string values to native Rust types. It's used internally by parser components but exposes public types that are part of the library's API.

## Feature Information

**Feature Name**: Format & Type Conversion

**Description**: Handles supported formats and type conversion for arguments. This module defines the available key-value formats and provides a robust, type-safe conversion system that transforms string values from command-line arguments into native Rust types. It leverages Rust's trait system to provide clean error handling, sensible defaults, and extension points for advanced use cases.

**Priority**: High

**Dependencies**: 
- [Feature 1: Result & Error Types](error-result-types.md)
- [Feature 5: Core Argument Types](core-argument-types.md)

## Requirements

### Functional Requirements
1. Define supported key-value pair formats (key=value, key only, key with empty value)
2. Implement type conversion from string values to native Rust types
3. Support for common types (strings, integers, booleans, characters)
4. Provide clear error messages for conversion failures
5. Allow for extensibility to support user-defined types
6. Support for default values when conversion fails
7. Handle empty and quoted values properly
8. Validate converted values against constraints

### API Requirements
- Provide a clean, extendable API for type conversion
- Expose format types as part of the public API
- Enable user-defined conversions
- Ensure strong type safety to catch errors at compile time
- Provide clear error information when conversions fail
- Support the `std::str::FromStr` trait for type conversions
- Enable clean error propagation via the `?` operator

### Performance Requirements
- Minimize allocations during type conversion
- Optimize common conversion paths
- Ensure efficient handling of large values
- Minimize template instantiation for better compile times
- Avoid unnecessary string copies during conversion

## Design

### Data Structures
```rust
/// Represents allowed formats for key-value pairs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// Result of key-value format detection
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FormatDetectionResult {
    /// The detected format
    pub format: AllowedKeyValueFormats,
    
    /// The key part
    pub key: String,
    
    /// The value part (if applicable)
    pub value: Option<String>,
}

/// Type conversion error information
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConversionError {
    /// The value that failed to convert
    pub value: String,
    
    /// The target type name
    pub target_type: &'static str,
    
    /// The error message
    pub message: String,
}

/// Trait for types that can be parsed from a string
pub trait FromArgValue: Sized {
    /// Converts a string to this type
    ///
    /// # Arguments
    ///
    /// * `value` - The string value to convert
    ///
    /// # Returns
    ///
    /// The converted value or an error
    fn from_arg_value(value: &str) -> Result<Self, Error>;
}

/// Configuration for type converters
#[derive(Debug, Clone)]
pub struct ConverterConfig {
    /// Whether to trim whitespace before conversion
    pub trim_whitespace: bool,
    
    /// Whether to handle empty strings specially
    pub handle_empty: bool,
    
    /// Whether to treat "none" and "null" as None for Option types
    pub recognize_none_values: bool,
}

/// Static configuration for the conversion system
#[derive(Debug, Clone)]
pub(crate) struct ConversionConfig {
    /// Whether to enable case-insensitive boolean parsing
    pub case_insensitive_booleans: bool,
    
    /// List of true values for boolean parsing
    pub true_values: Vec<&'static str>,
    
    /// List of false values for boolean parsing
    pub false_values: Vec<&'static str>,
    
    /// List of none values for option parsing
    pub none_values: Vec<&'static str>,
}
```

### Function Signatures
```rust
impl AllowedKeyValueFormats {
    /// Returns whether this format is compatible with the given format
    ///
    /// # Arguments
    ///
    /// * `other` - The format to check for compatibility with
    ///
    /// # Returns
    ///
    /// true if this format is compatible with the other format
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::AllowedKeyValueFormats;
    ///
    /// assert!(AllowedKeyValueFormats::KeyAll.is_compatible_with(AllowedKeyValueFormats::KeyValue));
    /// assert!(AllowedKeyValueFormats::KeyValue.is_compatible_with(AllowedKeyValueFormats::KeyValue));
    /// assert!(!AllowedKeyValueFormats::KeyValue.is_compatible_with(AllowedKeyValueFormats::KeyOnly));
    /// ```
    pub fn is_compatible_with(&self, other: AllowedKeyValueFormats) -> bool {
        match self {
            AllowedKeyValueFormats::KeyAll => true,
            _ => *self == other,
        }
    }
    
    /// Returns whether this format is compatible with any of the given formats
    ///
    /// # Arguments
    ///
    /// * `formats` - The formats to check for compatibility with
    ///
    /// # Returns
    ///
    /// true if this format is compatible with any of the given formats
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::AllowedKeyValueFormats;
    ///
    /// let formats = vec![
    ///     AllowedKeyValueFormats::KeyValue,
    ///     AllowedKeyValueFormats::KeyOnly,
    /// ];
    ///
    /// assert!(AllowedKeyValueFormats::KeyAll.is_compatible_with_any(&formats));
    /// assert!(AllowedKeyValueFormats::KeyValue.is_compatible_with_any(&formats));
    /// assert!(AllowedKeyValueFormats::KeyOnly.is_compatible_with_any(&formats));
    /// assert!(!AllowedKeyValueFormats::KeyEquals.is_compatible_with_any(&formats));
    /// ```
    pub fn is_compatible_with_any(&self, formats: &[AllowedKeyValueFormats]) -> bool {
        if *self == AllowedKeyValueFormats::KeyAll {
            return true;
        }
        
        formats.iter().any(|format| self.is_compatible_with(*format))
    }
    
    /// Returns all possible formats
    ///
    /// # Returns
    ///
    /// A vector of all possible formats
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::AllowedKeyValueFormats;
    ///
    /// let all_formats = AllowedKeyValueFormats::all();
    /// assert_eq!(all_formats.len(), 3); // KeyValue, KeyOnly, KeyEquals
    /// ```
    pub fn all() -> Vec<AllowedKeyValueFormats> {
        vec![
            AllowedKeyValueFormats::KeyValue,
            AllowedKeyValueFormats::KeyOnly,
            AllowedKeyValueFormats::KeyEquals,
        ]
    }
}

/// Functions for detecting and validating key-value formats
pub(crate) mod format {
    use super::*;
    use crate::error::{Error, Result};
    
    /// Detects the format of a key-value string
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to analyze
    ///
    /// # Returns
    ///
    /// The detected format, key, and value (if applicable)
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::conversion::format;
    ///
    /// let result = format::detect("USER=admin");
    /// assert_eq!(result.format, AllowedKeyValueFormats::KeyValue);
    /// assert_eq!(result.key, "USER");
    /// assert_eq!(result.value, Some("admin".to_string()));
    ///
    /// let result = format::detect("DEBUG");
    /// assert_eq!(result.format, AllowedKeyValueFormats::KeyOnly);
    /// assert_eq!(result.key, "DEBUG");
    /// assert_eq!(result.value, None);
    /// ```
    pub(crate) fn detect(input: &str) -> FormatDetectionResult {
        // Check for KEY=VALUE format
        if let Some(equals_pos) = input.find('=') {
            let key = input[..equals_pos].to_string();
            let value_part = &input[equals_pos + 1..];
            
            // Check if there's anything after the equals sign
            if value_part.is_empty() {
                // KEY= format (empty value)
                FormatDetectionResult {
                    format: AllowedKeyValueFormats::KeyEquals,
                    key,
                    value: Some(String::new()),
                }
            } else {
                // KEY=VALUE format
                FormatDetectionResult {
                    format: AllowedKeyValueFormats::KeyValue,
                    key,
                    value: Some(value_part.to_string()),
                }
            }
        } else {
            // KEY format (no value)
            FormatDetectionResult {
                format: AllowedKeyValueFormats::KeyOnly,
                key: input.to_string(),
                value: None,
            }
        }
    }
    
    /// Validates that the detected format is allowed by the specified formats
    ///
    /// # Arguments
    ///
    /// * `detected` - The detected format
    /// * `allowed_formats` - The allowed formats
    ///
    /// # Returns
    ///
    /// Ok(()) if the format is allowed, an error otherwise
    pub(crate) fn validate(
        detected: &FormatDetectionResult,
        allowed_formats: &[AllowedKeyValueFormats],
    ) -> Result<()> {
        // Check if the detected format is compatible with any allowed format
        if detected.format.is_compatible_with_any(allowed_formats) {
            Ok(())
        } else {
            Err(Error::InvalidKeyValue(format!(
                "Invalid format for key '{}': expected one of {:?}, got {:?}",
                detected.key, allowed_formats, detected.format
            )))
        }
    }
}

/// Default implementation of the converter configuration
impl Default for ConverterConfig {
    fn default() -> Self {
        Self {
            trim_whitespace: true,
            handle_empty: true,
            recognize_none_values: true,
        }
    }
}

/// Default implementation of the conversion configuration
impl Default for ConversionConfig {
    fn default() -> Self {
        Self {
            case_insensitive_booleans: true,
            true_values: vec!["true", "yes", "1", "on"],
            false_values: vec!["false", "no", "0", "off"],
            none_values: vec!["none", "null", ""],
        }
    }
}

/// Implementation of FromArgValue for common types
impl FromArgValue for String {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        Ok(value.to_string())
    }
}

impl FromArgValue for i32 {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        value.parse::<i32>().map_err(|_| {
            Error::InvalidIntValue(value.to_string())
        })
    }
}

impl FromArgValue for bool {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        // Use the default conversion configuration
        let config = ConversionConfig::default();
        
        // If case-insensitive, convert to lowercase for comparison
        let compare_value = if config.case_insensitive_booleans {
            value.to_lowercase()
        } else {
            value.to_string()
        };
        
        // Check if the value matches any true value
        if config.true_values.iter().any(|v| *v == compare_value) {
            return Ok(true);
        }
        
        // Check if the value matches any false value
        if config.false_values.iter().any(|v| *v == compare_value) {
            return Ok(false);
        }
        
        // If no match, return an error
        Err(Error::InvalidBoolValue(value.to_string()))
    }
}

impl FromArgValue for char {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        // A character must be exactly one character long
        let chars: Vec<char> = value.chars().collect();
        
        if chars.len() == 1 {
            Ok(chars[0])
        } else {
            Err(Error::InvalidInput(format!(
                "Expected a single character, got '{}' ({} characters)",
                value,
                chars.len()
            )))
        }
    }
}

/// Implementation of FromArgValue for Option types
impl<T: FromArgValue> FromArgValue for Option<T> {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        // Use the default converter configuration
        let config = ConverterConfig::default();
        
        // Check for empty strings if configured to handle them
        if config.handle_empty && value.is_empty() {
            return Ok(None);
        }
        
        // Check for "none" and "null" values if configured to recognize them
        if config.recognize_none_values {
            let config = ConversionConfig::default();
            if config.none_values.contains(&value.to_lowercase().as_str()) {
                return Ok(None);
            }
        }
        
        // If the value is not None, convert it to the target type
        match T::from_arg_value(value) {
            Ok(converted) => Ok(Some(converted)),
            Err(e) => Err(e),
        }
    }
}

/// Main type conversion functions
pub(crate) mod converter {
    use super::*;
    use crate::error::{Error, Result};
    use std::any::Any;
    use std::str::FromStr;
    
    /// Converts a string value to the specified type
    ///
    /// # Type Parameters
    ///
    /// * `T` - The target type to convert to
    ///
    /// # Arguments
    ///
    /// * `value` - The string value to convert
    /// * `config` - Optional configuration for the conversion
    ///
    /// # Returns
    ///
    /// The converted value or an error
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::conversion::converter;
    ///
    /// let i: i32 = converter::convert("123", None)?;
    /// assert_eq!(i, 123);
    ///
    /// let b: bool = converter::convert("true", None)?;
    /// assert!(b);
    ///
    /// let s: String = converter::convert("hello", None)?;
    /// assert_eq!(s, "hello");
    /// ```
    pub fn convert<T: FromArgValue>(
        value: &str,
        config: Option<&ConverterConfig>,
    ) -> Result<T> {
        // Use the provided config or the default
        let config = config.unwrap_or(&ConverterConfig::default());
        
        // Pre-process the value if needed
        let processed_value = if config.trim_whitespace {
            value.trim()
        } else {
            value
        };
        
        // Perform the conversion
        T::from_arg_value(processed_value)
    }
    
    /// Converts a string value to a boxed Any trait object
    ///
    /// This is used for type-erased storage of converted values.
    ///
    /// # Arguments
    ///
    /// * `value` - The string value to convert
    /// * `converter` - The conversion function to use
    ///
    /// # Returns
    ///
    /// A boxed Any containing the converted value, or an error
    pub(crate) fn convert_to_any<T, E>(
        value: &str,
        converter: fn(&str) -> std::result::Result<T, E>,
    ) -> Result<Box<dyn Any + 'static>>
    where
        T: 'static + Any,
        E: std::fmt::Display,
    {
        match converter(value) {
            Ok(converted) => Ok(Box::new(converted)),
            Err(e) => {
                let type_name = std::any::type_name::<T>();
                Err(Error::InvalidInput(format!(
                    "Failed to convert '{}' to {}: {}",
                    value, type_name, e
                )))
            }
        }
    }
    
    /// Helper function to create a type converter for FromStr types
    ///
    /// # Type Parameters
    ///
    /// * `T` - The target type to convert to, must implement FromStr
    ///
    /// # Returns
    ///
    /// A function that converts a string to the target type using FromStr
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::conversion::converter;
    /// use pam_args_rs::KeyValue;
    ///
    /// let kv = KeyValue::new("WIDTH", "Width in pixels")
    ///     .type_converter(converter::from_str::<i32>());
    /// ```
    pub fn from_str<T: FromStr + 'static>() -> fn(&str) -> std::result::Result<T, T::Err> {
        T::from_str
    }
}
```

### Implementation Approach

#### 1. Key-Value Format Representation

The `AllowedKeyValueFormats` enum defines the supported formats for key-value pairs:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
```

This design:
- Clearly defines the supported formats
- Makes format constraints explicit
- Enables compile-time verification
- Supports compatibility checking between formats

#### 2. Format Detection and Validation

The library includes functions for detecting and validating key-value formats:

```rust
pub(crate) fn detect(input: &str) -> FormatDetectionResult {
    // Check for KEY=VALUE format
    if let Some(equals_pos) = input.find('=') {
        let key = input[..equals_pos].to_string();
        let value_part = &input[equals_pos + 1..];
        
        // Check if there's anything after the equals sign
        if value_part.is_empty() {
            // KEY= format (empty value)
            FormatDetectionResult {
                format: AllowedKeyValueFormats::KeyEquals,
                key,
                value: Some(String::new()),
            }
        } else {
            // KEY=VALUE format
            FormatDetectionResult {
                format: AllowedKeyValueFormats::KeyValue,
                key,
                value: Some(value_part.to_string()),
            }
        }
    } else {
        // KEY format (no value)
        FormatDetectionResult {
            format: AllowedKeyValueFormats::KeyOnly,
            key: input.to_string(),
            value: None,
        }
    }
}

pub(crate) fn validate(
    detected: &FormatDetectionResult,
    allowed_formats: &[AllowedKeyValueFormats],
) -> Result<()> {
    // Check if the detected format is compatible with any allowed format
    if detected.format.is_compatible_with_any(allowed_formats) {
        Ok(())
    } else {
        Err(Error::InvalidKeyValue(format!(
            "Invalid format for key '{}': expected one of {:?}, got {:?}",
            detected.key, allowed_formats, detected.format
        )))
    }
}
```

This approach:
- Separates detection from validation
- Makes the parsing process clear and maintainable
- Provides meaningful error messages
- Enables format constraints to be enforced consistently

#### 3. Type Conversion Trait

The `FromArgValue` trait provides a clean interface for type conversion:

```rust
pub trait FromArgValue: Sized {
    /// Converts a string to this type
    fn from_arg_value(value: &str) -> Result<Self, Error>;
}
```

This trait:
- Defines a standard interface for conversion
- Enables extensibility for user-defined types
- Ensures consistent error handling
- Provides a foundation for the type conversion system

#### 4. Type Conversion for Common Types

The library implements `FromArgValue` for common types:

```rust
impl FromArgValue for String {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        Ok(value.to_string())
    }
}

impl FromArgValue for i32 {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        value.parse::<i32>().map_err(|_| {
            Error::InvalidIntValue(value.to_string())
        })
    }
}

impl FromArgValue for bool {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        // Use the default conversion configuration
        let config = ConversionConfig::default();
        
        // If case-insensitive, convert to lowercase for comparison
        let compare_value = if config.case_insensitive_booleans {
            value.to_lowercase()
        } else {
            value.to_string()
        };
        
        // Check if the value matches any true value
        if config.true_values.iter().any(|v| *v == compare_value) {
            return Ok(true);
        }
        
        // Check if the value matches any false value
        if config.false_values.iter().any(|v| *v == compare_value) {
            return Ok(false);
        }
        
        // If no match, return an error
        Err(Error::InvalidBoolValue(value.to_string()))
    }
}

impl FromArgValue for char {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        // A character must be exactly one character long
        let chars: Vec<char> = value.chars().collect();
        
        if chars.len() == 1 {
            Ok(chars[0])
        } else {
            Err(Error::InvalidInput(format!(
                "Expected a single character, got '{}' ({} characters)",
                value,
                chars.len()
            )))
        }
    }
}
```

This implementation:
- Covers the most common types used in arguments
- Provides clear error messages for conversion failures
- Handles edge cases like empty values
- Is extensible for additional types

#### 5. Optional Value Handling

The library includes special handling for `Option<T>` types:

```rust
impl<T: FromArgValue> FromArgValue for Option<T> {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        // Use the default converter configuration
        let config = ConverterConfig::default();
        
        // Check for empty strings if configured to handle them
        if config.handle_empty && value.is_empty() {
            return Ok(None);
        }
        
        // Check for "none" and "null" values if configured to recognize them
        if config.recognize_none_values {
            let config = ConversionConfig::default();
            if config.none_values.contains(&value.to_lowercase().as_str()) {
                return Ok(None);
            }
        }
        
        // If the value is not None, convert it to the target type
        match T::from_arg_value(value) {
            Ok(converted) => Ok(Some(converted)),
            Err(e) => Err(e),
        }
    }
}
```

This approach:
- Handles optional values naturally
- Recognizes common "none" representations
- Preserves the underlying type's conversion behavior
- Is configurable for different use cases

#### 6. Conversion Configuration

The library provides configuration structures for customizing conversion behavior:

```rust
#[derive(Debug, Clone)]
pub struct ConverterConfig {
    /// Whether to trim whitespace before conversion
    pub trim_whitespace: bool,
    
    /// Whether to handle empty strings specially
    pub handle_empty: bool,
    
    /// Whether to treat "none" and "null" as None for Option types
    pub recognize_none_values: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct ConversionConfig {
    /// Whether to enable case-insensitive boolean parsing
    pub case_insensitive_booleans: bool,
    
    /// List of true values for boolean parsing
    pub true_values: Vec<&'static str>,
    
    /// List of false values for boolean parsing
    pub false_values: Vec<&'static str>,
    
    /// List of none values for option parsing
    pub none_values: Vec<&'static str>,
}
```

This configuration system:
- Provides sensible defaults
- Enables customization where needed
- Separates public and internal configuration
- Covers common conversion requirements

#### 7. Conversion Helper Functions

The library includes helper functions for common conversion tasks:

```rust
pub fn convert<T: FromArgValue>(
    value: &str,
    config: Option<&ConverterConfig>,
) -> Result<T> {
    // Use the provided config or the default
    let config = config.unwrap_or(&ConverterConfig::default());
    
    // Pre-process the value if needed
    let processed_value = if config.trim_whitespace {
        value.trim()
    } else {
        value
    };
    
    // Perform the conversion
    T::from_arg_value(processed_value)
}

pub fn from_str<T: FromStr + 'static>() -> fn(&str) -> std::result::Result<T, T::Err> {
    T::from_str
}
```

These functions:
- Simplify common conversion tasks
- Provide a consistent interface
- Handle configuration properly
- Support the most common use cases

## Integration

### Integration with Other Components

The Format & Type Conversion module integrates with other components as follows:

1. **Core Argument Types**: Defines formats and conversions used by Flag and KeyValue types
2. **Parser Module**: Provides format detection and validation for parsing arguments
3. **Key-Value Store**: Enables type-safe access to stored values
4. **Validation System**: Supports value constraints and validation
5. **Error System**: Uses the library's error types for reporting conversion failures

### Usage Examples

```rust
use pam_args_rs::{ArgumentParser, Flag, KeyValue, AllowedKeyValueFormats, Error};
use pam_args_rs::conversion::converter;
use std::str::FromStr;

fn main() -> Result<(), Error> {
    // Create an argument parser with various argument types
    let parser = ArgumentParser::new()
        .flag(Flag::new("DEBUG", "Enable debug mode"))
        .key_value(
            KeyValue::new("USER", "Username for authentication")
                .type_converter(String::from_str)
        )
        .key_value(
            KeyValue::new("COUNT", "Number of items")
                .type_converter(i32::from_str)
        )
        .key_value(
            KeyValue::new("ENABLED", "Whether the feature is enabled")
                .type_converter(bool::from_str)
        )
        .key_value(
            KeyValue::new("VERBOSE", "Verbosity level")
                // Different ways to specify VERBOSE, including flag-like
                .allowed_formats(&[
                    AllowedKeyValueFormats::KeyValue,
                    AllowedKeyValueFormats::KeyOnly,
                ])
        );
    
    // Parse arguments
    let result = parser.parse(std::env::args().skip(1))?;
    
    // Access typed values
    if result.is_present("DEBUG") {
        println!("Debug mode enabled");
    }
    
    if let Some(user) = result.value_of::<String>("USER") {
        println!("User: {}", user);
    }
    
    if let Some(count) = result.value_of::<i32>("COUNT") {
        println!("Count: {}", count);
    }
    
    if result.value_of::<bool>("ENABLED").unwrap_or(false) {
        println!("Feature enabled");
    }
    
    if result.is_present("VERBOSE") {
        if let Some(level) = result.value_of::<i32>("VERBOSE") {
            println!("Verbosity level: {}", level);
        } else {
            println!("Verbosity enabled (default level)");
        }
    }
    
    Ok(())
}

// Example of implementing FromArgValue for a custom type
struct LogLevel(u8);

impl FromArgValue for LogLevel {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        match value.to_lowercase().as_str() {
            "error" => Ok(LogLevel(0)),
            "warn" => Ok(LogLevel(1)),
            "info" => Ok(LogLevel(2)),
            "debug" => Ok(LogLevel(3)),
            "trace" => Ok(LogLevel(4)),
            _ => {
                // Try to parse as a number
                match value.parse::<u8>() {
                    Ok(level) if level <= 4 => Ok(LogLevel(level)),
                    _ => Err(Error::InvalidValue(
                        "LOG_LEVEL".to_string(),
                        value.to_string(),
                    )),
                }
            }
        }
    }
}
```

## Testing Strategy

### Unit Test Cases

| # | Category | Input | Expected Output | Notes |
|---|----------|-------|-----------------|-------|
| 1 | Format Detection | `"USER=admin"` | `KeyValue, "USER", Some("admin")` | Test KEY=VALUE format |
| 2 | Format Detection | `"DEBUG"` | `KeyOnly, "DEBUG", None` | Test KEY format |
| 3 | Format Detection | `"EMPTY="` | `KeyEquals, "EMPTY", Some("")` | Test KEY= format |
| 4 | Format Validation | `KeyValue` with `[KeyValue]` allowed | Success | Test compatible format |
| 5 | Format Validation | `KeyOnly` with `[KeyValue]` allowed | Failure | Test incompatible format |
| 6 | Format Validation | Any format with `[KeyAll]` allowed | Success | Test KeyAll compatibility |
| 7 | Format Compatibility | `KeyAll.is_compatible_with(KeyValue)` | `true` | Test KeyAll compatibility |
| 8 | Format Compatibility | `KeyValue.is_compatible_with(KeyOnly)` | `false` | Test format incompatibility |
| 9 | Format All | `AllowedKeyValueFormats::all()` | Contains KeyValue, KeyOnly, KeyEquals | Test format enumeration |
| 10 | String Conversion | `String::from_arg_value("hello")` | `"hello"` | Test string conversion |
| 11 | String Conversion | `String::from_arg_value("")` | `""` | Test empty string |
| 12 | String Conversion | `String::from_arg_value("  spaced  ")` | `"  spaced  "` | Test preserving whitespace |
| 13 | String Conversion | `String::from_arg_value("special\nchars")` | `"special\nchars"` | Test special characters |
| 14 | Integer Conversion | `i32::from_arg_value("123")` | `123` | Test integer conversion |
| 15 | Integer Conversion | `i32::from_arg_value("-123")` | `-123` | Test negative integer |
| 16 | Integer Conversion | `i32::from_arg_value("0")` | `0` | Test zero |
| 17 | Integer Conversion | `i32::from_arg_value("2147483647")` | `2147483647` | Test i32 max value |
| 18 | Integer Conversion | `i32::from_arg_value("-2147483648")` | `-2147483648` | Test i32 min value |
| 19 | Integer Conversion | `i32::from_arg_value("abc")` | Error | Test invalid integer |
| 20 | Integer Conversion | `i32::from_arg_value("2147483648")` | Error | Test overflow |
| 21 | Integer Conversion | `i32::from_arg_value("123.45")` | Error | Test non-integer number |
| 22 | Boolean Conversion | `bool::from_arg_value("true")` | `true` | Test boolean true value |
| 23 | Boolean Conversion | `bool::from_arg_value("yes")` | `true` | Test alternative true value |
| 24 | Boolean Conversion | `bool::from_arg_value("1")` | `true` | Test numeric true value |
| 25 | Boolean Conversion | `bool::from_arg_value("on")` | `true` | Test 'on' as true |
| 26 | Boolean Conversion | `bool::from_arg_value("false")` | `false` | Test boolean false value |
| 27 | Boolean Conversion | `bool::from_arg_value("no")` | `false` | Test alternative false value |
| 28 | Boolean Conversion | `bool::from_arg_value("0")` | `false` | Test numeric false value |
| 29 | Boolean Conversion | `bool::from_arg_value("off")` | `false` | Test 'off' as false |
| 30 | Boolean Conversion | `bool::from_arg_value("TRUE")` | `true` | Test case insensitivity |
| 31 | Boolean Conversion | `bool::from_arg_value("YES")` | `true` | Test case insensitivity |
| 32 | Boolean Conversion | `bool::from_arg_value("invalid")` | Error | Test invalid boolean |
| 33 | Boolean Conversion | `bool::from_arg_value("")` | Error | Test empty value |
| 34 | Character Conversion | `char::from_arg_value("a")` | `'a'` | Test character conversion |
| 35 | Character Conversion | `char::from_arg_value("A")` | `'A'` | Test uppercase character |
| 36 | Character Conversion | `char::from_arg_value("1")` | `'1'` | Test numeric character |
| 37 | Character Conversion | `char::from_arg_value(" ")` | `' '` | Test space character |
| 38 | Character Conversion | `char::from_arg_value("π")` | `'π'` | Test Unicode character |
| 39 | Character Conversion | `char::from_arg_value("😀")` | `'😀'` | Test Unicode emoji |
| 40 | Character Conversion | `char::from_arg_value("")` | Error | Test empty string |
| 41 | Character Conversion | `char::from_arg_value("ab")` | Error | Test multiple characters |
| 42 | Option Conversion | `Option::<String>::from_arg_value("hello")` | `Some("hello")` | Test Option with value |
| 43 | Option Conversion | `Option::<String>::from_arg_value("")` | `None` | Test Option with empty string |
| 44 | Option Conversion | `Option::<String>::from_arg_value("none")` | `None` | Test Option with "none" value |
| 45 | Option Conversion | `Option::<String>::from_arg_value("null")` | `None` | Test Option with "null" value |
| 46 | Option Conversion | `Option::<i32>::from_arg_value("123")` | `Some(123)` | Test Option with integer |
| 47 | Option Conversion | `Option::<i32>::from_arg_value("")` | `None` | Test Option with empty string |
| 48 | Option Conversion | `Option::<i32>::from_arg_value("none")` | `None` | Test Option with "none" value |
| 49 | Option Conversion | `Option::<i32>::from_arg_value("invalid")` | Error | Test Option with invalid value |
| 50 | Option Conversion | `Option::<bool>::from_arg_value("true")` | `Some(true)` | Test Option with boolean |
| 51 | Option Conversion | `Option::<bool>::from_arg_value("")` | `None` | Test Option boolean with empty string |
| 52 | Option Conversion | `Option::<char>::from_arg_value("a")` | `Some('a')` | Test Option with character |
| 53 | Option Conversion | `Option::<char>::from_arg_value("")` | `None` | Test Option character with empty string |
| 54 | Conversion Helper | `convert::<i32>("123", None)` | `123` | Test convert helper function |
| 55 | Conversion Helper | `convert::<i32>("  123  ", None)` | `123` | Test whitespace trimming |
| 56 | Conversion Helper | `convert::<i32>("  123  ", Some(&config_no_trim))` | Error | Test without trimming |
| 57 | Conversion Helper | `convert::<String>("hello", None)` | `"hello"` | Test string conversion |
| 58 | Conversion Helper | `convert::<String>("  hello  ", None)` | `"hello"` | Test trimming |
| 59 | Conversion Helper | `convert::<bool>("TRUE", None)` | `true` | Test case insensitivity |
| 60 | Conversion Helper | `convert::<Option<i32>>("none", None)` | `None` | Test none value |
| 61 | FromStr Helper | `KeyValue with from_str::<i32>()` | Properly configured KeyValue | Test from_str helper |
| 62 | FromStr Helper | `KeyValue with from_str::<String>()` | Properly configured KeyValue | Test from_str helper with String |
| 63 | FromStr Helper | `KeyValue with from_str::<bool>()` | Properly configured KeyValue | Test from_str helper with bool |
| 64 | FromStr Helper | `KeyValue with from_str::<char>()` | Properly configured KeyValue | Test from_str helper with char |
| 65 | Custom Type | `LogLevel::from_arg_value("debug")` | `LogLevel(3)` | Test custom type conversion |
| 66 | Custom Type | `LogLevel::from_arg_value("DEBUG")` | `LogLevel(3)` | Test case insensitivity |
| 67 | Custom Type | `LogLevel::from_arg_value("2")` | `LogLevel(2)` | Test custom type numeric conversion |
| 68 | Custom Type | `LogLevel::from_arg_value("invalid")` | Error | Test invalid custom type value |
| 69 | Custom Type | `LogLevel::from_arg_value("5")` | Error | Test out of range value |
| 70 | Config Impact | `bool::from_arg_value("TRUE")` with case sensitivity | Error | Test case sensitivity config |
| 71 | Config Impact | `bool::from_arg_value("TRUE")` without case sensitivity | `true` | Test case insensitivity config |
| 72 | Config Impact | `convert::<String>("  hello  ", &config_trim)` | `"hello"` | Test trim config |
| 73 | Config Impact | `convert::<String>("  hello  ", &config_no_trim)` | `"  hello  "` | Test no trim config |
| 74 | Config Impact | `Option::<String>::from_arg_value("")` with handle_empty true | `None` | Test empty handling config |
| 75 | Config Impact | `Option::<String>::from_arg_value("")` with handle_empty false | Error | Test empty handling config |
| 76 | Format Combinations | Test format detection with quoted values | Correct detection | Test complex format patterns |
| 77 | Format Combinations | Test format detection with escaped characters | Correct detection | Test complex format patterns |
| 78 | Format Combinations | Test format detection with mixed formats | Correct detection | Test format interactions |
| 79 | Error Messages | Check error message for invalid integer | Contains original value | Test error information quality |
| 80 | Error Messages | Check error message for invalid boolean | Contains original value | Test error information quality |
| 81 | Error Messages | Check error message for invalid character | Contains original value | Test error information quality |
| 82 | Error Messages | Check error message for incompatible format | Contains format information | Test error information quality |
| 83 | Conversion Any | `convert_to_any` with i32 | Boxed i32 | Test type erasure |
| 84 | Conversion Any | `convert_to_any` with String | Boxed String | Test type erasure |
| 85 | Conversion Any | `convert_to_any` with bool | Boxed bool | Test type erasure |
| 86 | Conversion Any | `convert_to_any` with char | Boxed char | Test type erasure |
| 87 | Conversion Any | `convert_to_any` with invalid input | Error | Test error in type erasure |
| 88 | Thread Safety | Test conversions from multiple threads | Consistent results | Test thread safety |
| 89 | Thread Safety | Test format detection from multiple threads | Consistent results | Test thread safety |
| 90 | Performance | Test conversion of large string | Fast completion | Test performance |
| 91 | Performance | Test conversion of many values in sequence | Minimal overhead | Test performance scaling |
| 92 | Performance | Test format detection with long input | Fast completion | Test detection performance |
| 93 | Localization | Test boolean with non-English values | Proper conversion | Test international support |
| 94 | Localization | Test with non-ASCII characters | Proper handling | Test Unicode support |
| 95 | Boundary Cases | Test with extreme values | Proper handling | Test boundary conditions |
| 96 | Memory | Test with values that would cause allocation | Minimal allocations | Test memory efficiency |
| 97 | Integration | Test conversion in KeyValue constructor | Works correctly | Test integration |
| 98 | Integration | Test format validation in parser | Works correctly | Test integration |
| 99 | Integration | Test type-safe access in ParseResult | Works correctly | Test integration |
| 100 | Extensibility | Create new FromArgValue implementation | Works correctly | Test extensibility |

### Integration Tests

The Format & Type Conversion module should be tested in integration with other components to ensure correct end-to-end behavior:

1. **Parser Integration**
   - Test format detection and validation with the parser
   - Verify correct handling of different format constraints
   - Test error propagation for format violations
   - Verify format compatibility checks

2. **Key-Value Type Conversion**
   - Test conversion of various argument types
   - Verify correct handling of type constraints
   - Test error handling for conversion failures
   - Verify complex type conversion scenarios

3. **End-to-End Argument Processing**
   - Test format detection, validation, and conversion in sequence
   - Verify correct handling of mixed format arguments
   - Test integration with key-value storage
   - Verify proper type handling throughout the pipeline

### Testing Focus Areas

1. **Format Detection Accuracy**
   - Verify accurate detection of all formats
   - Test edge cases like empty values
   - Test with quoted values and special characters
   - Verify handling of unusual input patterns

2. **Type Conversion Robustness**
   - Test conversion of all supported types
   - Verify handling of boundary values
   - Test error cases and error messages
   - Verify consistent behavior across types

3. **Configuration Flexibility**
   - Test with different configuration settings
   - Verify behavior changes with configuration
   - Test default configurations
   - Verify configuration overrides work correctly

4. **Custom Type Support**
   - Test implementation of FromArgValue for custom types
   - Verify integration with the conversion system
   - Test error handling for custom conversions
   - Verify usability of the extension points

5. **Performance Characteristics**
   - Test conversion performance with various inputs
   - Verify minimal allocations during conversion
   - Test with large values and repeated conversions
   - Verify efficient handling of complex conversions

## Performance Considerations

### Memory Efficiency
- Minimize heap allocations during conversions
- Avoid unnecessary string copies
- Reuse format detection results where possible
- Use references instead of owned values where appropriate
- Keep conversion overhead minimal for common types

### Format Detection Optimization
- Single-pass detection algorithm with O(n) complexity
- Early return for common format patterns
- No regex overhead for simple formats
- Avoid unnecessary allocations for key and value extraction
- Cache detected formats for repeated validations

### Type Conversion Efficiency
- Use static dispatch for type conversion
- Leverage Rust's zero-cost abstractions for FromStr conversions
- Use specialized implementations for common types
- Avoid dynamic dispatch where possible
- Minimize branching in conversion hot paths

### Configuration System
- Use sensible defaults to minimize configuration overhead
- Lazy initialization of configuration where appropriate
- Cache configuration values for repeated use
- Allow for zero-cost configuration overrides
- Optimize boolean conversions which are performance-critical

### Integration Optimizations
- Ensure format validation is fast for common cases
- Optimize for the most frequently used types
- Provide specialized conversion paths for common scenarios
- Minimize indirection in the conversion pipeline
- Design for predictable performance characteristics