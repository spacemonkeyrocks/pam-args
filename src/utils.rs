//! Utility functions for the pam-args library.
//!
//! This module provides internal utility functions used across the library.
//! These utilities provide common functionality for string manipulation,
//! case conversion, escape sequence handling, and other operations that
//! are needed by various components.

use crate::error::{Error, Result};
use log::{debug, trace};

/// Configuration for text processing utilities
#[derive(Debug, Clone)]
pub(crate) struct TextProcessingConfig {
    /// Whether to consider case when comparing strings
    pub case_sensitive: bool,
    
    /// Character used for escaping special characters
    pub escape_char: char,
    
    /// Single quote character
    pub single_quote: char,
    
    /// Double quote character
    pub double_quote: char,
}

impl Default for TextProcessingConfig {
    fn default() -> Self {
        Self {
            case_sensitive: true,
            escape_char: '\\',
            single_quote: '\'',
            double_quote: '"',
        }
    }
}

/// Functions for string case handling
pub(crate) mod case {
    /// Converts a string to lowercase if case-insensitive mode is enabled
    ///
    /// # Arguments
    ///
    /// * `s` - The string to convert
    /// * `case_sensitive` - Whether to preserve the original case
    ///
    /// # Returns
    ///
    /// The original string if case_sensitive is true, or a lowercase version otherwise
    pub(crate) fn normalize(s: &str, case_sensitive: bool) -> String {
        if case_sensitive {
            s.to_string()
        } else {
            s.to_lowercase()
        }
    }
    
    /// Compares two strings with optional case sensitivity
    ///
    /// # Arguments
    ///
    /// * `a` - First string to compare
    /// * `b` - Second string to compare
    /// * `case_sensitive` - Whether to consider case when comparing
    ///
    /// # Returns
    ///
    /// true if the strings match according to the case sensitivity setting
    pub(crate) fn compare(a: &str, b: &str, case_sensitive: bool) -> bool {
        if case_sensitive {
            a == b
        } else {
            a.to_lowercase() == b.to_lowercase()
        }
    }
}

/// Functions for handling escape sequences
pub(crate) mod escaping {
    use super::*;
    
    /// Unescapes a string containing backslash escape sequences
    ///
    /// # Arguments
    ///
    /// * `s` - The string containing escape sequences
    /// * `config` - Configuration for text processing
    ///
    /// # Returns
    ///
    /// Result containing the unescaped string or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the string contains an invalid escape sequence
    pub(crate) fn unescape(s: &str, config: &TextProcessingConfig) -> Result<String> {
        let mut result = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();
        let mut in_escape = false;
        
        while let Some(c) = chars.next() {
            if in_escape {
                match c {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '\\' => result.push('\\'),
                    '\'' => result.push('\''),
                    '"' => result.push('"'),
                    ',' => result.push(','),
                    '[' => result.push('['),
                    ']' => result.push(']'),
                    _ => return Err(Error::InvalidInput(
                        format!("Invalid escape sequence \\{}", c)
                    )),
                }
                in_escape = false;
            } else if c == config.escape_char {
                in_escape = true;
            } else {
                result.push(c);
            }
        }
        
        // If we end with a backslash, it's an error
        if in_escape {
            return Err(Error::UnclosedDelimiter(
                "String ends with an escape character".to_string()
            ));
        }
        
        Ok(result)
    }
    
    /// Escapes special characters in a string
    ///
    /// # Arguments
    ///
    /// * `s` - The string to escape
    /// * `chars_to_escape` - Characters that should be escaped
    /// * `config` - Configuration for text processing
    ///
    /// # Returns
    ///
    /// A new string with special characters escaped
    pub(crate) fn escape(
        s: &str, 
        chars_to_escape: &[char], 
        config: &TextProcessingConfig
    ) -> String {
        let mut result = String::with_capacity(s.len() * 2);
        
        for c in s.chars() {
            if c == config.escape_char || chars_to_escape.contains(&c) {
                result.push(config.escape_char);
            }
            result.push(c);
        }
        
        result
    }
}

/// Functions for string manipulation
pub(crate) mod strings {
    use super::*;
    
    /// Trims whitespace from a string, respecting quoted content
    ///
    /// # Arguments
    ///
    /// * `s` - The string to trim
    /// * `config` - Configuration for text processing
    ///
    /// # Returns
    ///
    /// The trimmed string
    pub(crate) fn smart_trim(s: &str, config: &TextProcessingConfig) -> String {
        if s.len() < 2 {
            return s.trim().to_string();
        }
        
        let first_char = s.chars().next().unwrap();
        let last_char = s.chars().last().unwrap();
        
        // Check if the string is quoted
        let is_single_quoted = first_char == config.single_quote && last_char == config.single_quote;
        let is_double_quoted = first_char == config.double_quote && last_char == config.double_quote;
        
        if is_single_quoted || is_double_quoted {
            // For quoted strings, we keep the quotes and trim the content inside
            let quote = if is_single_quoted { config.single_quote } else { config.double_quote };
            let inner = &s[1..s.len() - 1];
            format!("{}{}{}", quote, inner.trim(), quote)
        } else {
            // For unquoted strings, just trim normally
            s.trim().to_string()
        }
    }
    
    /// Splits a string by a delimiter, respecting quotes and escape sequences
    ///
    /// # Arguments
    ///
    /// * `s` - The string to split
    /// * `delimiter` - The character to split on
    /// * `config` - Configuration for text processing
    ///
    /// # Returns
    ///
    /// Result containing a vector of split strings or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the string contains unclosed quotes
    pub(crate) fn smart_split(
        s: &str, 
        delimiter: char, 
        config: &TextProcessingConfig
    ) -> Result<Vec<String>> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut in_single_quote = false;
        let mut in_double_quote = false;
        let mut in_escape = false;
        
        for c in s.chars() {
            if in_escape {
                current.push(c);
                in_escape = false;
                continue;
            }
            
            if c == config.escape_char {
                current.push(c);
                in_escape = true;
                continue;
            }
            
            if c == config.single_quote && !in_double_quote {
                current.push(c);
                in_single_quote = !in_single_quote;
                continue;
            }
            
            if c == config.double_quote && !in_single_quote {
                current.push(c);
                in_double_quote = !in_double_quote;
                continue;
            }
            
            if c == delimiter && !in_single_quote && !in_double_quote {
                result.push(current);
                current = String::new();
            } else {
                current.push(c);
            }
        }
        
        // Add the last part
        result.push(current);
        
        // Check for unclosed quotes
        if in_single_quote {
            return Err(Error::UnclosedDelimiter(
                "Unclosed single quote".to_string()
            ));
        }
        
        if in_double_quote {
            return Err(Error::UnclosedDelimiter(
                "Unclosed double quote".to_string()
            ));
        }
        
        if in_escape {
            return Err(Error::UnclosedDelimiter(
                "String ends with an escape character".to_string()
            ));
        }
        
        Ok(result)
    }
    
    /// Checks if a string is a valid key name for arguments
    ///
    /// # Arguments
    ///
    /// * `key` - The key name to validate
    ///
    /// # Returns
    ///
    /// true if the key is valid, false otherwise
    pub(crate) fn is_valid_key_name(key: &str) -> bool {
        if key.is_empty() {
            return false;
        }
        
        let first_char = key.chars().next().unwrap();
        if !first_char.is_ascii_alphabetic() && first_char != '_' {
            return false;
        }
        
        key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    }
}

/// Functions for logging
pub(crate) mod logging {
    use log::{debug, trace};
    
    /// Logs a debug message about processing an argument
    ///
    /// # Arguments
    ///
    /// * `arg` - The argument being processed
    /// * `context` - Additional context information
    pub(crate) fn debug_processing_arg(arg: &str, context: &str) {
        debug!("Processing argument: '{}' ({})", arg, context);
    }
    
    /// Logs trace details about tokenization
    ///
    /// # Arguments
    ///
    /// * `input` - The input being tokenized
    /// * `tokens` - The resulting tokens
    pub(crate) fn trace_tokenization(input: &str, tokens: &[String]) {
        trace!("Tokenized '{}' into {:?}", input, tokens);
    }
    
    /// Logs detailed debug information during parsing
    ///
    /// # Arguments
    ///
    /// * `message` - The debug message
    /// * `data` - The data being processed
    pub(crate) fn debug_parsing(message: &str, data: impl std::fmt::Debug) {
        debug!("{}: {:?}", message, data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_case_normalize() {
        assert_eq!(case::normalize("DEBUG", true), "DEBUG");
        assert_eq!(case::normalize("DEBUG", false), "debug");
        assert_eq!(case::normalize("debug", true), "debug");
        assert_eq!(case::normalize("debug", false), "debug");
        assert_eq!(case::normalize("Debug", false), "debug");
    }
    
    #[test]
    fn test_case_compare() {
        assert!(case::compare("DEBUG", "DEBUG", true));
        assert!(!case::compare("DEBUG", "debug", true));
        assert!(case::compare("DEBUG", "debug", false));
        assert!(case::compare("Debug", "debug", false));
        assert!(case::compare("debug", "DEBUG", false));
    }
    
    #[test]
    fn test_escaping_unescape() {
        let config = TextProcessingConfig::default();
        
        // Basic escape sequences
        assert_eq!(escaping::unescape("Hello\\nWorld", &config).unwrap(), "Hello\nWorld");
        assert_eq!(escaping::unescape("Tab\\tTest", &config).unwrap(), "Tab\tTest");
        assert_eq!(escaping::unescape("Return\\rTest", &config).unwrap(), "Return\rTest");
        
        // Escaped special characters
        assert_eq!(escaping::unescape("Escaped\\\\backslash", &config).unwrap(), "Escaped\\backslash");
        assert_eq!(escaping::unescape("Quote\\\"test", &config).unwrap(), "Quote\"test");
        assert_eq!(escaping::unescape("Quote\\'test", &config).unwrap(), "Quote'test");
        assert_eq!(escaping::unescape("Comma\\,test", &config).unwrap(), "Comma,test");
        assert_eq!(escaping::unescape("Brackets\\[\\]test", &config).unwrap(), "Brackets[]test");
        
        // Error cases
        assert!(escaping::unescape("Invalid\\", &config).is_err());
        assert!(escaping::unescape("Invalid\\z", &config).is_err());
    }
    
    #[test]
    fn test_escaping_escape() {
        let config = TextProcessingConfig::default();
        
        // Basic escaping
        assert_eq!(escaping::escape("Hello,World", &[','], &config), "Hello\\,World");
        
        // Escaping quotes
        assert_eq!(escaping::escape("Quote\"Test", &['"'], &config), "Quote\\\"Test");
        
        // Multiple special chars
        assert_eq!(
            escaping::escape("Multiple[,]", &[',', '[', ']'], &config),
            "Multiple\\[\\,\\]"
        );
        
        // Escaping backslashes
        assert_eq!(
            escaping::escape("Backslash\\Test", &[], &config),
            "Backslash\\\\Test"
        );
    }
    
    #[test]
    fn test_strings_smart_trim() {
        let config = TextProcessingConfig::default();
        
        // Basic trimming
        assert_eq!(strings::smart_trim("  Hello  ", &config), "Hello");
        
        // Quoted content
        assert_eq!(strings::smart_trim("  \"Hello World\"  ", &config), "\"Hello World\"");
        assert_eq!(strings::smart_trim("  'Quoted'  ", &config), "'Quoted'");
        
        // Quoted content with internal spaces
        assert_eq!(strings::smart_trim("\"  Hello World  \"", &config), "\"Hello World\"");
        assert_eq!(strings::smart_trim("'  Quoted  '", &config), "'Quoted'");
        
        // Short strings
        assert_eq!(strings::smart_trim("a", &config), "a");
        assert_eq!(strings::smart_trim(" a ", &config), "a");
    }
    
    #[test]
    fn test_strings_smart_split() {
        let config = TextProcessingConfig::default();
        
        // Basic splitting
        assert_eq!(
            strings::smart_split("a,b,c", ',', &config).unwrap(),
            vec!["a", "b", "c"]
        );
        
        // Quoted content
        assert_eq!(
            strings::smart_split("a,\"b,c\",d", ',', &config).unwrap(),
            vec!["a", "\"b,c\"", "d"]
        );
        
        // Single-quoted content
        assert_eq!(
            strings::smart_split("a,'b,c',d", ',', &config).unwrap(),
            vec!["a", "'b,c'", "d"]
        );
        
        // Escaped delimiter
        assert_eq!(
            strings::smart_split("a,b\\,c,d", ',', &config).unwrap(),
            vec!["a", "b\\,c", "d"]
        );
        
        // Error cases
        assert!(strings::smart_split("\"Unclosed", ',', &config).is_err());
        assert!(strings::smart_split("'Unclosed", ',', &config).is_err());
        assert!(strings::smart_split("Escaped\\", ',', &config).is_err());
    }
    
    #[test]
    fn test_strings_is_valid_key_name() {
        // Valid key names
        assert!(strings::is_valid_key_name("DEBUG"));
        assert!(strings::is_valid_key_name("debug_mode"));
        assert!(strings::is_valid_key_name("_private"));
        assert!(strings::is_valid_key_name("a1b2c3"));
        
        // Invalid key names
        assert!(!strings::is_valid_key_name(""));
        assert!(!strings::is_valid_key_name("123invalid"));
        assert!(!strings::is_valid_key_name("invalid-name"));
        assert!(!strings::is_valid_key_name("invalid.name"));
        assert!(!strings::is_valid_key_name("invalid name"));
    }
}