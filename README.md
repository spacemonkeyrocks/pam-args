# pam-args - PAM Arguments Parser for Rust

## Overview

The `pam-args` crate provides a flexible and type-safe command-line argument parser specifically designed for PAM (Pluggable Authentication Modules) modules in Rust. It leverages Rust's powerful type system and ownership model to offer a safe and ergonomic API for parsing various argument types, including flags and key-value pairs, with values of different types such as strings, characters, integers, and booleans. The library includes robust error handling for required arguments, argument dependencies, exclusions, and allowed values, all wrapped in Rust's `Result` type for predictable error handling. Additionally, it offers configurable logging integration through the `log` crate.

## Quick Start

Add `pam-args` to your `Cargo.toml`:

```toml
[dependencies]
pam-args = "0.1.0"

# If you want to use the derive macro feature
pam-args = { version = "0.1.0", features = ["derive"] }
```

This library is primarily designed to be used by PAM modules.

## Multi-Stage Parsing Approach

The `pam-args` library uses a carefully designed multi-stage parser that:

1. Processes explicit flags and key-value pairs first
2. Handles dynamic multi key-value pairs if enabled
3. Collects non-argument text if enabled
4. Validates dependencies, exclusions, and required arguments
5. Automatically manages memory through Rust's ownership system

This approach ensures precise control over argument processing while maintaining a flexible API.

## Basic Example with Direct Field Binding

```rust
use pam_args_rs::{ArgumentParser, Flag, KeyValue, Error};
use std::str::FromStr;

// Define a struct to hold our configuration
#[derive(Debug, Default)]
struct Arguments {
    pad: bool,
    chr: Option<char>,
    width: Option<i32>,
    align: Option<String>,
    border: bool,
    env: bool,
    debug: bool,
    quiet: bool,
    message: String,  // Required argument
}

fn main() -> Result<(), Error> {
    // Create an instance of our config struct
    let mut args = Arguments::default();
    let mut message = String::new(); // For required field
    
    // Create a new parser with direct field binding
    let parser = ArgumentParser::new()
        .collect_non_argument_text(true) // Enable collecting non-argument text
        .flag(Flag::new("PAD", "Apply padding to output")
            .bind_to(&mut args.pad))
        .key_value(
            KeyValue::new("CHR", "Character to use for padding")
                .type_converter(char::from_str)
                .bind_to(&mut args.chr)
        )
        .key_value(
            KeyValue::new("WIDTH", "Width of the output")
                .type_converter(i32::from_str)
                .bind_to(&mut args.width)
        )
        .key_value(
            KeyValue::new("ALIGN", "Alignment (LEFT, CENTER, RIGHT)")
                .type_converter(String::from_str)
                .allowed_values(&["LEFT", "CENTER", "RIGHT"])
                .bind_to(&mut args.align)
        )
        .flag(Flag::new("BORDER", "Add border to output")
            .bind_to(&mut args.border))
        .flag(Flag::new("ENV", "Use environment variables")
            .bind_to(&mut args.env))
        .flag(Flag::new("DEBUG", "Enable debug mode")
            .bind_to(&mut args.debug))
        .flag(Flag::new("QUIET", "Suppress output")
            .bind_to(&mut args.quiet))
        .key_value(
            KeyValue::new("MESSAGE", "Message to display")
                .type_converter(String::from_str)
                .required()
                .bind_to(&mut message)
        )
        .conflicts("DEBUG", "QUIET");
    
    // Parse and get non-argument text
    let parse_result = parser.parse(std::env::args().skip(1))?;
    let non_args = parse_result.non_argument_text();
    
    // Transfer the required field to our struct
    args.message = message;

    // Values are already in our args struct!
    println!("Arguments parsed successfully.");
    if args.pad { println!("PAD: true"); }
    if let Some(chr) = args.chr { println!("CHR: {}", chr); }
    if let Some(width) = args.width { println!("WIDTH: {}", width); }
    if let Some(align) = &args.align { println!("ALIGN: {}", align); }
    if args.border { println!("BORDER: true"); }
    if args.env { println!("ENV: true"); }
    if args.debug { println!("DEBUG: true"); }
    if args.quiet { println!("QUIET: true"); }
    println!("MESSAGE: {}", args.message);
    if !non_args.is_empty() { println!("Non-Argument Text: {}", non_args.join(" ")); }

    Ok(())
}
```

## Features

The `pam-args` crate provides a robust set of features for parsing and managing command-line arguments in PAM (Pluggable Authentication Modules) modules. It offers a type-safe, memory-safe approach to argument handling through Rust's ownership system and type system.

Key capabilities include:

| Name| Description| Example| Important|
|---|---|---|---|
| **Type-Safe Argument Handling**| Leverages Rust's type system to ensure arguments are correctly typed at compile time.| Flags: `env`, `debug`<br>Key-Value Pairs: `user="admin"`, `width=80`| Prevents type errors at compile time, eliminating an entire class of runtime errors.|
| **Sum Types for Arguments**| Uses Rust's enums to represent different argument types in a type-safe manner.| `Argument::Flag("debug")`<br>`Argument::KeyValue("user", "admin")`| Ensures arguments are correctly handled based on their type.|
| **Strong Type Conversion**| Automatically converts string values to appropriate Rust types using the `FromStr` trait.| String: `message="Hello World"`<br>Integer: `width=100`<br>Boolean: `enabled=true`| Simplifies usage by handling type conversion in a safe, predictable way.|
| **Result-Based Error Handling** | Uses Rust's `Result` type for error handling instead of error codes. | `parser.parse().map_err(|e| log::error!("Parsing error: {}", e))?;` | Makes error handling explicit and impossible to ignore, with proper propagation via `?` operator and pattern matching for specific errors. |
| **Builder Pattern**| Uses the builder pattern for constructing parsers with complex configurations.| `ArgumentParserBuilder::new().with_case_sensitive(false).build()`| Provides a clear, fluent API for configuring the parser.|
| **Integration with Logging**| Integrates with Rust's standard `log` crate.| `debug!("Parsing arguments: {:?}", args)`| Enables flexible logging that works with any `log` implementation.|
| **Zero-Copy Parsing**| Uses Rust's borrowing system to avoid unnecessary copies.| Argument keys and values borrow from the original input when possible.| Improves performance by reducing memory allocations.|
| **Thread Safety**| Types implement `Send` and `Sync` for thread-safe operation where appropriate.| Use `ArgumentParser` across threads with `Arc`.| Enables safe concurrent use in multi-threaded environments.|
| **Direct Field Binding**| Bind arguments directly to struct fields while building the parser.| `Flag::new("debug").bind_to(&mut config.debug)`| Eliminates manual extraction of values after parsing.|
| **Derive Macro Support**| Generate argument parsers automatically from struct definitions with attributes.| `#[derive(ArgumentParser)]`<br>`struct Config { ... }`| Dramatically reduces boilerplate for common cases.|

## Multiple API Styles

The library offers flexible ways to define and use argument parsers, with the direct field binding approach providing an excellent balance of clarity and control:

```rust
// Direct Field Binding approach
let mut config = MyConfig::default();

let parser = ArgumentParser::new()
    .flag(Flag::new("debug", "Enable debugging")
        .bind_to(&mut config.debug))
    .key_value(KeyValue::new("user", "Username")
        .required()
        .bind_to(&mut config.username));

parser.parse(args)?;
// Values are automatically populated in the config struct
```

For other approaches, including the traditional builder pattern and derive macro approach, see [EXAMPLES.md](EXAMPLES.md) which provides comprehensive examples of all available methods.

## PAM Module Integration Example

This example shows how to integrate `pam-args` with a Rust PAM module using direct field binding:

```rust
use pam_args_rs::{ArgumentParser, Flag, KeyValue, Error};
use std::str::FromStr;
use pam_rs::{module, constants::PamResultCode, types::PamHandle};

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
    // Create our configuration instance
    let mut config = PamArguments::default();
    
    // Parse the arguments using pam-args with direct field binding
    let parser = ArgumentParser::new()
        .case_sensitive(false)
        .flag(Flag::new("debug", "Enable debug output")
            .bind_to(&mut config.debug))
        .flag(Flag::new("likeauth", "Return auth result for account management")
            .bind_to(&mut config.likeauth))
        .key_value(
            KeyValue::new("user", "Username to authenticate")
                .type_converter(String::from_str)
                .bind_to(&mut config.user)
        )
        .key_value(
            KeyValue::new("service", "Service name")
                .type_converter(String::from_str)
                .bind_to(&mut config.service)
        );
    
    // Parse arguments and handle errors
    match parser.parse(args) {
        Ok(_) => {
            // Successfully parsed arguments
            if config.debug {
                eprintln!("Debug mode enabled");
                eprintln!("Configuration: {:?}", config);
            }
            
            // Implementation of the PAM module logic...
            
            PamResultCode::SUCCESS
        },
        Err(e) => {
            eprintln!("Error parsing PAM arguments: {}", e);
            PamResultCode::SERVICE_ERR
        }
    }
}
```

## Documentation

The following documents are available for reference:
- [Examples](EXAMPLES.md): Provides practical usage examples of the `pam-args` crate, demonstrating various scenarios and configurations to help users understand how to implement and utilize the library effectively.
- [Specification](SPECIFICATION.md): Details the functional requirements and behavior of the library, serving as a comprehensive reference for understanding expected behavior.
- [Design](DESIGN.md): Explains the architectural and internal design principles of the `pam-args` crate, detailing how different components interact and the rationale behind key design decisions.
- [API Documentation](https://docs.rs/pam-args): Comprehensive API documentation generated from doc comments in the source code.
- [Contribution](CONTRIBUTION.md): Outlines the guidelines and best practices for contributing to the `pam-args` crate, including code standards, submission processes, and community expectations to foster collaborative development.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.