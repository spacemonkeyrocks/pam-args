use pam_args::{ParserConfig, ParserConfigBuilder, AllowedKeyValueFormats};

#[test]
fn test_config_builder() {
    // Create a custom configuration
    let config = ParserConfig::builder()
        .case_sensitive(false)
        .collect_non_argument_text(true)
        .enable_multi_key_value(true)
        .multi_key_value_formats(&[
            AllowedKeyValueFormats::KeyValue,
            AllowedKeyValueFormats::KeyOnly,
        ])
        .build();
    
    // Verify configuration settings
    assert!(!config.is_case_sensitive());
    assert!(config.collect_non_argument_text());
    assert!(config.enable_multi_key_value());
    assert_eq!(config.multi_key_value_formats().len(), 2);
    assert_eq!(config.multi_key_value_formats()[0], AllowedKeyValueFormats::KeyValue);
    assert_eq!(config.multi_key_value_formats()[1], AllowedKeyValueFormats::KeyOnly);
}

#[test]
fn test_default_config() {
    let config = ParserConfig::new();
    
    // Verify default settings
    assert!(config.is_case_sensitive());
    assert!(config.is_case_sensitive_values());
    assert!(!config.collect_non_argument_text());
    assert!(!config.enable_multi_key_value());
    assert_eq!(config.multi_key_value_formats().len(), 1);
    assert_eq!(config.multi_key_value_formats()[0], AllowedKeyValueFormats::KeyValue);
    assert_eq!(config.escape_char(), '\\');
    assert_eq!(config.single_quote(), '\'');
    assert_eq!(config.double_quote(), '"');
    assert_eq!(config.open_bracket(), '[');
    assert_eq!(config.close_bracket(), ']');
    assert_eq!(config.delimiter(), ',');
    assert!(config.trim_values());
}

#[test]
fn test_custom_delimiters() {
    let config = ParserConfig::builder()
        .escape_char('$')
        .quote_chars('`', '"')
        .bracket_chars('<', '>')
        .delimiter(';')
        .build();
    
    assert_eq!(config.escape_char(), '$');
    assert_eq!(config.single_quote(), '`');
    assert_eq!(config.double_quote(), '"');
    assert_eq!(config.open_bracket(), '<');
    assert_eq!(config.close_bracket(), '>');
    assert_eq!(config.delimiter(), ';');
}

#[test]
fn test_method_chaining() {
    let config = ParserConfig::builder()
        .case_sensitive(false)
        .case_sensitive_values(false)
        .collect_non_argument_text(true)
        .enable_multi_key_value(true)
        .multi_key_value_formats(&[AllowedKeyValueFormats::KeyAll])
        .trim_values(false)
        .build();
    
    assert!(!config.is_case_sensitive());
    assert!(!config.is_case_sensitive_values());
    assert!(config.collect_non_argument_text());
    assert!(config.enable_multi_key_value());
    assert_eq!(config.multi_key_value_formats()[0], AllowedKeyValueFormats::KeyAll);
    assert!(!config.trim_values());
}

#[test]
fn test_config_clone() {
    let config1 = ParserConfig::builder()
        .case_sensitive(false)
        .collect_non_argument_text(true)
        .build();
    
    let config2 = config1.clone();
    
    assert!(!config2.is_case_sensitive());
    assert!(config2.collect_non_argument_text());
}