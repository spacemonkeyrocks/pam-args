//! Integration tests for the storage module.
//!
//! This module contains integration tests for the storage module, testing the
//! interaction between different components of the storage system.

#[cfg(test)]
mod tests {
    use crate::storage::{DefaultKeyValueStore, KeyValueStore, KeyValueStoreExt, NonArgTextStore, FromArgValue};
    use crate::error::Error;
    use crate::AllowedKeyValueFormats;

    #[test]
    fn test_storage_integration() {
        // Create a case-insensitive key-value store
        let mut kv_store = DefaultKeyValueStore::new(false);
        
        // Add key-value pairs in different formats
        kv_store.add("USER", Some("admin"));       // KEY_VALUE format
        kv_store.add("FLAG", None);                // KEY_ONLY format
        kv_store.add("EMPTY", Some(""));           // KEY_EQUALS format
        
        // Check for key existence
        assert!(kv_store.has_key("user"));         // Case-insensitive lookup
        assert!(kv_store.has_key("FLAG"));
        assert!(kv_store.has_key("empty"));
        
        // Retrieve values
        assert_eq!(kv_store.get("USER"), Some("admin"));
        assert_eq!(kv_store.get("user"), Some("admin")); // Case-insensitive
        assert_eq!(kv_store.get("FLAG"), None);          // KEY_ONLY has no value
        assert_eq!(kv_store.get("EMPTY"), Some(""));     // KEY_EQUALS has empty value
        
        // Type conversion
        kv_store.add("PORT", Some("8080"));
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
        let port = i32::from_arg_value(port_value).unwrap();
        assert_eq!(port, 8080);
        
        let bool_value = "true";
        let flag = bool::from_arg_value(bool_value).unwrap();
        assert!(flag);
    }

    #[test]
    fn test_format_compatibility() {
        // Test compatibility between different key-value formats
        let key_value_format = AllowedKeyValueFormats::KeyValue;
        let key_only_format = AllowedKeyValueFormats::KeyOnly;
        let key_equals_format = AllowedKeyValueFormats::KeyEquals;
        let key_all_format = AllowedKeyValueFormats::KeyAll;
        
        // KeyAll is compatible with all formats
        assert!(key_all_format.is_compatible_with(key_value_format));
        assert!(key_all_format.is_compatible_with(key_only_format));
        assert!(key_all_format.is_compatible_with(key_equals_format));
        
        // Other formats are only compatible with themselves
        assert!(key_value_format.is_compatible_with(key_value_format));
        assert!(!key_value_format.is_compatible_with(key_only_format));
        assert!(!key_value_format.is_compatible_with(key_equals_format));
        
        // Test compatibility with a list of formats
        let formats = vec![key_value_format, key_only_format];
        assert!(key_all_format.is_compatible_with_any(&formats));
        assert!(key_value_format.is_compatible_with_any(&formats));
        assert!(key_only_format.is_compatible_with_any(&formats));
        assert!(!key_equals_format.is_compatible_with_any(&formats));
    }

    #[test]
    fn test_from_arg_value_error_handling() {
        // Test error handling for FromArgValue implementations
        
        // Invalid integer
        let result = i32::from_arg_value("not_a_number");
        assert!(result.is_err());
        match result {
            Err(Error::InvalidIntValue(val)) => assert_eq!(val, "not_a_number"),
            _ => panic!("Expected InvalidIntValue error"),
        }
        
        // Invalid boolean
        let result = bool::from_arg_value("maybe");
        assert!(result.is_err());
        match result {
            Err(Error::InvalidBoolValue(val)) => assert_eq!(val, "maybe"),
            _ => panic!("Expected InvalidBoolValue error"),
        }
        
        // Invalid character
        let result = char::from_arg_value("abc");
        assert!(result.is_err());
        match result {
            Err(Error::InvalidInput(_)) => (),
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_key_overwrite() {
        let mut store = DefaultKeyValueStore::new(true);
        
        // Add a key-value pair
        store.add("KEY", Some("value1"));
        assert_eq!(store.get("KEY"), Some("value1"));
        
        // Overwrite the key with a new value
        store.add("KEY", Some("value2"));
        assert_eq!(store.get("KEY"), Some("value2"));
        
        // Overwrite with a key-only format
        store.add("KEY", None);
        assert_eq!(store.get("KEY"), None);
        assert!(store.has_key("KEY"));
        
        // Overwrite with a key-equals format
        store.add("KEY", Some(""));
        assert_eq!(store.get("KEY"), Some(""));
    }

    #[test]
    fn test_case_sensitivity_changes() {
        let mut store = DefaultKeyValueStore::new(true);
        
        // Add keys with case-sensitive store
        store.add("KEY", Some("value"));
        store.add("key", Some("different"));
        
        // Both keys exist separately
        assert_eq!(store.get("KEY"), Some("value"));
        assert_eq!(store.get("key"), Some("different"));
        assert_eq!(store.len(), 2);
        
        // Change to case-insensitive
        store.set_case_sensitive(false);
        
        // Now the keys are treated as the same
        // Note: Which value is returned depends on HashMap iteration order
        // and is not guaranteed, so we just check that one of them is returned
        let value = store.get("KEY");
        assert!(value == Some("value") || value == Some("different"));
        
        // Add a new key with case-insensitive store
        store.add("KEY", Some("new_value"));
        
        // All case variations return the same value
        assert_eq!(store.get("KEY"), Some("new_value"));
        assert_eq!(store.get("key"), Some("new_value"));
        assert_eq!(store.get("Key"), Some("new_value"));
    }
}