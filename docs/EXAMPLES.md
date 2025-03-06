# Examples for pam-args

## Introduction

Welcome to the **Examples** section of the `pam-args` library documentation. This document illustrates the practical applications of the `pam-args` library through a series of comprehensive examples. Whether you're a beginner looking to get started or an experienced developer seeking advanced use cases, these examples will guide you through the various functionalities and configurations the library offers. Each example includes detailed explanations and sample code to demonstrate how to implement specific features, handle different argument types, manage dependencies, and integrate the library seamlessly within your Rust PAM modules. Explore these examples to gain a deeper understanding of how to leverage `pam-args` to its fullest potential in your projects.

## Basic Example

This example demonstrates the core functionality of `pam-args` with simple flags and key-value pairs. We'll show all three approaches to give you a clear comparison.

### Approach 1: Traditional Builder Pattern

```rust
use pam_args_rs::{ArgumentParser, Flag, KeyValue, Error};
use std::str::FromStr;

// Define a struct to hold our configuration
#[derive(Debug, Default)]
struct Arguments {
    env: bool,
    debug: bool,
    quiet: bool,
    user: Option<String>,
    host: Option<String>,
    message: String,  // Required argument
}

fn main() -> Result<(), Error> {
    // Create a new parser
    let matches = ArgumentParser::new()
        .case_sensitive(true)
        .collect_non_argument_text(true)
        .flag(Flag::new("ENV", "Enable environment variables"))
        .flag(Flag::new("DEBUG", "Enable debug mode"))
        .flag(Flag::new("QUIET", "Suppress output"))
        .key_value(
            KeyValue::new("USER", "Username for authentication")
                .type_converter(String::from_str)
        )
        .key_value(
            KeyValue::new("HOST", "Host to connect to")
                .type_converter(String::from_str)
                .depends_on("USER")  // HOST depends on USER
        )
        .key_value(
            KeyValue::new("MESSAGE", "Message to display")
                .type_converter(String::from_str)
                .required()  // Mark MESSAGE as required
        )
        .conflicts("DEBUG", "QUIET")  // DEBUG and QUIET are mutually exclusive
        .parse(std::env::args().skip(1))?;
    
    // Extract values into our struct
    let args = Arguments {
        env: matches.is_present("ENV"),
        debug: matches.is_present("DEBUG"),
        quiet: matches.is_present("QUIET"),
        user: matches.value_of::<String>("USER"),
        host: matches.value_of::<String>("HOST"),
        message: matches.value_of::<String>("MESSAGE").expect("MESSAGE is required"),
    };
    
    let non_args = matches.non_argument_text();

    // Use the parsed arguments
    println!("Arguments parsed successfully.");
    if args.env { println!("ENV: true"); }
    if args.debug { println!("DEBUG: true"); }
    if args.quiet { println!("QUIET: true"); }
    if let Some(user) = &args.user { println!("USER: {}", user); }
    if let Some(host) = &args.host { println!("HOST: {}", host); }
    println!("MESSAGE: {}", args.message);
    if !non_args.is_empty() { println!("Non-Argument Text: {}", non_args.join(" ")); }

    Ok(())
}
```

### Approach 2: Direct Field Binding

```rust
use pam_args_rs::{ArgumentParser, Flag, KeyValue, Error};
use std::str::FromStr;

// Define a struct to hold our configuration
#[derive(Debug, Default)]
struct Arguments {
    env: bool,
    debug: bool,
    quiet: bool,
    user: Option<String>,
    host: Option<String>,
    message: String,  // Required argument
}

fn main() -> Result<(), Error> {
    // Create an instance of our config struct
    let mut args = Arguments::default();
    let mut message = String::new(); // For required field

    // Create a new parser with direct field binding
    let parser = ArgumentParser::new()
        .case_sensitive(true)
        .collect_non_argument_text(true)
        .flag(Flag::new("ENV", "Enable environment variables")
            .bind_to(&mut args.env))
        .flag(Flag::new("DEBUG", "Enable debug mode")
            .bind_to(&mut args.debug))
        .flag(Flag::new("QUIET", "Suppress output")
            .bind_to(&mut args.quiet))
        .key_value(
            KeyValue::new("USER", "Username for authentication")
                .type_converter(String::from_str)
                .bind_to(&mut args.user)
        )
        .key_value(
            KeyValue::new("HOST", "Host to connect to")
                .type_converter(String::from_str)
                .depends_on("USER")
                .bind_to(&mut args.host)
        )
        .key_value(
            KeyValue::new("MESSAGE", "Message to display")
                .type_converter(String::from_str)
                .required()
                .bind_to(&mut message)  // Binding to a direct string
        )
        .conflicts("DEBUG", "QUIET");
    
    // Parse and get non-argument text
    let result = parser.parse(std::env::args().skip(1))?;
    let non_args = result.non_argument_text();
    
    // Transfer the required field to our struct
    args.message = message;

    // Values are already in our args struct!
    println!("Arguments parsed successfully.");
    if args.env { println!("ENV: true"); }
    if args.debug { println!("DEBUG: true"); }
    if args.quiet { println!("QUIET: true"); }
    if let Some(user) = &args.user { println!("USER: {}", user); }
    if let Some(host) = &args.host { println!("HOST: {}", host); }
    println!("MESSAGE: {}", args.message);
    if !non_args.is_empty() { println!("Non-Argument Text: {}", non_args.join(" ")); }

    Ok(())
}
```

### Approach 3: Derive Macro

```rust
use pam_args_rs::{Error, ArgumentParser};
use std::str::FromStr;

// Define a struct with derive macro
#[derive(Debug, Default, ArgumentParser)]
struct Arguments {
    #[argument(flag, name = "ENV", description = "Enable environment variables")]
    env: bool,
    
    #[argument(flag, name = "DEBUG", description = "Enable debug mode", excludes = "quiet")]
    debug: bool,
    
    #[argument(flag, name = "QUIET", description = "Suppress output", excludes = "debug")]
    quiet: bool,
    
    #[argument(key_value, name = "USER", description = "Username for authentication")]
    user: Option<String>,
    
    #[argument(key_value, name = "HOST", description = "Host to connect to", depends_on = "user")]
    host: Option<String>,
    
    #[argument(key_value, name = "MESSAGE", description = "Message to display", required = true)]
    message: String,
}

fn main() -> Result<(), Error> {
    // Parse arguments directly into the struct
    let mut args = Arguments::default();
    let non_args = args.parse_arguments(std::env::args().skip(1))?;

    // Values are already in our args struct!
    println!("Arguments parsed successfully.");
    if args.env { println!("ENV: true"); }
    if args.debug { println!("DEBUG: true"); }
    if args.quiet { println!("QUIET: true"); }
    if let Some(user) = &args.user { println!("USER: {}", user); }
    if let Some(host) = &args.host { println!("HOST: {}", host); }
    println!("MESSAGE: {}", args.message);
    if !non_args.is_empty() { println!("Non-Argument Text: {}", non_args.join(" ")); }

    Ok(())
}
```

**Sample Command:**

```bash
cargo run -- ENV DEBUG USER=admin HOST=localhost MESSAGE="Hello World" and some extra text
```

**Explanation:**

This basic example demonstrates:

- **Flags:** `ENV`, `DEBUG`
- **Key-Value Pairs:** `USER=admin`, `HOST=localhost`, `MESSAGE="Hello World"`
- **Dependencies:** `HOST` depends on `USER`. If `HOST` is provided without `USER`, parsing will fail.
- **Exclusions:** `DEBUG` and `QUIET` cannot both be set. Attempting to set both will result in an error.
- **Required Argument:** `MESSAGE` is required and must be provided.
- **Non-Argument Text:** `and some extra text` is collected as additional text that doesn't match any argument patterns.

## Advanced Example

This example showcases more complex features including argument dependencies, exclusions, and more detailed validation. We'll demonstrate how to implement this using both the direct binding approach and the derive macro approach.

### Using Direct Field Binding

```rust
use pam_args_rs::{ArgumentParser, Flag, KeyValue, Error, AllowedKeyValueFormats};
use std::str::FromStr;

// Define a struct to hold our configuration
#[derive(Debug, Default)]
struct AdvancedArguments {
    env: bool,
    debug: bool,
    quiet: bool,
    user: Option<String>,
    host: Option<String>,
    message: String,  // Required
    width: Option<i32>,
    align: Option<String>,
    chr: Option<char>,
    border: bool,
}

fn main() -> Result<(), Error> {
    // Create an instance of our config struct
    let mut args = AdvancedArguments::default();
    let mut message = String::new(); // For required field
    
    // Create a new parser with direct field binding
    let parser = ArgumentParser::new()
        .case_sensitive(true)
        .collect_non_argument_text(true)
        // Flags with direct binding
        .flag(Flag::new("ENV", "Enable environment variables")
            .bind_to(&mut args.env))
        .flag(Flag::new("DEBUG", "Enable debug mode")
            .excludes("QUIET")
            .bind_to(&mut args.debug))
        .flag(Flag::new("QUIET", "Suppress output")
            .excludes("DEBUG")
            .bind_to(&mut args.quiet))
        .flag(Flag::new("BORDER", "Add border to output")
            .bind_to(&mut args.border))
        
        // Key-value pairs with direct binding
        .key_value(
            KeyValue::new("USER", "Username for authentication")
                .type_converter(String::from_str)
                .allowed_formats(&[AllowedKeyValueFormats::KeyValue])
                .bind_to(&mut args.user)
        )
        .key_value(
            KeyValue::new("HOST", "Host to connect to")
                .type_converter(String::from_str)
                .depends_on("USER")
                .allowed_formats(&[AllowedKeyValueFormats::KeyValue])
                .bind_to(&mut args.host)
        )
        .key_value(
            KeyValue::new("WIDTH", "Width of the output")
                .type_converter(i32::from_str)
                .allowed_formats(&[AllowedKeyValueFormats::KeyValue])
                .bind_to(&mut args.width)
        )
        .key_value(
            KeyValue::new("ALIGN", "Alignment (LEFT, CENTER, RIGHT)")
                .type_converter(String::from_str)
                .allowed_values(&["LEFT", "CENTER", "RIGHT"])
                .allowed_formats(&[AllowedKeyValueFormats::KeyValue])
                .bind_to(&mut args.align)
        )
        .key_value(
            KeyValue::new("CHR", "Character to use for padding")
                .type_converter(char::from_str)
                .allowed_formats(&[AllowedKeyValueFormats::KeyValue])
                .bind_to(&mut args.chr)
        )
        .key_value(
            KeyValue::new("MESSAGE", "Message to display")
                .type_converter(String::from_str)
                .required()
                .allowed_formats(&[AllowedKeyValueFormats::KeyValue])
                .bind_to(&mut message)
        );
    
    // Parse and get non-argument text
    let result = parser.parse(std::env::args().skip(1))?;
    let non_args = result.non_argument_text();
    
    // Transfer the required field to our struct
    args.message = message;

    // Values are already in our args struct!
    println!("Arguments parsed successfully.");
    if args.env { println!("ENV: true"); }
    if args.debug { println!("DEBUG: true"); }
    if args.quiet { println!("QUIET: true"); }
    if args.border { println!("BORDER: true"); }
    if let Some(user) = &args.user { println!("USER: {}", user); }
    if let Some(host) = &args.host { println!("HOST: {}", host); }
    if let Some(width) = args.width { println!("WIDTH: {}", width); }
    if let Some(align) = &args.align { println!("ALIGN: {}", align); }
    if let Some(chr) = args.chr { println!("CHR: {}", chr); }
    println!("MESSAGE: {}", args.message);
    if !non_args.is_empty() { println!("Non-Argument Text: {}", non_args.join(" ")); }

    Ok(())
}
```

### Using Derive Macro

```rust
use pam_args_rs::{Error, ArgumentParser, AllowedKeyValueFormats};
use std::str::FromStr;

// Define a struct with derive macro for argument parsing
#[derive(Debug, Default, ArgumentParser)]
#[parser(case_sensitive = true, collect_non_argument_text = true)]
struct AdvancedArguments {
    #[argument(flag, name = "ENV", description = "Enable environment variables")]
    env: bool,
    
    #[argument(flag, name = "DEBUG", description = "Enable debug mode", excludes = "quiet")]
    debug: bool,
    
    #[argument(flag, name = "QUIET", description = "Suppress output", excludes = "debug")]
    quiet: bool,
    
    #[argument(flag, name = "BORDER", description = "Add border to output")]
    border: bool,
    
    #[argument(key_value, name = "USER", description = "Username for authentication", 
              allowed_formats = "KeyValue")]
    user: Option<String>,
    
    #[argument(key_value, name = "HOST", description = "Host to connect to", 
              depends_on = "user", allowed_formats = "KeyValue")]
    host: Option<String>,
    
    #[argument(key_value, name = "WIDTH", description = "Width of the output", 
              allowed_formats = "KeyValue")]
    width: Option<i32>,
    
    #[argument(key_value, name = "ALIGN", description = "Alignment (LEFT, CENTER, RIGHT)", 
              allowed_values = ["LEFT", "CENTER", "RIGHT"], allowed_formats = "KeyValue")]
    align: Option<String>,
    
    #[argument(key_value, name = "CHR", description = "Character to use for padding", 
              allowed_formats = "KeyValue")]
    chr: Option<char>,
    
    #[argument(key_value, name = "MESSAGE", description = "Message to display", 
              required = true, allowed_formats = "KeyValue")]
    message: String,
}

fn main() -> Result<(), Error> {
    // Parse arguments directly into the struct
    let mut args = AdvancedArguments::default();
    let non_args = args.parse_arguments(std::env::args().skip(1))?;

    // Values are already in our args struct!
    println!("Arguments parsed successfully.");
    if args.env { println!("ENV: true"); }
    if args.debug { println!("DEBUG: true"); }
    if args.quiet { println!("QUIET: true"); }
    if args.border { println!("BORDER: true"); }
    if let Some(user) = &args.user { println!("USER: {}", user); }
    if let Some(host) = &args.host { println!("HOST: {}", host); }
    if let Some(width) = args.width { println!("WIDTH: {}", width); }
    if let Some(align) = &args.align { println!("ALIGN: {}", align); }
    if let Some(chr) = args.chr { println!("CHR: {}", chr); }
    println!("MESSAGE: {}", args.message);
    if !non_args.is_empty() { println!("Non-Argument Text: {}", non_args.join(" ")); }

    Ok(())
}
```

**Sample Command:**

```bash
cargo run -- ENV DEBUG USER=admin HOST=localhost WIDTH=80 ALIGN=CENTER CHR=* BORDER MESSAGE="Hello World" and some extra text
```

**Explanation:**

This advanced example demonstrates:

- **Flags:** `ENV`, `DEBUG`, `BORDER`
- **Key-Value Pairs with Type Conversion:** 
  - `USER=admin` (String)
  - `HOST=localhost` (String)
  - `WIDTH=80` (Integer)
  - `ALIGN=CENTER` (String with allowed values)
  - `CHR=*` (Character)
  - `MESSAGE="Hello World"` (String)
- **Dependencies:** `HOST` depends on `USER`. If `HOST` is provided without `USER`, parsing will fail.
- **Exclusions:** `DEBUG` and `QUIET` cannot both be set. Attempting to set both will result in an error.
- **Required Argument:** `MESSAGE` is required and must be provided.
- **Type Safety:** Each argument is properly converted to its appropriate Rust type.
- **Value Validation:** `ALIGN` can only be one of `LEFT`, `CENTER`, or `RIGHT`.
- **Non-Argument Text:** `and some extra text` is collected as additional text.

## Multi Key-Value Pairs Example

This example demonstrates how to handle dynamic key-value pairs that are not known in advance.

```rust
use pam_args_rs::{ArgumentParser, KeyValueStore, Error, AllowedKeyValueFormats};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    // Create a new parser that collects all key-value pairs
    let matches = ArgumentParser::new()
        .case_sensitive(true)
        .collect_non_argument_text(true)
        .enable_multi_key_value(true)
        .multi_key_value_format(&[
            AllowedKeyValueFormats::KeyValue,
            AllowedKeyValueFormats::KeyOnly,
            AllowedKeyValueFormats::KeyEquals
        ])
        .parse(std::env::args().skip(1))?;
    
    // Access the key-value store
    let store = matches.key_value_store();
    let non_args = matches.non_argument_text();

    // Use the parsed arguments
    println!("Arguments parsed successfully.");
    
    // Access Multiple Key-Value Pairs
    if let Some(auth_method) = store.get("PAM_AUTH_METHODS") {
        println!("Auth Method: {}", auth_method);
    }

    if store.has_key("PAM_DELETE") {
        println!("Delete Key: Present (no value)");
    }

    if let Some(empty_val) = store.get("PAM_EMPTY") {
        println!("Empty Key: '{}'", empty_val);
    }

    if !non_args.is_empty() { 
        println!("Non-arguments: {}", non_args.join(" ")); 
    }

    Ok(())
}
```

**Sample Command:**

```bash
cargo run -- [PAM_AUTH_METHODS=password\,keyboard-interactive,PAM_DELETE,PAM_EMPTY=] and this is the rest
```

**Explanation:**

This example demonstrates how to handle multi key-value pairs with different formats:

- **KeyValue:** `PAM_AUTH_METHODS="password,keyboard-interactive"` - A standard key-value pair where both the key and the value are specified. Note the escaped comma that preserves it within the value.
- **KeyOnly:** `PAM_DELETE` - A key-only pair that signifies a flag-like presence without a value. The parser tracks the presence of this key.
- **KeyEquals:** `PAM_EMPTY=` - A key with an equals sign but without a value. The parser sets the value associated with this key to an empty string `""`.
- **Non-Argument Text:** `and this is the rest` is collected as text that doesn't match any argument patterns.

## Handling Delimited Text Example

This example demonstrates how `pam-args` handles complex arguments with various delimiters and special characters.

```rust
use pam_args_rs::{ArgumentParser, Flag, KeyValue, Error, AllowedKeyValueFormats};
use std::str::FromStr;

#[derive(Debug, Default)]
struct DelimitedArguments {
    env: bool,
    debug: bool,
    user: Option<String>,
    host: Option<String>,
    message: String,
}

fn main() -> Result<(), Error> {
    // Create a parser that handles complex delimited arguments
    let matches = ArgumentParser::new()
        .case_sensitive(true)
        .collect_non_argument_text(true)
        .enable_multi_key_value(true)
        .multi_key_value_format(&[
            AllowedKeyValueFormats::KeyValue, 
            AllowedKeyValueFormats::KeyOnly, 
            AllowedKeyValueFormats::KeyEquals
        ])
        .flag(Flag::new("ENV", "Enable environment variables"))
        .flag(Flag::new("DEBUG", "Enable debug mode"))
        .key_value(
            KeyValue::new("USER", "Username for authentication")
                .type_converter(String::from_str)
        )
        .key_value(
            KeyValue::new("HOST", "Host to connect to")
                .type_converter(String::from_str)
        )
        .key_value(
            KeyValue::new("MESSAGE", "Message to display")
                .type_converter(String::from_str)
                .required()  // Mark MESSAGE as required
        )
        .parse(std::env::args().skip(1))?;
    
    // Extract values
    let args = DelimitedArguments {
        env: matches.is_present("ENV"),
        debug: matches.is_present("DEBUG"),
        user: matches.value_of::<String>("USER"),
        host: matches.value_of::<String>("HOST"),
        message: matches.value_of::<String>("MESSAGE").expect("MESSAGE is required"),
    };
    
    let store = matches.key_value_store();
    let non_args = matches.non_argument_text();

    // Output the results
    println!("Arguments parsed successfully.");
    if args.env { println!("ENV: true"); }
    if args.debug { println!("DEBUG: true"); }
    if let Some(user) = &args.user { println!("USER: {}", user); }
    if let Some(host) = &args.host { println!("HOST: {}", host); }
    println!("MESSAGE: {}", args.message);
    
    // Print multi key-value pairs
    for key in store.keys() {
        if key != "USER" && key != "HOST" && key != "MESSAGE" {
            if let Some(value) = store.get(key) {
                println!("Multi KV - {}: {}", key, value);
            } else {
                println!("Multi KV - {}: (no value)", key);
            }
        }
    }
    
    if !non_args.is_empty() { 
        println!("Non-Argument Text: {}", non_args.join(" ")); 
    }

    Ok(())
}
```

**Sample Command:**

```bash
cargo run -- [ENV] DEBUG [USER=admin] HOST=localhost MESSAGE="'Hello' [], \"World\"" and quiet some 'extra \'text\'' [PAM_AUTH_METHODS=password\,keyboard-interactive,PAM_DELETE,PAM_EMPTY=,WITH_BRACKETS=\[\]] and this is the rest ", with 'more'\\"text\" in \[brackets\][]"
```

**Explanation:**

This example showcases the library's ability to handle complex delimited text:

1. **Bracketed Arguments:** Arguments enclosed in brackets like `[ENV]` or `[USER=admin]` are properly parsed.
2. **Quoted Strings:** Both single and double quotes are respected, including nested quotes, as in `MESSAGE="'Hello' [], \"World\""`.
3. **Escaped Characters:** Backslash-escaped special characters are properly handled, such as in `'extra \'text\''`.
4. **Multi Key-Value Pairs:** Complex grouped key-value pairs in brackets like `[PAM_AUTH_METHODS=password\,keyboard-interactive,PAM_DELETE,PAM_EMPTY=,WITH_BRACKETS=\[\]]` are parsed correctly.
5. **Non-Argument Text:** Complex text with quotes and special characters is captured accurately.

## Testing Your Argument Parser

This example demonstrates how to write tests for your argument parser using Rust's testing framework.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use pam_args_rs::{ArgumentParser, Flag, KeyValue, Error};
    use std::str::FromStr;

    // Helper function to create a parser for testing
    fn create_test_parser() -> ArgumentParser {
        ArgumentParser::new()
            .case_sensitive(true)
            .collect_non_argument_text(true)
            .flag(Flag::new("ENV", "Enable environment variables"))
            .flag(Flag::new("DEBUG", "Enable debug mode").excludes("QUIET"))
            .flag(Flag::new("QUIET", "Suppress output").excludes("DEBUG"))
            .key_value(
                KeyValue::new("USER", "Username for authentication")
                    .type_converter(String::from_str)
            )
            .key_value(
                KeyValue::new("HOST", "Host to connect to")
                    .type_converter(String::from_str)
                    .depends_on("USER")
            )
            .key_value(
                KeyValue::new("MESSAGE", "Message to display")
                    .type_converter(String::from_str)
                    .required()
            )
    }

    #[test]
    fn test_basic_parsing() -> Result<(), Error> {
        let args = vec!["ENV", "USER=admin", "MESSAGE=Hello"];
        let matches = create_test_parser().parse(args)?;
        
        assert!(matches.is_present("ENV"));
        assert!(!matches.is_present("DEBUG"));
        assert_eq!(matches.value_of::<String>("USER").unwrap(), "admin");
        assert_eq!(matches.value_of::<String>("MESSAGE").unwrap(), "Hello");
        assert!(matches.value_of::<String>("HOST").is_none());
        
        Ok(())
    }

    #[test]
    fn test_required_argument() {
        let args = vec!["ENV", "USER=admin"]; // Missing required MESSAGE
        let result = create_test_parser().parse(args);
        
        assert!(result.is_err());
        match result {
            Err(Error::RequiredArgMissing(arg)) => assert_eq!(arg, "MESSAGE"),
            _ => panic!("Expected RequiredArgMissing error"),
        }
    }

    #[test]
    fn test_dependency() {
        let args = vec!["HOST=localhost", "MESSAGE=Hello"]; // Missing USER which HOST depends on
        let result = create_test_parser().parse(args);
        
        assert!(result.is_err());
        match result {
            Err(Error::DependencyNotMet(arg, dep)) => {
                assert_eq!(arg, "HOST");
                assert_eq!(dep, "USER");
            },
            _ => panic!("Expected DependencyNotMet error"),
        }
    }

    #[test]
    fn test_exclusion() {
        let args = vec!["DEBUG", "QUIET", "MESSAGE=Hello"]; // DEBUG and QUIET are mutually exclusive
        let result = create_test_parser().parse(args);
        
        assert!(result.is_err());
        match result {
            Err(Error::MutuallyExclusiveArgs(arg1, arg2)) => {
                assert!((arg1 == "DEBUG" && arg2 == "QUIET") || 
                        (arg1 == "QUIET" && arg2 == "DEBUG"));
            },
            _ => panic!("Expected MutuallyExclusiveArgs error"),
        }
    }

    #[test]
    fn test_non_argument_text() -> Result<(), Error> {
        let args = vec!["ENV", "MESSAGE=Hello", "some", "extra", "text"];
        let matches = create_test_parser().parse(args)?;
        
        let non_args = matches.non_argument_text();
        assert_eq!(non_args, vec!["some", "extra", "text"]);
        
        Ok(())
    }
}
```

**Explanation:**

This testing example demonstrates:

1. **Unit Tests for Argument Parsing:** How to create focused tests that verify specific aspects of the argument parser.
2. **Error Handling Tests:** Verifying that errors are properly reported when constraints are violated.
3. **Test Helper Functions:** Creating reusable test fixtures to reduce duplication.
4. **Pattern Matching on Errors:** Using Rust's pattern matching to verify specific error types and their contents.
5. **Result Type in Tests:** Using Rust's `Result` type to make tests more concise for the success path.

## Integration with Actual PAM Modules

This example shows how to integrate `pam-args` with a Rust PAM module.

```rust
use pam_args_rs::{ArgumentParser, Flag, KeyValue, Error};
use std::str::FromStr;
use pam_rs::{module, constants::PamResultCode, conv::Conversation, types::PamHandle};

#[derive(Debug, Default)]
struct PamArguments {
    debug: bool,
    user: Option<String>,
    service: Option<String>,
    likeauth: bool,
}

#[module]
fn pam_sm_authenticate(
    pamh: &PamHandle,
    args: Vec<String>,
    _flags: u32,
) -> PamResultCode {
    // Parse the arguments using pam-args
    let parser = ArgumentParser::new()
        .case_sensitive(false)
        .flag(Flag::new("debug", "Enable debug output"))
        .flag(Flag::new("likeauth", "Return auth result for account management"))
        .key_value(
            KeyValue::new("user", "Username to authenticate")
                .type_converter(String::from_str)
        )
        .key_value(
            KeyValue::new("service", "Service name")
                .type_converter(String::from_str)
        );
    
    let matches = match parser.parse(args) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error parsing PAM arguments: {}", e);
            return PamResultCode::SERVICE_ERR;
        }
    };
    
    // Extract values
    let config = PamArguments {
        debug: matches.is_present("debug"),
        user: matches.value_of::<String>("user"),
        service: matches.value_of::<String>("service"),
        likeauth: matches.is_present("likeauth"),
    };
    
    // Use the configuration in the PAM module
    if config.debug {
        eprintln!("Debug mode enabled");
        eprintln!("Configuration: {:?}", config);
    }
    
    // Implementation of the PAM module logic...
    
    PamResultCode::SUCCESS
}
```

**Explanation:**

This integration example demonstrates:

1. **PAM Module Integration:** How to use `pam-args` within a Rust PAM module.
2. **Error Handling:** Converting parsing errors to appropriate PAM result codes.
3. **Case Insensitivity:** Setting the parser to be case-insensitive to match PAM's typical behavior.
4. **Configuration Extraction:** Using the parsed arguments to build a configuration object.
5. **Debug Output:** Conditionally enabling debug output based on arguments.

## Example: Using the Builder Pattern for Cleaner Code

This example demonstrates how to use Rust's builder pattern to create a cleaner, more maintainable argument parser configuration.

```rust
use pam_args_rs::{ArgumentParserBuilder, Flag, KeyValue, Error};
use std::str::FromStr;

// Define a struct to hold our configuration
#[derive(Debug, Default)]
struct MyConfig {
    debug: bool,
    user: Option<String>,
    width: Option<i32>,
    message: String,
}

// Create a function that returns a configured parser
fn create_parser() -> ArgumentParserBuilder {
    ArgumentParserBuilder::new()
        .case_sensitive(false)
        .collect_non_argument_text(true)
        .with_flag(Flag::new("debug", "Enable debug mode"))
        .with_key_value(
            KeyValue::new("user", "Username")
                .type_converter(String::from_str)
        )
        .with_key_value(
            KeyValue::new("width", "Output width")
                .type_converter(i32::from_str)
        )
        .with_key_value(
            KeyValue::new("message", "Message to display")
                .type_converter(String::from_str)
                .required()
        )
}

fn main() -> Result<(), Error> {
    // Create the parser from the builder and parse arguments
    let matches = create_parser()
        .build()
        .parse(std::env::args().skip(1))?;
    
    // Create our configuration
    let config = MyConfig {
        debug: matches.is_present("debug"),
        user: matches.value_of::<String>("user"),
        width: matches.value_of::<i32>("width"),
        message: matches.value_of::<String>("message").expect("MESSAGE is required"),
    };
    
    // Use the configuration
    println!("Configuration: {:?}", config);
    
    Ok(())
}
```

**Explanation:**

This builder pattern example demonstrates:

1. **Separation of Concerns:** Moving parser configuration into a separate function.
2. **Builder Pattern:** Using a dedicated builder struct to construct the parser.
3. **Method Chaining:** Fluent interface for configuring the parser.
4. **Reusability:** The parser configuration can be reused across different parts of the application.
5. **Readability:** The code is more concise and easier to understand.

## Comparing the Approaches

### When to Use Each Approach

1. **Traditional Builder Pattern:**
   - When you need maximum flexibility and control
   - When your argument parsing logic is dynamically determined at runtime
   - When you're working in environments where proc macros aren't available

2. **Direct Field Binding:**
   - When you want to avoid manual extraction of values after parsing
   - When you have a fixed set of arguments but still need fine-grained control
   - For iterative development where you might add or remove arguments frequently

3. **Derive Macro:**
   - For the cleanest, most concise code
   - When your argument structure is stable and well-defined
   - For quick development of simple argument parsers
   - When you want your code to be self-documenting

### Comparison Table

| Feature | Traditional Builder | Direct Binding | Derive Macro |
|---------|---------------------|----------------|--------------|
| Code Verbosity | High | Medium | Low |
| Flexibility | High | High | Medium |
| Clarity | Medium | High | Very High |
| Runtime Configuration | Yes | Yes | Limited |
| Learning Curve | Gradual | Moderate | Steeper initially |
| Best For | Complex logic | Balance of convenience and control | Maximum simplicity |

## Summary

These examples demonstrate the key features of the `pam-args` library, showcasing how to use it in various scenarios. The library provides a flexible, type-safe approach to argument parsing that leverages Rust's powerful type system and ownership model to offer a safe and ergonomic API.

With three different ways to interact with the library, `pam-args` offers options for different styles of coding and different use cases:

1. **Type Safety:** All approaches leverage Rust's type system to provide compile-time type checking.
2. **Result-Based Error Handling:** Errors are handled using Rust's `Result` type, making error handling explicit and composable.
3. **Multiple API Styles:** Choose between the traditional builder pattern, direct field binding, or derive macros to match your preferred coding style.
4. **Ownership Model:** The library takes advantage of Rust's ownership model to prevent memory leaks and use-after-free errors.
5. **Testing:** The library is designed to be easily testable, with examples showing how to write unit tests for your argument parsing logic.

For more information, refer to the API documentation and other guides in the documentation directory.