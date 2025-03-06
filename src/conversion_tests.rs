use colored::Colorize;
use crate::conversion::{ConverterConfig, converter};
use crate::args::AllowedKeyValueFormats;

#[test]
fn test_all_formats() {
    let all_formats = AllowedKeyValueFormats::all();
    assert_eq!(all_formats.len(), 3);
    assert!(all_formats.contains(&AllowedKeyValueFormats::KeyValue));
    assert!(all_formats.contains(&AllowedKeyValueFormats::KeyOnly));
    assert!(all_formats.contains(&AllowedKeyValueFormats::KeyEquals));
    assert!(!all_formats.contains(&AllowedKeyValueFormats::KeyAll));
}

#[test]
fn test_format_compatibility() {
    assert!(AllowedKeyValueFormats::KeyAll.is_compatible_with(AllowedKeyValueFormats::KeyValue));
    assert!(AllowedKeyValueFormats::KeyValue.is_compatible_with(AllowedKeyValueFormats::KeyValue));
    assert!(!AllowedKeyValueFormats::KeyValue.is_compatible_with(AllowedKeyValueFormats::KeyOnly));
}

#[test]
fn test_format_compatibility_any() {
    let formats = vec![
        AllowedKeyValueFormats::KeyValue,
        AllowedKeyValueFormats::KeyOnly,
    ];

    assert!(AllowedKeyValueFormats::KeyAll.is_compatible_with_any(&formats));
    assert!(AllowedKeyValueFormats::KeyValue.is_compatible_with_any(&formats));
    assert!(AllowedKeyValueFormats::KeyOnly.is_compatible_with_any(&formats));
    assert!(!AllowedKeyValueFormats::KeyEquals.is_compatible_with_any(&formats));
}

#[test]
fn test_string_conversion() {
    let s: String = converter::convert("hello", None).unwrap();
    assert_eq!(s, "hello");

    let s: String = converter::convert("  hello  ", None).unwrap();
    assert_eq!(s, "hello");

    let config = ConverterConfig {
        trim_whitespace: false,
        handle_empty: true,
        recognize_none_values: true,
    };
    let s: String = converter::convert("  hello  ", Some(&config)).unwrap();
    assert_eq!(s, "  hello  ");
}

#[test]
fn test_integer_conversion() {
    let i: i32 = converter::convert("123", None).unwrap();
    assert_eq!(i, 123);

    let i: i32 = converter::convert("-123", None).unwrap();
    assert_eq!(i, -123);

    let i: i32 = converter::convert("0", None).unwrap();
    assert_eq!(i, 0);

    let result = converter::convert::<i32>("abc", None);
    assert!(result.is_err());

    let result = converter::convert::<i32>("123.45", None);
    assert!(result.is_err());
}

#[test]
fn test_boolean_conversion() {
    let b: bool = converter::convert("true", None).unwrap();
    assert!(b);

    let b: bool = converter::convert("yes", None).unwrap();
    assert!(b);

    let b: bool = converter::convert("1", None).unwrap();
    assert!(b);

    let b: bool = converter::convert("on", None).unwrap();
    assert!(b);

    let b: bool = converter::convert("false", None).unwrap();
    assert!(!b);

    let b: bool = converter::convert("no", None).unwrap();
    assert!(!b);

    let b: bool = converter::convert("0", None).unwrap();
    assert!(!b);

    let b: bool = converter::convert("off", None).unwrap();
    assert!(!b);

    let b: bool = converter::convert("TRUE", None).unwrap();
    assert!(b);

    let result = converter::convert::<bool>("invalid", None);
    assert!(result.is_err());
}

#[test]
fn test_character_conversion() {
    let c: char = converter::convert("a", None).unwrap();
    assert_eq!(c, 'a');

    let c: char = converter::convert("A", None).unwrap();
    assert_eq!(c, 'A');

    let c: char = converter::convert("1", None).unwrap();
    assert_eq!(c, '1');

    let config = ConverterConfig {
        trim_whitespace: false,
        handle_empty: true,
        recognize_none_values: true,
    };
    let c: char = converter::convert(" ", Some(&config)).unwrap();
    assert_eq!(c, ' ');

    // Test error cases without unwrapping
    let empty_result = converter::convert::<char>("", None);
    assert!(empty_result.is_err());

    let multi_char_result = converter::convert::<char>("ab", None);
    assert!(multi_char_result.is_err());
}

// Separate test for empty string to avoid unwrapping errors
#[test]
fn test_character_conversion_empty() {
    let empty_result = converter::convert::<char>("", None);
    assert!(empty_result.is_err());
}

#[test]
fn test_option_conversion() {
    let o: Option<String> = converter::convert("hello", None).unwrap();
    assert_eq!(o, Some("hello".to_string()));

    let o: Option<String> = converter::convert("", None).unwrap();
    assert_eq!(o, None);

    let o: Option<String> = converter::convert("none", None).unwrap();
    assert_eq!(o, None);

    let o: Option<String> = converter::convert("null", None).unwrap();
    assert_eq!(o, None);

    let o: Option<i32> = converter::convert("123", None).unwrap();
    assert_eq!(o, Some(123));

    let o: Option<i32> = converter::convert("", None).unwrap();
    assert_eq!(o, None);

    let o: Option<i32> = converter::convert("none", None).unwrap();
    assert_eq!(o, None);

    let result = converter::convert::<Option<i32>>("invalid", None);
    assert!(result.is_err());
}

#[test]
fn test_from_str_helper() {
    let from_str_i32 = converter::from_str::<i32>();
    assert_eq!(from_str_i32("123").unwrap(), 123);
    assert!(from_str_i32("abc").is_err());

    let from_str_bool = converter::from_str::<bool>();
    assert_eq!(from_str_bool("true").unwrap(), true);
    assert!(from_str_bool("invalid").is_err());
}

#[test]
fn test_converter_config() {
    let default_config = ConverterConfig::default();
    assert!(default_config.trim_whitespace);
    assert!(default_config.handle_empty);
    assert!(default_config.recognize_none_values);

    // Create a custom config with recognize_none_values set to false
    let custom_config = ConverterConfig {
        trim_whitespace: false,
        handle_empty: true,
        recognize_none_values: false,
    };

    // Test that whitespace is not trimmed with custom config
    let s: String = converter::convert("  hello  ", Some(&custom_config)).unwrap();
    assert_eq!(s, "  hello  ");

    // Test that empty strings are still converted to None for Option types
    let empty_result: Option<String> = converter::convert("", Some(&custom_config)).unwrap();
    assert_eq!(empty_result, None);

    // Test that "none" is NOT treated as None when recognize_none_values is false
    let none_result: Option<String> = converter::convert("none", Some(&custom_config)).unwrap();
    assert_eq!(none_result, Some("none".to_string()));
}

// Separate test for none values with custom config
#[test]
fn test_converter_config_none_values() {
    let custom_config = ConverterConfig {
        trim_whitespace: false,
        handle_empty: true,
        recognize_none_values: false,
    };

    // Test that "none" is NOT treated as None when recognize_none_values is false
    let none_result: Option<String> = converter::convert("none", Some(&custom_config)).unwrap();
    assert_eq!(none_result, Some("none".to_string()));
}