//! Testing utilities for the pam-args library.
//!
//! This module provides utility functions and structures specifically designed for testing
//! the library and PAM modules that use it. These utilities provide functions for creating
//! mock arguments, verifying parsing results, and simulating different input scenarios.

use crate::error::{Error, Result};
use std::str::FromStr;
use colored::Colorize;

// Define macros at the top of the file so they're available throughout
#[macro_export]
#[doc(hidden)]
macro_rules! assert_colored {
    ($condition:expr, $msg:expr) => {
        if !$condition {
            use colored::Colorize;
            eprintln!("{}", $msg.red());
            panic!("{}", $msg);
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! assert_eq_colored {
    ($left:expr, $right:expr, $msg:expr) => {
        if $left != $right {
            use colored::Colorize;
            eprintln!("{}", $msg.red());
            panic!("assertion failed: `(left == right)`\n left: `{:?}`,\n right: `{:?}`\n{}", $left, $right, $msg);
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! assert_ne_colored {
    ($left:expr, $right:expr, $msg:expr) => {
        if $left == $right {
            use colored::Colorize;
            eprintln!("{}", $msg.red());
            panic!("assertion failed: `(left != right)`\n left: `{:?}`,\n right: `{:?}`\n{}", $left, $right, $msg);
        }
    };
}

/// Configuration for test argument generation
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// Whether to use brackets for multi key-value pairs
    pub use_brackets: bool,
    
    /// Whether to use quotes for values with spaces
    pub use_quotes: bool,
    
    /// Quote character to use (single or double)
    pub quote_char: char,
    
    /// Whether to include non-argument text
    pub include_non_arg_text: bool,
    
    /// Whether to mix argument types in random order
    pub randomize_order: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            use_brackets: false,
            use_quotes: true,
            quote_char: '"',
            include_non_arg_text: false,
            randomize_order: false,
        }
    }
}

impl TestConfig {
    /// Create a new test configuration with default settings
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a configuration that uses brackets for key-value pairs
    pub fn with_brackets() -> Self {
        Self {
            use_brackets: true,
            ..Self::default()
        }
    }
    
    /// Create a configuration that doesn't use brackets
    pub fn without_brackets() -> Self {
        Self {
            use_brackets: false,
            ..Self::default()
        }
    }
    
    /// Set whether to use quotes for values with spaces
    pub fn with_use_quotes(mut self, use_quotes: bool) -> Self {
        self.use_quotes = use_quotes;
        self
    }
    
    /// Set the quote character to use
    pub fn with_quote_char(mut self, quote_char: char) -> Self {
        self.quote_char = quote_char;
        self
    }
    
    /// Set whether to include non-argument text
    pub fn with_non_arg_text(mut self, include: bool) -> Self {
        self.include_non_arg_text = include;
        self
    }
    
    /// Set whether to randomize the order of arguments
    pub fn with_randomize_order(mut self, randomize: bool) -> Self {
        self.randomize_order = randomize;
        self
    }
}

/// Builder for creating test arguments
pub struct TestArgsBuilder {
    flags: Vec<String>,
    key_values: Vec<(String, String)>,
    non_arg_text: Vec<String>,
    config: TestConfig,
}

impl TestArgsBuilder {
    /// Create a new test arguments builder with default configuration
    pub fn new() -> Self {
        Self {
            flags: Vec::new(),
            key_values: Vec::new(),
            non_arg_text: Vec::new(),
            config: TestConfig::default(),
        }
    }
    
    /// Create a new test arguments builder with the specified configuration
    pub fn with_config(config: TestConfig) -> Self {
        Self {
            flags: Vec::new(),
            key_values: Vec::new(),
            non_arg_text: Vec::new(),
            config,
        }
    }
    
    /// Add a flag to the arguments
    pub fn add_flag<S: AsRef<str>>(mut self, flag: S) -> Self {
        self.flags.push(flag.as_ref().to_string());
        self
    }
    
    /// Add multiple flags to the arguments
    pub fn add_flags<S: AsRef<str>, I: IntoIterator<Item = S>>(mut self, flags: I) -> Self {
        for flag in flags {
            self.flags.push(flag.as_ref().to_string());
        }
        self
    }
    
    /// Add a key-value pair to the arguments
    pub fn add_key_value<K: AsRef<str>, V: AsRef<str>>(mut self, key: K, value: V) -> Self {
        self.key_values.push((key.as_ref().to_string(), value.as_ref().to_string()));
        self
    }
    
    /// Add multiple key-value pairs to the arguments
    pub fn add_key_values<K: AsRef<str>, V: AsRef<str>, I: IntoIterator<Item = (K, V)>>(
        mut self,
        key_values: I,
    ) -> Self {
        for (key, value) in key_values {
            self.key_values.push((key.as_ref().to_string(), value.as_ref().to_string()));
        }
        self
    }
    
    /// Add non-argument text to the arguments
    pub fn add_non_arg_text<S: AsRef<str>>(mut self, text: S) -> Self {
        self.non_arg_text.push(text.as_ref().to_string());
        self
    }
    
    /// Set the configuration for this builder
    pub fn with_config_override(mut self, config: TestConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Build the arguments as a vector of strings
    pub fn build(self) -> Vec<String> {
        let mut result = Vec::new();
        
        // Add flags
        for flag in self.flags {
            result.push(flag);
        }
        
        // Add key-value pairs
        for (key, value) in self.key_values {
            let formatted_value = if self.config.use_quotes && value.contains(' ') {
                format!("{}{}{}", self.config.quote_char, value, self.config.quote_char)
            } else {
                value
            };
            
            if self.config.use_brackets {
                result.push(format!("[{}={}]", key, formatted_value));
            } else {
                result.push(format!("{}={}", key, formatted_value));
            }
        }
        
        // Add non-argument text if enabled
        if self.config.include_non_arg_text {
            for text in self.non_arg_text {
                result.push(text);
            }
        }
        
        // Randomize order if enabled
        if self.config.randomize_order && result.len() > 1 {
            use std::collections::HashMap;
            
            // Fisher-Yates shuffle algorithm
            let mut rng = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            for i in (1..result.len()).rev() {
                // Simple PRNG for deterministic shuffling
                rng = (rng * 48271) % 0x7fffffff;
                let j = (rng % (i + 1) as u64) as usize;
                result.swap(i, j);
            }
        }
        
        result
    }
}

// Placeholder for ParseResult until it's implemented
#[doc(hidden)]
struct ParseResult {
    // This will be replaced with the actual implementation
}

impl ParseResult {
    // Placeholder methods
    pub fn is_present(&self, _flag: &str) -> bool {
        false
    }
    
    pub fn value_of<T>(&self, _key: &str) -> Option<T> {
        None
    }
    
    pub fn has_key(&self, _key: &str) -> bool {
        false
    }
    
    pub fn non_argument_text(&self) -> Vec<&str> {
        Vec::new()
    }
    
    pub fn flags(&self) -> Vec<&str> {
        Vec::new()
    }
    
    pub fn keys(&self) -> Vec<&str> {
        Vec::new()
    }
}

/// Struct for fluent assertions on parse results
pub struct TestAssertions<'a> {
    parse_result: &'a ParseResult,
}

impl<'a> TestAssertions<'a> {
    /// Create a new test assertions object for the given parse result
    pub fn new(parse_result: &'a ParseResult) -> Self {
        Self { parse_result }
    }
    
    /// Assert that a flag is present in the parse result
    pub fn assert_flag_present<S: AsRef<str>>(self, flag: S) -> Self {
        assert_colored!(
            self.parse_result.is_present(flag.as_ref()),
            format!("Expected flag '{}' to be present, but it was not", flag.as_ref())
        );
        self
    }
    
    /// Assert that a flag is not present in the parse result
    pub fn assert_flag_not_present<S: AsRef<str>>(self, flag: S) -> Self {
        assert_colored!(
            !self.parse_result.is_present(flag.as_ref()),
            format!("Expected flag '{}' to not be present, but it was", flag.as_ref())
        );
        self
    }
    
    /// Assert that a key-value pair has the expected value
    pub fn assert_value_equals<S: AsRef<str>, T: PartialEq + std::fmt::Debug>(
        self,
        key: S,
        expected: T,
    ) -> Self
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let actual = self.parse_result.value_of::<T>(key.as_ref());
        assert_colored!(
            actual.is_some(),
            format!("Expected key '{}' to have a value, but it did not", key.as_ref())
        );
        
        let actual = actual.unwrap();
        assert_eq_colored!(
            actual,
            expected,
            format!("Expected key '{}' to have value {:?}, but got {:?}", key.as_ref(), expected, actual)
        );
        
        self
    }
    
    /// Assert that a key-value pair is not present
    pub fn assert_key_not_present<S: AsRef<str>>(self, key: S) -> Self {
        assert_colored!(
            !self.parse_result.has_key(key.as_ref()),
            format!("Expected key '{}' to not be present, but it was", key.as_ref())
        );
        self
    }
    
    /// Assert that the non-argument text matches the expected value
    pub fn assert_non_arg_text<S: AsRef<str>>(self, expected: &[S]) -> Self {
        let actual = self.parse_result.non_argument_text();
        let expected_vec: Vec<&str> = expected.iter().map(|s| s.as_ref()).collect();
        
        assert_eq_colored!(
            actual,
            expected_vec,
            format!("Expected non-argument text to be {:?}, but got {:?}", expected_vec, actual)
        );
        
        self
    }
    
    /// Assert that the parse result has the expected number of flags
    pub fn assert_flag_count(self, expected: usize) -> Self {
        let actual = self.parse_result.flags().len();
        assert_eq_colored!(
            actual,
            expected,
            format!("Expected {} flags, but got {}", expected, actual)
        );
        self
    }
    
    /// Assert that the parse result has the expected number of key-value pairs
    pub fn assert_key_value_count(self, expected: usize) -> Self {
        let actual = self.parse_result.keys().len();
        assert_eq_colored!(
            actual,
            expected,
            format!("Expected {} key-value pairs, but got {}", expected, actual)
        );
        self
    }
}

/// Utility for simulating error conditions
pub struct ErrorSimulator {
    config: TestConfig,
}

impl ErrorSimulator {
    /// Create a new error simulator with default configuration
    pub fn new() -> Self {
        Self {
            config: TestConfig::default(),
        }
    }
    
    /// Create a new error simulator with the specified configuration
    pub fn with_config(config: TestConfig) -> Self {
        Self { config }
    }
    
    /// Generate arguments that will trigger a required argument missing error
    pub fn required_arg_missing<S: AsRef<str>>(&self, required_arg: S) -> Vec<String> {
        // Return empty arguments to trigger the required arg missing error
        Vec::new()
    }
    
    /// Generate arguments that will trigger a mutually exclusive args error
    pub fn mutually_exclusive_args<S1: AsRef<str>, S2: AsRef<str>>(
        &self,
        arg1: S1,
        arg2: S2,
    ) -> Vec<String> {
        // Include both mutually exclusive arguments
        vec![arg1.as_ref().to_string(), arg2.as_ref().to_string()]
    }
    
    /// Generate arguments that will trigger a dependency not met error
    pub fn dependency_not_met<S1: AsRef<str>, S2: AsRef<str>>(
        &self,
        arg: S1,
        dependency: S2,
    ) -> Vec<String> {
        // Include the argument but not its dependency
        vec![arg.as_ref().to_string()]
    }
    
    /// Generate arguments that will trigger an invalid value error
    pub fn invalid_value<S1: AsRef<str>, S2: AsRef<str>>(
        &self,
        arg: S1,
        invalid_value: S2,
    ) -> Vec<String> {
        // Create a key-value pair with an invalid value
        vec![format!("{}={}", arg.as_ref(), invalid_value.as_ref())]
    }
    
    /// Generate arguments that will trigger an invalid key-value format error
    pub fn invalid_key_value<S: AsRef<str>>(&self, invalid_kv: S) -> Vec<String> {
        // Include a malformed key-value pair
        vec![invalid_kv.as_ref().to_string()]
    }
    
    /// Generate arguments that will trigger an unrecognized argument error
    pub fn unrecognized_arg<S: AsRef<str>>(&self, unrecognized: S) -> Vec<String> {
        // Include an argument that isn't defined in the parser
        vec![unrecognized.as_ref().to_string()]
    }
    
    /// Generate arguments that will trigger an invalid integer value error
    pub fn invalid_int_value<S: AsRef<str>>(&self, key: S) -> Vec<String> {
        // Create a key-value pair with a non-integer value
        vec![format!("{}=not_an_integer", key.as_ref())]
    }
    
    /// Generate arguments that will trigger an invalid boolean value error
    pub fn invalid_bool_value<S: AsRef<str>>(&self, key: S) -> Vec<String> {
        // Create a key-value pair with a non-boolean value
        vec![format!("{}=not_a_boolean", key.as_ref())]
    }
}

// Placeholder for ArgumentParser until it's implemented
#[doc(hidden)]
struct ArgumentParser {
    // This will be replaced with the actual implementation
}

impl ArgumentParser {
    // Placeholder methods
    fn parse(&self, _args: Vec<String>) -> Result<ParseResult> {
        Ok(ParseResult {})
    }
}

/// Create a test parser with common configuration
fn create_test_parser() -> ArgumentParser {
    ArgumentParser {}
}

/// Create a PAM argument string from a module name and arguments
pub fn create_pam_arg_string<S1: AsRef<str>, I, S2: AsRef<str>>(
    module_name: S1,
    args: I,
) -> String
where
    I: IntoIterator<Item = S2>,
{
    let args_str: Vec<String> = args.into_iter().map(|s| s.as_ref().to_string()).collect();
    format!("{} {}", module_name.as_ref(), args_str.join(" "))
}

/// Split a PAM argument string into individual arguments
pub fn split_pam_arg_string<S: AsRef<str>>(arg_string: S) -> Vec<String> {
    let mut result = Vec::new();
    let mut in_quotes = false;
    let mut current = String::new();
    
    for c in arg_string.as_ref().chars() {
        if c == '"' {
            in_quotes = !in_quotes;
            current.push(c);
        } else if c == ' ' && !in_quotes {
            if !current.is_empty() {
                result.push(current);
                current = String::new();
            }
        } else {
            current.push(c);
        }
    }
    
    if !current.is_empty() {
        result.push(current);
    }
    
    result
}

/// Assert that a result contains the expected error variant
pub fn assert_error_variant<T, F>(result: &Result<T>, predicate: F)
where
    F: FnOnce(&Error) -> bool,
{
    assert_colored!(
        result.is_err(),
        "Expected an error, but got Ok(_)".to_string()
    );
    
    if let Err(e) = result {
        assert_colored!(
            predicate(e),
            format!("Error did not match expected variant: {:?}", e)
        );
    }
}

/// Assert that a result contains a RequiredArgMissing error for the specified argument
pub fn assert_required_arg_missing<T, S: AsRef<str>>(result: &Result<T>, arg: S) {
    assert_error_variant(result, |e| {
        matches!(e, Error::RequiredArgMissing(a) if a == arg.as_ref())
    });
}

/// Assert that a result contains a MutuallyExclusiveArgs error for the specified arguments
pub fn assert_mutually_exclusive_args<T, S1: AsRef<str>, S2: AsRef<str>>(
    result: &Result<T>,
    arg1: S1,
    arg2: S2,
) {
    assert_error_variant(result, |e| {
        matches!(e, Error::MutuallyExclusiveArgs(a1, a2) if 
            (a1 == arg1.as_ref() && a2 == arg2.as_ref()) ||
            (a1 == arg2.as_ref() && a2 == arg1.as_ref())
        )
    });
}

/// Assert that a result contains a DependencyNotMet error for the specified arguments
pub fn assert_dependency_not_met<T, S1: AsRef<str>, S2: AsRef<str>>(
    result: &Result<T>,
    arg: S1,
    dependency: S2,
) {
    assert_error_variant(result, |e| {
        matches!(e, Error::DependencyNotMet(a, d) if a == arg.as_ref() && d == dependency.as_ref())
    });
}

/// Assert that a result contains an InvalidValue error for the specified argument and value
pub fn assert_invalid_value<T, S1: AsRef<str>, S2: AsRef<str>>(
    result: &Result<T>,
    arg: S1,
    value: S2,
) {
    assert_error_variant(result, |e| {
        matches!(e, Error::InvalidValue(a, v) if a == arg.as_ref() && v == value.as_ref())
    });
}

/// Macro for defining test cases
#[doc(hidden)]
macro_rules! test_case {
    (args: [$($arg:expr),* $(,)?], assert: $assert_fn:expr) => {
        {
            let args = vec![$($arg.to_string()),*];
            let parser = crate::testing::create_test_parser();
            let result = parser.parse(args).expect("Failed to parse arguments");
            $assert_fn(&result);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_test_config_default() {
        let config = TestConfig::default();
        assert!(!config.use_brackets);
        assert!(config.use_quotes);
        assert_eq!(config.quote_char, '"');
        assert!(!config.include_non_arg_text);
        assert!(!config.randomize_order);
    }
    
    #[test]
    fn test_test_config_with_brackets() {
        let config = TestConfig::with_brackets();
        assert!(config.use_brackets);
        assert!(config.use_quotes);
        assert_eq!(config.quote_char, '"');
    }
    
    #[test]
    fn test_test_config_without_brackets() {
        let config = TestConfig::without_brackets();
        assert!(!config.use_brackets);
        assert!(config.use_quotes);
        assert_eq!(config.quote_char, '"');
    }
    
    #[test]
    fn test_test_config_with_use_quotes() {
        let config = TestConfig::default().with_use_quotes(false);
        assert!(!config.use_quotes);
    }
    
    #[test]
    fn test_test_config_with_quote_char() {
        let config = TestConfig::default().with_quote_char('\'');
        assert_eq!(config.quote_char, '\'');
    }
    
    #[test]
    fn test_test_config_with_non_arg_text() {
        let config = TestConfig::default().with_non_arg_text(true);
        assert!(config.include_non_arg_text);
    }
    
    #[test]
    fn test_test_config_with_randomize_order() {
        let config = TestConfig::default().with_randomize_order(true);
        assert!(config.randomize_order);
    }
    
    #[test]
    fn test_test_args_builder_add_flag() {
        let args = TestArgsBuilder::new().add_flag("DEBUG").build();
        assert_eq!(args, vec!["DEBUG"]);
    }
    
    #[test]
    fn test_test_args_builder_add_flags() {
        let args = TestArgsBuilder::new()
            .add_flags(vec!["DEBUG", "VERBOSE"])
            .build();
        assert_eq!(args, vec!["DEBUG", "VERBOSE"]);
    }
    
    #[test]
    fn test_test_args_builder_add_key_value() {
        let args = TestArgsBuilder::new()
            .add_key_value("USER", "admin")
            .build();
        assert_eq!(args, vec!["USER=admin"]);
    }
    
    #[test]
    fn test_test_args_builder_add_key_values() {
        let args = TestArgsBuilder::new()
            .add_key_values(vec![("USER", "admin"), ("WIDTH", "80")])
            .build();
        assert_eq!(args, vec!["USER=admin", "WIDTH=80"]);
    }
    
    #[test]
    fn test_test_args_builder_with_quotes() {
        let args = TestArgsBuilder::new()
            .add_key_value("MESSAGE", "Hello World")
            .build();
        assert_eq!(args, vec!["MESSAGE=\"Hello World\""]);
    }
    
    #[test]
    fn test_test_args_builder_without_quotes() {
        let config = TestConfig::default().with_use_quotes(false);
        let args = TestArgsBuilder::new()
            .with_config_override(config)
            .add_key_value("MESSAGE", "Hello World")
            .build();
        assert_eq!(args, vec!["MESSAGE=Hello World"]);
    }
    
    #[test]
    fn test_test_args_builder_with_brackets() {
        let config = TestConfig::with_brackets();
        let args = TestArgsBuilder::new()
            .with_config_override(config)
            .add_key_value("USER", "admin")
            .build();
        assert_eq!(args, vec!["[USER=admin]"]);
    }
    
    #[test]
    fn test_test_args_builder_with_non_arg_text() {
        let config = TestConfig::default().with_non_arg_text(true);
        let args = TestArgsBuilder::new()
            .with_config_override(config)
            .add_flag("DEBUG")
            .add_non_arg_text("Some text")
            .build();
        assert_eq!(args, vec!["DEBUG", "Some text"]);
    }
    
    #[test]
    fn test_test_args_builder_without_non_arg_text() {
        let args = TestArgsBuilder::new()
            .add_flag("DEBUG")
            .add_non_arg_text("Some text")
            .build();
        assert_eq!(args, vec!["DEBUG"]);
    }
    
    #[test]
    fn test_create_pam_arg_string() {
        let args = vec!["DEBUG", "USER=admin"];
        let arg_string = create_pam_arg_string("pam_test", args);
        assert_eq!(arg_string, "pam_test DEBUG USER=admin");
    }
    
    #[test]
    fn test_split_pam_arg_string() {
        let arg_string = "pam_test DEBUG USER=admin";
        let args = split_pam_arg_string(arg_string);
        assert_eq!(args, vec!["pam_test", "DEBUG", "USER=admin"]);
    }
    
    #[test]
    fn test_split_pam_arg_string_with_quotes() {
        let arg_string = "pam_test DEBUG USER=\"John Doe\"";
        let args = split_pam_arg_string(arg_string);
        assert_eq!(args, vec!["pam_test", "DEBUG", "USER=\"John Doe\""]);
    }
    
    #[test]
    fn test_error_simulator_required_arg_missing() {
        let simulator = ErrorSimulator::new();
        let args = simulator.required_arg_missing("USER");
        assert!(args.is_empty());
    }
    
    #[test]
    fn test_error_simulator_mutually_exclusive_args() {
        let simulator = ErrorSimulator::new();
        let args = simulator.mutually_exclusive_args("DEBUG", "QUIET");
        assert_eq!(args, vec!["DEBUG", "QUIET"]);
    }
    
    #[test]
    fn test_error_simulator_dependency_not_met() {
        let simulator = ErrorSimulator::new();
        let args = simulator.dependency_not_met("HOST", "USER");
        assert_eq!(args, vec!["HOST"]);
    }
    
    #[test]
    fn test_error_simulator_invalid_value() {
        let simulator = ErrorSimulator::new();
        let args = simulator.invalid_value("ALIGN", "INVALID");
        assert_eq!(args, vec!["ALIGN=INVALID"]);
    }
    
    #[test]
    fn test_error_simulator_invalid_key_value() {
        let simulator = ErrorSimulator::new();
        let args = simulator.invalid_key_value("USER:admin");
        assert_eq!(args, vec!["USER:admin"]);
    }
    
    #[test]
    fn test_error_simulator_unrecognized_arg() {
        let simulator = ErrorSimulator::new();
        let args = simulator.unrecognized_arg("UNKNOWN");
        assert_eq!(args, vec!["UNKNOWN"]);
    }
    
    #[test]
    fn test_error_simulator_invalid_int_value() {
        let simulator = ErrorSimulator::new();
        let args = simulator.invalid_int_value("WIDTH");
        assert_eq!(args, vec!["WIDTH=not_an_integer"]);
    }
    
    #[test]
    fn test_error_simulator_invalid_bool_value() {
        let simulator = ErrorSimulator::new();
        let args = simulator.invalid_bool_value("DEBUG");
        assert_eq!(args, vec!["DEBUG=not_a_boolean"]);
    }
}