## Performance Considerations

### Memory Efficiency
- Testing utilities use stack allocation where possible
- String operations minimize copies
- Builders preallocate vectors to reduce reallocations
- Colored output is only generated for failing tests

### Time Complexity
- Test argument building is O(n) in the number of arguments
- Assertions are O(1) for flag checks and O(n) for value comparisons
- Error simulation is O(1) for most error types
- Colored output adds negligible overhead only on test failure

### Optimizations
- Minimal string copying in test utilities
- Direct field access for efficient assertion checks
- Reuse of parser configurations across tests
- Lazy evaluation of formatted error messages# Feature 4: Testing Utilities

## Module Type
**Public/Testing**: This component provides utility functions and structures specifically designed for testing the library. While not part of the main API used by end-users, these utilities are public to allow developers to write tests for their PAM modules that use the library.

## Feature Information

**Feature Name**: Testing Utilities

**Description**: Implements helpers for testing the library and PAM modules that use it. These utilities provide functions for creating mock arguments, verifying parsing results, and simulating different input scenarios. By providing a comprehensive testing framework, developers can more easily validate their PAM module's argument handling and ensure correct behavior across different configurations.

**Priority**: High

**Dependencies**: 
- [Feature 1: Result & Error Types](error-result-types.md)
- [Feature 2: Utility Functions](utility-functions.md)

## Requirements

### Functional Requirements
1. Provide utilities for creating test argument arrays
2. Implement mock PAM module argument generators
3. Create assertion helpers for validating parsing results
4. Provide functions for simulating different input scenarios
5. Implement utilities for testing all supported argument formats
6. Offer helper functions for testing error conditions
7. Include colored assertion macros for more visible test diagnostics

### API Requirements
- Expose a clean, intuitive testing API
- Ensure consistent behavior across different testing environments
- Support both unit and integration testing approaches
- Allow for customization of test scenarios
- Provide detailed error messages for test failures
- Use colored output for better visibility in terminal test runners

### Performance Requirements
- Keep testing utilities lightweight
- Minimize allocation overhead in test code
- Enable fast test execution

## Design

### Data Structures
```rust
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

### Implementation Approach

#### 1. Testing Configuration
The `TestConfig` struct provides a flexible configuration for testing scenarios:

```rust
pub struct TestConfig {
    pub use_brackets: bool,
    pub use_quotes: bool,
    pub quote_char: char,
    pub include_non_arg_text: bool,
    pub randomize_order: bool,
}
```

This allows tests to:
- Control whether bracketed argument format is used
- Specify if quotes should be used for values with spaces
- Choose quote character (single or double)
- Include non-argument text
- Randomize argument order for robustness testing

#### 2. Test Argument Builder
The `TestArgsBuilder` uses the builder pattern to construct test arguments:

```rust
pub struct TestArgsBuilder {
    flags: Vec<String>,
    key_values: Vec<(String, String)>,
    non_arg_text: Vec<String>,
    config: TestConfig,
}

impl TestArgsBuilder {
    pub fn new() -> Self { /* ... */ }
    pub fn add_flag(mut self, flag: S) -> Self { /* ... */ }
    pub fn add_key_value(mut self, key: K, value: V) -> Self { /* ... */ }
    // Other methods...
    pub fn build(self) -> Vec<String> { /* ... */ }
}
```

This provides:
- A fluent interface for building test cases
- Support for different argument formats
- Configuration control
- Automatic formatting based on configuration

#### 3. Test Assertions
The `TestAssertions` struct provides a fluent interface for validating parse results:

```rust
pub struct TestAssertions<'a> {
    parse_result: &'a crate::ParseResult,
}

impl<'a> TestAssertions<'a> {
    pub fn assert_flag_present(self, flag: S) -> Self { /* ... */ }
    pub fn assert_value_equals<S, T>(self, key: S, expected: T) -> Self { /* ... */ }
    // Other assertion methods...
}
```

This enables:
- Chaining multiple assertions together
- Clear error messages on failure
- Type-safe value comparison

#### 4. Colored Assertion Macros
The module provides colored assertion macros for better test visibility:

```rust
#[macro_export]
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
macro_rules! assert_ne_colored {
    ($left:expr, $right:expr, $msg:expr) => {
        if $left == $right {
            use colored::Colorize;
            eprintln!("{}", $msg.red());
            panic!("assertion failed: `(left != right)`\n left: `{:?}`,\n right: `{:?}`\n{}", $left, $right, $msg);
        }
    };
}
```

This provides:
- Better visibility of test failures in the terminal
- Consistent formatting of error messages
- Clear highlighting of failed assertions

#### 5. Error Simulation
The `ErrorSimulator` provides methods to generate arguments that will trigger specific errors:

```rust
pub struct ErrorSimulator {
    config: TestConfig,
}

impl ErrorSimulator {
    pub fn required_arg_missing(&self, required_arg: S) -> Vec<String> { /* ... */ }
    pub fn mutually_exclusive_args(&self, arg1: S1, arg2: S2) -> Vec<String> { /* ... */ }
    // Other error simulation methods...
}
```

This allows testing:
- Error handling code paths
- Validation logic
- Error message formatting

#### 6. Standalone Testing Utilities
The module also provides standalone utility functions:

```rust
pub fn create_test_parser() -> crate::ArgumentParser { /* ... */ }
pub fn create_pam_arg_string<S1, I, S2>(module_name: S1, args: I) -> String { /* ... */ }
pub fn split_pam_arg_string<S: AsRef<str>>(arg_string: S) -> Vec<String> { /* ... */ }
```

These functions:
- Simplify common testing tasks
- Reduce test boilerplate
- Enable realistic PAM testing scenarios

#### 7. Test Case Macro
The `test_case!` macro provides a concise way to define test cases:

```rust
macro_rules! test_case {
    (args: [$($arg:expr),* $(,)?], assert: $assert_fn:expr) => { /* ... */ }
}
```

This provides:
- Reduced test boilerplate
- Standardized test structure
- Clean error reporting

### Error Handling in Tests
The testing utilities use Rust's assertion macros with colored output to report test failures:

```rust
assert_colored!(
    self.parse_result.is_present(flag.as_ref()),
    format!("Expected flag '{}' to be present, but it was not", flag.as_ref())
);
```

Specialized assertion functions check for specific error types:

```rust
pub fn assert_required_arg_missing<T, S: AsRef<str>>(result: &crate::Result<T>, arg: S) {
    assert_error_variant(result, |e| {
        matches!(e, Error::RequiredArgMissing(a) if a == arg.as_ref())
    });
}
```

## Integration

### Integration with Other Components

The testing utilities integrate with other components as follows:

1. **Error Types**: Uses the library's error types to validate error conditions
2. **ArgumentParser**: Provides utilities for creating and configuring the parser
3. **ParseResult**: Offers assertions for validating parse results
4. **Key-Value Store**: Integrates with key-value store for validation
5. **Colored Output**: Uses the `colored` crate for better test diagnostics

### Usage Examples

```rust
use pam_args_rs::{ArgumentParser, Flag, KeyValue, Error};
use pam_args_rs::testing::{TestArgsBuilder, TestAssertions, ErrorSimulator, test_case};
use pam_args_rs::{assert_colored, assert_eq_colored, assert_ne_colored};
use std::str::FromStr;

#[test]
fn test_basic_parsing() -> Result<(), Error> {
    // Create test arguments
    let args = TestArgsBuilder::new()
        .add_flag("DEBUG")
        .add_key_value("USER", "admin")
        .add_key_value("WIDTH", "80")
        .build();
    
    // Create and configure parser
    let parser = ArgumentParser::new()
        .flag(Flag::new("DEBUG", "Enable debug mode"))
        .key_value(
            KeyValue::new("USER", "Username for authentication")
                .type_converter(String::from_str)
        )
        .key_value(
            KeyValue::new("WIDTH", "Width of the output")
                .type_converter(i32::from_str)
        );
    
    // Parse arguments
    let result = parser.parse(args)?;
    
    // Assert the result using fluent interface
    TestAssertions::new(&result)
        .assert_flag_present("DEBUG")
        .assert_value_equals("USER", "admin")
        .assert_value_equals("WIDTH", 80);
    
    // Assert using colored macros
    assert_colored!(result.is_present("DEBUG"), "DEBUG flag should be present");
    assert_eq_colored!(
        result.value_of::<String>("USER").unwrap(), 
        "admin", 
        "USER value should be 'admin'"
    );
    assert_eq_colored!(
        result.value_of::<i32>("WIDTH").unwrap(), 
        80, 
        "WIDTH value should be 80"
    );
    
    Ok(())
}

#[test]
fn test_required_arg_missing() {
    // Create arguments that will trigger a required arg missing error
    let args = ErrorSimulator::new().required_arg_missing("MESSAGE");
    
    // Create parser with a required argument
    let parser = ArgumentParser::new()
        .key_value(
            KeyValue::new("MESSAGE", "Message to display")
                .type_converter(String::from_str)
                .required()
        );
    
    // Parse and expect error
    let result = parser.parse(args);
    
    // Assert the error
    assert::assert_required_arg_missing(&result, "MESSAGE");
}

#[test]
fn test_with_macro() {
    test_case! {
        args: ["DEBUG", "USER=admin", "WIDTH=80"],
        assert: |result| {
            assert_colored!(result.is_present("DEBUG"), "DEBUG flag should be present");
            assert_eq_colored!(
                result.value_of::<String>("USER").unwrap(), 
                "admin", 
                "USER value should be 'admin'"
            );
            assert_eq_colored!(
                result.value_of::<i32>("WIDTH").unwrap(), 
                80, 
                "WIDTH value should be 80"
            );
        }
    }
}
```

## Testing Strategy

### Unit Test Cases

| # | Category | Input | Expected Output | Notes |
|---|----------|-------|-----------------|-------|
| 1 | TestConfig | `TestConfig::new()` | Default config | Test default configuration |
| 2 | TestConfig | `TestConfig::with_brackets()` | Config with brackets enabled | Test bracketed configuration |
| 3 | TestConfig | `TestConfig::without_brackets()` | Config with brackets disabled | Test non-bracketed configuration |
| 4 | TestConfig | `TestConfig::new().with_use_quotes(false)` | Config with quotes disabled | Test quote configuration |
| 5 | TestArgsBuilder | `TestArgsBuilder::new().add_flag("DEBUG").build()` | `["DEBUG"]` | Test flag building |
| 6 | TestArgsBuilder | `TestArgsBuilder::new().add_key_value("USER", "admin").build()` | `["USER=admin"]` | Test key-value building |
| 7 | TestArgsBuilder | `TestArgsBuilder::new().add_non_arg_text("text").build()` | `[]` or `["text"]` depending on config | Test non-arg text building |
| 8 | TestArgsBuilder | `TestArgsBuilder::new().add_flags(vec!["a", "b"]).build()` | `["a", "b"]` | Test multiple flags |
| 9 | TestArgsBuilder | Complex builder with all argument types | Vector with correct format | Test comprehensive case |
| 10 | TestArgsBuilder | Builder with bracketed KV pairs | KV pairs in bracket format | Test bracketed format |
| 11 | TestArgsBuilder | Builder with quoted values | KV pairs with quoted values | Test quoted values |
| 12 | TestAssertions | Various assertions on known result | No panic | Test basic assertions |
| 13 | TestAssertions | Assertions on missing flag | Panic with clear message | Test negative assertions |
| 14 | TestAssertions | Type conversion in assertions | Type-safe comparison | Test type conversion |
| 15 | TestAssertions | Chained assertions | No panic if all pass | Test fluent interface |
| 16 | Colored Assertions | `assert_colored!(true, "message")` | No panic | Test colored assert macro |
| 17 | Colored Assertions | `assert_colored!(false, "message")` | Panic with red message | Test colored assert failure |
| 18 | Colored Assertions | `assert_eq_colored!(5, 5, "message")` | No panic | Test colored eq macro |
| 19 | Colored Assertions | `assert_eq_colored!(5, 6, "message")` | Panic with red message | Test colored eq failure |
| 20 | Colored Assertions | `assert_ne_colored!(5, 6, "message")` | No panic | Test colored ne macro |
| 21 | Colored Assertions | `assert_ne_colored!(5, 5, "message")` | Panic with red message | Test colored ne failure |
| 22 | ErrorSimulator | `required_arg_missing("USER")` | Empty args vector | Test missing arg simulation |
| 23 | ErrorSimulator | `mutually_exclusive_args("DEBUG", "QUIET")` | Vector with both args | Test exclusion simulation |
| 24 | ErrorSimulator | `dependency_not_met("HOST", "USER")` | Vector with HOST but no USER | Test dependency simulation |
| 25 | ErrorSimulator | `invalid_value("ALIGN", "INVALID")` | Vector with invalid value | Test invalid value simulation |
| 26 | ErrorSimulator | Other error simulations | Vectors that would trigger errors | Test other error cases |
| 27 | Standalone Utilities | `create_test_parser()` | Configured parser | Test parser creation |
| 28 | Standalone Utilities | `create_pam_arg_string()` | Correctly formatted string | Test PAM string creation |
| 29 | Standalone Utilities | `split_pam_arg_string()` | Correctly split vector | Test PAM string splitting |
| 30 | Assertion Functions | Various error assertions | No panic if correct | Test error assertions |
| 31 | Test Case Macro | `test_case!` macro | Test runs without panic | Test macro functionality |

### Integration Tests

Integration tests should focus on using the testing utilities to test the actual library components:

1. **Full Parser Testing**
   - Use testing utilities to create complex parsing scenarios
   - Test the full argument processing pipeline
   - Verify correct handling of complex cases
   - Use colored assertion macros for better test feedback

2. **Error Path Testing**
   - Use error simulator to test all error paths
   - Verify correct error propagation
   - Test error recovery scenarios
   - Validate error messages are properly formatted

3. **PAM Module Integration**
   - Create realistic PAM argument scenarios
   - Test with mock PAM module implementations
   - Verify correct handling of PAM-specific formats
   - Test with various configuration combinations

### Testing Focus Areas

1. **API Usability**
   - Ensure the testing API is intuitive and easy to use
   - Validate that test code is concise and readable
   - Verify that error messages are clear and helpful
   - Test the effectiveness of colored output for error visibility

2. **Comprehensive Coverage**
   - Test all argument formats and combinations
   - Verify all error conditions can be simulated
   - Test edge cases and unusual configurations
   - Ensure colored assertion macros work in all testing contexts

3. **Realistic Scenarios**
   - Test with realistic PAM module argument formats
   - Verify handling of actual command-line patterns
   - Test with large argument sets
   - Validate behavior with complex argument combinations

## Performance Considerations

### Memory Efficiency
- Testing utilities use stack allocation where possible
- String operations minimize copies
- Builders preallocate vectors to reduce reallocations

### Time Complexity
- Test argument building is O(n) in the number of arguments
- Assertions are O(1) for flag checks and O(n) for value comparisons
- Error simulation is O(1) for most error types

### Optimizations
- Minimal string copying in test utilities
- Direct field access for efficient assertion checks
- Reuse of parser configurations across tests
