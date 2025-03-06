# Feature 12: Key-Value Definition System

## Module Type
**Core**: This component is a core part of the parser subsystem that defines the configuration and validation rules for key-value pairs. It implements the internal representation of key-value definitions while providing a clean interface for the public API.

## Feature Information

**Feature Name**: Key-Value Definition System

**Description**: Define the configuration for key-value pairs, including allowed formats, values, and requirements. This component is responsible for encapsulating the rules that govern how key-value pairs should be parsed, validated, and converted to appropriate types. It provides a structured approach to defining complex constraints like allowed formats, required values, dependencies, exclusions, and value restrictions, enabling type-safe and validated key-value handling throughout the library.

**Priority**: High

**Dependencies**: 
- [Feature 1: Result & Error Types](error-result-types.md)
- [Feature 2: Utility Functions](utility-functions.md)
- [Feature 5: Core Argument Types](core-argument-types.md)
- [Feature 7: Format & Type Conversion](format-type-conversion.md)

## Requirements

### Functional Requirements
1. Define and store configuration for key-value pair arguments
2. Support specification of required versus optional key-value pairs
3. Enable definition of allowed values for specific keys
4. Support allowed format specifications (KEY=VALUE, KEY=, KEY)
5. Enable dependency relationships between key-value pairs
6. Support exclusion relationships between key-value pairs
7. Integrate with type conversion system for value parsing
8. Support direct field binding for automatic updates
9. Support callback registration for value change notifications
10. Provide metadata like descriptions for help text generation
11. Enable type-safe access to key-value configurations
12. Support validation of format and value constraints

### API Requirements
- Provide a clean, builder-style API for key-value definitions
- Ensure type safety for key-value configurations
- Support fluent interface for chained configuration
- Enable easy specification of constraints
- Make key-value definitions immutable after creation
- Provide clear access to definition properties
- Support integration with parser components
- Ensure thread safety for concurrent access

### Performance Requirements
- Minimize heap allocations during definition creation
- Optimize constraint validation for fast checking
- Support efficient lookups of key-value configurations
- Ensure minimal overhead for type conversion
- Optimize memory usage for storing definitions

## Design

### Data Structures
```rust
/// Definition of a key-value pair argument
#[derive(Debug, Clone)]
pub struct KeyValueDefinition {
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
    type_converter: Option<TypeConverterFn>,
    
    /// Field binding to update with parsed value during parsing
    binding: Option<ValueBinding>,
}

/// Type definition for the converter function
pub type TypeConverterFn = fn(&str) -> Result<Box<dyn Any + 'static>, Error>;

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
```

### Function Signatures
```rust
impl KeyValueDefinition {
    /// Creates a new key-value definition with the given name and description
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the key-value pair
    /// * `description` - The description of the key-value pair for help text
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_def::KeyValueDefinition;
    ///
    /// let definition = KeyValueDefinition::new("USER", "Username for authentication");
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
    /// The definition with the required flag set
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_def::KeyValueDefinition;
    ///
    /// let definition = KeyValueDefinition::new("USER", "Username for authentication")
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
    /// The definition with the dependency added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_def::KeyValueDefinition;
    ///
    /// let definition = KeyValueDefinition::new("PORT", "Port number")
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
    /// The definition with the exclusion added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_def::KeyValueDefinition;
    ///
    /// let definition = KeyValueDefinition::new("HOST", "Host name")
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
    /// The definition with the allowed formats set
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_def::{KeyValueDefinition, AllowedKeyValueFormats};
    ///
    /// let definition = KeyValueDefinition::new("DEBUG", "Enable debug mode")
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
    /// The definition with the allowed values set
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_def::KeyValueDefinition;
    ///
    /// let definition = KeyValueDefinition::new("ALIGN", "Text alignment")
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
    /// The definition with the type converter set
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_def::KeyValueDefinition;
    /// use std::str::FromStr;
    ///
    /// let definition = KeyValueDefinition::new("WIDTH", "Width in pixels")
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
    /// The definition with the binding added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_def::KeyValueDefinition;
    /// use std::str::FromStr;
    ///
    /// let mut username = String::new();
    /// let definition = KeyValueDefinition::new("USER", "Username for authentication")
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
    /// The definition with the binding added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_def::KeyValueDefinition;
    /// use std::str::FromStr;
    ///
    /// let mut username: Option<String> = None;
    /// let definition = KeyValueDefinition::new("USER", "Username for authentication")
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
    /// The definition with the callback added
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_def::KeyValueDefinition;
    /// use std::str::FromStr;
    /// use std::any::Any;
    ///
    /// let definition = KeyValueDefinition::new("USER", "Username for authentication")
    ///     .type_converter(String::from_str)
    ///     .on_set(|value: Box<dyn Any>| {
    ///         if let Ok(username) = value.downcast::<String>() {
    ///             println!("User: {}", username);
    ///         }
    ///     });
    /// ```
    pub fn on_set<F>(mut self, callback: F) -> Self
    where
        F: Fn(Box<dyn Any>) + Send + Sync + 'static,
    {
        self.binding = Some(ValueBinding::Callback(Box::new(callback)));
        self
    }
    
    /// Returns the name of this key-value definition
    ///
    /// # Returns
    ///
    /// The name of the key-value pair definition
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Returns the description of this key-value definition
    ///
    /// # Returns
    ///
    /// The description of the key-value pair definition
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
    
    /// Returns the dependencies of this key-value definition
    ///
    /// # Returns
    ///
    /// A slice of the dependencies
    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }
    
    /// Returns the exclusions of this key-value definition
    ///
    /// # Returns
    ///
    /// A slice of the exclusions
    pub fn exclusions(&self) -> &[String] {
        &self.exclusions
    }
    
    /// Returns the allowed formats for this key-value definition
    ///
    /// # Returns
    ///
    /// A slice of the allowed formats
    pub fn allowed_formats(&self) -> &[AllowedKeyValueFormats] {
        &self.allowed_formats
    }
    
    /// Returns the allowed values for this key-value definition
    ///
    /// # Returns
    ///
    /// An optional slice of the allowed values
    pub fn allowed_values(&self) -> Option<&[String]> {
        self.allowed_values.as_deref()
    }
    
    /// Checks if a value is allowed for this key-value definition
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
    
    /// Returns whether this key-value definition has a type converter
    ///
    /// # Returns
    ///
    /// true if the key-value definition has a type converter, false otherwise
    pub fn has_type_converter(&self) -> bool {
        self.type_converter.is_some()
    }
    
    /// Returns whether this key-value definition has a binding
    ///
    /// # Returns
    ///
    /// true if the key-value definition has a binding, false otherwise
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
                    },
                    BindingType::Integer(field) => {
                        if let Ok(i) = value.downcast::<i32>() {
                            **field = *i;
                            true
                        } else {
                            false
                        }
                    },
                    BindingType::Boolean(field) => {
                        if let Ok(b) = value.downcast::<bool>() {
                            **field = *b;
                            true
                        } else {
                            false
                        }
                    },
                    BindingType::Char(field) => {
                        if let Ok(c) = value.downcast::<char>() {
                            **field = *c;
                            true
                        } else {
                            false
                        }
                    },
                    BindingType::OptionalString(field) => {
                        if let Ok(s) = value.downcast::<String>() {
                            **field = Some(*s);
                            true
                        } else {
                            false
                        }
                    },
                    BindingType::OptionalInteger(field) => {
                        if let Ok(i) = value.downcast::<i32>() {
                            **field = Some(*i);
                            true
                        } else {
                            false
                        }
                    },
                    BindingType::OptionalBoolean(field) => {
                        if let Ok(b) = value.downcast::<bool>() {
                            **field = Some(*b);
                            true
                        } else {
                            false
                        }
                    },
                    BindingType::OptionalChar(field) => {
                        if let Ok(c) = value.downcast::<char>() {
                            **field = Some(*c);
                            true
                        } else {
                            false
                        }
                    },
                }
            },
            Some(ValueBinding::Callback(callback)) => {
                callback(value);
                true
            },
            None => false,
        }
    }
    
    /// Checks if the given format is allowed for this key-value definition
    ///
    /// # Arguments
    ///
    /// * `format` - The format to check
    ///
    /// # Returns
    ///
    /// true if the format is allowed, false otherwise
    pub fn is_format_allowed(&self, format: AllowedKeyValueFormats) -> bool {
        self.allowed_formats.iter().any(|&f| f == format || f == AllowedKeyValueFormats::KeyAll)
    }
    
    /// Validates a key-value token against this definition
    ///
    /// # Arguments
    ///
    /// * `key` - The key from the token
    /// * `value` - The optional value from the token
    ///
    /// # Returns
    ///
    /// Ok(()) if the token is valid, or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the token violates any constraints
    pub fn validate_token(&self, key: &str, value: Option<&str>) -> Result<(), Error> {
        // Check if the key matches this definition
        if !self.name.eq_ignore_ascii_case(key) {
            return Ok(());  // Not for this definition, skip
        }
        
        // Determine the format of the token
        let format = match value {
            Some(v) if !v.is_empty() => AllowedKeyValueFormats::KeyValue,
            Some(_) => AllowedKeyValueFormats::KeyEquals,
            None => AllowedKeyValueFormats::KeyOnly,
        };
        
        // Check if the format is allowed
        if !self.is_format_allowed(format) {
            return Err(Error::InvalidKeyValue(format!(
                "Format {:?} not allowed for key '{}'", format, key
            )));
        }
        
        // Check if the value is allowed (if present)
        if let Some(v) = value {
            if !self.is_value_allowed(v) {
                return Err(Error::InvalidValue(
                    key.to_string(),
                    v.to_string(),
                ));
            }
        }
        
        // Validation passed
        Ok(())
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

impl Default for AllowedKeyValueFormats {
    fn default() -> Self {
        Self::KeyValue
    }
}

impl std::fmt::Display for AllowedKeyValueFormats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KeyValue => write!(f, "KEY=VALUE"),
            Self::KeyOnly => write!(f, "KEY"),
            Self::KeyEquals => write!(f, "KEY="),
            Self::KeyAll => write!(f, "KEY=VALUE,KEY,KEY="),
        }
    }
}

/// Manager for key-value definitions
pub struct KeyValueDefinitionManager {
    /// Map of key names to definitions for fast lookup
    definitions: HashMap<String, KeyValueDefinition>,
    
    /// Whether keys are case-sensitive
    case_sensitive: bool,
}

impl KeyValueDefinitionManager {
    /// Creates a new key-value definition manager
    ///
    /// # Arguments
    ///
    /// * `case_sensitive` - Whether keys are case-sensitive
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_def::KeyValueDefinitionManager;
    ///
    /// let manager = KeyValueDefinitionManager::new(true);
    /// ```
    pub fn new(case_sensitive: bool) -> Self {
        Self {
            definitions: HashMap::new(),
            case_sensitive,
        }
    }
    
    /// Adds a key-value definition to the manager
    ///
    /// # Arguments
    ///
    /// * `definition` - The definition to add
    ///
    /// # Returns
    ///
    /// Result containing self on success, or an error if the definition conflicts
    ///
    /// # Errors
    ///
    /// Returns an error if a definition with the same name already exists
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_def::{KeyValueDefinitionManager, KeyValueDefinition};
    ///
    /// let mut manager = KeyValueDefinitionManager::new(true);
    /// manager.add_definition(KeyValueDefinition::new("USER", "Username"))?;
    /// # Ok::<(), pam_args_rs::Error>(())
    /// ```
    pub fn add_definition(&mut self, definition: KeyValueDefinition) -> Result<&mut Self, Error> {
        let key = if self.case_sensitive {
            definition.name().to_string()
        } else {
            definition.name().to_lowercase()
        };
        
        if self.definitions.contains_key(&key) {
            return Err(Error::DuplicateArgName(key));
        }
        
        self.definitions.insert(key, definition);
        Ok(self)
    }
    
    /// Gets a definition by name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the definition to get
    ///
    /// # Returns
    ///
    /// Option containing a reference to the definition, or None if not found
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_def::{KeyValueDefinitionManager, KeyValueDefinition};
    ///
    /// let mut manager = KeyValueDefinitionManager::new(true);
    /// manager.add_definition(KeyValueDefinition::new("USER", "Username"))?;
    ///
    /// let definition = manager.get_definition("USER");
    /// assert!(definition.is_some());
    /// # Ok::<(), pam_args_rs::Error>(())
    /// ```
    pub fn get_definition(&self, name: &str) -> Option<&KeyValueDefinition> {
        let key = if self.case_sensitive {
            name.to_string()
        } else {
            name.to_lowercase()
        };
        
        self.definitions.get(&key)
    }
    
    /// Returns all definitions in the manager
    ///
    /// # Returns
    ///
    /// Vector of references to all definitions
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_def::{KeyValueDefinitionManager, KeyValueDefinition};
    ///
    /// let mut manager = KeyValueDefinitionManager::new(true);
    /// manager.add_definition(KeyValueDefinition::new("USER", "Username"))?;
    /// manager.add_definition(KeyValueDefinition::new("HOST", "Hostname"))?;
    ///
    /// let definitions = manager.get_all_definitions();
    /// assert_eq!(definitions.len(), 2);
    /// # Ok::<(), pam_args_rs::Error>(())
    /// ```
    pub fn get_all_definitions(&self) -> Vec<&KeyValueDefinition> {
        self.definitions.values().collect()
    }
    
    /// Returns all required definitions in the manager
    ///
    /// # Returns
    ///
    /// Vector of references to required definitions
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::parser::kv_def::{KeyValueDefinitionManager, KeyValueDefinition};
    ///
    /// let mut manager = KeyValueDefinitionManager::new(true);
    /// manager.add_definition(KeyValueDefinition::new("USER", "Username").required())?;
    /// manager.add_definition(KeyValueDefinition::new("HOST", "Hostname"))?;
    ///
    /// let required = manager.get_required_definitions();
    /// assert_eq!(required.len(), 1);
    /// assert_eq!(required[0].name(), "USER");
    /// # Ok::<(), pam_args_rs::Error>(())
    /// ```
    pub fn get_required_definitions(&self) -> Vec<&KeyValueDefinition> {
        self.definitions.values()
            .filter(|def| def.is_required())
            .collect()