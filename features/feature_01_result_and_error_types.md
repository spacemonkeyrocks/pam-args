# Feature 1: Result & Error Types

## Module Type
**Core**: This component is a core part of the public API. Library users will interact with these types when handling errors from the library's functions.

## Feature Information

**Feature Name**: Result & Error Types

**Description**: Defines the library's error enum and result type aliases that will be used throughout the library for error handling. This component establishes a consistent, type-safe approach to error reporting and propagation, leveraging Rust's powerful error handling mechanisms.

**Priority**: High

**Dependencies**: None

## Requirements

### Functional Requirements
1. Define a comprehensive error type that covers all possible failure modes
2. Provide descriptive error variants with context information
3. Implement standard traits for the error type (`Debug`, `Display`, `Error`)
4. Define a result type alias for convenience
5. Support pattern matching on specific error conditions
6. Ensure errors are Send and Sync to allow for threading

### API Requirements
- Provide a clean, user-friendly error API
- Support conversion from other error types where appropriate
- Enable propagation via the `?` operator
- Include specific error details to aid in debugging
- Support serialization of errors when the `serde` feature is enabled

### Performance Requirements
- Minimize overhead of error creation and propagation
- Avoid excessive memory allocations in error paths
- Keep error types lightweight for efficient return through the stack

## Design

### Data Structures
```rust
/// Represents all possible error conditions in the pam_args-rs library
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// A required argument was not provided
    /// Contains the name of the missing argument
    RequiredArgMissing(String),
    
    /// Two mutually exclusive arguments were both provided
    /// Contains the names of the conflicting arguments
    MutuallyExclusiveArgs(String, String),
    
    /// A key-value argument has an invalid format
    /// Contains the problematic key-value string
    InvalidKeyValue(String),
    
    /// An unrecognized argument was encountered
    /// Contains the unrecognized argument string
    UnrecognizedArg(String),
    
    /// Failed to parse an argument value as an integer
    /// Contains the invalid value string
    InvalidIntValue(String),
    
    /// Failed to parse an argument value as a boolean
    /// Contains the invalid value string
    InvalidBoolValue(String),
    
    /// An argument dependency was not satisfied
    /// Contains the argument name and its dependency
    DependencyNotMet(String, String),
    
    /// An argument value is not in the allowed set
    /// Contains the argument name and the invalid value
    InvalidValue(String, String),
    
    /// An argument name is defined more than once
    /// Contains the duplicated argument name
    DuplicateArgName(String),
    
    /// A delimiter (quote or bracket) was not closed
    /// Contains information about the unclosed delimiter
    UnclosedDelimiter(String),
    
    /// Nested brackets are not supported
    /// Contains information about the nested brackets
    NestedBrackets(String),
    
    /// The input is invalid in some other way
    /// Contains a description of the issue
    InvalidInput(String),
    
    /// An unexpected error occurred
    /// Contains a description of the error
    UnexpectedError(String),
}

/// Type alias for Result with the library's Error type
pub type Result<T> = std::result::Result<T, Error>;
```

### Function Signatures
```rust
impl Error {
    /// Returns a string representation of the error code for this error
    ///
    /// This is useful for programmatic error handling or for
    /// generating error codes in logs.
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::Error;
    ///
    /// let err = Error::RequiredArgMissing("USER".to_string());
    /// assert_eq!(err.code(), "REQUIRED_ARG_MISSING");
    /// ```
    pub fn code(&self) -> &'static str {
        match self {
            Error::RequiredArgMissing(_) => "REQUIRED_ARG_MISSING",
            Error::MutuallyExclusiveArgs(_, _) => "MUTUALLY_EXCLUSIVE_ARGS",
            Error::InvalidKeyValue(_) => "INVALID_KEY_VALUE",
            Error::UnrecognizedArg(_) => "UNRECOGNIZED_ARG",
            Error::InvalidIntValue(_) => "INVALID_INT_VALUE",
            Error::InvalidBoolValue(_) => "INVALID_BOOL_VALUE",
            Error::DependencyNotMet(_, _) => "DEPENDENCY_NOT_MET",
            Error::InvalidValue(_, _) => "INVALID_VALUE",
            Error::DuplicateArgName(_) => "DUPLICATE_ARG_NAME",
            Error::UnclosedDelimiter(_) => "UNCLOSED_DELIMITER",
            Error::NestedBrackets(_) => "NESTED_BRACKETS",
            Error::InvalidInput(_) => "INVALID_INPUT",
            Error::UnexpectedError(_) => "UNEXPECTED_ERROR",
        }
    }
    
    /// Provides a detailed user-friendly description of the error
    ///
    /// Unlike the `Display` implementation which is concise,
    /// this method provides more details about the error and
    /// possible solutions.
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::Error;
    ///
    /// let err = Error::RequiredArgMissing("USER".to_string());
    /// println!("Error details: {}", err.details());
    /// ```
    pub fn details(&self) -> String {
        match self {
            Error::RequiredArgMissing(arg) => {
                format!(
                    "The required argument '{}' was not provided. \
                     Please ensure all required arguments are included.",
                    arg
                )
            },
            // Other detailed error messages...
            _ => self.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RequiredArgMissing(arg) => 
                write!(f, "Required argument missing: {}", arg),
            Error::MutuallyExclusiveArgs(arg1, arg2) => 
                write!(f, "Mutually exclusive arguments found: {} and {}", arg1, arg2),
            Error::InvalidKeyValue(kv) => 
                write!(f, "Invalid key-value pair: {}", kv),
            Error::UnrecognizedArg(arg) => 
                write!(f, "Unrecognized argument: {}", arg),
            Error::InvalidIntValue(val) => 
                write!(f, "Invalid integer value: {}", val),
            Error::InvalidBoolValue(val) => 
                write!(f, "Invalid boolean value: {}", val),
            Error::DependencyNotMet(arg, dep) => 
                write!(f, "Dependency not met: {} requires {}", arg, dep),
            Error::InvalidValue(arg, val) => 
                write!(f, "Invalid value for {}: {}", arg, val),
            Error::DuplicateArgName(arg) => 
                write!(f, "Duplicate argument name: {}", arg),
            Error::UnclosedDelimiter(info) => 
                write!(f, "Unclosed delimiter: {}", info),
            Error::NestedBrackets(info) => 
                write!(f, "Nested brackets not supported: {}", info),
            Error::InvalidInput(info) => 
                write!(f, "Invalid input: {}", info),
            Error::UnexpectedError(info) => 
                write!(f, "Unexpected error: {}", info),
        }
    }
}

impl std::error::Error for Error {}

// Safe to use in multi-threaded contexts
unsafe impl Send for Error {}
unsafe impl Sync for Error {}

#[cfg(feature = "serde")]
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        
        let mut state = serializer.serialize_struct("Error", 3)?;
        state.serialize_field("code", self.code())?;
        state.serialize_field("message", &self.to_string())?;
        
        // Add specific fields based on error variant
        match self {
            Error::RequiredArgMissing(arg) => {
                state.serialize_field("argument", arg)?;
            },
            // Other variants with specific fields...
            _ => {}
        }
        
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Error {
    // Implementation omitted for brevity
    // Would deserialize back into appropriate Error variant
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialization implementation
        unimplemented!()
    }
}
```

### Implementation Approach

#### 1. Error Type Design
The error type is designed as an enum to cover all possible error conditions. Each variant includes context information specific to that error type:

```rust
pub enum Error {
    RequiredArgMissing(String),    // Includes the name of the missing argument
    MutuallyExclusiveArgs(String, String),  // Includes both conflicting arguments
    // Other variants...
}
```

This approach:
- Makes error handling precise and context-aware
- Enables pattern matching for specific error handling
- Provides detailed information for debugging

#### 2. Standard Trait Implementations
The error type implements standard Rust error traits:

```rust
// Debug for developer-focused output
impl std::fmt::Debug for Error { /* ... */ }

// Display for user-friendly messages
impl std::fmt::Display for Error { /* ... */ }

// Error trait for integration with Rust's error handling ecosystem
impl std::error::Error for Error {}

// Send and Sync for thread safety
unsafe impl Send for Error {}
unsafe impl Sync for Error {}
```

#### 3. Result Type Alias
A type alias is provided for convenience and consistency:

```rust
pub type Result<T> = std::result::Result<T, Error>;
```

This allows functions to be declared as:

```rust
fn parse_arguments(&self, args: Vec<String>) -> Result<ParsedArgs> {
    // Implementation
}
```

Instead of the more verbose:

```rust
fn parse_arguments(&self, args: Vec<String>) -> std::result::Result<ParsedArgs, Error> {
    // Implementation
}
```

#### 4. Error Propagation Approach
The design facilitates error propagation via Rust's `?` operator:

```rust
fn process_input(&self, input: &str) -> Result<Output> {
    let tokens = self.tokenize(input)?;  // Error from tokenize is propagated
    let parsed = self.parse(tokens)?;    // Error from parse is propagated
    Ok(Output::new(parsed))
}
```

#### 5. Optional Serde Support
When the `serde` feature is enabled, Error implements Serialize and Deserialize:

```rust
#[cfg(feature = "serde")]
impl serde::Serialize for Error { /* ... */ }

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Error { /* ... */ }
```

This allows errors to be serialized for logging or API responses.

### Error Handling Strategy

The library's error handling strategy is designed around these principles:

1. **Early Detection**: Errors are detected as early as possible in the processing pipeline
2. **Rich Context**: Error variants include specific information about what went wrong
3. **No Panics**: The library avoids panicking, returning errors instead
4. **Ergonomic Propagation**: The `?` operator is used for clean error propagation
5. **Pattern Matching**: Errors support pattern matching for detailed error handling

Example usage pattern:

```rust
match parser.parse(args) {
    Ok(result) => {
        // Handle successful parse
    },
    Err(Error::RequiredArgMissing(arg)) => {
        eprintln!("Missing required argument: {}", arg);
    },
    Err(Error::MutuallyExclusiveArgs(a, b)) => {
        eprintln!("Cannot use both {} and {}", a, b);
    },
    Err(e) => {
        // Handle other errors
        eprintln!("Error: {}", e);
    }
}
```

## Integration

### Integration with Other Components

The error types integrate with other components as follows:

1. **All Library Functions**: Return `Result<T>` for operations that can fail
2. **Parser Components**: Generate appropriate error variants for parsing failures
3. **Validation Engine**: Use specific error variants for validation failures
4. **Public API**: Expose errors to users for handling and potential recovery

### Usage Examples

```rust
use pam_args_rs::{ArgumentParser, Error, Result};

fn process_arguments(args: Vec<String>) -> Result<Config> {
    let parser = ArgumentParser::new()
        // Configure parser...
        ;
    
    // Parse with error propagation
    let matches = parser.parse(args)?;
    
    // Pattern match on specific error conditions
    let config = match extract_config(matches) {
        Ok(config) => config,
        Err(Error::RequiredArgMissing(arg)) => {
            eprintln!("Required argument missing: {}", arg);
            return Err(Error::RequiredArgMissing(arg));
        },
        Err(e) => return Err(e),
    };
    
    Ok(config)
}
```

## Testing Strategy

### Unit Test Cases

| # | Category | Input | Expected Output | Notes |
|---|----------|-------|-----------------|-------|
| 1 | Error Creation | `Error::RequiredArgMissing("USER".to_string())` | Error instance with correct variant | Test basic construction |
| 2 | Error Creation | `Error::MutuallyExclusiveArgs("DEBUG".to_string(), "QUIET".to_string())` | Error instance with both argument names | Test construction with multiple fields |
| 3 | Display Trait | `format!("{}", Error::RequiredArgMissing("USER".to_string()))` | String: `"Required argument missing: USER"` | Test Display implementation |
| 4 | Error Code | `Error::RequiredArgMissing("USER".to_string()).code()` | String: `"REQUIRED_ARG_MISSING"` | Test code method |
| 5 | Error Details | `Error::RequiredArgMissing("USER".to_string()).details()` | Detailed explanation string | Test details method |
| 6 | Debug Trait | `format!("{:?}", Error::InvalidValue("ALIGN".to_string(), "TOP".to_string()))` | Debug representation string | Test Debug implementation |
| 7 | Error Trait | `let err: Box<dyn std::error::Error> = Box::new(Error::InvalidInput("test".to_string()))` | No panic, successful conversion | Test std::error::Error implementation |
| 8 | Pattern Matching | Pattern match on various error variants | Correct branch execution | Test pattern matching on error variants |
| 9 | Send/Sync Traits | Use error across thread boundaries | No compile errors | Test Send and Sync implementations |
| 10 | Result Type | `let result: Result<()> = Err(Error::InvalidIntValue("abc".to_string()))` | Valid Result instance | Test Result type alias |
| 11 | Serde | Serialize various error variants | Valid serialized representation | Test Serialize implementation (feature gated) |
| 12 | Serde | Deserialize various error variants | Valid Error instances | Test Deserialize implementation (feature gated) |
| 13 | Clone | `let err2 = err.clone()` | Identical error | Test Clone implementation |
| 14 | PartialEq | `err == err2` | true | Test PartialEq implementation |
| 15 | ? Operator | Code using `?` with different error variants | Proper error propagation | Test error propagation via ? |

### Integration Tests

The error type should be tested in integration with other components to ensure correct end-to-end behavior. Integration tests should focus on:

1. **Error Propagation**
   - Test that errors propagate correctly through the parse pipeline
   - Verify that context information is preserved
   - Test the `?` operator with different error variants

2. **Error Recovery**
   - Test scenarios where users can recover from errors
   - Verify that pattern matching on errors works as expected
   - Test handling multiple error conditions in sequence

3. **Cross-Component Error Generation**
   - Verify that each component generates the correct error variants
   - Test error consistency across the library
   - Verify that error messages are helpful and consistent

### Testing Focus Areas

1. **Error Variant Correctness**
   - Each error variant contains appropriate context
   - Error messages are clear and informative
   - Error codes are unique and descriptive

2. **Trait Implementations**
   - Debug output is comprehensive
   - Display messages are concise and user-friendly
   - Error trait is properly implemented
   - Send and Sync constraints are satisfied

3. **Pattern Matching**
   - Error variants can be uniquely identified
   - Context information is accessible
   - Match expressions work as expected

4. **Serialization/Deserialization**
   - Errors serialize to a useful representation
   - Error context is preserved in serialization
   - Deserialization restores the original error

## Performance Considerations

### Memory Efficiency
- Error variants use `String` for context information instead of `&str` to avoid lifetime complexity
- The enum is designed to be compact, minimizing memory overhead
- Context strings are kept concise to reduce memory usage

### Time Complexity
- Error creation is O(1) plus the cost of string allocation
- Error propagation is very fast due to Rust's zero-cost abstractions
- Pattern matching on error variants is optimized by the compiler

### Optimizations
- Minimal use of heap allocations in error paths
- No dynamic dispatch in the core error type
- Lightweight error propagation via the `?` operator
