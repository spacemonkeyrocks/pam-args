# Feature 13: Key-Value Parsing Logic

## Module Type
**Parser**: This component is a core part of the parser subsystem that implements the parsing and type conversion of key-value pairs. It builds on the key-value definition system and integrates with the rest of the parsing pipeline.

## Feature Information

**Feature Name**: Key-Value Parsing Logic

**Description**: Implement the parsing and type conversion of key-value pairs. This component is responsible for identifying key-value tokens from the input stream, validating them against defined key-value definitions, applying format-specific handling for different key-value forms (KEY=VALUE, KEY=, KEY), converting values to appropriate types, and updating bound fields when specified. It ensures that key-value pairs are correctly processed according to their definitions and constraints.

**Priority**: High

**Dependencies**: 
- [Feature 1: Result & Error Types](error-result-types.md)
- [Feature 2: Utility Functions](utility-functions.md)
- [Feature 7: Format & Type Conversion](format-type-conversion.md)
- [Feature 9: Input Tokenizer and Syntax Validator](tokenizer.md)
- [Feature 12: Key-Value Definition System](key-value-definition-system.md)

## Requirements

### Functional Requirements
1. Parse input tokens to identify key-value pairs in various formats
2. Support KEY=VALUE format with proper value extraction
3. Support KEY= format (empty value) and KEY format (flag-like usage meaning delete)
4. Match parsed key-value pairs against defined key-value definitions
5. Support case-sensitive and case-insensitive key matching
6. Apply type conversion to values based on definition specifications
7. Update bound fields when key-value pairs are successfully parsed
8. Validate key-value pairs against format and value constraints
9. Support fallback to multi key-value store for undefined keys
10. Preserve unprocessed tokens for subsequent processing stages
11. Generate clear error messages for parsing and validation failures
12. Support logging of key-value processing for diagnostic purposes

### API Requirements
- Provide a clean, focused interface for key-value pair parsing
- Integrate seamlessly with the tokenizer and key-value definition system
- Return structured results that indicate parsing outcomes and remaining tokens
- Support efficient key-value lookup after parsing
- Enable integration with validation components
- Facilitate binding updates for identified key-value pairs
- Ensure consistent error propagation throughout the parsing pipeline

### Performance Requirements
- Optimize for efficient key-value parsing
- Minimize allocations during parsing and conversion
- Process large numbers of key-value pairs efficiently
- Ensure O(n) or better performance for parsing operations
- Support efficient format detection and validation

## Design

### Data Structures
```rust
/// Result of key-value pair parsing
#[derive(Debug)]
pub(crate) struct KeyValueProcessResult {
    /// Map of keys to values for explicitly defined key-value pairs
    pub defined_pairs: HashMap<String, Option<String>>,
    
    /// Store for multi key-value pairs (undefined keys)
    pub multi_store: DefaultKeyValueStore,
    
    /// Tokens that were not processed as key-value pairs
    pub remaining_tokens: Vec<String>,
}

/// Configuration for the key-value pair processor
#[derive(Debug, Clone)]
pub(crate) struct KeyValueProcessorConfig {
    /// Whether keys are case-sensitive
    pub case_sensitive: bool,
    
    /// Whether to enable multi key-value store for undefined keys
    pub enable_multi_kv: bool,
    
    /// Allowed formats for multi key-value pairs
    pub multi_kv_formats: Vec<AllowedKeyValueFormats>,
}

impl Default for KeyValueProcessorConfig {
    fn default() -> Self {
        Self {
            case_sensitive: true,
            enable_multi_kv: false,
            multi_kv_formats: vec![AllowedKeyValueFormats::KeyValue],
        }
    }
}

/// Detected format of a key-value token
#[derive(Debug, Clone, Copy, PartialEq)]
enum DetectedFormat {
    /// KEY=VALUE format
    KeyValue,
    
    /// KEY= format (empty value)
    KeyEquals,
    
    /// KEY format (no equals sign)
    KeyOnly,
    
    /// Not a key-value pair
    NotKeyValue,
}

/// Parsed key-value pair
#[derive(Debug, Clone)]
struct ParsedKeyValue {
    /// The key part of the pair
    key: String,
    
    /// The value part of the pair (if any)
    value: Option<String>,
    
    /// The detected format of the pair
    format: DetectedFormat,
}
```

### Function Signatures
```rust
/// Main processor for key-value pair arguments
pub(crate) struct KeyValueArgumentProcessor {
    /// Configuration for the processor
    config: KeyValueProcessorConfig,
    
    /// Logging component for diagnostic purposes
    logger: Option<LogComponent>,
}

impl KeyValueArgumentProcessor {
    /// Creates a new key-value argument processor with default configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_parser::KeyValueArgumentProcessor;
    ///
    /// let processor = KeyValueArgumentProcessor::new();
    /// ```
    pub fn new() -> Self {
        Self {
            config: KeyValueProcessorConfig::default(),
            logger: None,
        }
    }
    
    /// Creates a new key-value argument processor with custom configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the processor
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_parser::{KeyValueArgumentProcessor, KeyValueProcessorConfig};
    ///
    /// let config = KeyValueProcessorConfig {
    ///     case_sensitive: false,
    ///     enable_multi_kv: true,
    ///     multi_kv_formats: vec![AllowedKeyValueFormats::KeyValue],
    /// };
    /// let processor = KeyValueArgumentProcessor::with_config(config);
    /// ```
    pub fn with_config(config: KeyValueProcessorConfig) -> Self {
        Self {
            config,
            logger: None,
        }
    }
    
    /// Enables logging for the processor
    ///
    /// # Arguments
    ///
    /// * `logger` - The logging component to use
    ///
    /// # Returns
    ///
    /// The processor with logging enabled
    pub fn with_logging(mut self, logger: LogComponent) -> Self {
        self.logger = Some(logger);
        self
    }
    
    /// Processes tokens to identify key-value pairs and update bindings
    ///
    /// # Arguments
    ///
    /// * `tokens` - The tokens to process
    /// * `definitions` - The key-value definitions to match against
    ///
    /// # Returns
    ///
    /// Result containing the processing result or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if key-value parsing or validation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_parser::KeyValueArgumentProcessor;
    /// use pam_args_rs::parser::kv_def::{KeyValueDefinitionManager, KeyValueDefinition};
    /// use std::str::FromStr;
    ///
    /// let processor = KeyValueArgumentProcessor::new();
    /// 
    /// let mut manager = KeyValueDefinitionManager::new(true);
    /// manager.add_definition(
    ///     KeyValueDefinition::new("USER", "Username")
    ///         .type_converter(String::from_str)
    /// )?;
    ///
    /// let tokens = vec!["USER=admin".to_string(), "DEBUG".to_string()];
    /// let result = processor.process(&tokens, manager.get_all_definitions())?;
    ///
    /// assert!(result.defined_pairs.contains_key("USER"));
    /// assert_eq!(result.remaining_tokens, vec!["DEBUG"]);
    /// # Ok::<(), pam_args_rs::Error>(())
    /// ```
    pub fn process(
        &self,
        tokens: &[String],
        definitions: Vec<&KeyValueDefinition>,
    ) -> Result<KeyValueProcessResult, Error> {
        let mut defined_pairs = HashMap::new();
        let mut multi_store = DefaultKeyValueStore::new(self.config.case_sensitive);
        let mut remaining_tokens = Vec::new();
        
        // Create a lookup map for faster definition checking
        let def_map: HashMap<String, &KeyValueDefinition> = self.create_definition_map(&definitions);
        
        // Log processing start if logger is enabled
        if let Some(logger) = &self.logger {
            log_debug!(
                logger,
                LogOperation::Parse,
                &format!("Processing {} tokens for key-value pairs", tokens.len()),
                tokens
            );
        }
        
        // Process each token
        for token in tokens {
            // Parse the token into a key-value pair
            let parsed = self.parse_token(token);
            
            match parsed.format {
                DetectedFormat::NotKeyValue => {
                    // Not a key-value pair, add to remaining tokens
                    remaining_tokens.push(token.clone());
                    continue;
                },
                _ => {
                    // Check if this is a defined key-value pair
                    let normalized_key = if self.config.case_sensitive {
                        parsed.key.clone()
                    } else {
                        parsed.key.to_lowercase()
                    };
                    
                    if let Some(definition) = def_map.get(&normalized_key) {
                        // Validate the key-value pair against its definition
                        definition.validate_token(&parsed.key, parsed.value.as_deref())?;
                        
                        // Log found key-value pair if logger is enabled
                        if let Some(logger) = &self.logger {
                            log_debug!(
                                logger,
                                LogOperation::Parse,
                                &format!(
                                    "Found defined key-value pair: {}={}",
                                    parsed.key,
                                    parsed.value.as_deref().unwrap_or("")
                                ),
                            );
                        }
                        
                        // If this is a defined key-value pair with a value and a type converter,
                        // convert the value and update the binding
                        if let Some(value) = &parsed.value {
                            if definition.has_type_converter() {
                                let converted = definition.convert_value(value)?;
                                
                                if definition.has_binding() {
                                    definition.update_binding(converted);
                                }
                            }
                        }
                        
                        // Add to defined pairs
                        defined_pairs.insert(definition.name().to_string(), parsed.value);
                    } else if self.config.enable_multi_kv {
                        // Check if this format is allowed for multi key-value pairs
                        let format = match parsed.format {
                            DetectedFormat::KeyValue => AllowedKeyValueFormats::KeyValue,
                            DetectedFormat::KeyEquals => AllowedKeyValueFormats::KeyEquals,
                            DetectedFormat::KeyOnly => AllowedKeyValueFormats::KeyOnly,
                            _ => unreachable!(), // Already handled NotKeyValue above
                        };
                        
                        if self.is_format_allowed_for_multi_kv(format) {
                            // Log multi key-value pair if logger is enabled
                            if let Some(logger) = &self.logger {
                                log_debug!(
                                    logger,
                                    LogOperation::Parse,
                                    &format!(
                                        "Found multi key-value pair: {}={}",
                                        parsed.key,
                                        parsed.value.as_deref().unwrap_or("")
                                    ),
                                );
                            }
                            
                            // Add to multi key-value store
                            multi_store.add(&parsed.key, parsed.value.as_deref());
                        } else {
                            // Format not allowed for multi key-value pairs, add to remaining tokens
                            remaining_tokens.push(token.clone());
                        }
                    } else {
                        // Not a defined key-value pair and multi key-value store is disabled,
                        // add to remaining tokens
                        remaining_tokens.push(token.clone());
                    }
                }
            }
        }
        
        // Return the processed result
        Ok(KeyValueProcessResult {
            defined_pairs,
            multi_store,
            remaining_tokens,
        })
    }
    
    /// Parses a token into a key-value pair
    ///
    /// # Arguments
    ///
    /// * `token` - The token to parse
    ///
    /// # Returns
    ///
    /// The parsed key-value pair
    fn parse_token(&self, token: &str) -> ParsedKeyValue {
        // Check if the token contains an equals sign
        if let Some(pos) = token.find('=') {
            let (key, value_part) = token.split_at(pos);
            
            // Skip the equals sign
            let value = &value_part[1..];
            
            // Return as KeyValue or KeyEquals based on value presence
            if value.is_empty() {
                ParsedKeyValue {
                    key: key.to_string(),
                    value: Some(String::new()),
                    format: DetectedFormat::KeyEquals,
                }
            } else {
                ParsedKeyValue {
                    key: key.to_string(),
                    value: Some(value.to_string()),
                    format: DetectedFormat::KeyValue,
                }
            }
        } else {
            // No equals sign, check if it could be a KeyOnly format
            if self.is_valid_key_name(token) {
                ParsedKeyValue {
                    key: token.to_string(),
                    value: None,
                    format: DetectedFormat::KeyOnly,
                }
            } else {
                // Not a key-value pair
                ParsedKeyValue {
                    key: token.to_string(),
                    value: None,
                    format: DetectedFormat::NotKeyValue,
                }
            }
        }
    }
    
    /// Checks if a string is a valid key name
    ///
    /// # Arguments
    ///
    /// * `key` - The key name to check
    ///
    /// # Returns
    ///
    /// true if the key name is valid, false otherwise
    fn is_valid_key_name(&self, key: &str) -> bool {
        if key.is_empty() {
            return false;
        }
        
        let first_char = key.chars().next().unwrap();
        if !first_char.is_ascii_alphabetic() && first_char != '_' {
            return false;
        }
        
        key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    }
    
    /// Creates a map of definition names to definition references for efficient lookup
    ///
    /// # Arguments
    ///
    /// * `definitions` - The key-value definitions to map
    ///
    /// # Returns
    ///
    /// HashMap mapping definition names to definition references
    fn create_definition_map<'a>(
        &self,
        definitions: &[&'a KeyValueDefinition],
    ) -> HashMap<String, &'a KeyValueDefinition> {
        let mut map = HashMap::with_capacity(definitions.len());
        
        for def in definitions {
            let key = if self.config.case_sensitive {
                def.name().to_string()
            } else {
                def.name().to_lowercase()
            };
            
            map.insert(key, *def);
        }
        
        map
    }
    
    /// Checks if a format is allowed for multi key-value pairs
    ///
    /// # Arguments
    ///
    /// * `format` - The format to check
    ///
    /// # Returns
    ///
    /// true if the format is allowed, false otherwise
    fn is_format_allowed_for_multi_kv(&self, format: AllowedKeyValueFormats) -> bool {
        self.config.multi_kv_formats.iter().any(|&f| {
            f == format || f == AllowedKeyValueFormats::KeyAll
        })
    }
    
    /// Validates that key-value dependencies are satisfied
    ///
    /// # Arguments
    ///
    /// * `found_keys` - Set of keys found during processing
    /// * `definitions` - The key-value definitions to check
    ///
    /// # Returns
    ///
    /// Ok(()) if validation passes, or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if a dependency is not met
    pub fn validate_dependencies(
        &self,
        found_keys: &HashSet<String>,
        definitions: &[&KeyValueDefinition],
    ) -> Result<(), Error> {
        for def in definitions {
            if found_keys.contains(def.name()) {
                // Check dependencies
                for dependency in def.dependencies() {
                    if !found_keys.contains(dependency) {
                        return Err(Error::DependencyNotMet(
                            def.name().to_string(),
                            dependency.to_string(),
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Validates that key-value exclusions are respected
    ///
    /// # Arguments
    ///
    /// * `found_keys` - Set of keys found during processing
    /// * `definitions` - The key-value definitions to check
    ///
    /// # Returns
    ///
    /// Ok(()) if validation passes, or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if mutually exclusive keys are found
    pub fn validate_exclusions(
        &self,
        found_keys: &HashSet<String>,
        definitions: &[&KeyValueDefinition],
    ) -> Result<(), Error> {
        for def in definitions {
            if found_keys.contains(def.name()) {
                // Check exclusions
                for exclusion in def.exclusions() {
                    if found_keys.contains(exclusion) {
                        return Err(Error::MutuallyExclusiveArgs(
                            def.name().to_string(),
                            exclusion.to_string(),
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Validates that required key-value pairs are present
    ///
    /// # Arguments
    ///
    /// * `found_keys` - Set of keys found during processing
    /// * `definitions` - The key-value definitions to check
    ///
    /// # Returns
    ///
    /// Ok(()) if validation passes, or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if a required key is missing
    pub fn validate_required(
        &self,
        found_keys: &HashSet<String>,
        definitions: &[&KeyValueDefinition],
    ) -> Result<(), Error> {
        for def in definitions {
            if def.is_required() && !found_keys.contains(def.name()) {
                return Err(Error::RequiredArgMissing(def.name().to_string()));
            }
        }
        
        Ok(())
    }
    
    /// Checks if a token is likely to be a key-value pair (for pre-filtering)
    ///
    /// # Arguments
    ///
    /// * `token` - The token to check
    ///
    /// # Returns
    ///
    /// true if the token appears to be a key-value pair, false otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_parser::KeyValueArgumentProcessor;
    ///
    /// let processor = KeyValueArgumentProcessor::new();
    ///
    /// assert!(processor.is_key_value_like("USER=admin"));
    /// assert!(processor.is_key_value_like("DEBUG"));  // Could be KEY format
    /// assert!(!processor.is_key_value_like("Some text"));
    /// ```
    pub fn is_key_value_like(&self, token: &str) -> bool {
        // Contains equals sign or could be a valid key name
        token.contains('=') || self.is_valid_key_name(token)
    }
}
```

### Implementation Approach

#### 1. Key-Value Identification Strategy
The key-value processor uses a straightforward but effective approach for identifying key-value pairs in token streams:

```rust
pub fn process(
    &self,
    tokens: &[String],
    definitions: Vec<&KeyValueDefinition>,
) -> Result<KeyValueProcessResult, Error> {
    let mut defined_pairs = HashMap::new();
    let mut multi_store = DefaultKeyValueStore::new(self.config.case_sensitive);
    let mut remaining_tokens = Vec::new();
    
    // Create a lookup map for faster definition checking
    let def_map: HashMap<String, &KeyValueDefinition> = self.create_definition_map(&definitions);
    
    // Process each token
    for token in tokens {
        // Parse the token into a key-value pair
        let parsed = self.parse_token(token);
        
        match parsed.format {
            DetectedFormat::NotKeyValue => {
                // Not a key-value pair, add to remaining tokens
                remaining_tokens.push(token.clone());
            },
            _ => {
                // Handle as key-value pair based on whether it's defined or not
                // ...
            }
        }
    }
    
    // Return the processed result
    Ok(KeyValueProcessResult {
        defined_pairs,
        multi_store,
        remaining_tokens,
    })
}
```

This approach:
- Creates a hash map for O(1) definition lookup
- Processes tokens in a single pass
- Handles case sensitivity through normalization
- Categorizes tokens into defined pairs, multi-key-value pairs, or remaining tokens
- Supports various key-value formats

#### 2. Token Parsing Logic
The processor uses a dedicated method to parse tokens into key-value pairs:

```rust
fn parse_token(&self, token: &str) -> ParsedKeyValue {
    // Check if the token contains an equals sign
    if let Some(pos) = token.find('=') {
        let (key, value_part) = token.split_at(pos);
        
        // Skip the equals sign
        let value = &value_part[1..];
        
        // Return as KeyValue or KeyEquals based on value presence
        if value.is_empty() {
            ParsedKeyValue {
                key: key.to_string(),
                value: Some(String::new()),
                format: DetectedFormat::KeyEquals,
            }
        } else {
            ParsedKeyValue {
                key: key.to_string(),
                value: Some(value.to_string()),
                format: DetectedFormat::KeyValue,
            }
        }
    } else {
        // No equals sign, check if it could be a KeyOnly format
        if self.is_valid_key_name(token) {
            ParsedKeyValue {
                key: token.to_string(),
                value: None,
                format: DetectedFormat::KeyOnly,
            }
        } else {
            // Not a key-value pair
            ParsedKeyValue {
                key: token.to_string(),
                value: None,
                format: DetectedFormat::NotKeyValue,
            }
        }
    }
}
```

This implementation:
- Efficiently detects different key-value formats
- Extracts keys and values accurately
- Validates key names for KEY format
- Distinguishes between KEY=VALUE, KEY=, KEY, and not-a-key-value formats

#### 3. Type Conversion and Binding
The processor integrates with the key-value definition's type conversion and binding capabilities:

```rust
// If this is a defined key-value pair with a value and a type converter,
// convert the value and update the binding
if let Some(value) = &parsed.value {
    if definition.has_type_converter() {
        let converted = definition.convert_value(value)?;
        
        if definition.has_binding() {
            definition.update_binding(converted);
        }
    }
}
```

This approach:
- Leverages the definition's type conversion logic
- Applies field binding updates when values are present
- Handles different value formats appropriately
- Propagates conversion errors with context
- Respects the definition's binding configuration

#### 4. Multi Key-Value Support
The processor supports undefined key-value pairs through a multi-key-value store:

```rust
if self.config.enable_multi_kv {
    // Check if this format is allowed for multi key-value pairs
    let format = match parsed.format {
        DetectedFormat::KeyValue => AllowedKeyValueFormats::KeyValue,
        DetectedFormat::KeyEquals => AllowedKeyValueFormats::KeyEquals,
        DetectedFormat::KeyOnly => AllowedKeyValueFormats::KeyOnly,
        _ => unreachable!(), // Already handled NotKeyValue above
    };
    
    if self.is_format_allowed_for_multi_kv(format) {
        // Add to multi key-value store
        multi_store.add(&parsed.key, parsed.value.as_deref());
    } else {
        // Format not allowed for multi key-value pairs, add to remaining tokens
        remaining_tokens.push(token.clone());
    }
}
```

This implementation:
- Supports configurable multi-key-value handling
- Validates formats against allowed multi-key-value formats
- Preserves tokens when format rules are violated
- Integrates with the key-value store component
- Provides a flexible fallback for undefined keys

#### 5. Validation Methods
The processor provides dedicated methods for validating dependencies, exclusions, and required keys:

```rust
pub fn validate_dependencies(
    &self,
    found_keys: &HashSet<String>,
    definitions: &[&KeyValueDefinition],
) -> Result<(), Error> {
    for def in definitions {
        if found_keys.contains(def.name()) {
            // Check dependencies
            for dependency in def.dependencies() {
                if !found_keys.contains(dependency) {
                    return Err(Error::DependencyNotMet(
                        def.name().to_string(),
                        dependency.to_string(),
                    ));
                }
            }
        }
    }
    
    Ok(())
}
```

These methods:
- Provide separate validation phases for different constraints
- Generate specific error types with context
- Operate efficiently on sets of found keys
- Support validation of complex relationships
- Return early with actionable error information

#### 6. Configuration and Logging
The processor supports flexible configuration and optional logging:

```rust
pub(crate) struct KeyValueProcessorConfig {
    pub case_sensitive: bool,
    pub enable_multi_kv: bool,
    pub multi_kv_formats: Vec<AllowedKeyValueFormats>,
}

impl KeyValueArgumentProcessor {
    pub fn with_config(config: KeyValueProcessorConfig) -> Self {
        Self {
            config,
            logger: None,
        }
    }
    
    pub fn with_logging(mut self, logger: LogComponent) -> Self {
        self.logger = Some(logger);
        self
    }
}
```

This approach:
- Provides explicit configuration options
- Defaults to secure, predictable behavior
- Keeps logging optional for performance
- Enables fine-tuning of processor behavior
- Supports a fluent interface for configuration

## Integration

### Integration with Other Components

The Key-Value Parsing Logic integrates with other components as follows:

1. **Input Tokenizer**: Receives tokenized input from the tokenizer component
2. **Key-Value Definition System**: Uses definitions to guide parsing and validation
3. **Format & Type Conversion**: Applies format detection and type conversion
4. **Storage & Access**: Stores parsed key-value pairs for retrieval
5. **Parser Pipeline**: Forms a key stage in the multi-stage parsing pipeline
6. **Validation System**: Provides validation methods for key-value constraints
7. **Field Binding**: Triggers field binding updates for parsed key-value pairs
8. **Logging System**: Integrates with the library's logging for diagnostics
9. **Error System**: Uses the library's error types for validation failures

### Usage Examples

```rust
use pam_args_rs::parser::kv_parser::{KeyValueArgumentProcessor, KeyValueProcessorConfig};
use pam_args_rs::parser::kv_def::{KeyValueDefinition, KeyValueDefinitionManager};
use pam_args_rs::AllowedKeyValueFormats;
use pam_args_rs::error::Result;
use std::str::FromStr;
use std::collections::HashSet;

fn process_arguments(tokens: &[String]) -> Result<()> {
    // Create a key-value definition manager
    let mut manager = KeyValueDefinitionManager::new(true);
    
    // Add definitions
    manager.add_definition(
        KeyValueDefinition::new("USER", "Username")
            .type_converter(String::from_str)
            .required()
    )?;
    
    manager.add_definition(
        KeyValueDefinition::new("PORT", "Port number")
            .type_converter(i32::from_str)
            .depends_on("HOST")
    )?;
    
    manager.add_definition(
        KeyValueDefinition::new("HOST", "Hostname")
            .type_converter(String::from_str)
    )?;
    
    // Create a key-value processor with multi-kv support
    let processor = KeyValueArgumentProcessor::with_config(
        KeyValueProcessorConfig {
            case_sensitive: false,
            enable_multi_kv: true,
            multi_kv_formats: vec![AllowedKeyValueFormats::KeyValue, AllowedKeyValueFormats::KeyEquals],
        }
    );
    
    // Process tokens to identify key-value pairs
    let result = processor.process(tokens, manager.get_all_definitions())?;
    
    // Get all found keys (both defined and multi-kv)
    let mut found_keys = HashSet::new();
    for key in result.defined_pairs.keys() {
        found_keys.insert(key.clone());
    }
    for key in result.multi_store.keys() {
        found_keys.insert(key.to_string());
    }
    
    // Validate constraints
    processor.validate_required(&found_keys, &manager.get_all_definitions())?;
    processor.validate_dependencies(&found_keys, &manager.get_all_definitions())?;
    processor.validate_exclusions(&found_keys, &manager.get_all_definitions())?;
    
    // Print found key-value pairs
    println!("Defined key-value pairs:");
    for (key, value) in &result.defined_pairs {
        println!("  - {}: {:?}", key, value);
    }
    
    // Print multi key-value pairs
    println!("Multi key-value pairs:");
    for key in result.multi_store.keys() {
        let value = result.multi_store.get(key);
        println!("  - {}: {:?}", key, value);
    }
    
    // Print remaining tokens
    println!("Remaining tokens:");
    for token in &result.remaining_tokens {
        println!("  - {}", token);
    }
    
    Ok(())
}

// Example with logging
fn process_with_logging(tokens: &[String], definitions: Vec<&KeyValueDefinition>) -> Result<()> {
    // Initialize logging
    let logger = LogComponent::Parser;
    
    // Create a processor with logging
    let processor = KeyValueArgumentProcessor::new()
        .with_logging(logger);
    
    // Process with detailed logging
    let result = processor.process(tokens, definitions)?;
    
    // Collect found keys
    let mut found_keys = HashSet::new();
    for key in result.defined_pairs.keys() {
        found_keys.insert(key.clone());
    }
    
    // Validate with logging
    processor.validate_required(&found_keys, &definitions)?;
    processor.validate_dependencies(&found_keys, &definitions)?;
    processor.validate_exclusions(&found_keys, &definitions)?;
    
    Ok(())
}

// Example with pre-filtering optimization
fn process_optimized(tokens: &[String], definitions: Vec<&KeyValueDefinition>) -> Result<()> {
    let processor = KeyValueArgumentProcessor::new();
    
    // Pre-filter to avoid unnecessary parsing
    let (potential_kv_pairs, definitely_not_kv): (Vec<_>, Vec<_>) = tokens
        .iter()
        .partition(|token| processor.is_key_value_like(token));
    
    // Process only potential key-value pairs
    let result = processor.process(&potential_kv_pairs, definitions)?;
    
    // Combine remaining tokens with definitely-not-kv
    let all_remaining = [
        result.remaining_tokens.as_slice(),
        definitely_not_kv.as_slice()
    ].concat();
    
    // Continue processing with all_remaining
    println!("Found {} defined key-value pairs", result.defined_pairs.len());
    println!("Found {} multi key-value pairs", result.multi_store.len());
    println!("Remaining tokens: {}", all_remaining.len());
    
    Ok(())
}
```

## Testing Strategy

### Unit Test Cases

| #  | Category                 | Input                                                          | Expected Output                                                                  | Notes                          |
|----|--------------------------|----------------------------------------------------------------|----------------------------------------------------------------------------------|--------------------------------|
| 1  | Basic Key-Value          | `["USER=admin"]` with USER definition                          | `defined_pairs={"USER": Some("admin")}, remaining=[]`                            | Test basic key-value parsing   |
| 2  | Empty Value              | `["USER="]` with USER definition                               | `defined_pairs={"USER": Some("")}, remaining=[]`                                 | Test KEY= format               |
| 3  | Key-Only                 | `["DEBUG"]` with DEBUG definition supporting KeyOnly           | `defined_pairs={"DEBUG": None}, remaining=[]`                                    | Test KEY format                |
| 4  | No Match                 | `["XYZ=value"]` with no matching definition                    | `defined_pairs={}, remaining=["XYZ=value"]` or multi-store                       | Test non-matching key-value    |
| 5  | Mixed Input              | `["USER=admin", "DEBUG"]` with USER definition                 | `defined_pairs={"USER": Some("admin")}, remaining=["DEBUG"]`                     | Test mixed input               |
| 6  | Multiple Pairs           | `["USER=admin", "HOST=localhost"]` with both definitions       | `defined_pairs={"USER": Some("admin"), "HOST": Some("localhost")}, remaining=[]` | Test multiple key-value pairs  |
| 7  | Case Sensitivity True    | `["user=admin"]` with USER definition and case_sensitive=true  | `defined_pairs={}, remaining=["user=admin"]`                                     | Test case-sensitive matching   |
| 8  | Case Sensitivity False   | `["user=admin"]` with USER definition and case_sensitive=false | `defined_pairs={"USER": Some("admin")}, remaining=[]`                            | Test case-insensitive matching |
| 9  | Type Conversion          | `["PORT=8080"]` with PORT:i32 definition                       | i32 conversion succeeds                                                          | Test type conversion           |
| 10 | Type Conversion Error    | `["PORT=invalid"]` with PORT:i32 definition                    | Error: InvalidInput                                                              | Test conversion error          |
| 11 | Binding Update           | `["USER=admin"]` with bound USER definition                    | Field binding updated                                                            | Test binding update            |
| 12 | Multi-KV Enabled         | `["CUSTOM=value"]` with multi-kv enabled                       | Added to multi-store                                                             | Test multi-kv handling         |
| 13 | Multi-KV Disabled        | `["CUSTOM=value"]` with multi-kv disabled                      | Added to remaining tokens                                                        | Test multi-kv disabled         |
| 14 | Format Validation        | `["DEBUG"]` with DEBUG definition not supporting KeyOnly       | Error: InvalidKeyValue                                                           | Test format validation         |
| 15 | Required Validation      | Missing required key                                           | Error: RequiredArgMissing                                                        | Test required validation       |
| 16 | Dependency Validation    | Key without required dependency                                | Error: DependencyNotMet                                                          | Test dependency validation     |
| 17 | Exclusion Validation     | Mutually exclusive keys                                        | Error: MutuallyExclusiveArgs                                                     | Test exclusion validation      |
| 18 | Allowed Values           | Valid allowed value                                            | Parsing succeeds                                                                 | Test allowed values validation |
| 19 | Invalid Value            | Invalid allowed value                                          | Error: InvalidValue                                                              | Test invalid value rejection   |
| 20 | Definition Map           | Multiple definitions                                           | Map with all definitions                                                         | Test definition map creation   |
| 21 | Key-Value-Like Detection | Various inputs                                                 | Correct classification                                                           | Test key-value detection       |
| 22 | Valid Key Name           | Valid and invalid key names                                    | Correct validation                                                               | Test key name validation       |
| 23 | Empty Input              | `[]` with any definitions                                      | All empty results                                                                | Test empty input handling      |
| 24 | Parser Integration       | Complex tokenized input                                        | Correct parsing result                                                           | Test with tokenizer output     |
| 25 | Logger Integration       | With logger enabled                                            | Proper logging calls                                                             | Test logging integration       |
| 26 | No Logger                | With logger disabled                                           | No logging calls                                                                 | Test no-logging case           |
| 27 | Multi-KV Formats         | Various formats with different allowed formats                 | Correct filtering                                                                | Test multi-kv format filtering |
| 28 | Performance              | Large input set                                                | Fast processing                                                                  | Test processing performance    |
| 29 | Memory Usage             | Large input set                                                | Minimal allocations                                                              | Test memory efficiency         |
| 30 | End-to-End Process       | Complete processing pipeline                                   | Correct end result                                                               | Test full processing flow      |

### Integration Tests

The Key-Value Parsing Logic should be tested in integration with other components to ensure correct end-to-end behavior. Integration tests should focus on:

1. **Tokenizer Integration**
   - Test processing of tokenized input from the tokenizer
   - Verify correct handling of different token formats
   - Test integration with bracketed content processing
   - Verify preservation of token semantics across components

2. **Key-Value Definition Integration**
   - Test integration with the key-value definition system
   - Verify proper constraint application
   - Test type conversion across components
   - Verify binding updates in complex scenarios

3. **Parser Pipeline Integration**
   - Test integration within the multi-stage parsing pipeline
   - Verify correct handling of result passing between stages
   - Test error propagation through the pipeline
   - Verify consistent behavior in the full pipeline

4. **Storage Integration**
   - Test integration with key-value storage
   - Verify correct population of defined and multi-kv stores
   - Test access patterns after parsing
   - Verify type-safe retrieval of parsed values

### Testing Focus Areas

1. **Format Handling**
   - Test all key-value formats (KEY=VALUE, KEY=, KEY)
   - Verify correct format detection
   - Test format validation against definitions
   - Verify proper handling of format constraints

2. **Type Conversion**
   - Test conversion for all supported types
   - Verify error handling for conversion failures
   - Test complex type conversions
   - Verify binding updates after conversion

3. **Multi-KV Handling**
   - Test with multi-kv enabled and disabled
   - Verify format filtering for multi-kv pairs
   - Test access to multi-kv store after parsing
   - Verify proper integration with storage component

4. **Validation Logic**
   - Test all validation methods
   - Verify correct error generation
   - Test complex validation scenarios
   - Verify validation with mixed input types

5. **Performance Characteristics**
   - Test parsing speed with various inputs
   - Verify memory usage patterns
   - Test scaling with large input sets
   - Verify efficient handling of complex inputs

## Performance Considerations

### Memory Efficiency
- Minimize string copying during key-value parsing
- Use HashMap with capacity hints for efficient storage
- Share references when possible to avoid duplication
- Process tokens in-place without excessive copying
- Reuse allocations for improved performance

### Parsing Strategy
- Single-pass processing for efficient parsing
- Early detection of non-key-value tokens
- Avoid unnecessary format conversions
- Use efficient string operations for key-value extraction
- Apply pre-filtering for large token sets

### Type Conversion Optimization
- Defer conversion until needed
- Use static dispatch for type converters
- Cache conversion results when appropriate
- Minimize allocations during conversion
- Use zero-copy approaches where possible

### Validation Efficiency
- Use HashSet for O(1) key lookup during validation
- Validate only what's necessary based on found keys
- Return early from validation when errors are found
- Process validations in optimal order
- Share validation results between validation phases

### Configuration Performance
- Use default configuration for common cases
- Apply configuration at initialization, not during processing
- Optimize lookup strategy based on case sensitivity setting
- Use boolean flags for fast conditional paths
- Precompute format compatibility for multi-kv handling