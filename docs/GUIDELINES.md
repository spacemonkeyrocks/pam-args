# Implementation Guidelines

## General Principles

### DRY (Don't Repeat Yourself)
- Avoid code duplication by abstracting common logic into reusable functions.
- Create utility functions for operations that are performed in multiple places.
- Ensure that every piece of knowledge has a single, unambiguous representation.

### Single Responsibility Principle
- Each function and module should have one well-defined responsibility.
- If a function is doing multiple things, break it down into smaller functions.
- A good test: can you describe what the function does in a single sentence?

### Function Design
- Keep functions under 50-100 lines of code for better readability.
- Use clear, descriptive names that indicate what the function does.
- Avoid hardcoded values; use constants or configuration parameters.
- Limit nesting depth to 3-4 levels; extract deeper nesting into helper functions.

### Error Handling
- Use the `anyhow` crate for error handling.
- Provide clear, specific error messages that assist in troubleshooting.
- Display error messages in red using the `colored` crate.
- Display warnings in orange using the `colored` crate.
- Properly propagate errors up the call stack using the `?` operator.
- Handle expected error cases gracefully.
- Include examples of handling specific types of errors (e.g., I/O errors, network errors).

## Documentation Requirements

### Code Documentation
- Add rustdoc comments to all public functions, structs, and enums.
- Document parameters, return values, and potential errors.
- Include examples where appropriate.
- Explain any non-obvious implementation details.
- Provide examples of well-documented code snippets to illustrate best practices.

#### Rust Documentation Format
```rust
/// Brief description of the function
///
/// More detailed description of what the function does and how it behaves.
/// Multiple paragraphs can be used for longer explanations.
///
/// # Parameters
/// * `param1` - Description of the first parameter
/// * `param2` - Description of the second parameter
///
/// # Returns
/// Description of the return value
///
/// # Errors
/// This function will return an error if:
/// * Condition 1
/// * Condition 2
///
/// # Panics
/// This function will panic if:
/// * Condition 1
///
/// # Examples
/// ```
/// let result = my_function(param1, param2);
/// assert_eq!(result, expected);
/// ```
fn my_function(param1: Type1, param2: Type2) -> Result<ReturnType, Error> {
    // Function body
}
```

### Internal Documentation
- Add regular comments to complex or non-obvious code.
- Document any assumptions or invariants.
- Add TODO comments for future improvements or known limitations.
- Include guidelines for documenting private functions and modules.

## Testing Requirements

### Test-Driven Development
- Write tests before implementing functionality.
- Follow the "Red-Green-Refactor" cycle:
  1. Write a failing test.
  2. Implement the minimum code to make the test pass.
  3. Refactor while keeping tests passing.
- Run tests frequently during development.
- Consider writing property-based tests using crates like `proptest`.

### Testing Structure

#### Unit Tests
- Unit tests focus on testing individual functions or "units" of code in isolation.
- They serve two primary purposes:
  - Ensure individual parts of the program behave as expected
  - Prevent future changes from altering existing behavior
- Should be located in the same file as the implementation using `#[cfg(test)]` module.
- Focus on edge cases and error conditions.
- Mock dependencies when necessary using crates like `mockall`.
- Should use colored asserts (`assert_color`, `assert_eq_color`and `assert_ne_color!`) from `tests/common/mod.rs`, to improve readability.
- Asserts should have a clear failure reason, to make it easier to debug.

```rust
mod common;

// In your implementation file
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq_colored!(add(2, 2), 4, 
            format("Values should be the same: {} <> {}", add(2, 2), 4));
        assert_eq_colored!(add(-1, 1), 0, 
            format("Values should be the same: {} <> {}", add(-1, 1), 0));
        assert_eq_colored!(add(0, 0), 0, 
            format("Values should be 0: {} <> {}", add(0, 0), 0"));
    }
}
```

#### Unit Testing Best Practices
- Only test functionality in the file it's defined in.
- Only test one thing at a time for clarity and maintainability.
- Write a separate test case for each code path to ensure complete coverage.
- Don't assert on intermediate steps; focus on the function's final output.
- Use descriptive test names that explain what is being tested and expected behavior.
  - Good: `reports_error_when_invalid_syntax_encountered`
  - Bad: `functionality_works`
- Include ticket/bug IDs in test names when applicable: `fix_1234`, `feature_42`.
- Use appropriate assertion macros: `assert_color`, `assert_eq_color`and `assert_ne_color!`.
- For testing code that may panic, use `#[should_panic]` attribute.
- For slow tests, use `#[ignore]` attribute and run them with `cargo test -- --ignored`.
- For Result-returning functions, make your test return Result to use the `?` operator:

```rust
#[test]
fn result_test() -> Result<(), String> {
    let result = function_that_returns_result()?;
    assert_colored!(result, "The function should return a value");
    Ok(())
}
```

#### Integration Tests
- Integration tests verify that multiple modules or components work together correctly.
- Located in the `tests/` directory at the project root (separate from source code).
- Each file in the `tests/` directory is compiled as a separate crate.
- Should use colored asserts (`assert_color`, `assert_eq_color`and `assert_ne_color!`) from `tests/common/mod.rs`, to improve readability.
- Asserts should have a clear failure reason, to make it easier to debug.
- Integration tests must explicitly import the crate under test:

```rust
mod common;

// In tests/integration_test.rs
use my_crate;

#[test]
fn test_full_workflow() {
    let input = "test input";
    let result = my_crate::process_workflow(input);
    assert_colored!(result.is_ok(), "Workflow should be successful");
}
```

#### Integration Test Structure and Organization
- Organize integration tests in the `tests/` directory in a logical structure.
- Use subdirectories for grouping related tests, but remember only files directly in `tests/` are treated as integration tests.
- Create shared test utilities in `tests/common/mod.rs`.
- For large test suites, use modules to organize tests into subdirectories:

```
<crate_root>
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    ├── common/
    │   └── mod.rs      # Shared utilities
    ├── integration_1.rs
    ├── domain/
    │   ├── mod.rs      # Include submodules
    │   ├── auth.rs
    │   └── user.rs
    └── test_domain.rs  # Import domain module
```

- In `test_domain.rs`:
```rust
mod domain;  // This imports the tests from the domain/ subdirectory
```

- In `domain/mod.rs`:
```rust
mod auth;    // This imports tests from auth.rs
mod user;    // This imports tests from user.rs
```

#### When to Use Integration Tests vs. Unit Tests
- Use **unit tests** when:
  - Testing individual functions or methods
  - Testing specific edge cases or error conditions
  - Testing internal implementation details
  - You need to mock dependencies

- Use **integration tests** when:
  - Testing workflows that span multiple components
  - Testing public APIs as a client would use them
  - Verifying that components work together correctly
  - Testing end-to-end functionality

### Advanced Testing Techniques

#### Fluent Testing
- Consider using the `spectral` crate for fluent assertions:

```rust
#[test]
fn with_spectral() {
    use spectral::prelude::*;
    let nums = vec![1, 2, 3];
    assert_that(&nums).has_length(3);
    assert_that(&nums).contains(1);
}
```

#### Snapshot Testing
- Use the `insta` crate for snapshot testing of complex outputs:

```rust
#[test]
fn test_complex_output() {
    let output = generate_complex_output();
    insta::assert_yaml_snapshot!(output);
}
```

#### Mocking
- Use the `mockall` crate for creating mock objects:

```rust
use mockall::automock;

#[automock]
trait Database {
    fn get_user(&self, id: u32) -> Result<User, Error>;
}

#[test]
fn test_with_mock_db() {
    let mut mock_db = MockDatabase::new();
    mock_db.expect_get_user()
        .with(predicate::eq(1))
        .times(1)
        .returning(|_| Ok(User::new("test_user")));
    
    let service = UserService::new(mock_db);
    let user = service.get_user(1).unwrap();
    assert_eq!(user.name, "test_user");
}
```

#### Property-Based Testing
- Use the `proptest` crate to test properties of functions with many possible inputs:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_addition_commutative(a in 0..100, b in 0..100) {
        assert_eq!(add(a, b), add(b, a));
    }
}
```

#### Fuzz Testing
- For safety-critical code, use `cargo-afl` for fuzz testing:

```rust
// In src/bin/fuzz.rs
#[macro_use] extern crate afl;

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            let _ = my_crate::parse(s);
        }
    });
}
```

### Test Data Generation

#### Test Tables
- Use test tables for testing multiple inputs with a single test function:

```rust
#[test]
fn test_multiple_cases() {
    let cases = vec![
        ("input1", "expected1", "case 1 description"),
        ("input2", "expected2", "case 2 description"),
        ("input3", "expected3", "case 3 description"),
    ];
    
    for (input, expected, description) in cases {
        let result = process(input);
        assert_eq!(result, expected, "{}", description);
    }
}
```

#### Automated Test Generation with Macros
- Use macros to generate multiple test cases:

```rust
macro_rules! test_cases {
    ($(
        $test_name:ident: $input:expr => $expected:expr
    ),* $(,)?) => {
        $(
            #[test]
            fn $test_name() {
                let result = function_under_test($input);
                assert_eq!(result, $expected);
            }
        )*
    };
}

test_cases! {
    empty_string: "" => 0,
    single_word: "hello" => 5,
    multiple_words: "hello world" => 11,
}
```

#### Fake Data Generation
- Use the `fake` crate for generating realistic test data:

```rust
use fake::{Fake, Faker};
use fake::faker::name::raw::*;
use fake::locales::EN;

#[test]
fn test_with_fake_data() {
    let name: String = Name(EN).fake();
    let user = User::new(name);
    assert!(user.name.len() > 0);
}
```

### Test Coverage
- Aim for at least 90% test coverage.
- Use `cargo-tarpaulin` or other coverage tools to measure test coverage.
- Test happy paths, edge cases, and error conditions.
- Test with various inputs, including invalid and boundary values.
- Test interactions between components.

### Debugging Test Failures
- Add meaningful debug output to tests using `println!` or `dbg!`.
- Consider using `env_logger` in tests to control verbosity.
- For complex test cases, add step-by-step debug output.
- Run specific failing tests with `cargo test test_name` for focused debugging.

## Performance Considerations

### Efficiency
- Be mindful of performance-critical sections.
- Avoid unnecessary allocations and copies.
- Use appropriate data structures for the task at hand.
- Consider time and space complexity of algorithms.
- Provide detailed examples of performance-critical sections and how to optimize them.
- Include guidelines for profiling and benchmarking code to identify performance bottlenecks.

### Resource Management
- Free resources when they are no longer needed.
- Avoid memory leaks and resource exhaustion.
- Handle large inputs gracefully.

## Dependencies

### External Crates
- Use the recommended crates listed in the project specification.
- Use the latest stable versions of dependencies.
- Follow best practices for each crate.
- Provide guidelines for updating dependencies and handling breaking changes.
- Include recommendations for tools to manage and audit dependencies (e.g., `cargo-audit`).

### Internal Dependencies
- Keep dependencies between modules minimal and well-defined.
- Use clear interfaces between components.
- Avoid circular dependencies.

## Coding Style

### Rust Idioms
- Follow Rust idioms and conventions.
- Use pattern matching where appropriate.
- Take advantage of Rust's type system and ownership model.
- Use iterators and closures for collections processing.
- Include recommendations for using Rust's advanced features (e.g., traits, lifetimes) effectively.

### Consistency
- Maintain consistent coding style throughout the project.
- Use consistent naming conventions.
- Follow standard Rust formatting (use rustfmt).
- Follow standard Rust linting rules (use clippy).
- Add guidelines for handling warnings and lints generated by `clippy` and `rustfmt`.
