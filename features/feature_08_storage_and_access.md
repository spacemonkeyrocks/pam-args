# Feature 8: Storage & Access

## Module Type
**Core**: This component implements the storage and retrieval system for key-value pairs parsed from arguments. It provides a trait-based abstraction layer for flexible storage implementations along with a default implementation based on Rust's standard collections.

## Feature Information

**Feature Name**: Storage & Access

**Description**: Implements the `KeyValueStore` trait and its implementations, providing a flexible and efficient way to store and retrieve key-value pairs. This component is responsible for managing the storage of argument values, supporting multiple key-value formats, and offering a clean API for accessing stored values. It leverages Rust's ownership system and type safety to provide a robust storage solution without manual memory management.

**Priority**: High

**Dependencies**: 
- [Feature 1: Result & Error Types](error-result-types.md)
- [Feature 5: Core Argument Types](core-argument-types.md)

## Requirements

### Functional Requirements
1. Provide a trait-based abstraction for key-value storage
2. Support three key-value states: KEY_VALUE, KEY_ONLY, and KEY_EQUALS
3. Implement a default storage backend using Rust's standard collections
4. Enable case-insensitive key lookups when configured
5. Support retrieving key lists for iteration
6. Allow checking for key existence without retrieving value
7. Provide type-safe value retrieval with conversion
8. Support multiple values for the same key when needed
9. Handle non-argument text storage and retrieval
10. Integrate cleanly with the parsing pipeline

### API Requirements
- Expose a clean, trait-based API for storage operations
- Provide intuitive methods for checking existence and retrieving values
- Ensure consistent error handling for all operations
- Support both owned and borrowed string operations
- Enable type-safe value access with minimal boilerplate
- Support flexible key lookup strategies
- Make the API thread-safe for concurrent access

### Performance Requirements
- Optimize for fast key lookup
- Minimize heap allocations during storage operations
- Support efficient iteration over stored keys
- Ensure consistent performance regardless of store size
- Minimize memory overhead for small stores

## Design

### Data Structures
```rust
/// Trait defining the interface for key-value storage
pub trait KeyValueStore {
    /// Adds a key-value pair to the store
    ///
    /// # Arguments
    ///
    /// * `key` - The key to add
    /// * `value` - The value to associate with the key, or None for key-only entries
    fn add(&mut self, key: &str, value: Option<&str>);
    
    /// Retrieves a value from the store
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// The value associated with the key, or None if not found or key-only
    fn get(&self, key: &str) -> Option<&str>;
    
    /// Checks if a key exists in the store
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check
    ///
    /// # Returns
    ///
    /// true if the key exists, false otherwise
    fn has_key(&self, key: &str) -> bool;
    
    /// Returns all keys in the store
    ///
    /// # Returns
    ///
    /// A vector of key references
    fn keys(&self) -> Vec<&str>;
    
    /// Returns the number of entries in the store
    ///
    /// # Returns
    ///
    /// The number of entries
    fn len(&self) -> usize;
    
    /// Checks if the store is empty
    ///
    /// # Returns
    ///
    /// true if the store is empty, false otherwise
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Default implementation of KeyValueStore using HashMap
#[derive(Debug, Clone)]
pub struct DefaultKeyValueStore {
    /// Storage for key-value pairs
    store: HashMap<String, Option<String>>,
    
    /// Whether keys are case-sensitive
    case_sensitive: bool,
}

/// Storage for non-argument text
#[derive(Debug, Clone, Default)]
pub struct NonArgTextStore {
    /// Vector of non-argument text
    text: Vec<String>,
}
```

### Function Signatures
```rust
impl DefaultKeyValueStore {
    /// Creates a new, empty key-value store
    ///
    /// # Arguments
    ///
    /// * `case_sensitive` - Whether keys are case-sensitive
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::storage::DefaultKeyValueStore;
    ///
    /// // Create a case-sensitive store
    /// let store = DefaultKeyValueStore::new(true);
    /// ```
    pub fn new(case_sensitive: bool) -> Self {
        Self {
            store: HashMap::new(),
            case_sensitive,
        }
    }
    
    /// Sets whether keys are case-sensitive
    ///
    /// # Arguments
    ///
    /// * `case_sensitive` - Whether keys are case-sensitive
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::storage::DefaultKeyValueStore;
    ///
    /// let mut store = DefaultKeyValueStore::new(true);
    /// store.set_case_sensitive(false);
    /// ```
    pub fn set_case_sensitive(&mut self, case_sensitive: bool) {
        self.case_sensitive = case_sensitive;
    }
    
    /// Normalizes a key according to case sensitivity settings
    ///
    /// # Arguments
    ///
    /// * `key` - The key to normalize
    ///
    /// # Returns
    ///
    /// The normalized key
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::storage::DefaultKeyValueStore;
    ///
    /// let store = DefaultKeyValueStore::new(false);
    /// assert_eq!(store.normalize_key("DEBUG"), "debug");
    /// ```
    pub fn normalize_key(&self, key: &str) -> String {
        if self.case_sensitive {
            key.to_string()
        } else {
            key.to_lowercase()
        }
    }
    
    /// Gets a value with type conversion
    ///
    /// # Type Parameters
    ///
    /// * `T` - The target type
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// The converted value, or None if not found or conversion failed
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::storage::DefaultKeyValueStore;
    ///
    /// let mut store = DefaultKeyValueStore::new(true);
    /// store.add("WIDTH", Some("80"));
    ///
    /// assert_eq!(store.value_of::<i32>("WIDTH"), Some(80));
    /// ```
    pub fn value_of<T>(&self, key: &str) -> Option<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.get(key).and_then(|value| value.parse::<T>().ok())
    }
    
    /// Clears the store
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::storage::DefaultKeyValueStore;
    ///
    /// let mut store = DefaultKeyValueStore::new(true);
    /// store.add("KEY", Some("VALUE"));
    /// store.clear();
    /// assert!(store.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.store.clear();
    }
}

impl KeyValueStore for DefaultKeyValueStore {
    fn add(&mut self, key: &str, value: Option<&str>) {
        let normalized_key = self.normalize_key(key);
        let owned_value = value.map(String::from);
        self.store.insert(normalized_key, owned_value);
    }
    
    fn get(&self, key: &str) -> Option<&str> {
        let normalized_key = self.normalize_key(key);
        self.store.get(&normalized_key).and_then(|opt| opt.as_deref())
    }
    
    fn has_key(&self, key: &str) -> bool {
        let normalized_key = self.normalize_key(key);
        self.store.contains_key(&normalized_key)
    }
    
    fn keys(&self) -> Vec<&str> {
        self.store.keys().map(|s| s.as_str()).collect()
    }
    
    fn len(&self) -> usize {
        self.store.len()
    }
}

impl NonArgTextStore {
    /// Creates a new, empty non-argument text store
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::storage::NonArgTextStore;
    ///
    /// let store = NonArgTextStore::new();
    /// ```
    pub fn new() -> Self {
        Self { text: Vec::new() }
    }
    
    /// Adds a non-argument text string to the store
    ///
    /// # Arguments
    ///
    /// * `text` - The text to add
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::storage::NonArgTextStore;
    ///
    /// let mut store = NonArgTextStore::new();
    /// store.add("This is non-argument text");
    /// ```
    pub fn add<S: Into<String>>(&mut self, text: S) {
        self.text.push(text.into());
    }
    
    /// Adds multiple non-argument text strings to the store
    ///
    /// # Arguments
    ///
    /// * `texts` - The texts to add
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::storage::NonArgTextStore;
    ///
    /// let mut store = NonArgTextStore::new();
    /// store.add_multiple(vec!["Text 1", "Text 2"]);
    /// ```
    pub fn add_multiple<S, I>(&mut self, texts: I)
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        for text in texts {
            self.text.push(text.into());
        }
    }
    
    /// Retrieves all non-argument text
    ///
    /// # Returns
    ///
    /// A slice of all stored text strings
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::storage::NonArgTextStore;
    ///
    /// let mut store = NonArgTextStore::new();
    /// store.add("Text");
    ///
    /// assert_eq!(store.texts(), &["Text"]);
    /// ```
    pub fn texts(&self) -> &[String] {
        &self.text
    }
    
    /// Returns the number of text entries
    ///
    /// # Returns
    ///
    /// The number of entries
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::storage::NonArgTextStore;
    ///
    /// let mut store = NonArgTextStore::new();
    /// store.add("Text");
    ///
    /// assert_eq!(store.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.text.len()
    }
    
    /// Checks if the store is empty
    ///
    /// # Returns
    ///
    /// true if the store is empty, false otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::storage::NonArgTextStore;
    ///
    /// let store = NonArgTextStore::new();
    /// assert!(store.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
    
    /// Clears the store
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args_rs::storage::NonArgTextStore;
    ///
    /// let mut store = NonArgTextStore::new();
    /// store.add("Text");
    /// store.clear();
    /// assert!(store.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.text.clear();
    }
}

/// Type conversion extension for key-value stores
pub trait KeyValueStoreExt: KeyValueStore {
    /// Gets a value with type conversion
    ///
    /// # Type Parameters
    ///
    /// * `T` - The target type
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// The converted value, or None if not found or conversion failed
    fn value_of<T>(&self, key: &str) -> Option<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.get(key).and_then(|value| value.parse::<T>().ok())
    }
}

// Implement the extension trait for any type that implements KeyValueStore
impl<T: KeyValueStore> KeyValueStoreExt for T {}

/// A trait for converting string arguments to specific types
pub trait FromArgValue: Sized {
    /// Converts a string argument value to this type
    ///
    /// # Arguments
    ///
    /// * `value` - The string value to convert
    ///
    /// # Returns
    ///
    /// The converted value or an error
    fn from_arg_value(value: &str) -> Result<Self, Error>;
}

// Implement FromArgValue for common types
impl FromArgValue for String {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        Ok(value.to_string())
    }
}

impl FromArgValue for i32 {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        value.parse::<i32>().map_err(|_| {
            Error::InvalidIntValue(value.to_string())
        })
    }
}

impl FromArgValue for bool {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        match value.to_lowercase().as_str() {
            "true" | "yes" | "1" | "on" => Ok(true),
            "false" | "no" | "0" | "off" => Ok(false),
            _ => Err(Error::InvalidBoolValue(value.to_string())),
        }
    }
}

impl FromArgValue for char {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        let mut chars = value.chars();
        match (chars.next(), chars.next()) {
            (Some(c), None) => Ok(c),
            _ => Err(Error::InvalidInput(format!(
                "Expected a single character, got: '{}'", value
            ))),
        }
    }
}

// Implement FromArgValue for Option<T> types
impl<T: FromArgValue> FromArgValue for Option<T> {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        if value.is_empty() {
            Ok(None)
        } else {
            T::from_arg_value(value).map(Some)
        }
    }
}
```

### Implementation Approach

#### 1. Trait-Based Abstraction
The storage system is designed around the `KeyValueStore` trait which provides a clean abstraction for key-value storage operations:

```rust
pub trait KeyValueStore {
    fn add(&mut self, key: &str, value: Option<&str>);
    fn get(&self, key: &str) -> Option<&str>;
    fn has_key(&self, key: &str) -> bool;
    fn keys(&self) -> Vec<&str>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}
```

This trait-based approach provides several benefits:
- Enables multiple storage implementations
- Allows for easy mocking in tests
- Provides a consistent interface regardless of backend
- Supports extension through additional traits
- Enables composition with other components

#### 2. Default Implementation
The `DefaultKeyValueStore` implements the `KeyValueStore` trait using Rust's standard `HashMap`:

```rust
pub struct DefaultKeyValueStore {
    store: HashMap<String, Option<String>>,
    case_sensitive: bool,
}

impl KeyValueStore for DefaultKeyValueStore {
    fn add(&mut self, key: &str, value: Option<&str>) {
        let normalized_key = self.normalize_key(key);
        let owned_value = value.map(String::from);
        self.store.insert(normalized_key, owned_value);
    }
    
    // Other method implementations...
}
```

This implementation:
- Uses `Option<String>` to represent the three key-value states
- Supports case-insensitive key lookup through normalization
- Handles key normalization automatically based on configuration
- Provides efficient access to stored values
- Leverages Rust's ownership system for memory safety

#### 3. Key-Value Format Support
The storage system supports three key-value formats through the `Option<String>` value type:

1. **KEY_VALUE**: Represented as `Some("value")`
2. **KEY_ONLY**: Represented as `None`
3. **KEY_EQUALS**: Represented as `Some("")` (empty string)

This approach:
- Eliminates the need for enum representation from the C library
- Uses Rust's `Option` type naturally for the key-only case
- Distinguishes between key-only and key-equals formats
- Provides a memory-efficient representation
- Aligns with Rust's conventions for optional values

#### 4. Type Conversion Extension
The `KeyValueStoreExt` trait adds type conversion capabilities to any `KeyValueStore` implementation:

```rust
pub trait KeyValueStoreExt: KeyValueStore {
    fn value_of<T>(&self, key: &str) -> Option<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        self.get(key).and_then(|value| value.parse::<T>().ok())
    }
}

// Implement for any KeyValueStore
impl<T: KeyValueStore> KeyValueStoreExt for T {}
```

This design:
- Leverages Rust's trait system for clean extension
- Uses Rust's `FromStr` trait for type conversion
- Provides a concise API for type-safe value retrieval
- Applies to any current or future `KeyValueStore` implementation
- Separates core storage from conversion concerns

#### 5. Custom Type Conversion
The `FromArgValue` trait provides a customization point for type conversion:

```rust
pub trait FromArgValue: Sized {
    fn from_arg_value(value: &str) -> Result<Self, Error>;
}

// Implementation for common types
impl FromArgValue for String {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        Ok(value.to_string())
    }
}

impl FromArgValue for i32 {
    fn from_arg_value(value: &str) -> Result<Self, Error> {
        value.parse::<i32>().map_err(|_| {
            Error::InvalidIntValue(value.to_string())
        })
    }
}

// Other implementations...
```

This approach:
- Enables custom type conversion logic
- Integrates with the library's error system
- Supports user-defined types
- Provides detailed error information
- Leverages Rust's Result type for error handling

#### 6. Non-Argument Text Storage
The `NonArgTextStore` provides a dedicated container for non-argument text:

```rust
pub struct NonArgTextStore {
    text: Vec<String>,
}

impl NonArgTextStore {
    pub fn new() -> Self {
        Self { text: Vec::new() }
    }
    
    pub fn add<S: Into<String>>(&mut self, text: S) {
        self.text.push(text.into());
    }
    
    // Other methods...
}
```

This design:
- Separates concerns between key-value and non-argument storage
- Provides a simple and focused API
- Uses efficient vector storage for ordered text
- Supports both single and bulk additions
- Preserves the original order of non-argument text

## Integration

### Integration with Other Components

The Storage & Access component integrates with other components as follows:

1. **Parser**: Provides the storage destination for parsed key-value pairs
2. **Validation**: Enables lookup of values for cross-argument validation
3. **Public API**: Exposes the storage interface for client access
4. **Error System**: Uses the library's error types for failed conversions
5. **Type Conversion**: Provides the foundation for type-safe value access

### Usage Examples

```rust
use pam_args_rs::storage::{DefaultKeyValueStore, KeyValueStore, KeyValueStoreExt, NonArgTextStore};
use pam_args_rs::error::Error;

fn main() -> Result<(), Error> {
    // Create a case-insensitive key-value store
    let mut kv_store = DefaultKeyValueStore::new(false);
    
    // Add key-value pairs in different formats
    kv_store.add("USER", Some("admin"));       // KEY_VALUE format
    kv_store.add("DEBUG", None);               // KEY_ONLY format
    kv_store.add("EMPTY", Some(""));           // KEY_EQUALS format
    
    // Check for key existence
    assert!(kv_store.has_key("user"));         // Case-insensitive lookup
    assert!(kv_store.has_key("DEBUG"));
    assert!(kv_store.has_key("empty"));
    
    // Retrieve values
    assert_eq!(kv_store.get("USER"), Some("admin"));
    assert_eq!(kv_store.get("debug"), Some("admin")); // Case-insensitive
    assert_eq!(kv_store.get("DEBUG"), None);          // KEY_ONLY has no value
    assert_eq!(kv_store.get("EMPTY"), Some(""));      // KEY_EQUALS has empty value
    
    // Type conversion
    assert_eq!(kv_store.add("PORT", Some("8080")), ());
    assert_eq!(kv_store.value_of::<i32>("PORT"), Some(8080));
    
    // Create a non-argument text store
    let mut text_store = NonArgTextStore::new();
    
    // Add non-argument text
    text_store.add("This is some text");
    text_store.add_multiple(vec!["More text", "Even more text"]);
    
    // Retrieve non-argument text
    assert_eq!(text_store.texts(), &[
        "This is some text",
        "More text",
        "Even more text"
    ]);
    
    // Using FromArgValue trait
    let port_value = "8080";
    let port = i32::from_arg_value(port_value)?;
    assert_eq!(port, 8080);
    
    let bool_value = "true";
    let flag = bool::from_arg_value(bool_value)?;
    assert!(flag);
    
    Ok(())
}
```

## Testing Strategy

### Unit Test Cases

| # | Category | Input | Expected Output | Notes |
|---|----------|-------|-----------------|-------|
| 1 | KeyValueStore Creation | `DefaultKeyValueStore::new(true)` | Empty store with case sensitivity | Test basic initialization |
| 2 | KeyValueStore Add | `store.add("KEY", Some("value"))` | Store contains the entry | Test basic addition |
| 3 | KeyValueStore Get | `store.get("KEY")` | `Some("value")` | Test basic retrieval |
| 4 | KeyValueStore Has Key | `store.has_key("KEY")` | `true` | Test key existence check |
| 5 | KeyValueStore Keys | `store.keys()` | `["KEY"]` | Test retrieving all keys |
| 6 | KeyValueStore Length | `store.len()` | `1` | Test store length |
| 7 | KeyValueStore Empty | `store.is_empty()` | `false` | Test empty check |
| 8 | KEY_VALUE Format | `store.add("KEY", Some("value"))` | `store.get("KEY") == Some("value")` | Test standard key-value format |
| 9 | KEY_ONLY Format | `store.add("KEY", None)` | `store.get("KEY") == None && store.has_key("KEY") == true` | Test key-only format |
| 10 | KEY_EQUALS Format | `store.add("KEY", Some(""))` | `store.get("KEY") == Some("")` | Test key-equals format |
| 11 | Case Sensitivity True | `store.add("KEY", Some("value"))` | `store.get("key") == None` | Test case-sensitive lookup |
| 12 | Case Sensitivity False | `store.add("KEY", Some("value"))` | `store.get("key") == Some("value")` | Test case-insensitive lookup |
| 13 | Key Normalization | `store.normalize_key("KEY")` | `"key"` or `"KEY"` depending on case sensitivity | Test key normalization |
| 14 | Value Type Conversion | `store.value_of::<i32>("PORT")` | `Some(8080)` | Test value conversion |
| 15 | Value Type Conversion Failure | `store.value_of::<i32>("USER")` | `None` | Test conversion failure |
| 16 | NonArgTextStore Creation | `NonArgTextStore::new()` | Empty text store | Test text store initialization |
| 17 | NonArgTextStore Add | `text_store.add("text")` | Store contains text | Test adding text |
| 18 | NonArgTextStore Add Multiple | `text_store.add_multiple(vec!["a", "b"])` | Store contains all texts in order | Test adding multiple texts |
| 19 | NonArgTextStore Texts | `text_store.texts()` | Slice of stored texts | Test retrieving all texts |
| 20 | NonArgTextStore Length | `text_store.len()` | Number of texts | Test text store length |
| 21 | NonArgTextStore Empty | `text_store.is_empty()` | `false` | Test text store empty check |
| 22 | String FromArgValue | `String::from_arg_value("test")` | `Ok("test".to_string())` | Test string conversion |
| 23 | i32 FromArgValue | `i32::from_arg_value("123")` | `Ok(123)` | Test integer conversion |
| 24 | i32 FromArgValue Error | `i32::from_arg_value("abc")` | `Err(Error::InvalidIntValue)` | Test integer conversion error |
| 25 | bool FromArgValue | `bool::from_arg_value("true")` | `Ok(true)` | Test boolean conversion |
| 26 | bool FromArgValue Variants | `bool::from_arg_value("yes")` | `Ok(true)` | Test boolean conversion variants |
| 27 | bool FromArgValue Error | `bool::from_arg_value("invalid")` | `Err(Error::InvalidBoolValue)` | Test boolean conversion error |
| 28 | char FromArgValue | `char::from_arg_value("a")` | `Ok('a')` | Test character conversion |
| 29 | char FromArgValue Error | `char::from_arg_value("abc")` | `Err(Error::InvalidInput)` | Test character conversion error |
| 30 | Option FromArgValue | `Option::<i32>::from_arg_value("123")` | `Ok(Some(123))` | Test option conversion |
| 31 | Option FromArgValue Empty | `Option::<i32>::from_arg_value("")` | `Ok(None)` | Test option conversion for empty string |
| 32 | Key Overwrite | `store.add("KEY", Some("value1"))` then `store.add("KEY", Some("value2"))` | `store.get("KEY") == Some("value2")` | Test key overwrite behavior |
| 33 | Clear Store | `store.clear()` | `store.is_empty() == true` | Test clearing the store |
| 34 | Multiple Keys | Add multiple keys and check `store.keys().len()` | Number of added keys | Test multiple key handling |
| 35 | KeyValueStoreExt Integration | Implement custom `KeyValueStore` and use `value_of<T>()` | Correct converted value | Test extension trait with custom implementation |

### Integration Tests

The Storage & Access module should be tested in integration with other components to ensure correct end-to-end behavior. Integration tests should focus on:

1. **Parser Integration**
   - Test storage of parsed arguments
   - Verify correct format handling
   - Test storage of different argument types
   - Verify case sensitivity handling during parsing and retrieval

2. **Validation Integration**
   - Test validation using stored values
   - Verify dependency and exclusion checking
   - Test error generation for constraint violations
   - Verify consistent validation across multiple arguments

3. **API Integration**
   - Test client access to parsed values
   - Verify type-safe access patterns
   - Test error handling during value conversion
   - Verify consistent behavior across different access methods

4. **Thread Safety**
   - Test concurrent access where applicable
   - Verify consistent results with concurrent readers
   - Test thread safety of immutable stores
   - Verify proper synchronization of mutable operations

### Testing Focus Areas

1. **Format Handling**
   - Verify correct handling of all key-value formats
   - Test edge cases in format processing
   - Verify format compatibility with other components
   - Test format conversion and normalization

2. **Case Sensitivity**
   - Test with both case-sensitive and case-insensitive configurations
   - Verify consistent behavior regardless of configuration
   - Test mixed-case keys and values
   - Verify case handling during lookup and storage

3. **Type Conversion**
   - Test conversion of all supported types
   - Verify error handling for conversion failures
   - Test custom type implementations
   - Verify consistent conversion behavior

4. **Memory Safety**
   - Test with various allocation patterns
   - Verify no memory leaks
   - Test with large key and value sets
   - Verify proper cleanup of resources

5. **Performance Characteristics**
   - Test with large stores
   - Verify efficient lookup in all cases
   - Test with realistic usage patterns
   - Verify minimal performance degradation with store size

## Performance Considerations

### Memory Efficiency
- Use `Option<String>` to efficiently represent three key-value states
- Use Rust's standard collections for optimal memory usage
- Leverage the ownership system to avoid manual memory management
- Avoid unnecessary string duplications during lookup
- Use string interning for frequently accessed keys

### Lookup Optimization
- Use HashMap for O(1) key lookup
- Normalize keys only once during addition
- Minimize string allocations during lookup
- Use string views where possible to avoid copying
- Cache frequently accessed keys and values

### Storage Strategy
- Use capacity hints for vectors and hashmaps
- Reuse allocations where possible
- Use small-string optimization for common keys
- Avoid excessive reallocations during additions
- Batch operations for better performance

### Type Conversion
- Implement FromArgValue for common types
- Use static dispatch for type conversion
- Leverage Rust's native parsing for efficiency
- Cache conversion results for repeated lookups
- Minimize allocations during conversion

### Thread Safety
- Implement Send and Sync for thread-safe storage
- Use immutable access where possible
- Provide efficient, thread-safe read access
- Use Atomic operations for concurrent counters
- Allow for lock-free reads in concurrent scenarios