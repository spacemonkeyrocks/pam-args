# Feature 6: Configuration

## Module Type
**Core**: This component stores parser configuration settings that control the behavior of the argument parsing process. While primarily an internal implementation detail, it exposes a public interface for users to customize parsing behavior.

## Feature Information

**Feature Name**: Configuration

**Description**: Implements a type-safe, immutable configuration system that stores parser settings such as case sensitivity, multi key-value pair handling, and non-argument text collection. This component provides a centralized way to manage parser behavior, ensuring consistent application of settings across different parsing stages. By using the builder pattern, it offers a fluent, ergonomic API for customizing parser behavior while maintaining immutability and thread safety.

**Priority**: High

**Dependencies**: None

## Requirements

### Functional Requirements
1. Store configuration for case sensitivity of argument names
2. Store configuration for case sensitivity of argument values
3. Enable/disable collection of non-argument text
4. Enable/disable multi key-value pair handling
5. Configure allowed formats for multi key-value pairs
6. Store custom delimiters for parsing (escape character, quotes, brackets)
7. Enable/disable whitespace trimming for values
8. Support custom delimiters for comma-separated values
9. Ensure configuration immutability after creation

### API Requirements
- Provide a clean, builder-style API for configuration
- Ensure all configuration options have sensible defaults
- Make configuration thread-safe for concurrent use
- Allow configuration to be cloned for reuse
- Provide clear documentation for each configuration option
- Keep the public API minimal and focused

### Performance Requirements
- Minimize heap allocations in configuration objects
- Ensure fast access to configuration settings
- Optimize configuration for the most common use cases
- Keep configuration objects small in memory
- Make configuration options zero-cost for unused features

## Design

### Data Structures
```rust
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
```

### Function Signatures
```rust
impl ParserConfig {
    /// Creates a new parser configuration with default settings
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::ParserConfig;
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
    /// use pam_args_rs::ParserConfig;
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
    /// use pam_args_rs::ParserConfig;
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
    /// use pam_args_rs::ParserConfig;
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
    /// use pam_args_rs::ParserConfig;
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
    /// use pam_args_rs::{ParserConfig, AllowedKeyValueFormats};
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
    /// use pam_args_rs::ParserConfig;
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
    /// use pam_args_rs::ParserConfig;
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
    /// use pam_args_rs::ParserConfig;
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
    /// use pam_args_rs::ParserConfig;
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
    /// use pam_args_rs::ParserConfig;
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
    /// use pam_args_rs::ParserConfig;
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
    /// use pam_args_rs::ParserConfig;
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
    /// use pam_args_rs::ParserConfig;
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
    /// use pam_args_rs::ParserConfigBuilder;
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
    /// use pam_args_rs::ParserConfigBuilder;
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
    /// use pam_args_rs::ParserConfigBuilder;
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
    /// use pam_args_rs::ParserConfigBuilder;
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
    /// use pam_args_rs::ParserConfigBuilder;
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
    /// use pam_args_rs::{ParserConfigBuilder, AllowedKeyValueFormats};
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
    /// use pam_args_rs::ParserConfigBuilder;
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
    /// use pam_args_rs::ParserConfigBuilder;
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
    /// use pam_args_rs::ParserConfigBuilder;
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
    /// use pam_args_rs::ParserConfigBuilder;
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
    /// use pam_args_rs::ParserConfigBuilder;
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
    /// use pam_args_rs::ParserConfigBuilder;
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
```

### Implementation Approach

#### 1. Immutable Configuration Design
The `ParserConfig` struct is designed to be immutable after creation, enforcing a clear separation between configuration and use:

```rust
#[derive(Debug, Clone)]
pub struct ParserConfig {
    case_sensitive: bool,
    case_sensitive_values: bool,
    collect_non_argument_text: bool,
    // Other configuration fields...
}
```

This approach:
- Prevents accidental modification of configuration during parsing
- Enables thread-safe sharing of configuration
- Provides clear ownership semantics
- Makes configuration reuse safe and predictable

#### 2. Builder Pattern for Configuration

The `ParserConfigBuilder` uses the builder pattern to provide a fluent API for configuration:

```rust
let config = ParserConfigBuilder::new()
    .case_sensitive(false)
    .collect_non_argument_text(true)
    .enable_multi_key_value(true)
    .multi_key_value_formats(&[AllowedKeyValueFormats::KeyValue, AllowedKeyValueFormats::KeyOnly])
    .build();
```

This approach:
- Makes configuration clear and readable
- Allows for method chaining
- Supports partial configuration with sensible defaults
- Separates configuration building from usage

#### 3. Sensible Defaults
The configuration system provides sensible defaults aligned with PAM module conventions:

```rust
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
```

These defaults:
- Match common PAM module expectations
- Make simple configurations easy to specify
- Reduce boilerplate for common cases
- Follow the principle of least surprise

#### 4. Centralized Configuration Management

The configuration is centralized to ensure consistency across the parser:

```rust
pub fn is_case_sensitive(&self) -> bool {
    self.case_sensitive
}

pub fn collect_non_argument_text(&self) -> bool {
    self.collect_non_argument_text
}

// Other configuration accessors...
```

This approach:
- Ensures consistent application of settings
- Provides a single source of truth for configuration
- Makes configuration changes explicit and traceable
- Simplifies testing of different configurations

#### 5. Type Safety with Enums

The configuration uses enums for type-safe configuration options:

```rust
/// Represents allowed formats for key-value pairs
/// 
/// These formats can be combined in an array to support multiple formats simultaneously.
/// For example, to allow both KEY=VALUE and KEY= formats but not KEY format:
/// ```
/// use pam_args_rs::{ParserConfigBuilder, AllowedKeyValueFormats};
///
/// let builder = ParserConfigBuilder::new()
///     .multi_key_value_formats(&[
///         AllowedKeyValueFormats::KeyValue,  // Allow USER=admin
///         AllowedKeyValueFormats::KeyEquals, // Allow USER= (empty value)
///     ]);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AllowedKeyValueFormats {
    /// KEY=VALUE format (e.g., "USER=admin")
    KeyValue,
    
    /// KEY format without value (e.g., "DEBUG")
    KeyOnly,
    
    /// KEY= format with empty value (e.g., "EMPTY=")
    KeyEquals,
    
    /// Convenience type that includes all formats (equivalent to specifying all 3 other formats)
    KeyAll,
}
```

This design:
- Prevents invalid configuration values
- Makes the API self-documenting
- Allows for compiler-checked configuration
- Enables pattern matching for configuration logic

#### 6. Flexible Delimiter Configuration

The configuration supports customization of all delimiters used in parsing:

```rust
pub fn escape_char(mut self, escape_char: char) -> Self {
    self.config.escape_char = escape_char;
    self
}

pub fn quote_chars(mut self, single_quote: char, double_quote: char) -> Self {
    self.config.single_quote = single_quote;
    self.config.double_quote = double_quote;
    self
}

pub fn bracket_chars(mut self, open_bracket: char, close_bracket: char) -> Self {
    self.config.open_bracket = open_bracket;
    self.config.close_bracket = close_bracket;
    self
}

pub fn delimiter(mut self, delimiter: char) -> Self {
    self.config.delimiter = delimiter;
    self
}
```

This flexibility:
- Accommodates different syntax conventions
- Supports special use cases
- Enables parsing of non-standard formats
- Allows adaptation to existing command-line interfaces

#### 7. Memory Optimization

The configuration is designed to be memory-efficient:

- Uses primitive types where possible
- Stores only the necessary configuration data
- Uses smart defaults to minimize configuration size
- Avoids unnecessary allocations

## Integration

### Integration with Other Components

The configuration integrates with other components as follows:

1. **ArgumentParser**: Uses the configuration to control overall parsing behavior
2. **Tokenizer**: References delimiter settings for text processing
3. **Parser Modules**: Consult configuration for parsing decisions
4. **Key-Value Store**: Uses case sensitivity settings for lookups
5. **Validation**: Uses configuration to guide validation rules

### Usage Examples

```rust
use pam_args_rs::{ArgumentParser, ParserConfig, AllowedKeyValueFormats, Flag, KeyValue};
use std::str::FromStr;

// Create a custom configuration
let config = ParserConfig::builder()
    .case_sensitive(false)  // Case-insensitive argument names
    .collect_non_argument_text(true)  // Collect non-argument text
    .enable_multi_key_value(true)  // Enable multi key-value pairs
    .multi_key_value_formats(&[
        AllowedKeyValueFormats::KeyValue,  // KEY=VALUE format
        AllowedKeyValueFormats::KeyOnly,   // KEY format
    ])
    .build();

// Create a parser with the custom configuration
let parser = ArgumentParser::with_config(config)
    .flag(Flag::new("DEBUG", "Enable debug mode"))
    .flag(Flag::new("VERBOSE", "Enable verbose output"))
    .key_value(
        KeyValue::new("USER", "Username for authentication")
            .type_converter(String::from_str)
            .required()
    );

// Parse arguments
let args = vec!["debug", "user=admin", "some non-argument text"];
let result = parser.parse(args)?;

// The parser will use the custom configuration:
// - "debug" will match "DEBUG" because of case-insensitive matching
// - "some non-argument text" will be collected as non-argument text
// - Any undefined arguments like "CUSTOM=value" would be treated as multi key-value pairs
```

## Testing Strategy

### Unit Test Cases

| # | Category | Input | Expected Output | Notes |
|---|----------|-------|-----------------|-------|
| 1 | Default Construction | `ParserConfig::new()` | Default configuration values | Test default creation |
| 2 | Builder Construction | `ParserConfigBuilder::new().build()` | Default configuration values | Test builder with defaults |
| 3 | Case Sensitivity | `builder.case_sensitive(false).build()` | Config with case_sensitive=false | Test case sensitivity setting |
| 4 | Value Case Sensitivity | `builder.case_sensitive_values(false).build()` | Config with case_sensitive_values=false | Test value case sensitivity for allowed values validation |
| 5 | Non-Argument Text | `builder.collect_non_argument_text(true).build()` | Config with collect_non_argument_text=true | Test non-argument text collection |
| 6 | Multi Key-Value | `builder.enable_multi_key_value(true).build()` | Config with enable_multi_key_value=true | Test multi key-value enabling |
| 7 | Key-Value Formats | `builder.multi_key_value_formats(&[AllowedKeyValueFormats::KeyValue, AllowedKeyValueFormats::KeyEquals]).build()` | Config with combined formats | Test format combination |
| 8 | Escape Character | `builder.escape_char('$').build()` | Config with escape_char='$' | Test escape character setting |
| 9 | Quote Characters | `builder.quote_chars('\'', '"').build()` | Config with specified quote chars | Test quote character setting |
| 10 | Bracket Characters | `builder.bracket_chars('<', '>').build()` | Config with specified bracket chars | Test bracket character setting |
| 11 | Delimiter | `builder.delimiter(';').build()` | Config with delimiter=';' | Test delimiter setting |
| 12 | Value Trimming | `builder.trim_values(false).build()` | Config with trim_values=false | Test value trimming setting |
| 13 | Method Chaining | Chain multiple builder methods | Correctly configured object | Test builder pattern fluidity |
| 14 | Accessor Methods | Call accessor methods on config | Correct property values | Test all accessor methods |
| 15 | Default Builder | `ParserConfigBuilder::default().build()` | Default configuration values | Test builder default implementation |
| 16 | Clone Implementation | `config.clone()` | Equal but distinct config object | Test cloneability |
| 17 | Debug Implementation | `format!("{:?}", config)` | String representing config | Test debug formatting |
| 18 | Builder Accessor Defaults | `builder.new()` methods | Builder with default values | Test builder initial state |
| 19 | Config Immutability | After creation, attempt to modify config | Compiler error | Test immutability of config |
| 20 | Builder Overrides | Set a value twice with builder | Last value takes precedence | Test builder value overrides |
| 21 | All Key-Value Format | `builder.multi_key_value_formats(&[AllowedKeyValueFormats::KeyAll])` | Formats include all variants | Test KeyAll format behavior |
| 22 | Empty Format List | `builder.multi_key_value_formats(&[])` | Empty format list | Test empty format list handling |
| 23 | Builder Method Return | Check return type of builder methods | Builder instance returned | Test builder method return types |
| 24 | Configuration Factory | `ParserConfig::builder()` | Builder initialized with defaults | Test factory method |
| 25 | Configuration Combinations | Complex configuration combinations | Configuration with all settings properly applied | Test interaction between settings |

### Integration Tests

The configuration should be tested in integration with other components to ensure correct end-to-end behavior:

1. **ArgumentParser Integration**
   - Test parser initialization with custom configurations
   - Verify configuration affects parsing behavior
   - Test boundary cases for configuration options
   - Verify configuration consistency across parsing stages

2. **Tokenizer Integration**
   - Test configuration of custom delimiters
   - Verify correct tokenization based on config
   - Test special character handling with custom escape characters
   - Verify bracket and quote handling with custom characters

3. **Case Sensitivity Testing**
   - Test case-insensitive argument matching
   - Verify case-insensitive value comparison
   - Test mixed case argument definitions
   - Verify case handling consistency

### Testing Focus Areas

1. **Default Behavior**
   - Verify sensible defaults work as expected
   - Test common use cases with default configuration
   - Verify default behavior matches documentation
   - Test alignment with PAM module conventions

2. **Custom Configuration**
   - Test all customizable options
   - Verify custom configurations are applied correctly
   - Test extreme or unusual configurations
   - Verify configuration consistency

3. **Configuration Immutability**
   - Verify configuration cannot be modified after creation
   - Test thread safety with shared configurations
   - Verify configuration reuse works correctly
   - Test configuration cloning behavior

4. **Configuration Performance**
   - Measure configuration overhead
   - Test configuration access patterns
   - Verify zero-cost abstractions work as expected
   - Test configuration with large argument sets

5. **API Usability**
   - Verify builder pattern fluidity
   - Test API ergonomics
   - Verify intuitive behavior
   - Test documentation examples

## Performance Considerations

### Memory Efficiency
- Use primitive types for configuration options
- Avoid unnecessary allocations in configuration objects
- Keep configuration structures compact
- Use enums and small vectors to minimize memory usage
- Share configuration references rather than cloning where possible

### Configuration Access
- Provide direct access to configuration options
- Avoid indirection or virtual dispatch for configuration lookup
- Cache frequently used configuration values
- Use inlining for small accessor functions
- Optimize for the most common access patterns
- Document relationships between related configuration options (e.g., case sensitivity settings)

### Builder Optimizations
- Allocate container fields only when needed
- Use efficient chaining implementation
- Minimize intermediate allocations during building
- Consider arena allocation for complex configurations
- Optimize the build() method for common configurations