//! Format and type conversion for the pam-args library.
//!
//! This module defines the available key-value formats and provides a robust, type-safe
//! conversion system that transforms string values from command-line arguments into
//! native Rust types. It leverages Rust's trait system to provide clean error handling,
//! sensible defaults, and extension points for advanced use cases.

use std::any::Any;
use std::str::FromStr;
use std::fmt;
use crate::args::AllowedKeyValueFormats;
use crate::error::{Error, Result};

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
    /// * `config` - Optional configuration for the conversion
    ///
    /// # Returns
    ///
    /// The converted value or an error
    fn from_arg_value(value: &str, config: Option<&ConverterConfig>) -> Result<Self>;
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

/// Prints the contents of the ConverterConfig
impl fmt::Display for ConverterConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ConverterConfig:\n  trim_whitespace: {}\n  handle_empty: {}\n  recognize_none_values: {}",
            self.trim_whitespace, self.handle_empty, self.recognize_none_values
        )
    }
}

/// Prints the contents of the ConversionConfig
impl fmt::Display for ConversionConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ConversionConfig:\n  case_insensitive_booleans: {}\n  true_values: {:?}\n  false_values: {:?}\n  none_values: {:?}",
            self.case_insensitive_booleans, self.true_values, self.false_values, self.none_values
        )
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
    /// ```ignore
    /// use crate::conversion::format;
    /// use crate::AllowedKeyValueFormats;
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

/// Implementation of FromArgValue for common types
impl FromArgValue for String {
    fn from_arg_value(value: &str, _config: Option<&ConverterConfig>) -> Result<Self> {
        Ok(value.to_string())
    }
}

impl FromArgValue for i32 {
    fn from_arg_value(value: &str, _config: Option<&ConverterConfig>) -> Result<Self> {
        value.parse::<i32>().map_err(|_| {
            Error::InvalidIntValue(value.to_string())
        })
    }
}

impl FromArgValue for bool {
    fn from_arg_value(value: &str, _config: Option<&ConverterConfig>) -> Result<Self> {
        // Use the default conversion configuration
        let conversion_config = ConversionConfig::default();
        
        // If case-insensitive, convert to lowercase for comparison
        let compare_value = if conversion_config.case_insensitive_booleans {
            value.to_lowercase()
        } else {
            value.to_string()
        };
        
        // Check if the value matches any true value
        if conversion_config.true_values.iter().any(|v| *v == compare_value) {
            return Ok(true);
        }
        
        // Check if the value matches any false value
        if conversion_config.false_values.iter().any(|v| *v == compare_value) {
            return Ok(false);
        }
        
        // If no match, return an error
        Err(Error::InvalidBoolValue(value.to_string()))
    }
}

impl FromArgValue for char {
    fn from_arg_value(value: &str, _config: Option<&ConverterConfig>) -> Result<Self> {
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
    fn from_arg_value(value: &str, config: Option<&ConverterConfig>) -> Result<Self> {
        // Unwrap the configuration or use the default
        let default_config = ConverterConfig::default();
        let config = config.unwrap_or(&default_config);

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
        match T::from_arg_value(value, Some(config)) {
            Ok(converted) => {
                Ok(Some(converted))
            },
            Err(e) => Err(e),
        }
    }
}

/// Main type conversion functions
pub mod converter {
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
    /// ```ignore
    /// use crate::converter;
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
        let default_config = ConverterConfig::default();
        let config = config.unwrap_or(&default_config);
        
        // Print the configuration details
        println!("#DEBUG: default_config: {}", default_config.to_string());
        println!("#DEBUG: config: {}", config.to_string());

        // Pre-process the value if needed
        let processed_value = if config.trim_whitespace {
            value.trim()
        } else {
            value
        };
        
        // Perform the conversion
        T::from_arg_value(processed_value, Some(config))
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
    /// ```ignore
    /// use crate::converter;
    /// use crate::KeyValue;
    ///
    /// let kv = KeyValue::new("WIDTH", "Width in pixels")
    ///     .type_converter(converter::from_str::<i32>());
    /// ```
    pub fn from_str<T: FromStr + 'static>() -> fn(&str) -> std::result::Result<T, T::Err> {
        T::from_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::AllowedKeyValueFormats;
    
    #[test]
    fn test_format_detection() {
        // Test KEY=VALUE format
        let result = format::detect("USER=admin");
        assert_eq!(result.format, AllowedKeyValueFormats::KeyValue);
        assert_eq!(result.key, "USER");
        assert_eq!(result.value, Some("admin".to_string()));
        
        // Test KEY format
        let result = format::detect("DEBUG");
        assert_eq!(result.format, AllowedKeyValueFormats::KeyOnly);
        assert_eq!(result.key, "DEBUG");
        assert_eq!(result.value, None);
        
        // Test KEY= format
        let result = format::detect("EMPTY=");
        assert_eq!(result.format, AllowedKeyValueFormats::KeyEquals);
        assert_eq!(result.key, "EMPTY");
        assert_eq!(result.value, Some("".to_string()));
    }
    
    #[test]
    fn test_format_validation() {
        // Test valid format
        let result = format::detect("USER=admin");
        assert!(format::validate(&result, &[AllowedKeyValueFormats::KeyValue]).is_ok());
        
        // Test invalid format
        let result = format::detect("DEBUG");
        assert!(format::validate(&result, &[AllowedKeyValueFormats::KeyValue]).is_err());
        
        // Test KeyAll format
        let result = format::detect("DEBUG");
        assert!(format::validate(&result, &[AllowedKeyValueFormats::KeyAll]).is_ok());
        
        // Test with any format
        let result = format::detect("USER=admin");
        assert!(format::validate(&result, &[AllowedKeyValueFormats::KeyAll]).is_ok());
    }
    
    #[test]
    fn test_string_conversion() {
        assert_eq!(String::from_arg_value("hello", None).unwrap(), "hello");
        assert_eq!(String::from_arg_value("", None).unwrap(), "");
        assert_eq!(String::from_arg_value("  spaced  ", None).unwrap(), "  spaced  ");
    }
    
    #[test]
    fn test_integer_conversion() {
        assert_eq!(i32::from_arg_value("123", None).unwrap(), 123);
        assert_eq!(i32::from_arg_value("-123", None).unwrap(), -123);
        assert_eq!(i32::from_arg_value("0", None).unwrap(), 0);
        assert!(i32::from_arg_value("abc", None).is_err());
        assert!(i32::from_arg_value("123.45", None).is_err());
    }
    
    #[test]
    fn test_boolean_conversion() {
        assert_eq!(bool::from_arg_value("true", None).unwrap(), true);
        assert_eq!(bool::from_arg_value("yes", None).unwrap(), true);
        assert_eq!(bool::from_arg_value("1", None).unwrap(), true);
        assert_eq!(bool::from_arg_value("on", None).unwrap(), true);
        
        assert_eq!(bool::from_arg_value("false", None).unwrap(), false);
        assert_eq!(bool::from_arg_value("no", None).unwrap(), false);
        assert_eq!(bool::from_arg_value("0", None).unwrap(), false);
        assert_eq!(bool::from_arg_value("off", None).unwrap(), false);
        
        assert_eq!(bool::from_arg_value("TRUE", None).unwrap(), true);
        assert_eq!(bool::from_arg_value("YES", None).unwrap(), true);
        
        assert!(bool::from_arg_value("invalid", None).is_err());
        assert!(bool::from_arg_value("", None).is_err());
    }
    
    #[test]
    fn test_character_conversion() {
        assert_eq!(char::from_arg_value("a", None).unwrap(), 'a');
        assert_eq!(char::from_arg_value("A", None).unwrap(), 'A');
        assert_eq!(char::from_arg_value("1", None).unwrap(), '1');
        assert_eq!(char::from_arg_value(" ", None).unwrap(), ' ');
        
        assert!(char::from_arg_value("", None).is_err());
        assert!(char::from_arg_value("ab", None).is_err());
    }
    
    #[test]
    fn test_option_conversion() {
        assert_eq!(Option::<String>::from_arg_value("hello", None).unwrap(), Some("hello".to_string()));
        assert_eq!(Option::<String>::from_arg_value("", None).unwrap(), None);
        assert_eq!(Option::<String>::from_arg_value("none", None).unwrap(), None);
        assert_eq!(Option::<String>::from_arg_value("null", None).unwrap(), None);
        
        assert_eq!(Option::<i32>::from_arg_value("123", None).unwrap(), Some(123));
        assert_eq!(Option::<i32>::from_arg_value("", None).unwrap(), None);
        assert_eq!(Option::<i32>::from_arg_value("none", None).unwrap(), None);
        assert!(Option::<i32>::from_arg_value("invalid", None).is_err());
    }
    
    #[test]
    fn test_convert_helper() {
        let config = ConverterConfig::default();
        
        assert_eq!(converter::convert::<i32>("123", None).unwrap(), 123);
        assert_eq!(converter::convert::<i32>("  123  ", None).unwrap(), 123);
        
        let no_trim_config = ConverterConfig {
            trim_whitespace: false,
            ..config.clone()
        };
        assert!(converter::convert::<i32>("  123  ", Some(&no_trim_config)).is_err());
        
        assert_eq!(converter::convert::<String>("hello", None).unwrap(), "hello");
        assert_eq!(converter::convert::<String>("  hello  ", None).unwrap(), "hello");
        
        assert_eq!(converter::convert::<bool>("TRUE", None).unwrap(), true);
        assert_eq!(converter::convert::<Option<i32>>("none", None).unwrap(), None);
    }
    
    #[test]
    fn test_from_str_helper() {
        let from_str_i32 = converter::from_str::<i32>();
        assert_eq!(from_str_i32("123").unwrap(), 123);
        assert!(from_str_i32("abc").is_err());
    }
}