# Feature 2: Utility Functions

## Module Type
**Internal**: This component provides internal utility functions used across the library. While not directly part of the public API, these utilities provide essential functionality for other components.

## Feature Information

**Feature Name**: Utility Functions

**Description**: Implements shared helper methods used across different modules in the library. These utilities provide common functionality for string manipulation, case conversion, escape sequence handling, and other operations that are needed by various components. By centralizing these functions, we ensure consistency and reduce code duplication.

**Priority**: High

**Dependencies**: 
- [Feature 1: Result & Error Types](error-result-types.md)

## Requirements

### Functional Requirements
1. Provide case conversion functions (for case-insensitive operations)
2. Implement escape sequence handling for backslash-escaped characters
3. Provide string manipulation utilities for argument parsing
4. Implement string trimming functions that respect quoted content
5. Offer utilities for validating and normalizing argument formats
6. Create common logging utilities that integrate with the `log` crate

### API Requirements
- Provide clean, reusable functions with clear signatures
- Ensure functions are well-optimized for common cases
- Support both owned and borrowed string types where appropriate
- Return clear errors using the library's error types
- Avoid unnecessary allocations and copies

### Performance Requirements
- Minimize allocations in hot code paths
- Optimize string operations for minimal overhead
- Cache results where appropriate to avoid redundant processing
- Allow for zero-copy operations where possible

## Design

### Data Structures
```rust
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
```

### Function Signatures
```rust
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
    use crate::error::{Error, Result};
    
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
    use crate::error::{Error, Result};
    
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
```

### Implementation Approach

#### 1. Case Handling

The case module provides functions for handling case sensitivity in string operations:

```rust
pub(crate) fn normalize(s: &str, case_sensitive: bool) -> String {
    if case_sensitive {
        s.to_string()
    } else {
        s.to_lowercase()
    }
}

pub(crate) fn compare(a: &str, b: &str, case_sensitive: bool) -> bool {
    if case_sensitive {
        a == b
    } else {
        a.to_lowercase() == b.to_lowercase()
    }
}
```

These functions:
- Support configurable case sensitivity
- Minimize allocations when case sensitivity is enabled
- Provide both normalization and comparison operations

#### 2. Escape Sequence Handling

The escaping module handles backslash-escaped special characters:

```rust
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
```

These functions:
- Support a configurable escape character
- Handle common escape sequences
- Provide proper error handling for invalid sequences
- Pre-allocate capacity to reduce reallocations

#### 3. String Manipulation

The strings module provides utilities for manipulating strings during parsing:

```rust
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
```

These functions:
- Handle quoted text correctly during operations
- Respect escape sequences
- Validate key names according to the library's requirements
- Provide proper error handling for invalid inputs

#### 4. Logging Utilities

The logging module provides consistent logging functions for different components:

```rust
pub(crate) fn debug_processing_arg(arg: &str, context: &str) {
    debug!("Processing argument: '{}' ({})", arg, context);
}

pub(crate) fn trace_tokenization(input: &str, tokens: &[String]) {
    trace!("Tokenized '{}' into {:?}", input, tokens);
}

pub(crate) fn debug_parsing(message: &str, data: impl std::fmt::Debug) {
    debug!("{}: {:?}", message, data);
}
```

These functions:
- Provide consistent logging patterns across the library
- Use appropriate log levels for different types of information
- Support debugging during development and troubleshooting

### Utility Module Organization

The utility functions are organized into logical submodules:

1. **case**: Functions for handling case sensitivity
2. **escaping**: Functions for processing escape sequences
3. **strings**: General string manipulation utilities
4. **logging**: Consistent logging functions

This organization:
- Makes it easy to find related functions
- Allows for namespace separation
- Provides clear boundaries between different utility types
- Enables importing only what's needed

## Integration

### Integration with Other Components

The utility functions integrate with other components as follows:

1. **Tokenizer**: Uses string utilities for processing delimited text
2. **Parser**: Uses case handling for case-insensitive argument matching
3. **Key-Value Store**: Uses case normalization for key lookup
4. **Logging**: Uses logging utilities for consistent debug output
5. **Error Handling**: Uses the library's error types for error reporting

### Usage Examples

```rust
use crate::utils::{case, escaping, strings, TextProcessingConfig};
use crate::error::Result;

// Case sensitivity handling
let normalized = case::normalize("DEBUG", false);
assert_eq!(normalized, "debug");

let matches = case::compare("Debug", "debug", false);
assert!(matches);

// Escape sequence handling
let config = TextProcessingConfig::default();
let unescaped = escaping::unescape("Hello\\nWorld", &config)?;
assert_eq!(unescaped, "Hello\nWorld");

let escaped = escaping::escape("Hello,World", &[','], &config);
assert_eq!(escaped, "Hello\\,World");

// String manipulation
let trimmed = strings::smart_trim("  \"Hello World\"  ", &config);
assert_eq!(trimmed, "\"Hello World\"");

let parts = strings::smart_split("a,b,\"c,d\",e", ',', &config)?;
assert_eq!(parts, vec!["a", "b", "\"c,d\"", "e"]);

let valid = strings::is_valid_key_name("DEBUG_MODE");
assert!(valid);
```

## Testing Strategy

### Unit Test Cases

| # | Category | Input | Expected Output | Notes |
|---|----------|-------|-----------------|-------|
| 1 | Case | `normalize("DEBUG", true)` | `"DEBUG"` | Case-sensitive mode |
| 2 | Case | `normalize("DEBUG", false)` | `"debug"` | Case-insensitive mode |
| 3 | Case | `compare("Debug", "debug", true)` | `false` | Case-sensitive comparison |
| 4 | Case | `compare("Debug", "debug", false)` | `true` | Case-insensitive comparison |
| 5 | Escaping | `unescape("Hello\\nWorld", &config)` | `"Hello\nWorld"` | Basic escape sequence |
| 6 | Escaping | `unescape("Quote\\\"test", &config)` | `"Quote\"test"` | Escaped quote |
| 7 | Escaping | `unescape("Escaped\\\\backslash", &config)` | `"Escaped\\backslash"` | Escaped backslash |
| 8 | Escaping | `unescape("Invalid\\", &config)` | Error | Trailing escape character |
| 9 | Escaping | `unescape("Invalid\\z", &config)` | Error | Invalid escape sequence |
| 10 | Escaping | `escape("Hello,World", &[','], &config)` | `"Hello\\,World"` | Basic escaping |
| 11 | Escaping | `escape("Quote\"Test", &['"'], &config)` | `"Quote\\\"Test"` | Escaping quotes |
| 12 | Escaping | `escape("Multiple[,]", &[',', '[', ']'], &config)` | `"Multiple\\[\\,\\]"` | Multiple special chars |
| 13 | Strings | `smart_trim("  Hello  ", &config)` | `"Hello"` | Basic trimming |
| 14 | Strings | `smart_trim("  \"Hello World\"  ", &config)` | `"\"Hello World\""` | Quoted content |
| 15 | Strings | `smart_trim("  'Quoted'  ", &config)` | `"'Quoted'"` | Single-quoted content |
| 16 | Strings | `smart_split("a,b,c", ',', &config)` | `["a", "b", "c"]` | Basic splitting |
| 17 | Strings | `smart_split("a,\"b,c\",d", ',', &config)` | `["a", "\"b,c\"", "d"]` | Quoted content |
| 18 | Strings | `smart_split("a,'b,c',d", ',', &config)` | `["a", "'b,c'", "d"]` | Single-quoted content |
| 19 | Strings | `smart_split("a,b\\,c,d", ',', &config)` | `["a", "b\\,c", "d"]` | Escaped delimiter |
| 20 | Strings | `smart_split("\"Unclosed", ',', &config)` | Error | Unclosed quote |
| 21 | Strings | `is_valid_key_name("DEBUG")` | `true` | Valid key name |
| 22 | Strings | `is_valid_key_name("debug_mode")` | `true` | Underscore allowed |
| 23 | Strings | `is_valid_key_name("123invalid")` | `false` | Cannot start with digit |
| 24 | Strings | `is_valid_key_name("")` | `false` | Empty string |
| 25 | Strings | `is_valid_key_name("invalid-name")` | `false` | Invalid character |

### Integration Tests

The utility functions should be tested in integration with other components to ensure correct end-to-end behavior. Integration tests should focus on:

1. **Tokenizer Integration**
   - Test that case handling works correctly with tokenizer
   - Verify that escape sequence handling integrates properly
   - Test string utilities with complex tokenization scenarios

2. **Parser Integration**
   - Test case-insensitive argument matching
   - Verify proper handling of quoted arguments
   - Test validation of key names during parsing

3. **Key-Value Store Integration**
   - Test case-insensitive lookup with key-value store
   - Verify proper normalization during key-value operations

### Testing Focus Areas

1. **Edge Cases**
   - Empty strings
   - Strings with only whitespace
   - Mixed quotes and escape sequences
   - Special characters in various positions
   - Very long strings

2. **Error Handling**
   - Invalid escape sequences
   - Unclosed quotes
   - Trailing escape characters
   - Malformed key names

3. **Performance Testing**
   - Test with large inputs to verify performance
   - Test allocation patterns with various input sizes
   - Verify caching behavior where applicable

## Performance Considerations

### Memory Efficiency
- Functions preallocate strings with appropriate capacity when possible
- String operations avoid unnecessary copies
- Borrowed string slices are used where ownership is not required

### Time Complexity
- Case handling functions are O(n) in string length
- Escape sequence functions are O(n) in string length
- String manipulation functions are O(n) in string length
- Key name validation is O(n) in string length

### Optimizations
- Case comparison short-circuits on case-sensitive mode
- String capacity is preallocated to minimize reallocations
- Character-by-character processing avoids regex overhead
- Early returns are used for invalid inputs
