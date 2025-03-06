# Feature 9: Input Tokenizer and Syntax Validator

## Module Type
**Internal**: This component is an internal implementation detail and not part of the public API. Library users will not interact with this module directly.

## Feature Information

**Feature Name**: Input Tokenizer and Syntax Validator

**Description**: Handles initial tokenization of input strings, including validation of syntax, quoted text, bracketed text, and escaped characters. This component is responsible for transforming pre-tokenized command-line arguments into structured tokens that can be processed by subsequent parser stages.

**Priority**: High

**Dependencies**: 
- [Feature 1: Result & Error Types](error-result-types.md)
- [Feature 2: Utility Functions](utility-functions.md)

## Requirements

### Functional Requirements
1. Process pre-tokenized command-line arguments (which PAM modules receive as `args: Vec<String>`)
2. Detect and handle bracketed arguments containing comma-separated values
3. Process quoted text with both single (`'`) and double (`"`) quotes
4. Handle escape sequences with backslash (`\`)
5. Support nested delimiters (quotes within brackets)
6. Validate syntax and detect unclosed delimiters
7. Preserve escape sequences and delimiters for subsequent parser stages
8. Maintain the original order of arguments

### API Requirements
- Provide a clean, efficient tokenization interface
- Support configuration options (e.g., escape character, quote characters)
- Return descriptive errors for syntax issues
- Integrate seamlessly with other parser components

### Performance Requirements
- Tokenize input with minimal allocations
- Process large inputs efficiently
- Handle complex nested structures without excessive recursion

## Design

### Data Structures
```rust
/// Represents the current state of the tokenizer during parsing
#[derive(Debug, Clone, Copy, PartialEq)]
enum TokenizerState {
    /// Normal parsing state, outside of any delimiter
    Normal,
    
    /// Inside single quotes
    InSingleQuote,
    
    /// Inside double quotes
    InDoubleQuote,
    
    /// Inside square brackets
    InBracket,
    
    /// After a backslash, next character is escaped
    EscapeSequence,
}

/// Configuration options for the tokenizer
#[derive(Debug, Clone)]
pub struct TokenizerConfig {
    /// Character used for escaping special characters
    pub escape_char: char,
    
    /// Single quote character
    pub single_quote: char,
    
    /// Double quote character
    pub double_quote: char,
    
    /// Opening bracket character
    pub open_bracket: char,
    
    /// Closing bracket character
    pub close_bracket: char,
    
    /// Delimiter for comma-separated values within brackets
    pub delimiter: char,
}

/// Result of tokenization
#[derive(Debug, Clone)]
pub struct TokenizationResult {
    /// Tokens extracted from the input
    pub tokens: Vec<String>,
    
    /// Whether the input contained bracket-delimited content
    pub has_bracketed_content: bool,
}

/// Represents a token with information about its original delimiters
#[derive(Debug, Clone)]
struct Token {
    /// The actual content of the token
    content: String,
    
    /// The original input string that produced this token
    original: String,
    
    /// Whether this token was enclosed in brackets
    was_bracketed: bool,
    
    /// Whether this token was enclosed in quotes
    was_quoted: bool,
}
```

### Function Signatures
```rust
/// Main tokenizer struct that handles input processing
pub struct Tokenizer {
    config: TokenizerConfig,
}

impl Tokenizer {
    /// Creates a new tokenizer with default configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::Tokenizer;
    ///
    /// let tokenizer = Tokenizer::new();
    /// ```
    pub fn new() -> Self {
        Self {
            config: TokenizerConfig::default(),
        }
    }
    
    /// Creates a new tokenizer with custom configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration options for the tokenizer
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::{Tokenizer, TokenizerConfig};
    ///
    /// let config = TokenizerConfig {
    ///     escape_char: '\\',
    ///     single_quote: '\'',
    ///     double_quote: '"',
    ///     open_bracket: '[',
    ///     close_bracket: ']',
    ///     delimiter: ',',
    /// };
    ///
    /// let tokenizer = Tokenizer::with_config(config);
    /// ```
    pub fn with_config(config: TokenizerConfig) -> Self {
        Self { config }
    }
    
    /// Tokenizes a single pre-tokenized argument
    ///
    /// This method processes a single argument and handles special formats
    /// like bracketed content with comma-separated values.
    ///
    /// # Arguments
    ///
    /// * `arg` - Input argument to tokenize
    ///
    /// # Returns
    ///
    /// Result containing the tokenization result or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * A delimiter is unclosed (e.g., a quote or bracket without a matching close)
    /// * Nested brackets are encountered (which are not supported)
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::Tokenizer;
    ///
    /// let tokenizer = Tokenizer::new();
    /// let result = tokenizer.tokenize_arg("[DEBUG,HOST=localhost,USER='admin']")?;
    ///
    /// assert_eq!(result.tokens, vec!["DEBUG", "HOST=localhost", "USER='admin'"]);
    /// assert!(result.has_bracketed_content);
    /// # Ok::<(), pam_args_rs::Error>(())
    /// ```
    pub fn tokenize_arg(&self, arg: &str) -> Result<TokenizationResult, Error> {
        // Implementation details provided in next section
        unimplemented!()
    }
    
    /// Tokenizes multiple pre-tokenized arguments
    ///
    /// This method processes multiple arguments, handling special formats
    /// for each argument.
    ///
    /// # Arguments
    ///
    /// * `args` - Iterator of arguments to tokenize
    ///
    /// # Returns
    ///
    /// Result containing the tokenization result or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if any argument fails to tokenize.
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::Tokenizer;
    ///
    /// let tokenizer = Tokenizer::new();
    /// let args = vec!["DEBUG", "[HOST=localhost,USER=admin]", "VERBOSE"];
    /// let result = tokenizer.tokenize_args(args)?;
    ///
    /// assert_eq!(result.tokens, vec!["DEBUG", "HOST=localhost", "USER=admin", "VERBOSE"]);
    /// assert!(result.has_bracketed_content);
    /// # Ok::<(), pam_args_rs::Error>(())
    /// ```
    pub fn tokenize_args<I, S>(&self, args: I) -> Result<TokenizationResult, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        // Implementation details provided in next section
        unimplemented!()
    }
    
    /// Processes a single bracketed argument
    ///
    /// This method extracts the content of a bracketed argument and splits it
    /// into individual tokens based on the delimiter character.
    ///
    /// # Arguments
    ///
    /// * `bracketed` - Input string containing a bracketed argument
    ///
    /// # Returns
    ///
    /// Result containing a vector of tokens or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The bracketed argument is malformed
    /// * A nested delimiter is unclosed
    fn process_bracketed(&self, bracketed: &str) -> Result<Vec<String>, Error> {
        // Implementation details provided in next section
        unimplemented!()
    }
    
    /// Extracts the content of a bracketed string (without the brackets)
    ///
    /// # Arguments
    ///
    /// * `bracketed` - Input string containing a bracketed argument
    ///
    /// # Returns
    ///
    /// Result containing the content without brackets or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if the input is not properly bracketed
    fn extract_bracket_content(&self, bracketed: &str) -> Result<&str, Error> {
        // Implementation details provided in next section
        unimplemented!()
    }
    
    /// Splits comma-separated values while respecting quotes and escape sequences
    ///
    /// # Arguments
    ///
    /// * `content` - Content to split by commas
    ///
    /// # Returns
    ///
    /// Result containing a vector of split values or an error
    fn split_by_commas(&self, content: &str) -> Result<Vec<String>, Error> {
        // Implementation details provided in next section
        unimplemented!()
    }
}

impl Default for TokenizerConfig {
    fn default() -> Self {
        Self {
            escape_char: '\\',
            single_quote: '\'',
            double_quote: '"',
            open_bracket: '[',
            close_bracket: ']',
            delimiter: ',',
        }
    }
}
```

### Implementation Approach

#### 1. Processing Strategy
The tokenizer processes pre-tokenized arguments that PAM modules receive. The implementation uses a character-by-character approach with a state machine for complex inputs like bracketed arguments. This enables precise handling of delimiters, quotes, and escape sequences.

Key processing steps:
1. Examine each argument to determine if it's a special format (e.g., bracketed)
2. For bracketed arguments, process the content character-by-character
3. Track state transitions between normal text, quoted sections, and escape sequences
4. Validate syntax and ensure all delimiters are properly closed

#### 2. Tokenize Arguments Method
```rust
pub fn tokenize_args<I, S>(&self, args: I) -> Result<TokenizationResult, Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut all_tokens = Vec::new();
    let mut has_bracketed = false;
    
    for arg in args {
        let arg_str = arg.as_ref();
        let result = self.tokenize_arg(arg_str)?;
        
        all_tokens.extend(result.tokens);
        has_bracketed = has_bracketed || result.has_bracketed_content;
    }
    
    Ok(TokenizationResult {
        tokens: all_tokens,
        has_bracketed_content: has_bracketed,
    })
}
```

#### 3. Tokenize Single Argument Method
```rust
pub fn tokenize_arg(&self, arg: &str) -> Result<TokenizationResult, Error> {
    // Check if input is a bracketed argument
    if arg.starts_with(self.config.open_bracket) && arg.ends_with(self.config.close_bracket) {
        // Process bracketed content
        let tokens = self.process_bracketed(arg)?;
        return Ok(TokenizationResult {
            tokens,
            has_bracketed_content: true,
        });
    }
    
    // For non-bracketed input, simply return it as a single token
    Ok(TokenizationResult {
        tokens: vec![arg.to_string()],
        has_bracketed_content: false,
    })
}
```

#### 4. Process Bracketed Content Method
```rust
fn process_bracketed(&self, bracketed: &str) -> Result<Vec<String>, Error> {
    // Extract content between brackets
    let content = self.extract_bracket_content(bracketed)?;
    
    // Split by commas, respecting quotes and escape sequences
    self.split_by_commas(content)
}

fn extract_bracket_content(&self, bracketed: &str) -> Result<&str, Error> {
    if !bracketed.starts_with(self.config.open_bracket) || 
       !bracketed.ends_with(self.config.close_bracket) {
        return Err(Error::InvalidInput(format!(
            "Input must start with '{}' and end with '{}'", 
            self.config.open_bracket, 
            self.config.close_bracket
        )));
    }
    
    // Extract content between brackets
    Ok(&bracketed[1..bracketed.len() - 1])
}
```

#### 5. Character-by-Character Processing
```rust
fn split_by_commas(&self, content: &str) -> Result<Vec<String>, Error> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut state = TokenizerState::Normal;
    
    // Process character by character to handle delimiters and escape sequences
    for c in content.chars() {
        match (state, c) {
            // Handle escape sequences
            (TokenizerState::Normal, ch) if ch == self.config.escape_char => {
                current.push(ch);
                state = TokenizerState::EscapeSequence;
            },
            (TokenizerState::EscapeSequence, c) => {
                current.push(c);
                state = TokenizerState::Normal;
            },
            
            // Handle quotes
            (TokenizerState::Normal, ch) if ch == self.config.single_quote => {
                current.push(ch);
                state = TokenizerState::InSingleQuote;
            },
            (TokenizerState::InSingleQuote, ch) if ch == self.config.escape_char => {
                current.push(ch);
                state = TokenizerState::EscapeSequence;
            },
            (TokenizerState::InSingleQuote, ch) if ch == self.config.single_quote => {
                current.push(ch);
                state = TokenizerState::Normal;
            },
            (TokenizerState::InSingleQuote, c) => {
                current.push(c);
            },
            
            (TokenizerState::Normal, ch) if ch == self.config.double_quote => {
                current.push(ch);
                state = TokenizerState::InDoubleQuote;
            },
            (TokenizerState::InDoubleQuote, ch) if ch == self.config.escape_char => {
                current.push(ch);
                state = TokenizerState::EscapeSequence;
            },
            (TokenizerState::InDoubleQuote, ch) if ch == self.config.double_quote => {
                current.push(ch);
                state = TokenizerState::Normal;
            },
            (TokenizerState::InDoubleQuote, c) => {
                current.push(c);
            },
            
            // Handle delimiters in normal state
            (TokenizerState::Normal, ch) if ch == self.config.delimiter => {
                result.push(current.trim().to_string());
                current = String::new();
            },
            
            // Normal character in normal state
            (TokenizerState::Normal, c) => {
                current.push(c);
            },
        }
    }
    
    // Check for unclosed delimiters
    match state {
        TokenizerState::Normal => {
            // Add the last token
            if !current.is_empty() || result.is_empty() {
                result.push(current.trim().to_string());
            }
            Ok(result)
        },
        TokenizerState::InSingleQuote => {
            Err(Error::UnclosedDelimiter(format!("Unclosed single quote in: {}", content)))
        },
        TokenizerState::InDoubleQuote => {
            Err(Error::UnclosedDelimiter(format!("Unclosed double quote in: {}", content)))
        },
        TokenizerState::EscapeSequence => {
            Err(Error::UnclosedDelimiter(format!("Trailing escape character in: {}", content)))
        },
        _ => unreachable!("Invalid state after processing"),
    }
}
```

### Error Handling
The tokenizer uses the library's centralized error system but specifically generates these error types:

```rust
// In src/error.rs
pub enum Error {
    // ... other error types ...
    
    /// Error for unclosed delimiters like quotes or brackets
    UnclosedDelimiter(String),
    
    /// Error for nested brackets (which are not supported)
    NestedBrackets(String),
    
    /// Error for invalid input format
    InvalidInput(String),
}
```

The tokenizer focuses on detecting and reporting these specific syntax errors:
1. Unclosed quotes (both single and double)
2. Unclosed brackets
3. Trailing escape characters
4. Nested brackets (which are explicitly not supported)

Error messages include the problematic input string to help with debugging.

## Integration

### Integration with Other Components

The tokenizer integrates with other components as follows:

1. **Main Parser**: The tokenizer is used by the main parser to process raw arguments before flag and key-value pair identification
2. **Bracket Content Processor**: Works together with the tokenizer to handle complex bracketed arguments
3. **Logging**: Integrates with the library's logging system for debug information

### Usage Examples

```rust
// Basic tokenization of a single argument
let tokenizer = Tokenizer::new();
let result = tokenizer.tokenize_arg("USER=admin")?;
assert_eq!(result.tokens, vec!["USER=admin"]);
assert!(!result.has_bracketed_content);

// Bracketed content
let result = tokenizer.tokenize_arg("[DEBUG,USER=admin,HOST=localhost]")?;
assert_eq!(result.tokens, vec!["DEBUG", "USER=admin", "HOST=localhost"]);
assert!(result.has_bracketed_content);

// Processing multiple arguments
let args = vec!["DEBUG", "[USER=admin,HOST=localhost]", "Some text"];
let result = tokenizer.tokenize_args(args)?;
assert_eq!(result.tokens, vec!["DEBUG", "USER=admin", "HOST=localhost", "Some text"]);
assert!(result.has_bracketed_content);
```

## Testing Strategy

### Unit Test Cases

| # | Category | Input | Expected Output | Notes |
|---|----------|-------|-----------------|-------|
| 1 | Happy Flow | `"DEBUG"` | `["DEBUG"]` | Single token without special handling |
| 2 | Happy Flow | `"[KEY=value,FLAG]"` | `["KEY=value", "FLAG"]` | Bracketed content with comma separation |
| 3 | Happy Flow | `"[KEY1=value1,KEY2=value2,KEY3=value3]"` | `["KEY1=value1", "KEY2=value2", "KEY3=value3"]` | Multiple comma-separated values |
| 4 | Happy Flow | `["FLAG", "[KEY1=value1,KEY2=value2]", "TEXT"]` | `["FLAG", "KEY1=value1", "KEY2=value2", "TEXT"]` | Multiple arguments including bracketed content |
| 5 | Happy Flow | `"[KEY=\"Value with spaces\"]"` | `["KEY=\"Value with spaces\""]` | Quoted value in bracketed content |
| 6 | Happy Flow | `"[KEY1='Single quoted',KEY2=\"Double quoted\"]"` | `["KEY1='Single quoted'", "KEY2=\"Double quoted\""]` | Mixed quote types |
| 7 | Happy Flow | `"[KEY=Value with \\, escaped comma]"` | `["KEY=Value with \\, escaped comma"]` | Escaped delimiter |
| 8 | Happy Flow | `"[KEY=\\[Not a nested bracket\\]]"` | `["KEY=\\[Not a nested bracket\\]]"]` | Escaped brackets |
| 9 | Happy Flow | `"[,,,]"` | `["", "", "", ""]` | Empty elements |
| 10 | Happy Flow | `"[]"` | `[""]` | Empty brackets |
| 11 | Negative Testing | `"[Unclosed bracket"` | Error: InvalidInput | Unclosed bracket |
| 12 | Negative Testing | `"[KEY=\"Unclosed quote]"` | Error: UnclosedDelimiter | Unclosed quote in bracketed content |
| 13 | Negative Testing | `"[KEY=value\\"` | Error: UnclosedDelimiter | Trailing escape character |
| 14 | Edge Case | `"[KEY=\"Value, with comma\"]"` | `["KEY=\"Value, with comma\""]` | Comma inside quotes should not split token |
| 15 | Edge Case | `"[KEY1=value1,,KEY2=value2]"` | `["KEY1=value1", "", "KEY2=value2"]` | Empty value between commas |
| 16 | Edge Case | `"[ KEY1 = value1 , KEY2 = value2 ]"` | `[" KEY1 = value1 ", " KEY2 = value2 "]` | Whitespace preservation |
| 17 | Edge Case | `"[KEY=\'Mixed \"quotes\' inside]"` | `["KEY=\'Mixed \"quotes\' inside"]` | Mixed quotes |
| 18 | Edge Case | `"[KEY=value\\nwith\\tescape\\\\sequences]"` | `["KEY=value\\nwith\\tescape\\\\sequences"]` | Various escape sequences |
| 19 | Edge Case | `"[VERY_LONG_KEY=very_long_value_that_exceeds_typical_buffer_sizes...]"` | (Long value preserved correctly) | Buffer handling |
| 20 | Edge Case | `"[KEY=, VALUE=test]"` | `["KEY=", " VALUE=test"]` | Empty value with space after comma |

### Integration Tests

The tokenizer should be tested in integration with other components to ensure correct end-to-end behavior. Integration tests should focus on:

1. **Parser Integration**
   - Test that tokenized output can be correctly processed by the Flag Argument Processor
   - Test that tokenized output can be correctly processed by the Key-Value Argument Processor
   - Verify that complex, multi-part arguments are properly handled through the entire parsing pipeline

2. **End-to-End Scenarios**
   - Test realistic PAM module argument scenarios
   - Verify handling of common argument patterns used in PAM configurations
   - Test with arguments containing mixed formats (flags, key-value pairs, bracketed content)

3. **Cross-Component Validation**
   - Verify that tokenizer error states propagate correctly to higher-level components
   - Test that tokenized content maintains required metadata for downstream processing
   - Verify preservation of argument order and structure across the parsing pipeline

### Testing Focus Areas

1. **Syntax Validation**
   - Proper handling of delimiters
   - Error detection for unclosed delimiters
   - Validation of bracket pairs

2. **Escape Sequence Handling**
   - Proper preservation of escape sequences
   - Correct interpretation of escaped delimiters

3. **Whitespace Handling**
   - Preservation of significant whitespace
   - Trimming where appropriate

4. **State Machine Transitions**
   - Transitions between different tokenizer states
   - Recovery from error states

5. **Integration Testing**
   - Integration with the main parser
   - Handling of mixed argument types
   - Verification of end-to-end argument processing

## Performance Considerations

### Memory Efficiency
- The tokenizer uses `String` for output tokens but shares references to input during processing
- Preallocates vectors based on estimated token count to reduce reallocations
- Avoids unnecessary string copies and prefers to work with string slices where possible

### Time Complexity
- The tokenization process is O(n) where n is the input length
- Single-pass algorithm that processes each character exactly once
- Uses direct character comparison instead of regex for better performance

### Optimizations
- Optimized to handle common cases efficiently
- Uses dedicated enum variants for each parser state to avoid string comparisons
- Specialized handling for bracketed content to avoid redundant parsing

## Documentation

### Internal Developer Documentation
```rust
//! # Internal Tokenizer Implementation
//!
//! This module provides the implementation of the tokenizer component,
//! which is responsible for processing pre-tokenized input arguments into
//! structured tokens for further processing.
//!
//! ## Design Notes
//!
//! The tokenizer uses a state machine approach to track parsing context
//! and handle delimiters correctly. The main states are:
//!
//! - `Normal`: Outside any delimiter, processing regular text
//! - `InSingleQuote`: Inside single quotes
//! - `InDoubleQuote`: Inside double quotes
//! - `InBracket`: Inside square brackets
//! - `EscapeSequence`: After a backslash, next character is escaped
//!
//! ## Implementation Details
//!
//! The tokenizer processes bracketed arguments character by character,
//! transitioning between states as delimiters are encountered. Special
//! handling is provided for escape sequences to allow delimiters to appear
//! within quoted or bracketed text.
```