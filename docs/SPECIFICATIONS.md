# Specifications for pam-args

## Introduction

Welcome to the **Specifications** document for the `pam-args` library. This comprehensive guide outlines the functional and technical requirements that define the behavior and capabilities of the library. Here, you'll find detailed descriptions of argument structures, including the argument configuration, supported argument and variable types, and the various formats for key-value pairs. The specifications also cover the parsing rules that govern how arguments are interpreted and processed, ensuring consistent and reliable behavior across different use cases. Additionally, this document elaborates on the validation mechanisms that enforce required arguments, manage dependencies and exclusions, and restrict values to predefined sets. Error handling protocols are also thoroughly described, providing clear guidance on how the library responds to different parsing failures. By adhering to these specifications, developers can effectively utilize and extend the `pam-args` library, ensuring robust argument parsing and management within their Rust PAM modules.

## Argument Types and Parsing

### Argument Definition

In `pam-args`, arguments are defined using dedicated structs like `Flag` and `KeyValue` that provide a type-safe interface for specifying command-line arguments. This approach leverages Rust's type system to ensure correctness at compile time.

```rust
// Flag definition
Flag::new("DEBUG", "Enable debug mode")
    .excludes("QUIET")
    .bind_to(&mut config.debug)

// Key-Value definition
KeyValue::new("USER", "Username for authentication")
    .type_converter(String::from_str)
    .required()
    .depends_on("HOST")
    .allowed_values(&["admin", "user", "guest"])
    .allowed_formats(&[AllowedKeyValueFormats::KeyValue])
    .bind_to(&mut config.user)
```

> **IMPORTANT**: Unlike the C library where an `is_set` field is managed by the parser, in Rust this state tracking is handled internally through the ownership system and only exposed through methods like `is_present()`.

In addition to the explicitly defined arguments, the parser supports two special types:

1. **Multi Key-Value Pairs**: When you want to parse dynamic key-value pairs that are not known in advance, you can use this option.
   - By default, it is disabled. To enable it, call `enable_multi_key_value(true)`. Any argument that is not specified explicitly will be added to the multi key-value store.
   - The parser automatically detects the key-value pair format (`KeyValue`, `KeyEquals`, `KeyOnly`) based on the presence or absence of the equals sign and the value. This behavior is controlled using the `multi_key_value_format()` method, which works similarly to the `allowed_formats()` method of the `KeyValue` struct.

2. **Non-Argument Text**: Any arguments that are not explicitly defined and don't match the multi key-value pair format will be collected into a vector of strings. This text can be accessed through the `non_argument_text()` method.
   - **The parser automatically detects non-argument text and collects it if non-argument text collection is enabled. It is disabled by default.**

**Important Notes:**
- When using Multi Key-Value Pairs and Non-Argument Text Collection together, and the `multi_key_value_format()` is used with `KeyOnly` or `KeyAll`, then the multi key-value pairs must be enclosed in brackets `[]`.
  - **Example:** `[KEY1=VALUE1,KEY2,KEY3=,KEY4=VALUE4] with some random text` would identify `KEY1`, `KEY2`, `KEY3`, and `KEY4` as multi key-value pairs and `with some random text` as non-argument text.
- When using Multi Key-Value Pairs and Non-Argument Text Collection together, and only `KeyValue` and `KeyEquals` are supported, then the multi key-value pairs don't need to be enclosed in brackets `[]`, as the parser can identify these key-value pairs based on the equals `=` sign.
  - **Example:** `KEY1=VALUE1,KEY2= with some random text` would identify `KEY1`, `KEY2` as multi key-value pairs and `with some random text` as non-argument text.
- When using Multi Key-Value Pairs and Non-Argument Text Collection together, the multi key-value pairs are processed first, followed by non-argument text.

### Argument Names

Argument names in `pam-args` are used to identify explicitly defined command-line arguments. By default, the parser treats them as case-sensitive, meaning that `ENV` and `env` are treated differently. **To change this behavior and make the parser case-insensitive for argument names, use `case_sensitive(false)`.** Note that this only affects argument names, not their values.

```rust
// This works with case_sensitive(true)
let parser = ArgumentParser::new()
    .case_sensitive(true)
    .flag(Flag::new("env", "Environment flag"))
    .flag(Flag::new("ENV", "Another environment flag"));

// It does not work with case_sensitive(false), an error will be thrown (Error::DuplicateArgName)
let parser = ArgumentParser::new()
    .case_sensitive(false)
    .flag(Flag::new("env", "Environment flag"))
    .flag(Flag::new("ENV", "Another environment flag")); // Error!
```

> **TIP:** To make the parser case-insensitive for argument names and their values, use `case_sensitive(false)` and `case_sensitive_values(false)`. This also means that it is easier to distinguish between flags and non-argument text.

**Example:**

- If `ENV` and `DEBUG` are defined as flags, the command `pam-debugplus.so ENV DEBUG This is a message with env and debug in it` will collect `ENV` and `DEBUG` as flags, and the text `This is a message with env and debug in it` as non-argument text.

### Argument Types

Argument types define the structure and behavior of the explicitly defined arguments. The library currently supports the following types:

- **Flags:** Binary flags that represent boolean options. Flags do not require a value.
  - **Example:** `DEBUG`, `QUIET`

- **Key-Value Pairs:** Pairs of keys and values, where the key is associated with a specific value.
  - **Example:** `USER=admin`, `WIDTH=80`

### Variable Types

Variable types specify the kind of data an argument can hold. These types are only applicable to key-value pairs:

- **Strings:** Text values that can include spaces and special characters.
  - **Example:** `MESSAGE="This is a message"`, `PATH='C:\Program Files'`

- **Characters:** Single character values.
  - **Example:** `CHR='A'`, `SYMBOL="!"`

- **Integers:** Numerical values representing integer options. Accepted values are signed and unsigned integers.
  - **Example:** `WIDTH=80`, `TIMEOUT=30`

- **Booleans:** String values representing boolean states within key-value pairs. Accepted values are `true`, `false`, `yes`, `no`, `1`, `0`.
  - **Example:** `ENABLED=true`, `VERBOSE=no`

**Important Notes:**

- In Rust, type conversion is handled through the `FromStr` trait, allowing for flexible and safe type conversion.
- Multi key-value pairs can support different types through explicit type conversion by the user after retrieval.

### Key-Value Pair Formats

Key-value pairs can be specified in different formats to support various use cases. These formats define how the key and value are separated and the interpretation of missing values:

| Format | Description | Example | Resulting Value |
|---|---|---|---|
| **KeyValue** | Standard key-value pair where both key and value are provided. | `USER=admin` | Value will be "admin" |
| **KeyOnly** | Keys without an equals sign or value, used to unset or mark the key for special handling. | `RESET` | Value will be None, indicating that the key is present without a value. |
| **KeyEquals** | Keys with an equals sign but no value, used to set a key to an empty value. | `KEY=` | Value will be an empty string "" |
| **KeyAll** | A convenience type that enables the KeyValue, KeyEquals, and KeyOnly formats for a particular argument. | `KEY1=value1`, `KEY2=`, `KEY3` | Value will be the appropriate value, or None if only the key is present without a value. |

**Important Notes:**

- The `allowed_formats()` method is used to specify the allowed formats for a specific key-value pair argument.
- You can combine multiple formats by passing them in an array, for example: `allowed_formats(&[AllowedKeyValueFormats::KeyValue, AllowedKeyValueFormats::KeyEquals])` to allow both `KEY=VALUE` and `KEY=` formats but not the `KEY` format.
- By default, if the `allowed_formats()` method is not called, the parser defaults to `KeyValue`.
- The `KeyAll` is a convenience type that is equivalent to setting all format types.
- If `allowed_formats()` or `multi_key_value_format()` is not set, it defaults to `KeyValue`.

**Example of combining specific formats:**

```rust
// Key-Value that only accepts KEY=VALUE and KEY= formats
KeyValue::new("USER", "Username for authentication")
    .type_converter(String::from_str)
    .allowed_formats(&[
        AllowedKeyValueFormats::KeyValue,  // Allow USER=admin
        AllowedKeyValueFormats::KeyEquals  // Allow USER=
        // KeyOnly format not included, so bare "USER" would be rejected
    ])
```

## Advanced Parsing Features

### Explicit Key-Value Pairs vs. Multi Key-Value Pairs

This section describes the difference between explicitly defined key-value pairs and implicitly defined multi key-value pairs.

**Explicit Key-Value Pairs** are defined explicitly using the `key_value()` method on the parser. The user provides type conversion functions through the `type_converter()` method, and the key-value pair format is explicitly set for every key-value pair argument using `allowed_formats()`.

**Multi Key-Value Pairs** are supported implicitly by calling `enable_multi_key_value(true)`. It defaults to `KeyValue`. It can be changed using `multi_key_value_format()` and works the same way as the `allowed_formats()` method.

**Comparison:**

| Format | Description | Example | Resulting Value |
|---|---|---|---|
| **KeyValue** | Standard key-value pair where both key and value are provided. | `USER=admin` | Value will be "admin" |
| **KeyOnly** | Keys without an equals sign or value, used to unset or mark the key for special handling. | `RESET` | Value will be None, indicating that the key is present without a value. |
| **KeyEquals** | Keys with an equals sign but no value, used to set a key to an empty value. | `KEY=` | Value will be an empty string "" |
| **KeyAll** | A convenience type that enables the KeyValue, KeyEquals, and KeyOnly formats for a particular argument. | `KEY1=value1`, `KEY2=`, `KEY3` | Value will be the appropriate value, or None if only the key is present without a value. |

**Important Notes:**

- **Explicit flags and key-value pairs are always processed first.** This means that the parser will first check if the argument is an explicitly defined flag or key-value pair. If it is, it will be processed according to the rules defined in the `Flag` or `KeyValue` struct. Otherwise, the parser will check if multi key-value pairs are enabled and if the argument is a valid multi key-value pair.
- **Order of Processing:**
  1. Flags and key-value pairs specified explicitly are processed first.
  2. If multi key-value pairs are enabled, any remaining arguments are processed as multi key-value pairs.
  3. If non-argument text collection is enabled, any remaining arguments are collected as non-argument text.

**Example comparing explicit and multi key-value pairs:**

```rust
// Setup with both explicit and multi key-value pairs
let parser = ArgumentParser::new()
    // Enable multi key-value pairs for any undefined arguments
    .enable_multi_key_value(true)
    .multi_key_value_format(&[AllowedKeyValueFormats::KeyValue]);
    // Define explicit key-value pair with strong typing and validation
    .key_value(
        KeyValue::new("HOST", "Host to connect to")
            .type_converter(String::from_str)
            .allowed_values(&["localhost", "example.com"])
    )

// Parse arguments
let matches = parser.parse(vec!["HOST=localhost", "PORT=8080"])?;

// Retrieve explicit key-value pair - with type conversion and validation
let host: String = matches.value_of("HOST").expect("HOST is required");
println!("Host: {}", host);  // "localhost" as a String

// Retrieve multi key-value pair - as a raw string from the key-value store
let store = matches.key_value_store();
if let Some(port_str) = store.get("PORT") {
    // Manual type conversion needed
    let port: u16 = port_str.parse().expect("Invalid port number");
    println!("Port: {}", port);  // 8080 as u16
}
```

In this example, `HOST` is an explicit key-value pair with type conversion and allowed values validation, while `PORT` is handled through the multi key-value store without predefined validation.

### Delimited Text

Delimiters are optional and useful to be explicit about what you want to be handled by the parser. To handle spaces and special characters in arguments, the library supports various delimiters:

- **Quoted Text:**
  - Enclose text in single `'` or double `"` quotes.
  - **Examples:**
    - `message="Value with spaces and 'quotes'"`
    - `message='Value with spaces and "quotes"'`
  - Quotes and backslashes can be included in the text by escaping them with a backslash `\`.
  - **Examples:**
    - `message="He said, \"Hello\""`
    - `message='It\'s a test'`
    - `message='It is a \\'`

- **Bracketed Text:**
  - Enclose text in square brackets `[ ]` to include spaces and special characters.
  - **Examples:**
    - `message=[Text with spaces]`
    - `data=[Complex value with 'quotes' and "double quotes"]`
  - Brackets and backslashes can be included in the text by escaping them with a backslash `\`.
  - **Examples:**
    - `message=[Includes \[escaped brackets\]]`
    - `message=[Includes an escaped \\]`

### Delimited Arguments

This is a special type of delimiter as it is not used for text specifically but for arguments, specifically multi key-value pairs. Although it is valid to place them around any argument, only their value is limited in those cases.

- **Bracketed Arguments (with or without Content):**
  - Enclose arguments and their content in square brackets `[ ]` that include spaces and special characters.
  - **Examples:**
    - `[debug]`
    - `[message_1=Text with spaces,message_2=Another text with spaces']`
    - `[message_1=Text with spaces and "double quotes",message_2=Another text with comma, spaces and 'single quotes']`
  - Brackets, commas, and backslashes can be included in the text by escaping them with a backslash `\`.
  - **Examples:**
    - `[message_1=Includes \[escaped brackets\] and an escaped comma\,,message_2=Includes 'single quotes' and "double quotes"]`
    - `[message_1=Includes escaped \\,message_2=And random text with comma\, spaces and 'quotes'"]`

**Important Notes:**

- If both multi key-value pairs and non-argument text are enabled and `KeyOnly` is set, then the Multi Key-Value Pairs **must** be delimited with brackets to avoid key-value pairs without an equals sign being interpreted as non-argument text.

### Non-Argument Text Collection

- **Description:** Any unrecognized text that does not match known arguments is collected into a vector of strings.
- **Example:**
  - If `debug` is defined as a flag, then `pam-debugplus.so [debug] [This is a message]` will recognize `debug` as a flag and `This is a message` as non-argument text.
  - `pam-debugplus.so [This is the beginning] and this is the rest`: Non-argument text = `["This is the beginning", "and", "this", "is", "the", "rest"]`
- **Behavior:**
  - **Disabled by Default:** Non-argument text collection is disabled. When disabled and unrecognized text is encountered, an error will be thrown.
  - **Enabling:** Use `collect_non_argument_text(true)` to enable this feature.

**Important Notes:**

- The text collector gathers all non-argument text around known arguments (flags and key-value pairs).
- If non-argument text is interleaved with arguments, it will be collected into the non-argument text.
- **Example:** If `env` and `debug` are defined as flags, the command `pam-debugplus.so env This is text with debug in the middle` will collect `env` and `debug` as flags, and the text `This is text with in the middle` as non-argument text.

## Validation and Dependencies

### Required Arguments

- **Description:** Arguments can be marked as required. If a required argument is not provided, an error will be thrown.
  - **Example:** If `MESSAGE` is required (using `required()`) and not provided, parsing will fail with `Error::RequiredArgMissing`.

### Argument Dependencies

- **Description:** Arguments can depend on other arguments. If a dependency is not met, an error will be thrown.
  - **Example:** If `HOST` depends on `USER` (using `depends_on("USER")`), and `HOST` is provided without `USER`, parsing will fail with `Error::DependencyNotMet`.

### Argument Exclusions

- **Description:** Certain arguments can be mutually exclusive. If both are provided, parsing will fail.
  - **Example:** `DEBUG` and `QUIET` cannot both be set (using `excludes("QUIET")` or `.conflicts("DEBUG", "QUIET")`), and any attempt to set both will result in `Error::MutuallyExclusiveArgs`.

### Allowed Values

- **Description:** Arguments can be restricted to a set of allowed values. If an invalid value is provided, parsing will fail.
  - **Example:** If `ALIGN` can only be `LEFT`, `CENTER`, or `RIGHT` (using `allowed_values(&["LEFT", "CENTER", "RIGHT"])`), any other value will cause parsing to fail with `Error::InvalidValue`.

## Flexibility and Usability

### Order Independence

- **Description:** Arguments can be provided in any order.
  - **Example:** If `ENV` and `DEBUG` are defined as flags, the commands `pam-debugplus.so ENV DEBUG This is a message` and `pam-debugplus.so DEBUG This is a message ENV` will collect `ENV` and `DEBUG` as flags, and the text `This is a message` as non-argument text.

### Parsing Rules

The library uses a multi-step parser approach that processes the provided arguments based on the defined options. It handles flags, single key-value pairs, multiple key-value pairs, and non-argument text. Multi-step means that it will process explicit flags first, then explicit key-value pairs, then multi key-value pairs, and finally non-argument text.

**Parsing Rules and Precedence Order:**

1. **Explicit Flags:** Processed first.
2. **Explicit Key-Value Pairs:** Processed after flags.
3. **Multi Key-Value Pairs and Non-Argument Text:**
   - **If both are enabled:**
     - **Inside Square Brackets `[ ]`:**
       - `KeyValue`: `key=value`
       - `KeyEquals`: `key=`
       - `KeyOnly`: `key` (without equals sign)
     - **Outside Square Brackets:**
       - `KeyValue`: `key=value`
       - `KeyEquals`: `key=`
       - Any argument without an equals sign is treated as non-argument text.
   - **If only Multi Key-Value Pairs are enabled:**
     - Applies both inside and outside square brackets.
     - All formats (`KeyValue`, `KeyEquals`, `KeyOnly`) are considered.
   - **If only Non-Argument Text is enabled:**
     - Only explicitly defined arguments (flags and key-value pairs) are processed.
     - Everything else is collected as non-argument text.
4. **Square Brackets `[ ]`:** Always used to group key-value pairs.
5. **Unrecognized Arguments:** If an argument is not recognized and doesn't fit the above rules, it is treated as an error when non-argument text is disabled.

The library aims to interpret as much structured data as possible before falling back to collecting non-argument text.

## Parsing Errors

These error types help in identifying specific errors during argument parsing. Users should check the `Result` returned by `parse()` and handle errors appropriately in their application.

```rust
enum Error {
    RequiredArgMissing(String),            // Required argument missing
    MutuallyExclusiveArgs(String, String), // Mutually exclusive arguments found
    InvalidKeyValue(String),               // Invalid key value, does not match the allowed formats
    UnrecognizedArg(String),               // Unrecognized argument when arguments are left
    InvalidIntValue(String),               // Invalid integer value
    InvalidBoolValue(String),              // Invalid boolean value
    DependencyNotMet(String, String),      // Dependency not met
    InvalidValue(String, String),          // Invalid value, does not match the allowed values
    DuplicateArgName(String),              // An argument name is defined more than once
    UnclosedDelimiter(String),             // Unclosed delimiter (quotes or brackets)
    NestedBrackets(String),                // Nested brackets are not supported
    InvalidInput(String),                  // Invalid input
    UnexpectedError(String),               // Unexpected error
}
```

## Debugging Support

- **Enable Debugging:** Integrate with the standard Rust `log` crate to enable logging at different levels.
- **Behavior:** When enabled, the library will log detailed information about the command-line arguments, how they are parsed, and any errors that occur.

**Important Notes:**

1. The library integrates with the `log` crate to provide debug logging at various levels. You can configure any `log` implementation (e.g., `env_logger`, `simple_logger`) to view these logs.
2. Debug messages are emitted at different log levels (`trace`, `debug`, `info`, `warn`, `error`) depending on their importance.
3. To enable debug logging:

```rust
// When using env_logger
env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
```

## Memory Management

Unlike the C library which requires manual memory management, `pam-args` leverages Rust's ownership system to automatically manage memory. Resources are freed when they go out of scope, eliminating the need for explicit cleanup functions.

**Important Note:**

- Rust's ownership system ensures that all allocated memory is properly freed when it's no longer needed, preventing memory leaks.
- There is no need for an explicit `cleanup()` function as with the C library.

## Thread Safety

The `pam-args` library is designed to be thread-safe. All public types implement `Send` and `Sync` where appropriate, allowing for safe concurrent use in multi-threaded environments.

While PAM modules typically operate in a single-threaded context, thread safety is valuable for several reasons:

1. **Future-proofing**: Applications using the library may eventually need to handle arguments in a multi-threaded context.
2. **Integration flexibility**: The library can be used in broader applications beyond just PAM modules.
3. **Concurrent validation**: In complex applications, argument validation might be performed concurrently with other operations.

```rust
// Safe to use across threads
let parser = Arc::new(ArgumentParser::new()
    .flag(Flag::new("DEBUG", "Enable debug mode"))
    // ...more configuration
);

// Can be safely cloned and passed to multiple threads
let parser_clone = Arc::clone(&parser);
let handle = thread::spawn(move || {
    let args = vec!["DEBUG", "USER=admin"];
    match parser_clone.parse(args) {
        Ok(matches) => { /* Process results */ },
        Err(e) => { /* Handle error */ }
    }
});
```

This thread safety comes with no performance penalty for single-threaded use, as Rust's ownership system and zero-cost abstractions ensure that thread safety features only incur costs when actually used across threads.