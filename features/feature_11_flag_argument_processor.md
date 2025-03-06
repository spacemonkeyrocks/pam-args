# Feature 11: Flag Argument Processor

## Module Type
**Internal**: This component is an internal implementation detail of the parser subsystem that identifies and processes flag arguments from tokenized input. It is not exposed directly as part of the public API but is a critical piece of the parsing pipeline.

## Feature Information

**Feature Name**: Flag Argument Processor

**Description**: Process explicitly defined flags from tokenized input. This component is responsible for identifying which tokenized arguments match defined flag patterns, validating them according to the flag definitions, and preparing them for further processing. It ensures that flags are correctly recognized regardless of their position in the argument list and integrates with the validation system to apply flag-specific constraints.

**Priority**: High

**Dependencies**: 
- [Feature 1: Result & Error Types](error-result-types.md)
- [Feature 2: Utility Functions](utility-functions.md)
- [Feature 5: Core Argument Types](core-argument-types.md)
- [Feature 9: Input Tokenizer and Syntax Validator](tokenizer.md)

## Requirements

### Functional Requirements
1. Identify and extract flag arguments from tokenized input
2. Match input tokens against defined flag patterns
3. Support case-sensitive and case-insensitive flag matching
4. Properly handle multiple flags in any order
5. Track which flags have been set during parsing
6. Apply flag bindings to update bound fields when flags are found
7. Preserve unprocessed tokens for subsequent processing stages
8. Ensure consistent handling regardless of flag position in input
9. Generate appropriate errors for flag validation failures
10. Support logging of flag processing for diagnostic purposes

### API Requirements
- Provide a clean, focused interface for flag processing
- Integrate seamlessly with the input tokenizer and subsequent parser stages
- Return clear results indicating which flags were found and which tokens remain
- Support efficient flag lookup by name
- Enable integration with validation components
- Facilitate binding updates for identified flags

### Performance Requirements
- Optimize for fast flag identification
- Minimize allocations during flag processing
- Process large numbers of flags efficiently
- Ensure O(n) or better performance for flag matching
- Support efficient lookup of flag definitions

## Design

### Data Structures
```rust
/// Result of flag argument processing
#[derive(Debug)]
pub(crate) struct FlagProcessResult {
    /// Set of flags that were found in the input
    pub found_flags: HashSet<String>,
    
    /// Tokens that were not processed as flags
    pub remaining_tokens: Vec<String>,
}

/// Configuration for the flag processor
#[derive(Debug, Clone)]
pub(crate) struct FlagProcessorConfig {
    /// Whether flags are case-sensitive
    pub case_sensitive: bool,
}

impl Default for FlagProcessorConfig {
    fn default() -> Self {
        Self {
            case_sensitive: true,
        }
    }
}
```

### Function Signatures
```rust
/// Main processor for flag arguments
pub(crate) struct FlagArgumentProcessor {
    /// Configuration for the processor
    config: FlagProcessorConfig,
    
    /// Logging component for diagnostic purposes
    logger: Option<LogComponent>,
}

impl FlagArgumentProcessor {
    /// Creates a new flag argument processor with default configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::flag::FlagArgumentProcessor;
    ///
    /// let processor = FlagArgumentProcessor::new();
    /// ```
    pub fn new() -> Self {
        Self {
            config: FlagProcessorConfig::default(),
            logger: None,
        }
    }
    
    /// Creates a new flag argument processor with custom configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the processor
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::flag::{FlagArgumentProcessor, FlagProcessorConfig};
    ///
    /// let config = FlagProcessorConfig {
    ///     case_sensitive: false,
    /// };
    /// let processor = FlagArgumentProcessor::with_config(config);
    /// ```
    pub fn with_config(config: FlagProcessorConfig) -> Self {
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
    
    /// Processes tokens to identify flags and update bindings
    ///
    /// # Arguments
    ///
    /// * `tokens` - The tokens to process
    /// * `flags` - The flag definitions to match against
    ///
    /// # Returns
    ///
    /// Result containing the processing result or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if flag validation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::flag::FlagArgumentProcessor;
    /// use pam_args_rs::Flag;
    ///
    /// let processor = FlagArgumentProcessor::new();
    /// let tokens = vec!["DEBUG".to_string(), "USER=admin".to_string()];
    /// let flags = vec![Flag::new("DEBUG", "Enable debug mode")];
    ///
    /// let result = processor.process(&tokens, &flags)?;
    /// assert!(result.found_flags.contains("DEBUG"));
    /// assert_eq!(result.remaining_tokens, vec!["USER=admin"]);
    /// # Ok::<(), pam_args_rs::Error>(())
    /// ```
    pub fn process(
        &self,
        tokens: &[String],
        flags: &[Flag],
    ) -> Result<FlagProcessResult, Error> {
        let mut found_flags = HashSet::new();
        let mut remaining_tokens = Vec::new();
        
        // Create a lookup map for faster flag checking
        let flag_map: HashMap<String, &Flag> = self.create_flag_map(flags);
        
        // Log processing start if logger is enabled
        if let Some(logger) = &self.logger {
            log_debug!(
                logger,
                LogOperation::Parse,
                &format!("Processing {} tokens for flags", tokens.len()),
                tokens
            );
        }
        
        // Process each token
        for token in tokens {
            // Check if the token is a flag
            let normalized_token = if self.config.case_sensitive {
                token.clone()
            } else {
                token.to_lowercase()
            };
            
            if let Some(flag) = flag_map.get(&normalized_token) {
                // Log flag found if logger is enabled
                if let Some(logger) = &self.logger {
                    log_debug!(
                        logger,
                        LogOperation::Parse,
                        &format!("Found flag: {}", flag.name()),
                    );
                }
                
                // Add to found flags
                found_flags.insert(flag.name().to_string());
                
                // Update binding if present
                if flag.has_binding() {
                    flag.update_binding(true);
                }
            } else {
                // Not a flag, add to remaining tokens
                remaining_tokens.push(token.clone());
            }
        }
        
        // Return the processed result
        Ok(FlagProcessResult {
            found_flags,
            remaining_tokens,
        })
    }
    
    /// Creates a map of flag names to flag references for efficient lookup
    ///
    /// # Arguments
    ///
    /// * `flags` - The flag definitions to map
    ///
    /// # Returns
    ///
    /// HashMap mapping flag names to flag references
    fn create_flag_map<'a>(&self, flags: &'a [Flag]) -> HashMap<String, &'a Flag> {
        let mut map = HashMap::with_capacity(flags.len());
        
        for flag in flags {
            let key = if self.config.case_sensitive {
                flag.name().to_string()
            } else {
                flag.name().to_lowercase()
            };
            
            map.insert(key, flag);
        }
        
        map
    }
    
    /// Validates that flag exclusions are respected
    ///
    /// # Arguments
    ///
    /// * `found_flags` - Set of flags found during processing
    /// * `flags` - The flag definitions to check
    ///
    /// # Returns
    ///
    /// Ok(()) if validation passes, or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if mutually exclusive flags are found
    pub fn validate_exclusions(
        &self,
        found_flags: &HashSet<String>,
        flags: &[Flag],
    ) -> Result<(), Error> {
        for flag in flags {
            if found_flags.contains(flag.name()) {
                // Check exclusions
                for exclusion in flag.exclusions() {
                    if found_flags.contains(exclusion) {
                        return Err(Error::MutuallyExclusiveArgs(
                            flag.name().to_string(),
                            exclusion.to_string(),
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Validates that flag dependencies are satisfied
    ///
    /// # Arguments
    ///
    /// * `found_flags` - Set of flags found during processing
    /// * `flags` - The flag definitions to check
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
        found_flags: &HashSet<String>,
        flags: &[Flag],
    ) -> Result<(), Error> {
        for flag in flags {
            if found_flags.contains(flag.name()) {
                // Check dependencies
                for dependency in flag.dependencies() {
                    if !found_flags.contains(dependency) {
                        return Err(Error::DependencyNotMet(
                            flag.name().to_string(),
                            dependency.to_string(),
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Checks if a token is likely to be a flag (for pre-filtering)
    ///
    /// # Arguments
    ///
    /// * `token` - The token to check
    ///
    /// # Returns
    ///
    /// true if the token appears to be a flag, false otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::flag::FlagArgumentProcessor;
    ///
    /// let processor = FlagArgumentProcessor::new();
    ///
    /// assert!(processor.is_flag_like("DEBUG"));
    /// assert!(!processor.is_flag_like("USER=admin"));
    /// ```
    pub fn is_flag_like(&self, token: &str) -> bool {
        // A flag-like token doesn't contain equals sign
        // and consists of alphanumeric characters and underscores
        !token.contains('=') && 
            token.chars().all(|c| c.is_alphanumeric() || c == '_')
    }
}
```

### Implementation Approach

#### 1. Flag Identification Strategy
The flag processor uses a straightforward but efficient approach for identifying flags in token streams:

```rust
pub fn process(
    &self,
    tokens: &[String],
    flags: &[Flag],
) -> Result<FlagProcessResult, Error> {
    let mut found_flags = HashSet::new();
    let mut remaining_tokens = Vec::new();
    
    // Create a lookup map for faster flag checking
    let flag_map: HashMap<String, &Flag> = self.create_flag_map(flags);
    
    // Process each token
    for token in tokens {
        // Check if the token is a flag
        let normalized_token = if self.config.case_sensitive {
            token.clone()
        } else {
            token.to_lowercase()
        };
        
        if let Some(flag) = flag_map.get(&normalized_token) {
            // Found a flag
            found_flags.insert(flag.name().to_string());
            
            // Update binding if present
            if flag.has_binding() {
                flag.update_binding(true);
            }
        } else {
            // Not a flag, add to remaining tokens
            remaining_tokens.push(token.clone());
        }
    }
    
    // Return the processed result
    Ok(FlagProcessResult {
        found_flags,
        remaining_tokens,
    })
}
```

This approach:
- Creates a hash map for O(1) flag lookup
- Processes tokens in a single pass
- Handles case sensitivity through normalization
- Updates bindings when flags are found
- Preserves unprocessed tokens for later stages
- Tracks found flags in a set for efficient validation

#### 2. Flag Map Creation
For efficient lookup, the processor creates a map of flag names to flag references:

```rust
fn create_flag_map<'a>(&self, flags: &'a [Flag]) -> HashMap<String, &'a Flag> {
    let mut map = HashMap::with_capacity(flags.len());
    
    for flag in flags {
        let key = if self.config.case_sensitive {
            flag.name().to_string()
        } else {
            flag.name().to_lowercase()
        };
        
        map.insert(key, flag);
    }
    
    map
}
```

This implementation:
- Preallocates the hash map to the number of flags
- Normalizes keys based on case sensitivity configuration
- Uses references to avoid copying flag objects
- Applies normalization only once per flag
- Provides O(1) lookup for the processing phase

#### 3. Flag Validation
The processor provides separate methods for validating exclusions and dependencies:

```rust
pub fn validate_exclusions(
    &self,
    found_flags: &HashSet<String>,
    flags: &[Flag],
) -> Result<(), Error> {
    for flag in flags {
        if found_flags.contains(flag.name()) {
            // Check exclusions
            for exclusion in flag.exclusions() {
                if found_flags.contains(exclusion) {
                    return Err(Error::MutuallyExclusiveArgs(
                        flag.name().to_string(),
                        exclusion.to_string(),
                    ));
                }
            }
        }
    }
    
    Ok(())
}

pub fn validate_dependencies(
    &self,
    found_flags: &HashSet<String>,
    flags: &[Flag],
) -> Result<(), Error> {
    for flag in flags {
        if found_flags.contains(flag.name()) {
            // Check dependencies
            for dependency in flag.dependencies() {
                if !found_flags.contains(dependency) {
                    return Err(Error::DependencyNotMet(
                        flag.name().to_string(),
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
- Check only relevant constraints for found flags
- Return early with detailed error information
- Provide distinct validation phases that can be applied separately
- Use set lookups for efficient checking
- Generate appropriate error types for validation failures

#### 4. Configuration and Logging Support
The processor provides configuration options and optional logging:

```rust
pub struct FlagProcessorConfig {
    pub case_sensitive: bool,
}

impl FlagArgumentProcessor {
    pub fn with_config(config: FlagProcessorConfig) -> Self {
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
- Makes configuration explicit and customizable
- Provides sensible defaults
- Keeps logging optional for performance
- Uses a builder pattern for clean configuration
- Integrates with the library's logging system

#### 5. Result Structure
The processor returns a clear result structure:

```rust
pub(crate) struct FlagProcessResult {
    pub found_flags: HashSet<String>,
    pub remaining_tokens: Vec<String>,
}
```

This design:
- Clearly separates processed and unprocessed tokens
- Uses a set for efficient flag lookup in validation phases
- Preserves the order of remaining tokens
- Makes the result easy to use in subsequent processing stages
- Avoids unnecessary copying of token content

## Integration

### Integration with Other Components

The Flag Argument Processor integrates with other components as follows:

1. **Input Tokenizer**: Receives tokenized input from the tokenizer component
2. **Core Argument Types**: Works with Flag definitions from core argument types
3. **Parser Pipeline**: Forms part of the multi-stage parsing pipeline
4. **Validation System**: Provides validation methods for flag constraints
5. **Logging System**: Integrates with the library's logging for diagnostics
6. **Field Binding**: Triggers field binding updates for found flags
7. **Error System**: Uses the library's error types for validation failures

### Usage Examples

```rust
use pam_args_rs::parser::flag::{FlagArgumentProcessor, FlagProcessorConfig};
use pam_args_rs::Flag;
use pam_args_rs::error::Result;

fn process_arguments(tokens: &[String], flags: &[Flag]) -> Result<()> {
    // Create a case-insensitive flag processor
    let processor = FlagArgumentProcessor::with_config(
        FlagProcessorConfig {
            case_sensitive: false,
        }
    );
    
    // Process tokens to identify flags
    let result = processor.process(tokens, flags)?;
    
    // Validate flag constraints
    processor.validate_exclusions(&result.found_flags, flags)?;
    processor.validate_dependencies(&result.found_flags, flags)?;
    
    // Print found flags
    println!("Found flags:");
    for flag in &result.found_flags {
        println!("  - {}", flag);
    }
    
    // Print remaining tokens
    println!("Remaining tokens:");
    for token in &result.remaining_tokens {
        println!("  - {}", token);
    }
    
    Ok(())
}

// Example with logging
fn process_with_logging(tokens: &[String], flags: &[Flag]) -> Result<()> {
    // Initialize logging
    let logger = LogComponent::Parser;
    
    // Create a processor with logging
    let processor = FlagArgumentProcessor::new()
        .with_logging(logger);
    
    // Process with detailed logging
    let result = processor.process(tokens, flags)?;
    
    // Validation with logging
    processor.validate_exclusions(&result.found_flags, flags)?;
    processor.validate_dependencies(&result.found_flags, flags)?;
    
    Ok(())
}

// Example with early filtering optimization
fn process_optimized(tokens: &[String], flags: &[Flag]) -> Result<()> {
    let processor = FlagArgumentProcessor::new();
    
    // Pre-filter to avoid unnecessary hash lookups
    let (potential_flags, definitely_not_flags): (Vec<_>, Vec<_>) = tokens
        .iter()
        .partition(|token| processor.is_flag_like(token));
    
    // Process only potential flags
    let result = processor.process(&potential_flags, flags)?;
    
    // Combine remaining tokens with definitely-not-flags
    let all_remaining = [
        result.remaining_tokens.as_slice(),
        definitely_not_flags.as_slice()
    ].concat();
    
    // Continue processing with all_remaining
    println!("Found {} flags", result.found_flags.len());
    println!("Remaining tokens: {}", all_remaining.len());
    
    Ok(())
}
```

## Testing Strategy

### Unit Test Cases

| #  | Category                      | Input                                                  | Expected Output                                   | Notes                                   |
|----|-------------------------------|--------------------------------------------------------|---------------------------------------------------|-----------------------------------------|
| 1  | Basic Flag Match              | `["DEBUG"]` with `DEBUG` flag                          | `found_flags={"DEBUG"}, remaining=[]`             | Test basic flag matching                |
| 2  | No Match                      | `["XYZ"]` with `DEBUG` flag                            | `found_flags={}, remaining=["XYZ"]`               | Test non-matching flag                  |
| 3  | Mixed Input                   | `["DEBUG", "USER=admin"]` with `DEBUG` flag            | `found_flags={"DEBUG"}, remaining=["USER=admin"]` | Test mixed flag and non-flag input      |
| 4  | Multiple Flags                | `["DEBUG", "VERBOSE"]` with both flags                 | `found_flags={"DEBUG", "VERBOSE"}, remaining=[]`  | Test multiple flag matching             |
| 5  | Case Sensitivity True         | `["debug"]` with `DEBUG` flag and case_sensitive=true  | `found_flags={}, remaining=["debug"]`             | Test case-sensitive matching            |
| 6  | Case Sensitivity False        | `["debug"]` with `DEBUG` flag and case_sensitive=false | `found_flags={"DEBUG"}, remaining=[]`             | Test case-insensitive matching          |
| 7  | Empty Input                   | `[]` with any flags                                    | `found_flags={}, remaining=[]`                    | Test empty input handling               |
| 8  | Binding Update                | `["DEBUG"]` with bound flag                            | Flag binding updated                              | Test binding update mechanism           |
| 9  | No Binding                    | `["DEBUG"]` with unbound flag                          | No binding update                                 | Test no-binding case                    |
| 10 | Flag Map Creation             | Multiple flags                                         | Map with all flags                                | Test flag map creation                  |
| 11 | Exclusion Validation Success  | Non-exclusive flags                                    | No error                                          | Test successful exclusion validation    |
| 12 | Exclusion Validation Failure  | Mutually exclusive flags                               | Error: MutuallyExclusiveArgs                      | Test exclusion validation failure       |
| 13 | Dependency Validation Success | Flags with satisfied dependencies                      | No error                                          | Test successful dependency validation   |
| 14 | Dependency Validation Failure | Flag with unsatisfied dependency                       | Error: DependencyNotMet                           | Test dependency validation failure      |
| 15 | Flag-Like Detection True      | `"FLAG"`                                               | `true`                                            | Test flag-like token detection          |
| 16 | Flag-Like Detection False     | `"KEY=value"`                                          | `false`                                           | Test non-flag-like token detection      |
| 17 | Flag Order Independence       | Flags in different order                               | Same found flags                                  | Test order independence                 |
| 18 | Multiple Identical Flags      | `["DEBUG", "DEBUG"]`                                   | `found_flags={"DEBUG"}`                           | Test duplicate flag handling            |
| 19 | Large Flag Set                | Many flags defined                                     | Correct matches                                   | Test performance with large flag set    |
| 20 | Flag With Special Chars       | `["FLAG_NAME_123"]`                                    | Correctly matched                                 | Test flags with underscores and numbers |
| 21 | Mixed Case Flags              | `["Debug", "VERBOSE"]` with case_sensitive=false       | Both flags found                                  | Test mixed case matching                |
| 22 | Complex Flag Names            | Various complex names                                  | Correct matching                                  | Test complex flag name handling         |
| 23 | Binding Types                 | Different binding types                                | All bindings updated                              | Test different binding mechanisms       |
| 24 | Logger Integration            | With logger enabled                                    | Proper logging calls                              | Test logging integration                |
| 25 | No Logger                     | With logger disabled                                   | No logging calls                                  | Test no-logging case                    |
| 26 | Performance                   | Large input set                                        | Fast processing                                   | Test processing performance             |
| 27 | Memory Usage                  | Large input set                                        | Minimal allocations                               | Test memory efficiency                  |
| 28 | Different Config Creation     | Various configs                                        | Processor with correct config                     | Test configuration options              |
| 29 | Chained Configuration         | Multiple config methods                                | Correctly configured processor                    | Test builder pattern                    |
| 30 | Integration Flow              | Complete process flow                                  | Correct end-to-end behavior                       | Test full processing flow               |

### Integration Tests

The Flag Argument Processor should be tested in integration with other components to ensure correct end-to-end behavior. Integration tests should focus on:

1. **Tokenizer Integration**
   - Test processing of tokenized input
   - Verify correct handling of different token formats
   - Test integration with bracketed content processing
   - Verify preservation of token semantics across components

2. **Parser Pipeline Integration**
   - Test integration within the multi-stage parsing pipeline
   - Verify correct handling of result passing between stages
   - Test error propagation through the pipeline
   - Verify consistent behavior in the full pipeline

3. **Validation System Integration**
   - Test integration with validation components
   - Verify correct constraint application
   - Test complex validation scenarios
   - Verify error generation and propagation

4. **Field Binding Integration**
   - Test binding updates in the context of full parsing
   - Verify field updates in complex scenarios
   - Test binding with various field types
   - Verify correct behavior with different binding mechanisms

### Testing Focus Areas

1. **Correctness**
   - Verify accurate flag identification
   - Test constraint validation logic
   - Verify proper handling of edge cases
   - Test with various input patterns

2. **Performance**
   - Test processing speed with large input sets
   - Verify efficient handling of many flags
   - Test memory usage patterns
   - Verify scaling behavior

3. **Error Handling**
   - Test all error conditions
   - Verify clear and actionable error messages
   - Test error propagation
   - Verify consistent error reporting

4. **Configuration**
   - Test different configuration options
   - Verify behavior changes with configuration
   - Test interaction of multiple configuration settings
   - Verify defaults work as expected

5. **Binding Mechanism**
   - Test field binding updates
   - Verify correct handling of different binding types
   - Test binding in complex scenarios
   - Verify binding thread safety

## Performance Considerations

### Memory Efficiency
- Use HashSet for efficient flag storage and lookup
- Avoid unnecessary string copies through strategic cloning
- Create flag lookup map only once per processing operation
- Reuse token allocations for remaining tokens
- Minimize allocations during validation

### Lookup Optimization
- Use HashMap for O(1) flag lookup
- Apply case normalization only once during map creation
- Use hash-based lookup for validation
- Perform early filtering of non-flag tokens when possible
- Cache lookup results for repeated validation

### Flag Validation Strategy
- Check only relevant constraints for found flags
- Use set operations for efficient validation
- Return early from validation when errors are found
- Process exclusions and dependencies separately
- Apply validation only when needed

### Processing Pipeline Efficiency
- Single-pass processing for flag identification
- Minimal copying of tokens
- Separate flags from non-flags efficiently
- Preserve remaining tokens without unnecessary operations
- Avoid duplicate work across processing stages

### Configuration Performance
- Use default configuration for common cases
- Initialize configuration once per processor
- Avoid repeated normalization for case sensitivity
- Use boolean flags for fast conditional paths
- Apply configuration at optimal points in processing