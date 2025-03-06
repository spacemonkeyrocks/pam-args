# Feature 5: Core Argument Types

## Module Type
**Core**: This component defines the fundamental argument types that represent command-line arguments in PAM modules. These types are central to the public API and are directly used by library consumers to define their argument structure.

## Feature Information

**Feature Name**: Core Argument Types

**Description**: Defines the fundamental argument types that represent command-line arguments in PAM modules. This component provides type-safe representations for flags, key-value pairs, and their associated metadata, leveraging Rust's powerful type system to ensure correctness at compile time. The core argument types form the foundation of the library's public API and enable users to define and validate command-line arguments in a safe, expressive manner.

**Priority**: High

**Dependencies**: 
- [Feature 1: Result & Error Types](error-result-types.md)

## Requirements

### Functional Requirements
1. Define type-safe representations for flags (boolean arguments)
2. Define type-safe representations for key-value pairs with various value types
3. Support argument metadata (descriptions, default values, etc.)
4. Provide mechanisms for argument validation constraints (required, allowed values, etc.)
5. Support dependency and exclusion relationships between arguments
6. Enable direct binding to caller-provided variables
7. Ensure proper error propagation when constraints are violated
8. Support argument format specification (different ways to specify key-value pairs)
9. Implement display functionality for help text generation

### API Requirements
- Provide a clean, builder-style API for defining arguments
- Ensure strong type safety to catch errors at compile time
- Allow for flexible configuration of arguments
- Enable clear expression of argument constraints
- Support different ways of accessing argument values after parsing
- Keep argument definitions immutable after creation
- Provide clear documentation of argument properties

### Performance Requirements
- Minimize heap allocations during argument definition
- Optimize argument access patterns for fast lookups
- Ensure efficient type conversions for argument values
- Support zero-copy string handling where possible
- Minimize template bloat with strategic use of generics

## Design

### Data Structures
```rust
/// Represents a flag (boolean) command-line argument
#[derive(Debug, Clone)]
pub struct Flag {
    /// The name of the flag
    name: String,
    
    /// Description of the flag for help text
    description: String,
    
    /// List of arguments that this flag depends on
    dependencies: Vec<String>,
    
    /// List of arguments that this flag conflicts with
    exclusions: Vec<String>,
    
    /// Field binding to update with flag value during parsing
    binding: Option<FlagBinding>,
}

/// Represents a key-value pair command-line argument
#[derive(Debug, Clone)]
pub struct KeyValue {
    /// The name of the key-value pair
    name: String,
    
    /// Description of the key-value pair for help text
    description: String,
    
    /// Whether this key-value pair is required
    required: bool,
    
    /// List of arguments that this key-value pair depends on
    dependencies: Vec<String>,
    
    /// List of arguments that this key-value pair conflicts with
    exclusions: Vec<String>,
    
    /// Allowed formats for this key-value pair
    allowed_formats: Vec<AllowedKeyValueFormats>,
    
    /// Allowed values for this key-value pair (if restricted)
    allowed_values: Option<Vec<String>>,
    
    /// Type converter function for this key-value pair
    type_converter: Option<TypeConverter>,
    
    /// Field binding to update with parsed value during parsing
    binding: Option<ValueBinding>,
}

/// Represents allowed formats for key-value pairs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AllowedKeyValueFormats {
    /// KEY=VALUE format (e.g., "USER=admin")
    KeyValue,
    
    /// KEY format without value (e.g., "DEBUG")
    KeyOnly,
    
    /// KEY= format with empty value (e.g., "EMPTY=")
    KeyEquals,
    
    /// Convenience type for all formats
    KeyAll,
}

/// Type conversion function signature
pub type TypeConverter = fn(&str) -> Result<Box<dyn Any + 'static>, Error>;

/// Binding for flag values
enum FlagBinding {
    /// No binding
    None,
    
    /// Binding to a field
    Field(&'static mut bool),
    
    /// Binding to a callback function
    Callback(Box<dyn Fn(bool) + Send + Sync>),
}

/// Binding for key-value pair values
enum ValueBinding {
    /// No binding
    None,
    
    /// Binding to a field of the specified type
    Field(BindingType),
    
    /// Binding to a callback function
    Callback(Box<dyn Fn(Box<dyn Any>) + Send + Sync>),
}

/// Types of field bindings for key-value pairs
enum BindingType {
    /// String binding
    String(&'static mut String),
    
    /// Integer binding
    Integer(&'static mut i32),
    
    /// Boolean binding
    Boolean(&'static mut bool),
    
    /// Character binding
    Char(&'static mut char),
    
    /// Option<String> binding
    OptionalString(&'static mut Option<String>),
    
    /// Option<Integer> binding
    OptionalInteger(&'static mut Option<i32>),
    
    /// Option<Boolean> binding
    OptionalBoolean(&'static mut Option<bool>),
    
    /// Option<Character> binding
    OptionalChar(&'static mut Option<char>),
}
```

### Function Signatures
```rust
impl Flag {
    /// Creates a new flag with the given name and description
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the flag
    /// * `description` - The description of the flag for help text
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::Flag;
    ///
    /// let debug_flag = Flag::new("DEBUG", "Enable debug mode");
    /// ```
    pub fn new<S1, S2>(name: S1, description: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            name: name.into(),
            description: description.into(),
            dependencies: Vec::new(),
            exclusions: Vec::new(),
            binding: None,
        }
    }
    
    /// Adds a dependency to this flag
    ///
    /// The flag will only be considered if the dependency is present.
    ///
    /// # Arguments
    ///
    /// * `dependency` - The name of the argument that this flag depends on
    ///
    /// # Returns
    ///
    /// The flag with the dependency added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::Flag;
    ///
    /// let flag = Flag::new("VERBOSE", "Enable verbose output")
    ///     .depends_on("DEBUG");
    /// ```
    pub fn depends_on<S: Into<String>>(mut self, dependency: S) -> Self {
        self.dependencies.push(dependency.into());
        self
    }
    
    /// Adds an exclusion to this flag
    ///
    /// The flag will be rejected if the excluded argument is present.
    ///
    /// # Arguments
    ///
    /// * `exclusion` - The name of the argument that this flag excludes
    ///
    /// # Returns
    ///
    /// The flag with the exclusion added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::Flag;
    ///
    /// let flag = Flag::new("DEBUG", "Enable debug mode")
    ///     .excludes("QUIET");
    /// ```
    pub fn excludes<S: Into<String>>(mut self, exclusion: S) -> Self {
        self.exclusions.push(exclusion.into());
        self
    }
    
    /// Binds this flag to a boolean field that will be updated during parsing
    ///
    /// # Arguments
    ///
    /// * `field` - Mutable reference to a boolean field to update
    ///
    /// # Returns
    ///
    /// The flag with the binding added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::Flag;
    ///
    /// let mut debug = false;
    /// let flag = Flag::new("DEBUG", "Enable debug mode")
    ///     .bind_to(&mut debug);
    /// ```
    pub fn bind_to(mut self, field: &mut bool) -> Self {
        // Convert the mutable reference to a static reference for storage
        // This is safe because the ArgumentParser ensures the reference
        // remains valid for the lifetime of the binding
        let static_ref: &'static mut bool = unsafe {
            std::mem::transmute::<&mut bool, &'static mut bool>(field)
        };
        
        self.binding = Some(FlagBinding::Field(static_ref));
        self
    }
    
    /// Binds this flag to a callback function that will be called during parsing
    ///
    /// # Arguments
    ///
    /// * `callback` - Function to call with the flag value
    ///
    /// # Returns
    ///
    /// The flag with the callback added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::Flag;
    ///
    /// let flag = Flag::new("DEBUG", "Enable debug mode")
    ///     .on_set(|value| {
    ///         println!("Debug mode: {}", value);
    ///     });
    /// ```
    pub fn on_set<F>(mut self, callback: F) -> Self
    where
        F: Fn(bool) + Send + Sync + 'static,
    {
        self.binding = Some(FlagBinding::Callback(Box::new(callback)));
        self
    }
    
    /// Returns the name of this flag
    ///
    /// # Returns
    ///
    /// The name of the flag
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Returns the description of this flag
    ///
    /// # Returns
    ///
    /// The description of the flag
    pub fn description(&self) -> &str {
        &self.description
    }
    
    /// Returns the dependencies of this flag
    ///
    /// # Returns
    ///
    /// A slice of the dependencies
    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }
    
    /// Returns the exclusions of this flag
    ///
    /// # Returns
    ///
    /// A slice of the exclusions
    pub fn exclusions(&self) -> &[String] {
        &self.exclusions
    }
    
    /// Returns whether this flag has a binding
    ///
    /// # Returns
    ///
    /// true if the flag has a binding, false otherwise
    pub fn has_binding(&self) -> bool {
        self.binding.is_some()
    }
    
    /// Updates the bound field with the given value
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set
    ///
    /// # Returns
    ///
    /// true if the update was successful, false otherwise
    pub(crate) fn update_binding(&self, value: bool) -> bool {
        match &self.binding {
            Some(FlagBinding::Field(field)) => {
                **field = value;
                true
            }
            Some(FlagBinding::Callback(callback)) => {
                callback(value);
                true
            }
            None => false,
        }
    }
}

impl KeyValue {
    /// Creates a new key-value pair with the given name and description
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the key-value pair
    /// * `description` - The description of the key-value pair for help text
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::KeyValue;
    ///
    /// let user_kv = KeyValue::new("USER", "Username for authentication");
    /// ```
    pub fn new<S1, S2>(name: S1, description: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            name: name.into(),
            description: description.into(),
            required: false,
            dependencies: Vec::new(),
            exclusions: Vec::new(),
            allowed_formats: vec![AllowedKeyValueFormats::KeyValue],
            allowed_values: None,
            type_converter: None,
            binding: None,
        }
    }
    
    /// Sets this key-value pair as required
    ///
    /// # Returns
    ///
    /// The key-value pair with the required flag set
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::KeyValue;
    ///
    /// let kv = KeyValue::new("USER", "Username for authentication")
    ///     .required();
    /// ```
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    
    /// Adds a dependency to this key-value pair
    ///
    /// The key-value pair will only be considered if the dependency is present.
    ///
    /// # Arguments
    ///
    /// * `dependency` - The name of the argument that this key-value pair depends on
    ///
    /// # Returns
    ///
    /// The key-value pair with the dependency added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::KeyValue;
    ///
    /// let kv = KeyValue::new("PORT", "Port number")
    ///     .depends_on("HOST");
    /// ```
    pub fn depends_on<S: Into<String>>(mut self, dependency: S) -> Self {
        self.dependencies.push(dependency.into());
        self
    }
    
    /// Adds an exclusion to this key-value pair
    ///
    /// The key-value pair will be rejected if the excluded argument is present.
    ///
    /// # Arguments
    ///
    /// * `exclusion` - The name of the argument that this key-value pair excludes
    ///
    /// # Returns
    ///
    /// The key-value pair with the exclusion added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::KeyValue;
    ///
    /// let kv = KeyValue::new("HOST", "Host name")
    ///     .excludes("LOCAL");
    /// ```
    pub fn excludes<S: Into<String>>(mut self, exclusion: S) -> Self {
        self.exclusions.push(exclusion.into());
        self
    }
    
    /// Sets the allowed formats for this key-value pair
    ///
    /// # Arguments
    ///
    /// * `formats` - Slice of allowed formats
    ///
    /// # Returns
    ///
    /// The key-value pair with the allowed formats set
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::{KeyValue, AllowedKeyValueFormats};
    ///
    /// let kv = KeyValue::new("DEBUG", "Enable debug mode")
    ///     .allowed_formats(&[
    ///         AllowedKeyValueFormats::KeyOnly,
    ///         AllowedKeyValueFormats::KeyValue,
    ///     ]);
    /// ```
    pub fn allowed_formats(mut self, formats: &[AllowedKeyValueFormats]) -> Self {
        self.allowed_formats = formats.to_vec();
        self
    }
    
    /// Sets the allowed values for this key-value pair
    ///
    /// # Arguments
    ///
    /// * `values` - Slice of allowed values
    ///
    /// # Returns
    ///
    /// The key-value pair with the allowed values set
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::KeyValue;
    ///
    /// let kv = KeyValue::new("ALIGN", "Text alignment")
    ///     .allowed_values(&["LEFT", "CENTER", "RIGHT"]);
    /// ```
    pub fn allowed_values<S: AsRef<str>>(mut self, values: &[S]) -> Self {
        self.allowed_values = Some(values.iter().map(|s| s.as_ref().to_string()).collect());
        self
    }
    
    /// Sets the type converter function for this key-value pair
    ///
    /// # Arguments
    ///
    /// * `converter` - The function to use for type conversion
    ///
    /// # Returns
    ///
    /// The key-value pair with the type converter set
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::KeyValue;
    /// use std::str::FromStr;
    ///
    /// let kv = KeyValue::new("WIDTH", "Width in pixels")
    ///     .type_converter(i32::from_str);
    /// ```
    pub fn type_converter<T, E>(mut self, converter: fn(&str) -> Result<T, E>) -> Self
    where
        T: 'static + std::any::Any,
        E: std::fmt::Display,
    {
        // Wrap the converter to return our Error type
        let wrapper = move |s: &str| -> Result<Box<dyn Any + 'static>, Error> {
            match converter(s) {
                Ok(value) => Ok(Box::new(value)),
                Err(e) => {
                    let type_name = std::any::type_name::<T>();
                    Err(Error::InvalidInput(format!(
                        "Failed to convert '{}' to {}: {}",
                        s, type_name, e
                    )))
                }
            }
        };
        
        self.type_converter = Some(wrapper);
        self
    }
    
    /// Binds this key-value pair to a field that will be updated during parsing
    ///
    /// # Arguments
    ///
    /// * `field` - Mutable reference to a field to update
    ///
    /// # Returns
    ///
    /// The key-value pair with the binding added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::KeyValue;
    /// use std::str::FromStr;
    ///
    /// let mut username = String::new();
    /// let kv = KeyValue::new("USER", "Username for authentication")
    ///     .type_converter(String::from_str)
    ///     .bind_to(&mut username);
    /// ```
    pub fn bind_to<T: 'static>(mut self, field: &mut T) -> Self
    where
        T: std::fmt::Debug,
    {
        // Implementation depends on the specific type of T
        // For simplicity, let's assume T is String, i32, bool, or char
        let binding = if let Some(string_field) = check_type::<String>(field) {
            Some(ValueBinding::Field(BindingType::String(unsafe {
                std::mem::transmute::<&mut String, &'static mut String>(string_field)
            })))
        } else if let Some(int_field) = check_type::<i32>(field) {
            Some(ValueBinding::Field(BindingType::Integer(unsafe {
                std::mem::transmute::<&mut i32, &'static mut i32>(int_field)
            })))
        } else if let Some(bool_field) = check_type::<bool>(field) {
            Some(ValueBinding::Field(BindingType::Boolean(unsafe {
                std::mem::transmute::<&mut bool, &'static mut bool>(bool_field)
            })))
        } else if let Some(char_field) = check_type::<char>(field) {
            Some(ValueBinding::Field(BindingType::Char(unsafe {
                std::mem::transmute::<&mut char, &'static mut char>(char_field)
            })))
        } else {
            // Not a supported type
            None
        };
        
        self.binding = binding;
        self
    }
    
    /// Binds this key-value pair to an Optional field that will be updated during parsing
    ///
    /// # Arguments
    ///
    /// * `field` - Mutable reference to an Option field to update
    ///
    /// # Returns
    ///
    /// The key-value pair with the binding added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::KeyValue;
    /// use std::str::FromStr;
    ///
    /// let mut username: Option<String> = None;
    /// let kv = KeyValue::new("USER", "Username for authentication")
    ///     .type_converter(String::from_str)
    ///     .bind_to_option(&mut username);
    /// ```
    pub fn bind_to_option<T: 'static>(mut self, field: &mut Option<T>) -> Self
    where
        T: std::fmt::Debug,
    {
        // Implementation depends on the specific type of T
        // For simplicity, let's assume T is String, i32, bool, or char
        let binding = if let Some(string_field) = check_type::<Option<String>>(field) {
            Some(ValueBinding::Field(BindingType::OptionalString(unsafe {
                std::mem::transmute::<&mut Option<String>, &'static mut Option<String>>(string_field)
            })))
        } else if let Some(int_field) = check_type::<Option<i32>>(field) {
            Some(ValueBinding::Field(BindingType::OptionalInteger(unsafe {
                std::mem::transmute::<&mut Option<i32>, &'static mut Option<i32>>(int_field)
            })))
        } else if let Some(bool_field) = check_type::<Option<bool>>(field) {
            Some(ValueBinding::Field(BindingType::OptionalBoolean(unsafe {
                std::mem::transmute::<&mut Option<bool>, &'static mut Option<bool>>(bool_field)
            })))
        } else if let Some(char_field) = check_type::<Option<char>>(field) {
            Some(ValueBinding::Field(BindingType::OptionalChar(unsafe {
                std::mem::transmute::<&mut Option<char>, &'static mut Option<char>>(char_field)
            })))
        } else {
            // Not a supported type
            None
        };
        
        self.binding = binding;
        self
    }
    
    /// Binds this key-value pair to a callback function that will be called during parsing
    ///
    /// # Arguments
    ///
    /// * `callback` - Function to call with the parsed value
    ///
    /// # Returns
    ///
    /// The key-value pair with the callback added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::KeyValue;
    /// use std::str::FromStr;
    ///
    /// let kv = KeyValue::new("USER", "Username for authentication")
    ///     .type_converter(String::from_str)
    ///     .on_set(|value: &str| {
    ///         println!("User: {}", value);
    ///     });
    /// ```
    pub fn on_set<F>(mut self, callback: F) -> Self
    where
        F: Fn(Box<dyn Any>) + Send + Sync + 'static,
    {
        self.binding = Some(ValueBinding::Callback(Box::new(callback)));
        self
    }
    
    /// Returns the name of this key-value pair
    ///
    /// # Returns
    ///
    /// The name of the key-value pair
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Returns the description of this key-value pair
    ///
    /// # Returns
    ///
    /// The description of the key-value pair
    pub fn description(&self) -> &str {
        &self.description
    }
    
    /// Returns whether this key-value pair is required
    ///
    /// # Returns
    ///
    /// true if the key-value pair is required, false otherwise
    pub fn is_required(&self) -> bool {
        self.required
    }
    
    /// Returns the dependencies of this key-value pair
    ///
    /// # Returns
    ///
    /// A slice of the dependencies
    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }
    
    /// Returns the exclusions of this key-value pair
    ///
    /// # Returns
    ///
    /// A slice of the exclusions
    pub fn exclusions(&self) -> &[String] {
        &self.exclusions
    }
    
    /// Returns the allowed formats for this key-value pair
    ///
    /// # Returns
    ///
    /// A slice of the allowed formats
    pub fn allowed_formats(&self) -> &[AllowedKeyValueFormats] {
        &self.allowed_formats
    }
    
    /// Returns the allowed values for this key-value pair
    ///
    /// # Returns
    ///
    /// An optional slice of the allowed values
    pub fn allowed_values(&self) -> Option<&[String]> {
        self.allowed_values.as_deref()
    }
    
    /// Checks if a value is allowed for this key-value pair
    ///
    /// # Arguments
    ///
    /// * `value` - The value to check
    ///
    /// # Returns
    ///
    /// true if the value is allowed, false otherwise
    pub fn is_value_allowed(&self, value: &str) -> bool {
        match &self.allowed_values {
            Some(values) => values.iter().any(|v| v == value),
            None => true,
        }
    }
    
    /// Returns whether this key-value pair has a type converter
    ///
    /// # Returns
    ///
    /// true if the key-value pair has a type converter, false otherwise
    pub fn has_type_converter(&self) -> bool {
        self.type_converter.is_some()
    }
    
    /// Returns whether this key-value pair has a binding
    ///
    /// # Returns
    ///
    /// true if the key-value pair has a binding, false otherwise
    pub fn has_binding(&self) -> bool {
        self.binding.is_some()
    }
    
    /// Converts a string value to the target type using the type converter
    ///
    /// # Arguments
    ///
    /// * `value` - The string value to convert
    ///
    /// # Returns
    ///
    /// The converted value as a boxed Any, or an error
    pub(crate) fn convert_value(&self, value: &str) -> Result<Box<dyn Any + 'static>, Error> {
        match &self.type_converter {
            Some(converter) => converter(value),
            None => Ok(Box::new(value.to_string())),
        }
    }
    
    /// Updates the bound field with the given value
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set
    ///
    /// # Returns
    ///
    /// true if the update was successful, false otherwise
    pub(crate) fn update_binding(&self, value: Box<dyn Any + 'static>) -> bool {
        match &self.binding {
            Some(ValueBinding::Field(binding_type)) => {
                match binding_type {
                    BindingType::String(field) => {
                        if let Ok(s) = value.downcast::<String>() {
                            **field = *s;
                            true
                        } else {
                            false
                        }
                    }
                    BindingType::Integer(field) => {
                        if let Ok(i) = value.downcast::<i32>() {
                            **field = *i;
                            true
                        } else {
                            false
                        }
                    }
                    BindingType::Boolean(field) => {
                        if let Ok(b) = value.downcast::<bool>() {
                            **field = *b;
                            true
                        } else {
                            false
                        }
                    }
                    BindingType::Char(field) => {
                        if let Ok(c) = value.downcast::<char>() {
                            **field = *c;
                            true
                        } else {
                            false
                        }
                    }
                    BindingType::OptionalString(field) => {
                        if let Ok(s) = value.downcast::<String>() {
                            **field = Some(*s);
                            true
                        } else {
                            false
                        }
                    }
                    BindingType::OptionalInteger(field) => {
                        if let Ok(i) = value.downcast::<i32>() {
                            **field = Some(*i);
                            true
                        } else {
                            false
                        }
                    }
                    BindingType::OptionalBoolean(field) => {
                        if let Ok(b) = value.downcast::<bool>() {
                            **field = Some(*b);
                            true
                        } else {
                            false
                        }
                    }
                    BindingType::OptionalChar(field) => {
                        if let Ok(c) = value.downcast::<char>() {
                            **field = Some(*c);
                            true
                        } else {
                            false
                        }
                    }
                }
            }
            Some(ValueBinding::Callback(callback)) => {
                callback(value);
                true
            }
            None => false,
        }
    }
}

/// Helper function to check if a value is of a specific type
fn check_type<T: 'static, U>(value: &mut U) -> Option<&mut T> {
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<U>() {
        // This is safe because we've verified the types match
        unsafe { Some(std::mem::transmute::<&mut U, &mut T>(value)) }
    } else {
        None
    }
}
```

### Implementation Approach

#### 1. Type-Safe Argument Representations

The library defines two primary argument types: `Flag` for boolean arguments and `KeyValue` for key-value pairs:

```rust
pub struct Flag {
    name: String,
    description: String,
    dependencies: Vec<String>,
    exclusions: Vec<String>,
    binding: Option<FlagBinding>,
}

pub struct KeyValue {
    name: String,
    description: String,
    required: bool,
    dependencies: Vec<String>,
    exclusions: Vec<String>,
    allowed_formats: Vec<AllowedKeyValueFormats>,
    allowed_values: Option<Vec<String>>,
    type_converter: Option<TypeConverter>,
    binding: Option<ValueBinding>,
}
```

These structures:
- Encapsulate all metadata about arguments
- Provide a foundation for validation
- Enable rich constraint expression
- Support binding to user variables

#### 2. Builder Pattern for Configuration

Both argument types use the builder pattern for configuration:

```rust
// For Flag
let debug_flag = Flag::new("DEBUG", "Enable debug mode")
    .depends_on("VERBOSE")
    .excludes("QUIET")
    .bind_to(&mut debug_enabled);

// For KeyValue
let user_kv = KeyValue::new("USER", "Username for authentication")
    .required()
    .type_converter(String::from_str)
    .allowed_values(&["admin", "user", "guest"])
    .bind_to(&mut username);
```

This approach:
- Provides a fluent, readable API
- Makes argument configuration concise
- Allows for method chaining
- Makes constraints explicit and clear

#### 3. Type Conversion System

The library uses a flexible type conversion system for key-value pairs:

```rust
pub fn type_converter<T, E>(mut self, converter: fn(&str) -> Result<T, E>) -> Self
where
    T: 'static + std::any::Any,
    E: std::fmt::Display,
{
    // Wrap the converter to return our Error type
    let wrapper = move |s: &str| -> Result<Box<dyn Any + 'static>, Error> {
        match converter(s) {
            Ok(value) => Ok(Box::new(value)),
            Err(e) => {
                let type_name = std::any::type_name::<T>();
                Err(Error::InvalidInput(format!(
                    "Failed to convert '{}' to {}: {}",
                    s, type_name, e
                )))
            }
        }
    };
    
    self.type_converter = Some(wrapper);
    self
}
```

This system:
- Leverages Rust's type system for type safety
- Supports any type that implements `FromStr`
- Provides clear error messages on conversion failure
- Uses type erasure for flexible storage

#### 4. Direct Field Binding

The library supports direct binding to user variables:

```rust
pub fn bind_to<T: 'static>(mut self, field: &mut T) -> Self
where
    T: std::fmt::Debug,
{
    // Implementation depends on the specific type of T
    let binding = if let Some(string_field) = check_type::<String>(field) {
        Some(ValueBinding::Field(BindingType::String(unsafe {
            std::mem::transmute::<&mut String, &'static mut String>(string_field)
        })))
    } else if let Some(int_field) = check_type::<i32>(field) {
        Some(ValueBinding::Field(BindingType::Integer(unsafe {
            std::mem::transmute::<&mut i32, &'static mut i32>(int_field)
        })))
    } else {
        // Other types...
        None
    };
    
    self.binding = binding;
    self
}
```

This mechanism:
- Simplifies access to parsed values
- Updates user variables automatically
- Avoids manual extraction after parsing
- Ensures type safety through static type checking

#### 5. Validation Constraints

The library supports rich validation constraints:

```rust
// Required fields
pub fn required(mut self) -> Self {
    self.required = true;
    self
}

// Allowed values
pub fn allowed_values<S: AsRef<str>>(mut self, values: &[S]) -> Self {
    self.allowed_values = Some(values.iter().map(|s| s.as_ref().to_string()).collect());
    self
}

// Value format constraints
pub fn allowed_formats(mut self, formats: &[AllowedKeyValueFormats]) -> Self {
    self.allowed_formats = formats.to_vec();
    self
}

// Dependencies
pub fn depends_on<S: Into<String>>(mut self, dependency: S) -> Self {
    self.dependencies.push(dependency.into());
    self
}

// Exclusions
pub fn excludes<S: Into<String>>(mut self, exclusion: S) -> Self {
    self.exclusions.push(exclusion.into());
    self
}
```

These constraints:
- Enable complex validation rules
- Ensure data integrity
- Express relationships between arguments
- Provide clear error messages when violated

#### 6. Memory Safety Considerations

The library uses strategic `unsafe` code for field binding:

```rust
pub fn bind_to(mut self, field: &mut bool) -> Self {
    // Convert the mutable reference to a static reference for storage
    let static_ref: &'static mut bool = unsafe {
        std::mem::transmute::<&mut bool, &'static mut bool>(field)
    };
    
    self.binding = Some(FlagBinding::Field(static_ref));
    self
}
```

This approach:
- Is carefully limited to specific operations
- Only extends lifetimes of references that are guaranteed to be valid
- Is contained within well-defined boundaries
- Provides significant usability benefits

#### 7. Format Specification

The library supports different formats for key-value pairs:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AllowedKeyValueFormats {
    /// KEY=VALUE format (e.g., "USER=admin")
    KeyValue,
    
    /// KEY format without value (e.g., "DEBUG")
    KeyOnly,
    
    /// KEY= format with empty value (e.g., "EMPTY=")
    KeyEquals,
    
    /// Convenience type for all formats
    KeyAll,
}
```

This feature:
- Accommodates various command-line conventions
- Allows for flexible argument formats
- Supports PAM-specific syntax patterns
- Enables precise control over accepted formats

## Integration

### Integration with Other Components

The core argument types integrate with other components as follows:

1. **ArgumentParser**: Uses these types to define the expected arguments
2. **Parser Module**: Processes raw arguments based on argument definitions
3. **Validation System**: Uses constraints to validate parsed arguments
4. **Field Binding System**: Updates bound fields with parsed values
5. **Error System**: Propagates validation errors using the defined error types

### Usage Examples

```rust
use pam_args_rs::{ArgumentParser, Flag, KeyValue, Error};
use std::str::FromStr;

// Define a struct to hold our configuration
#[derive(Debug, Default)]
struct Config {
    debug: bool,
    verbose: bool,
    quiet: bool,
    username: String,
    host: Option<String>,
    port: Option<i32>,
}

fn main() -> Result<(), Error> {
    // Create an instance of our configuration
    let mut config = Config::default();
    
    // Create an argument parser with our argument definitions
    let parser = ArgumentParser::new()
        .flag(Flag::new("DEBUG", "Enable debug mode")
            .bind_to(&mut config.debug))
        .flag(Flag::new("VERBOSE", "Enable verbose output")
            .bind_to(&mut config.verbose))
        .flag(Flag::new("QUIET", "Suppress output")
            .excludes("VERBOSE")
            .bind_to(&mut config.quiet))
        .key_value(KeyValue::new("USER", "Username for authentication")
            .type_converter(String::from_str)
            .required()
            .bind_to(&mut config.username))
        .key_value(KeyValue::new("HOST", "Host to connect to")
            .type_converter(String::from_str)
            .bind_to_option(&mut config.host))
        .key_value(KeyValue::new("PORT", "Port to connect to")
            .type_converter(i32::from_str)
            .bind_to_option(&mut config.port));
    
    // Parse the command-line arguments
    let result = parser.parse(std::env::args().skip(1))?;
    
    // The configuration has been updated automatically
    println!("Configuration: {:?}", config);
    
    // We can also access the values through the parse result
    if result.is_present("DEBUG") {
        println!("Debug mode enabled");
    }
    
    if let Some(host) = config.host.as_ref() {
        println!("Host: {}", host);
        
        if let Some(port) = config.port {
            println!("Port: {}", port);
        } else {
            println!("Using default port");
        }
    }
    
    Ok(())
}
```

## Testing Strategy

### Unit Test Cases

| # | Category | Input | Expected Output | Notes |
|---|----------|-------|-----------------|-------|
| 1 | Flag Creation | `Flag::new("DEBUG", "Debug mode")` | Flag with correct name and description | Test basic construction |
| 2 | Flag Dependencies | `Flag::new("X", "").depends_on("Y")` | Flag with dependency on Y | Test dependency addition |
| 3 | Flag Exclusions | `Flag::new("X", "").excludes("Y")` | Flag with exclusion for Y | Test exclusion addition |
| 4 | Flag Binding | `Flag::new("X", "").bind_to(&mut bool_var)` | Flag with binding to bool_var | Test field binding |
| 5 | Flag Callback | `Flag::new("X", "").on_set(callback)` | Flag with callback set | Test callback binding |
| 6 | Flag Accessors | Various accessor methods on Flag | Correct property values | Test all accessor methods |
| 7 | Flag Update Binding | `flag.update_binding(true)` | `bool_var == true` | Test binding update mechanism |
| 8 | KeyValue Creation | `KeyValue::new("USER", "Username")` | KeyValue with correct name and description | Test basic construction |
| 9 | KeyValue Required | `KeyValue::new("X", "").required()` | KeyValue with required=true | Test required flag setting |
| 10 | KeyValue Dependencies | `KeyValue::new("X", "").depends_on("Y")` | KeyValue with dependency on Y | Test dependency addition |
| 11 | KeyValue Exclusions | `KeyValue::new("X", "").excludes("Y")` | KeyValue with exclusion for Y | Test exclusion addition |
| 12 | KeyValue Formats | `KeyValue::new("X", "").allowed_formats(&[...])` | KeyValue with specified formats | Test format specification |
| 13 | KeyValue Values | `KeyValue::new("X", "").allowed_values(&[...])` | KeyValue with allowed values | Test value restriction |
| 14 | KeyValue Type Converter | `KeyValue::new("X", "").type_converter(i32::from_str)` | KeyValue with converter set | Test type converter setting |
| 15 | KeyValue Binding | `KeyValue::new("X", "").bind_to(&mut var)` | KeyValue with binding to var | Test field binding |
| 16 | KeyValue Option Binding | `KeyValue::new("X", "").bind_to_option(&mut opt_var)` | KeyValue with binding to opt_var | Test option field binding |
| 17 | KeyValue Callback | `KeyValue::new("X", "").on_set(callback)` | KeyValue with callback set | Test callback binding |
| 18 | KeyValue Accessors | Various accessor methods on KeyValue | Correct property values | Test all accessor methods |
| 19 | KeyValue Validation | `kv.is_value_allowed("test")` | Correct boolean result | Test value validation |
| 20 | KeyValue Conversion | `kv.convert_value("123")` | Correctly converted value | Test value conversion |
| 21 | KeyValue Update Binding | `kv.update_binding(Box::new("test"))` | `var == "test"` | Test binding update mechanism |
| 22 | Multiple Dependencies | `Flag::new("X", "").depends_on("Y").depends_on("Z")` | Flag with both dependencies | Test multiple dependencies |
| 23 | Multiple Exclusions | `Flag::new("X", "").excludes("Y").excludes("Z")` | Flag with both exclusions | Test multiple exclusions |
| 24 | Type Conversion Success | `kv.convert_value("123")` for i32 converter | Box containing 123 | Test successful conversion |
| 25 | Type Conversion Failure | `kv.convert_value("abc")` for i32 converter | Error::InvalidInput | Test failed conversion |
| 26 | Invalid Field Type | `KeyValue::new("X", "").bind_to(&mut complex_type)` | Binding = None | Test unsupported type handling |
| 27 | Allowed Values Check | `kv.is_value_allowed("valid")` vs `kv.is_value_allowed("invalid")` | true vs false | Test allowed values validation |
| 28 | Flag Immutability | After creation, attempt to modify a Flag | Compiler error | Test immutability of Flag after creation |
| 29 | KeyValue Immutability | After creation, attempt to modify a KeyValue | Compiler error | Test immutability of KeyValue after creation |
| 30 | Builder Pattern Chaining | Chain multiple methods on Flag and KeyValue | Correctly configured objects | Test builder pattern fluidity |

### Integration Tests

The core argument types should be tested in integration with other components to ensure correct end-to-end behavior:

1. **ArgumentParser Integration**
   - Test Flag and KeyValue used with ArgumentParser
   - Verify correct parsing of arguments
   - Test validation of constraints
   - Verify field binding works correctly

2. **End-to-End Parsing**
   - Test complete argument parsing workflows
   - Verify correct handling of all argument types
   - Test complex constraint scenarios
   - Verify error propagation works correctly

3. **Field Binding System**
   - Test binding to various field types
   - Verify automatic field updates
   - Test option field binding
   - Verify callback invocation

### Testing Focus Areas

1. **Type Safety**
   - Verify type conversion works correctly
   - Test with various Rust types
   - Verify compiler catches type errors
   - Test with generic types

2. **Constraint Validation**
   - Test required arguments
   - Test dependencies and exclusions
   - Test allowed values
   - Test format restrictions

3. **Error Handling**
   - Verify clear error messages
   - Test error propagation
   - Test recovery from errors
   - Verify error context information

4. **Memory Safety**
   - Test field binding safety
   - Verify no memory leaks
   - Test with various binding scenarios
   - Verify thread safety where applicable

5. **API Usability**
   - Test API ergonomics
   - Verify intuitive behavior
   - Test documentation examples
   - Verify builder pattern fluidity

## Performance Considerations

### Memory Efficiency
- Use `String` for names to avoid lifetime complexity
- Store only what's necessary in each argument type
- Use enums to minimize memory usage for variants
- Avoid unnecessary cloning of strings
- Use references where possible to avoid ownership transfers

### Type System Optimization
- Use static dispatch for type conversion
- Leverage Rust's zero-cost abstractions
- Use type erasure only where necessary for flexibility
- Avoid unnecessary monomorphization
- Minimize template instantiation with strategic generics

### Binding System Efficiency
- Use direct field updates for maximum performance
- Avoid dynamic dispatch where possible
- Use unsafe code strategically for performance-critical bindings
- Minimize indirection in field access
- Cache type information to avoid repeated lookups

### Parser Integration Efficiency
- Design for efficient lookup during parsing
- Optimize constraint checking for common cases
- Support early returns for validation failures
- Minimize string comparisons during parsing
- Use data structures optimized for the access patterns of parsing