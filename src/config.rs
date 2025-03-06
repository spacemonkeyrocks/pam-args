//! Configuration for the pam-args library.
//!
//! This module defines the configuration system for the argument parser,
//! providing a way to customize parsing behavior through a builder pattern.
//! The configuration is immutable after creation, ensuring thread safety
//! and consistent behavior during parsing.

use crate::args::AllowedKeyValueFormats;

/// Configuration for the argument parser
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// Whether argument names are case-sensitive
    case_sensitive: bool,
    
    /// Whether argument values are case-sensitive when validating against allowed values
    /// This is separate from case_sensitive which controls argument name matching
    case_sensitive_values: bool,
    
    /// Whether to collect non-argument text
    collect_non_argument_text: bool,
    
    /// Whether to handle multi key-value pairs
    enable_multi_key_value: bool,
    
    /// Allowed formats for multi key-value pairs
    multi_key_value_formats: Vec<AllowedKeyValueFormats>,
    
    /// Character used for escaping special characters (default: '\')
    /// Most users will never need to change this from the default
    escape_char: char,
    
    /// Single quote character (default: ''')
    /// Custom quote characters can be useful when integrating with systems that use different conventions
    single_quote: char,
    
    /// Double quote character (default: '"')
    /// Custom quote characters can be useful when integrating with systems that use different conventions
    double_quote: char,
    
    /// Opening bracket character (default: '[')
    /// Custom bracket characters can be useful for avoiding conflicts in specific environments
    open_bracket: char,
    
    /// Closing bracket character (default: ']')
    /// Custom bracket characters can be useful for avoiding conflicts in specific environments
    close_bracket: char,
    
    /// Delimiter for comma-separated values (default: ',')
    /// Custom delimiters can be useful when comma is commonly used in values
    delimiter: char,
    
    /// Whether to trim whitespace from values
    trim_values: bool,
}

/// Builder for creating parser configurations
#[derive(Debug, Clone)]
pub struct ParserConfigBuilder {
    /// The configuration being built
    config: ParserConfig,
}

impl ParserConfig {
    /// Creates a new parser configuration with default settings
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfig;
    ///
    /// let config = ParserConfig::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Returns whether argument names are case-sensitive
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfig;
    ///
    /// let config = ParserConfig::new();
    /// assert!(config.is_case_sensitive());
    /// ```
    pub fn is_case_sensitive(&self) -> bool {
        self.case_sensitive
    }
    
    /// Returns whether argument values are case-sensitive when validating against allowed values
    /// 
    /// This is separate from `is_case_sensitive()` which controls argument name matching.
    /// For example, if this is set to false, allowed values like "LEFT" would match "left" 
    /// when validating.
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfig;
    ///
    /// let config = ParserConfig::new();
    /// assert!(config.is_case_sensitive_values());
    /// ```
    pub fn is_case_sensitive_values(&self) -> bool {
        self.case_sensitive_values
    }
    
    /// Returns whether non-argument text is collected
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfig;
    ///
    /// let config = ParserConfig::new();
    /// assert!(!config.collect_non_argument_text());
    /// ```
    pub fn collect_non_argument_text(&self) -> bool {
        self.collect_non_argument_text
    }
    
    /// Returns whether multi key-value pairs are enabled
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfig;
    ///
    /// let config = ParserConfig::new();
    /// assert!(!config.enable_multi_key_value());
    /// ```
    pub fn enable_multi_key_value(&self) -> bool {
        self.enable_multi_key_value
    }
    
    /// Returns the allowed formats for multi key-value pairs
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::{ParserConfig, AllowedKeyValueFormats};
    ///
    /// let config = ParserConfig::new();
    /// assert_eq!(config.multi_key_value_formats()[0], AllowedKeyValueFormats::KeyValue);
    /// ```
    pub fn multi_key_value_formats(&self) -> &[AllowedKeyValueFormats] {
        &self.multi_key_value_formats
    }
    
    /// Returns the escape character
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfig;
    ///
    /// let config = ParserConfig::new();
    /// assert_eq!(config.escape_char(), '\\');
    /// ```
    pub fn escape_char(&self) -> char {
        self.escape_char
    }
    
    /// Returns the single quote character
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfig;
    ///
    /// let config = ParserConfig::new();
    /// assert_eq!(config.single_quote(), '\'');
    /// ```
    pub fn single_quote(&self) -> char {
        self.single_quote
    }
    
    /// Returns the double quote character
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfig;
    ///
    /// let config = ParserConfig::new();
    /// assert_eq!(config.double_quote(), '"');
    /// ```
    pub fn double_quote(&self) -> char {
        self.double_quote
    }
    
    /// Returns the opening bracket character
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfig;
    ///
    /// let config = ParserConfig::new();
    /// assert_eq!(config.open_bracket(), '[');
    /// ```
    pub fn open_bracket(&self) -> char {
        self.open_bracket
    }
    
    /// Returns the closing bracket character
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfig;
    ///
    /// let config = ParserConfig::new();
    /// assert_eq!(config.close_bracket(), ']');
    /// ```
    pub fn close_bracket(&self) -> char {
        self.close_bracket
    }
    
    /// Returns the delimiter character
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfig;
    ///
    /// let config = ParserConfig::new();
    /// assert_eq!(config.delimiter(), ',');
    /// ```
    pub fn delimiter(&self) -> char {
        self.delimiter
    }
    
    /// Returns whether values are trimmed
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfig;
    ///
    /// let config = ParserConfig::new();
    /// assert!(config.trim_values());
    /// ```
    pub fn trim_values(&self) -> bool {
        self.trim_values
    }
    
    /// Returns a builder for creating a new configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfig;
    ///
    /// let config = ParserConfig::builder()
    ///     .case_sensitive(false)
    ///     .collect_non_argument_text(true)
    ///     .build();
    /// ```
    pub fn builder() -> ParserConfigBuilder {
        ParserConfigBuilder::new()
    }
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            case_sensitive: true,
            case_sensitive_values: true,
            collect_non_argument_text: false,
            enable_multi_key_value: false,
            multi_key_value_formats: vec![AllowedKeyValueFormats::KeyValue],
            escape_char: '\\',
            single_quote: '\'',
            double_quote: '"',
            open_bracket: '[',
            close_bracket: ']',
            delimiter: ',',
            trim_values: true,
        }
    }
}

impl ParserConfigBuilder {
    /// Creates a new parser configuration builder with default settings
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfigBuilder;
    ///
    /// let builder = ParserConfigBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            config: ParserConfig::default(),
        }
    }
    
    /// Sets whether argument names are case-sensitive
    ///
    /// # Arguments
    ///
    /// * `case_sensitive` - Whether argument names are case-sensitive
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfigBuilder;
    ///
    /// let builder = ParserConfigBuilder::new()
    ///     .case_sensitive(false);
    /// ```
    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.config.case_sensitive = case_sensitive;
        self
    }
    
    /// Sets whether argument values are case-sensitive when validating against allowed values
    ///
    /// This is separate from `case_sensitive()` which controls argument name matching.
    /// When set to false, values like "LEFT" would match "left" during allowed values validation.
    ///
    /// # Arguments
    ///
    /// * `case_sensitive_values` - Whether argument values are case-sensitive
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfigBuilder;
    ///
    /// let builder = ParserConfigBuilder::new()
    ///     .case_sensitive_values(false);
    /// ```
    pub fn case_sensitive_values(mut self, case_sensitive_values: bool) -> Self {
        self.config.case_sensitive_values = case_sensitive_values;
        self
    }
    
    /// Sets whether non-argument text is collected
    ///
    /// # Arguments
    ///
    /// * `collect_non_argument_text` - Whether non-argument text is collected
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfigBuilder;
    ///
    /// let builder = ParserConfigBuilder::new()
    ///     .collect_non_argument_text(true);
    /// ```
    pub fn collect_non_argument_text(mut self, collect_non_argument_text: bool) -> Self {
        self.config.collect_non_argument_text = collect_non_argument_text;
        self
    }
    
    /// Sets whether multi key-value pairs are enabled
    ///
    /// # Arguments
    ///
    /// * `enable_multi_key_value` - Whether multi key-value pairs are enabled
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfigBuilder;
    ///
    /// let builder = ParserConfigBuilder::new()
    ///     .enable_multi_key_value(true);
    /// ```
    pub fn enable_multi_key_value(mut self, enable_multi_key_value: bool) -> Self {
        self.config.enable_multi_key_value = enable_multi_key_value;
        self
    }
    
    /// Sets the allowed formats for multi key-value pairs
    ///
    /// Multiple formats can be combined to support different key-value representations.
    /// The formats will be checked in the order they are specified.
    ///
    /// # Arguments
    ///
    /// * `formats` - Slice of allowed formats
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::{ParserConfigBuilder, AllowedKeyValueFormats};
    ///
    /// // Allow three different formats:
    /// // - KEY=VALUE (e.g., "USER=admin")
    /// // - KEY (e.g., "DEBUG" with no value)
    /// // - KEY= (e.g., "EMPTY=" with empty value)
    /// let builder = ParserConfigBuilder::new()
    ///     .multi_key_value_formats(&[
    ///         AllowedKeyValueFormats::KeyValue,
    ///         AllowedKeyValueFormats::KeyOnly,
    ///         AllowedKeyValueFormats::KeyEquals,
    ///     ]);
    ///     
    /// // Alternatively, use KeyAll as a shorthand for all formats:
    /// let builder = ParserConfigBuilder::new()
    ///     .multi_key_value_formats(&[AllowedKeyValueFormats::KeyAll]);
    /// ```
    pub fn multi_key_value_formats(mut self, formats: &[AllowedKeyValueFormats]) -> Self {
        self.config.multi_key_value_formats = formats.to_vec();
        self
    }
    
    /// Sets the escape character
    ///
    /// # Arguments
    ///
    /// * `escape_char` - The escape character
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfigBuilder;
    ///
    /// let builder = ParserConfigBuilder::new()
    ///     .escape_char('$');
    /// ```
    pub fn escape_char(mut self, escape_char: char) -> Self {
        self.config.escape_char = escape_char;
        self
    }
    
    /// Sets the quote characters
    ///
    /// # Arguments
    ///
    /// * `single_quote` - The single quote character
    /// * `double_quote` - The double quote character
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfigBuilder;
    ///
    /// let builder = ParserConfigBuilder::new()
    ///     .quote_chars('\'', '"');
    /// ```
    pub fn quote_chars(mut self, single_quote: char, double_quote: char) -> Self {
        self.config.single_quote = single_quote;
        self.config.double_quote = double_quote;
        self
    }
    
    /// Sets the bracket characters
    ///
    /// # Arguments
    ///
    /// * `open_bracket` - The opening bracket character
    /// * `close_bracket` - The closing bracket character
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfigBuilder;
    ///
    /// let builder = ParserConfigBuilder::new()
    ///     .bracket_chars('<', '>');
    /// ```
    pub fn bracket_chars(mut self, open_bracket: char, close_bracket: char) -> Self {
        self.config.open_bracket = open_bracket;
        self.config.close_bracket = close_bracket;
        self
    }
    
    /// Sets the delimiter character
    ///
    /// # Arguments
    ///
    /// * `delimiter` - The delimiter character
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfigBuilder;
    ///
    /// let builder = ParserConfigBuilder::new()
    ///     .delimiter(';');
    /// ```
    pub fn delimiter(mut self, delimiter: char) -> Self {
        self.config.delimiter = delimiter;
        self
    }
    
    /// Sets whether values are trimmed
    ///
    /// # Arguments
    ///
    /// * `trim_values` - Whether values are trimmed
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfigBuilder;
    ///
    /// let builder = ParserConfigBuilder::new()
    ///     .trim_values(false);
    /// ```
    pub fn trim_values(mut self, trim_values: bool) -> Self {
        self.config.trim_values = trim_values;
        self
    }
    
    /// Builds the configuration
    ///
    /// # Returns
    ///
    /// The built configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::ParserConfigBuilder;
    ///
    /// let config = ParserConfigBuilder::new()
    ///     .case_sensitive(false)
    ///     .collect_non_argument_text(true)
    ///     .build();
    /// ```
    pub fn build(self) -> ParserConfig {
        self.config
    }
}

impl Default for ParserConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ParserConfig::new();
        assert!(config.is_case_sensitive());
        assert!(config.is_case_sensitive_values());
        assert!(!config.collect_non_argument_text());
        assert!(!config.enable_multi_key_value());
        assert_eq!(config.multi_key_value_formats().len(), 1);
        assert_eq!(config.multi_key_value_formats()[0], AllowedKeyValueFormats::KeyValue);
        assert_eq!(config.escape_char(), '\\');
        assert_eq!(config.single_quote(), '\'');
        assert_eq!(config.double_quote(), '"');
        assert_eq!(config.open_bracket(), '[');
        assert_eq!(config.close_bracket(), ']');
        assert_eq!(config.delimiter(), ',');
        assert!(config.trim_values());
    }

    #[test]
    fn test_builder_default() {
        let config = ParserConfigBuilder::new().build();
        assert!(config.is_case_sensitive());
        assert!(config.is_case_sensitive_values());
        assert!(!config.collect_non_argument_text());
        assert!(!config.enable_multi_key_value());
    }

    #[test]
    fn test_case_sensitivity() {
        let config = ParserConfigBuilder::new()
            .case_sensitive(false)
            .build();
        assert!(!config.is_case_sensitive());
    }

    #[test]
    fn test_value_case_sensitivity() {
        let config = ParserConfigBuilder::new()
            .case_sensitive_values(false)
            .build();
        assert!(!config.is_case_sensitive_values());
    }

    #[test]
    fn test_non_argument_text() {
        let config = ParserConfigBuilder::new()
            .collect_non_argument_text(true)
            .build();
        assert!(config.collect_non_argument_text());
    }

    #[test]
    fn test_multi_key_value() {
        let config = ParserConfigBuilder::new()
            .enable_multi_key_value(true)
            .build();
        assert!(config.enable_multi_key_value());
    }

    #[test]
    fn test_key_value_formats() {
        let config = ParserConfigBuilder::new()
            .multi_key_value_formats(&[
                AllowedKeyValueFormats::KeyValue,
                AllowedKeyValueFormats::KeyEquals,
            ])
            .build();
        assert_eq!(config.multi_key_value_formats().len(), 2);
        assert_eq!(config.multi_key_value_formats()[0], AllowedKeyValueFormats::KeyValue);
        assert_eq!(config.multi_key_value_formats()[1], AllowedKeyValueFormats::KeyEquals);
    }

    #[test]
    fn test_escape_character() {
        let config = ParserConfigBuilder::new()
            .escape_char('$')
            .build();
        assert_eq!(config.escape_char(), '$');
    }

    #[test]
    fn test_quote_characters() {
        let config = ParserConfigBuilder::new()
            .quote_chars('`', '"')
            .build();
        assert_eq!(config.single_quote(), '`');
        assert_eq!(config.double_quote(), '"');
    }

    #[test]
    fn test_bracket_characters() {
        let config = ParserConfigBuilder::new()
            .bracket_chars('<', '>')
            .build();
        assert_eq!(config.open_bracket(), '<');
        assert_eq!(config.close_bracket(), '>');
    }

    #[test]
    fn test_delimiter() {
        let config = ParserConfigBuilder::new()
            .delimiter(';')
            .build();
        assert_eq!(config.delimiter(), ';');
    }

    #[test]
    fn test_value_trimming() {
        let config = ParserConfigBuilder::new()
            .trim_values(false)
            .build();
        assert!(!config.trim_values());
    }

    #[test]
    fn test_method_chaining() {
        let config = ParserConfigBuilder::new()
            .case_sensitive(false)
            .case_sensitive_values(false)
            .collect_non_argument_text(true)
            .enable_multi_key_value(true)
            .multi_key_value_formats(&[AllowedKeyValueFormats::KeyAll])
            .escape_char('$')
            .quote_chars('`', '"')
            .bracket_chars('<', '>')
            .delimiter(';')
            .trim_values(false)
            .build();
        
        assert!(!config.is_case_sensitive());
        assert!(!config.is_case_sensitive_values());
        assert!(config.collect_non_argument_text());
        assert!(config.enable_multi_key_value());
        assert_eq!(config.multi_key_value_formats()[0], AllowedKeyValueFormats::KeyAll);
        assert_eq!(config.escape_char(), '$');
        assert_eq!(config.single_quote(), '`');
        assert_eq!(config.double_quote(), '"');
        assert_eq!(config.open_bracket(), '<');
        assert_eq!(config.close_bracket(), '>');
        assert_eq!(config.delimiter(), ';');
        assert!(!config.trim_values());
    }

    #[test]
    fn test_clone() {
        let config1 = ParserConfigBuilder::new()
            .case_sensitive(false)
            .build();
        
        let config2 = config1.clone();
        assert!(!config2.is_case_sensitive());
    }
}