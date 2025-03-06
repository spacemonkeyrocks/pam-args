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

use crate::error::{Error, Result};
use log::{debug, trace};

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

/// Main tokenizer struct that handles input processing
pub struct Tokenizer {
    config: TokenizerConfig,
}

impl Tokenizer {
    /// Creates a new tokenizer with default configuration
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // This is an internal module, not part of the public API
    /// use pam_args::internal::tokenizer::Tokenizer;
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
    /// ```ignore
    /// use pam_args::tokenizer::{Tokenizer, TokenizerConfig};
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
    /// ```ignore
    /// use pam_args::tokenizer::Tokenizer;
    ///
    /// let tokenizer = Tokenizer::new();
    /// let result = tokenizer.tokenize_arg("[DEBUG,HOST=localhost,USER='admin']")?;
    ///
    /// assert_eq!(result.tokens, vec!["DEBUG", "HOST=localhost", "USER='admin'"]);
    /// assert!(result.has_bracketed_content);
    /// # Ok::<(), pam_args::Error>(())
    /// ```
    pub fn tokenize_arg(&self, arg: &str) -> Result<TokenizationResult> {
        trace!("Tokenizing argument: '{}'", arg);
        
        // Check if input is a bracketed argument
        if arg.starts_with(self.config.open_bracket) {
            if !arg.ends_with(self.config.close_bracket) {
                return Err(Error::UnclosedDelimiter(format!(
                    "Unclosed bracket in: {}", arg
                )));
            }
            
            debug!("Processing bracketed content: '{}'", arg);
            // Process bracketed content
            let tokens = self.process_bracketed(arg)?;
            return Ok(TokenizationResult {
                tokens,
                has_bracketed_content: true,
            });
        }
        
        // For non-bracketed input, simply return it as a single token
        trace!("Returning non-bracketed argument as-is: '{}'", arg);
        Ok(TokenizationResult {
            tokens: vec![arg.to_string()],
            has_bracketed_content: false,
        })
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
    /// ```ignore
    /// use pam_args::tokenizer::Tokenizer;
    ///
    /// let tokenizer = Tokenizer::new();
    /// let args = vec!["DEBUG", "[HOST=localhost,USER=admin]", "VERBOSE"];
    /// let result = tokenizer.tokenize_args(args)?;
    ///
    /// assert_eq!(result.tokens, vec!["DEBUG", "HOST=localhost", "USER=admin", "VERBOSE"]);
    /// assert!(result.has_bracketed_content);
    /// # Ok::<(), pam_args::Error>(())
    /// ```
    pub fn tokenize_args<I, S>(&self, args: I) -> Result<TokenizationResult>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut all_tokens = Vec::new();
        let mut has_bracketed = false;
        
        for arg in args {
            let arg_str = arg.as_ref();
            debug!("Processing argument in tokenize_args: '{}'", arg_str);
            let result = self.tokenize_arg(arg_str)?;
            
            all_tokens.extend(result.tokens);
            has_bracketed = has_bracketed || result.has_bracketed_content;
        }
        
        trace!("Tokenization complete, found {} tokens", all_tokens.len());
        Ok(TokenizationResult {
            tokens: all_tokens,
            has_bracketed_content: has_bracketed,
        })
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
    fn process_bracketed(&self, bracketed: &str) -> Result<Vec<String>> {
        // Extract content between brackets
        let content = self.extract_bracket_content(bracketed)?;
        trace!("Extracted bracket content: '{}'", content);
        
        // Split by commas, respecting quotes and escape sequences
        self.split_by_commas(content)
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
    fn extract_bracket_content(&self, bracketed: &str) -> Result<String> {
        if !bracketed.starts_with(self.config.open_bracket) ||
           !bracketed.ends_with(self.config.close_bracket) {
            return Err(Error::InvalidInput(format!(
                "Input must start with '{}' and end with '{}'",
                self.config.open_bracket,
                self.config.close_bracket
            )));
        }
        
        // Check for trailing escape character
        let content = &bracketed[1..bracketed.len() - 1];
        if content.ends_with(self.config.escape_char) {
            return Err(Error::InvalidInput(format!(
                "Trailing escape character in: {}", bracketed
            )));
        }
        
        // Extract content between brackets
        Ok(content.to_string())
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
    fn split_by_commas(&self, content: String) -> Result<Vec<String>> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut state = TokenizerState::Normal;
        
        // Handle empty content case
        if content.is_empty() {
            return Ok(vec![String::new()]);
        }
        
        // Check if content ends with a delimiter to handle trailing empty token
        let ends_with_delimiter = content.ends_with(self.config.delimiter);
        
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
                
                // Handle brackets (should not occur in this context, but handle for completeness)
                (TokenizerState::InBracket, _) => {
                    return Err(Error::NestedBrackets(format!(
                        "Nested brackets are not supported: {}", &content
                    )));
                },
                
                // Handle delimiters in normal state
                (TokenizerState::Normal, ch) if ch == self.config.delimiter => {
                    result.push(current.to_string());
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
                    result.push(current.to_string());
                }
                
                // If the content ended with a delimiter, add an empty token
                if ends_with_delimiter {
                    result.push(String::new());
                }
                
                trace!("Split into {} tokens: {:?}", result.len(), result);
                Ok(result)
            },
            TokenizerState::InSingleQuote => {
                Err(Error::UnclosedDelimiter(format!("Unclosed single quote in: {}", &content)))
            },
            TokenizerState::InDoubleQuote => {
                Err(Error::UnclosedDelimiter(format!("Unclosed double quote in: {}", &content)))
            },
            TokenizerState::InBracket => {
                Err(Error::UnclosedDelimiter(format!("Unclosed bracket in: {}", &content)))
            },
            TokenizerState::EscapeSequence => {
                Err(Error::UnclosedDelimiter(format!("Trailing escape character in: {}", &content)))
            },
            _ => unreachable!("Invalid state after processing"),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = TokenizerConfig::default();
        assert_eq!(config.escape_char, '\\');
        assert_eq!(config.single_quote, '\'');
        assert_eq!(config.double_quote, '"');
        assert_eq!(config.open_bracket, '[');
        assert_eq!(config.close_bracket, ']');
        assert_eq!(config.delimiter, ',');
    }

    #[test]
    fn test_tokenize_simple_arg() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("DEBUG").unwrap();
        assert_eq!(result.tokens, vec!["DEBUG"]);
        assert!(!result.has_bracketed_content);
    }

    #[test]
    fn test_tokenize_bracketed_arg() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("[KEY=value,FLAG]").unwrap();
        assert_eq!(result.tokens, vec!["KEY=value", "FLAG"]);
        assert!(result.has_bracketed_content);
    }

    #[test]
    fn test_tokenize_multiple_comma_separated_values() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("[KEY1=value1,KEY2=value2,KEY3=value3]").unwrap();
        assert_eq!(result.tokens, vec!["KEY1=value1", "KEY2=value2", "KEY3=value3"]);
        assert!(result.has_bracketed_content);
    }

    #[test]
    fn test_tokenize_multiple_args() {
        let tokenizer = Tokenizer::new();
        let args = vec!["FLAG", "[KEY1=value1,KEY2=value2]", "TEXT"];
        let result = tokenizer.tokenize_args(args).unwrap();
        assert_eq!(result.tokens, vec!["FLAG", "KEY1=value1", "KEY2=value2", "TEXT"]);
        assert!(result.has_bracketed_content);
    }

    #[test]
    fn test_quoted_values_in_brackets() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("[KEY=\"Value with spaces\"]").unwrap();
        assert_eq!(result.tokens, vec!["KEY=\"Value with spaces\""]);
        assert!(result.has_bracketed_content);
    }

    #[test]
    fn test_mixed_quote_types() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("[KEY1='Single quoted',KEY2=\"Double quoted\"]").unwrap();
        assert_eq!(result.tokens, vec!["KEY1='Single quoted'", "KEY2=\"Double quoted\""]);
        assert!(result.has_bracketed_content);
    }

    #[test]
    fn test_escaped_delimiter() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("[KEY=Value with \\, escaped comma]").unwrap();
        assert_eq!(result.tokens, vec!["KEY=Value with \\, escaped comma"]);
        assert!(result.has_bracketed_content);
    }

    #[test]
    fn test_escaped_brackets() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("[KEY=\\[Not a nested bracket\\]]").unwrap();
        assert_eq!(result.tokens, vec!["KEY=\\[Not a nested bracket\\]"]);
        assert!(result.has_bracketed_content);
    }

    #[test]
    fn test_empty_elements() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("[,,,]").unwrap();
        assert_eq!(result.tokens, vec!["", "", "", ""]); // Three commas create four empty elements
        assert!(result.has_bracketed_content);
    }

    #[test]
    fn test_empty_brackets() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("[]").unwrap();
        assert_eq!(result.tokens, vec![""]);
        assert!(result.has_bracketed_content);
    }

    #[test]
    fn test_unclosed_bracket() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("[Unclosed bracket");
        assert!(result.is_err());
        match result {
            Err(Error::UnclosedDelimiter(_)) => (),
            _ => panic!("Expected UnclosedDelimiter error"),
        }
    }

    #[test]
    fn test_unclosed_quote_in_bracketed_content() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("[KEY=\"Unclosed quote]");
        assert!(result.is_err());
        match result {
            Err(Error::UnclosedDelimiter(_)) => (),
            _ => panic!("Expected UnclosedDelimiter error"),
        }
    }

    #[test]
    fn test_trailing_escape_character() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("[KEY=value\\]");
        assert!(result.is_err());
        match result {
            Err(Error::InvalidInput(_)) => (),
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_comma_inside_quotes() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("[KEY=\"Value, with comma\"]").unwrap();
        assert_eq!(result.tokens, vec!["KEY=\"Value, with comma\""]);
        assert!(result.has_bracketed_content);
    }

    #[test]
    fn test_empty_value_between_commas() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("[KEY1=value1,,KEY2=value2]").unwrap();
        assert_eq!(result.tokens, vec!["KEY1=value1", "", "KEY2=value2"]);
        assert!(result.has_bracketed_content);
    }

    #[test]
    fn test_whitespace_preservation() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("[ KEY1 = value1 , KEY2 = value2 ]").unwrap();
        assert_eq!(result.tokens, vec![" KEY1 = value1 ", " KEY2 = value2 "]);
        assert!(result.has_bracketed_content);
    }

    #[test]
    fn test_mixed_quotes() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("[KEY=\'Mixed \"quotes\' inside]").unwrap();
        assert_eq!(result.tokens, vec!["KEY=\'Mixed \"quotes\' inside"]);
        assert!(result.has_bracketed_content);
    }

    #[test]
    fn test_escape_sequences() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("[KEY=value\\nwith\\tescape\\\\sequences]").unwrap();
        assert_eq!(result.tokens, vec!["KEY=value\\nwith\\tescape\\\\sequences"]);
        assert!(result.has_bracketed_content);
    }

    #[test]
    fn test_empty_value_with_space() {
        let tokenizer = Tokenizer::new();
        let result = tokenizer.tokenize_arg("[KEY=, VALUE=test]").unwrap();
        assert_eq!(result.tokens, vec!["KEY=", " VALUE=test"]);
        assert!(result.has_bracketed_content);
    }

    #[test]
    fn test_custom_config() {
        let config = TokenizerConfig {
            escape_char: '&',
            single_quote: '`',
            double_quote: '*',
            open_bracket: '{',
            close_bracket: '}',
            delimiter: ';',
        };
        let tokenizer = Tokenizer::with_config(config);
        
        let result = tokenizer.tokenize_arg("{KEY1=value1;KEY2=value2}").unwrap();
        assert_eq!(result.tokens, vec!["KEY1=value1", "KEY2=value2"]);
        assert!(result.has_bracketed_content);
        
        let result = tokenizer.tokenize_arg("{KEY=*Value; with semicolon*}").unwrap();
        assert_eq!(result.tokens, vec!["KEY=*Value; with semicolon*"]);
        assert!(result.has_bracketed_content);
    }
}