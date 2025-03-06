//! Error types and result definitions for the pam-args library.
//!
//! This module defines the library's error enum and result type aliases that will be used
//! throughout the library for error handling. It establishes a consistent, type-safe approach
//! to error reporting and propagation, leveraging Rust's powerful error handling mechanisms.

/// Represents all possible error conditions in the pam-args library
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// A required argument was not provided
    /// Contains the name of the missing argument
    RequiredArgMissing(String),
    
    /// Two mutually exclusive arguments were both provided
    /// Contains the names of the conflicting arguments
    MutuallyExclusiveArgs(String, String),
    
    /// A key-value argument has an invalid format
    /// Contains the problematic key-value string
    InvalidKeyValue(String),
    
    /// An unrecognized argument was encountered
    /// Contains the unrecognized argument string
    UnrecognizedArg(String),
    
    /// Failed to parse an argument value as an integer
    /// Contains the invalid value string
    InvalidIntValue(String),
    
    /// Failed to parse an argument value as a boolean
    /// Contains the invalid value string
    InvalidBoolValue(String),
    
    /// An argument dependency was not satisfied
    /// Contains the argument name and its dependency
    DependencyNotMet(String, String),
    
    /// An argument value is not in the allowed set
    /// Contains the argument name and the invalid value
    InvalidValue(String, String),
    
    /// An argument name is defined more than once
    /// Contains the duplicated argument name
    DuplicateArgName(String),
    
    /// A delimiter (quote or bracket) was not closed
    /// Contains information about the unclosed delimiter
    UnclosedDelimiter(String),
    
    /// Nested brackets are not supported
    /// Contains information about the nested brackets
    NestedBrackets(String),
    
    /// The input is invalid in some other way
    /// Contains a description of the issue
    InvalidInput(String),
    
    /// An unexpected error occurred
    /// Contains a description of the error
    UnexpectedError(String),
}

/// Type alias for Result with the library's Error type
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Returns a string representation of the error code for this error
    ///
    /// This is useful for programmatic error handling or for
    /// generating error codes in logs.
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::Error;
    ///
    /// let err = Error::RequiredArgMissing("USER".to_string());
    /// assert_eq!(err.code(), "REQUIRED_ARG_MISSING");
    /// ```
    pub fn code(&self) -> &'static str {
        match self {
            Error::RequiredArgMissing(_) => "REQUIRED_ARG_MISSING",
            Error::MutuallyExclusiveArgs(_, _) => "MUTUALLY_EXCLUSIVE_ARGS",
            Error::InvalidKeyValue(_) => "INVALID_KEY_VALUE",
            Error::UnrecognizedArg(_) => "UNRECOGNIZED_ARG",
            Error::InvalidIntValue(_) => "INVALID_INT_VALUE",
            Error::InvalidBoolValue(_) => "INVALID_BOOL_VALUE",
            Error::DependencyNotMet(_, _) => "DEPENDENCY_NOT_MET",
            Error::InvalidValue(_, _) => "INVALID_VALUE",
            Error::DuplicateArgName(_) => "DUPLICATE_ARG_NAME",
            Error::UnclosedDelimiter(_) => "UNCLOSED_DELIMITER",
            Error::NestedBrackets(_) => "NESTED_BRACKETS",
            Error::InvalidInput(_) => "INVALID_INPUT",
            Error::UnexpectedError(_) => "UNEXPECTED_ERROR",
        }
    }
    
    /// Provides a detailed user-friendly description of the error
    ///
    /// Unlike the `Display` implementation which is concise,
    /// this method provides more details about the error and
    /// possible solutions.
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::Error;
    ///
    /// let err = Error::RequiredArgMissing("USER".to_string());
    /// println!("Error details: {}", err.details());
    /// ```
    pub fn details(&self) -> String {
        match self {
            Error::RequiredArgMissing(arg) => {
                format!(
                    "The required argument '{}' was not provided. \
                     Please ensure all required arguments are included.",
                    arg
                )
            },
            Error::MutuallyExclusiveArgs(arg1, arg2) => {
                format!(
                    "The arguments '{}' and '{}' cannot be used together. \
                     Please provide only one of these arguments.",
                    arg1, arg2
                )
            },
            Error::InvalidKeyValue(kv) => {
                format!(
                    "The key-value pair '{}' has an invalid format. \
                     Key-value pairs should be in the format 'key=value'.",
                    kv
                )
            },
            Error::UnrecognizedArg(arg) => {
                format!(
                    "The argument '{}' is not recognized. \
                     Please check for typos or refer to the documentation for valid arguments.",
                    arg
                )
            },
            Error::InvalidIntValue(val) => {
                format!(
                    "The value '{}' could not be parsed as an integer. \
                     Please provide a valid integer value.",
                    val
                )
            },
            Error::InvalidBoolValue(val) => {
                format!(
                    "The value '{}' could not be parsed as a boolean. \
                     Valid boolean values include 'true', 'false', 'yes', 'no', '1', '0', 'on', 'off'.",
                    val
                )
            },
            Error::DependencyNotMet(arg, dep) => {
                format!(
                    "The argument '{}' requires '{}' to also be provided. \
                     Please include the required dependency.",
                    arg, dep
                )
            },
            Error::InvalidValue(arg, val) => {
                format!(
                    "The value '{}' is not valid for the argument '{}'. \
                     Please refer to the documentation for allowed values.",
                    val, arg
                )
            },
            Error::DuplicateArgName(arg) => {
                format!(
                    "The argument name '{}' is defined more than once. \
                     Each argument name must be unique.",
                    arg
                )
            },
            Error::UnclosedDelimiter(info) => {
                format!(
                    "An unclosed delimiter was found: {}. \
                     Please ensure all quotes and brackets are properly closed.",
                    info
                )
            },
            Error::NestedBrackets(info) => {
                format!(
                    "Nested brackets are not supported: {}. \
                     Please restructure your arguments to avoid nested brackets.",
                    info
                )
            },
            Error::InvalidInput(info) => {
                format!(
                    "The input is invalid: {}. \
                     Please check your input and try again.",
                    info
                )
            },
            Error::UnexpectedError(info) => {
                format!(
                    "An unexpected error occurred: {}. \
                     This is likely a bug in the library. Please report this issue.",
                    info
                )
            },
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RequiredArgMissing(arg) => 
                write!(f, "Required argument missing: {}", arg),
            Error::MutuallyExclusiveArgs(arg1, arg2) => 
                write!(f, "Mutually exclusive arguments found: {} and {}", arg1, arg2),
            Error::InvalidKeyValue(kv) => 
                write!(f, "Invalid key-value pair: {}", kv),
            Error::UnrecognizedArg(arg) => 
                write!(f, "Unrecognized argument: {}", arg),
            Error::InvalidIntValue(val) => 
                write!(f, "Invalid integer value: {}", val),
            Error::InvalidBoolValue(val) => 
                write!(f, "Invalid boolean value: {}", val),
            Error::DependencyNotMet(arg, dep) => 
                write!(f, "Dependency not met: {} requires {}", arg, dep),
            Error::InvalidValue(arg, val) => 
                write!(f, "Invalid value for {}: {}", arg, val),
            Error::DuplicateArgName(arg) => 
                write!(f, "Duplicate argument name: {}", arg),
            Error::UnclosedDelimiter(info) => 
                write!(f, "Unclosed delimiter: {}", info),
            Error::NestedBrackets(info) => 
                write!(f, "Nested brackets not supported: {}", info),
            Error::InvalidInput(info) => 
                write!(f, "Invalid input: {}", info),
            Error::UnexpectedError(info) => 
                write!(f, "Unexpected error: {}", info),
        }
    }
}

impl std::error::Error for Error {}

// Safe to use in multi-threaded contexts
unsafe impl Send for Error {}
unsafe impl Sync for Error {}

#[cfg(feature = "serde")]
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        
        let mut state = serializer.serialize_struct("Error", 3)?;
        state.serialize_field("code", self.code())?;
        state.serialize_field("message", &self.to_string())?;
        
        // Add specific fields based on error variant
        match self {
            Error::RequiredArgMissing(arg) => {
                state.serialize_field("argument", arg)?;
            },
            Error::MutuallyExclusiveArgs(arg1, arg2) => {
                state.serialize_field("argument1", arg1)?;
                state.serialize_field("argument2", arg2)?;
            },
            Error::InvalidKeyValue(kv) => {
                state.serialize_field("key_value", kv)?;
            },
            Error::UnrecognizedArg(arg) => {
                state.serialize_field("argument", arg)?;
            },
            Error::InvalidIntValue(val) => {
                state.serialize_field("value", val)?;
            },
            Error::InvalidBoolValue(val) => {
                state.serialize_field("value", val)?;
            },
            Error::DependencyNotMet(arg, dep) => {
                state.serialize_field("argument", arg)?;
                state.serialize_field("dependency", dep)?;
            },
            Error::InvalidValue(arg, val) => {
                state.serialize_field("argument", arg)?;
                state.serialize_field("value", val)?;
            },
            Error::DuplicateArgName(arg) => {
                state.serialize_field("argument", arg)?;
            },
            Error::UnclosedDelimiter(info) => {
                state.serialize_field("info", info)?;
            },
            Error::NestedBrackets(info) => {
                state.serialize_field("info", info)?;
            },
            Error::InvalidInput(info) => {
                state.serialize_field("info", info)?;
            },
            Error::UnexpectedError(info) => {
                state.serialize_field("info", info)?;
            },
        }
        
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Error {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;
        
        struct ErrorVisitor;
        
        impl<'de> Visitor<'de> for ErrorVisitor {
            type Value = Error;
            
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a pam-args Error")
            }
            
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Error, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut code: Option<String> = None;
                let mut message: Option<String> = None;
                let mut argument: Option<String> = None;
                let mut argument1: Option<String> = None;
                let mut argument2: Option<String> = None;
                let mut dependency: Option<String> = None;
                let mut value: Option<String> = None;
                let mut key_value: Option<String> = None;
                let mut info: Option<String> = None;
                
                while let Some(key) = map.next_key()? {
                    match key {
                        "code" => {
                            code = Some(map.next_value()?);
                        }
                        "message" => {
                            message = Some(map.next_value()?);
                        }
                        "argument" => {
                            argument = Some(map.next_value()?);
                        }
                        "argument1" => {
                            argument1 = Some(map.next_value()?);
                        }
                        "argument2" => {
                            argument2 = Some(map.next_value()?);
                        }
                        "dependency" => {
                            dependency = Some(map.next_value()?);
                        }
                        "value" => {
                            value = Some(map.next_value()?);
                        }
                        "key_value" => {
                            key_value = Some(map.next_value()?);
                        }
                        "info" => {
                            info = Some(map.next_value()?);
                        }
                        _ => {
                            // Skip unknown fields
                            let _: de::IgnoredAny = map.next_value()?;
                        }
                    }
                }
                
                let code = code.ok_or_else(|| de::Error::missing_field("code"))?;
                
                match code.as_str() {
                    "REQUIRED_ARG_MISSING" => {
                        let arg = argument.ok_or_else(|| de::Error::missing_field("argument"))?;
                        Ok(Error::RequiredArgMissing(arg))
                    }
                    "MUTUALLY_EXCLUSIVE_ARGS" => {
                        let arg1 = argument1.ok_or_else(|| de::Error::missing_field("argument1"))?;
                        let arg2 = argument2.ok_or_else(|| de::Error::missing_field("argument2"))?;
                        Ok(Error::MutuallyExclusiveArgs(arg1, arg2))
                    }
                    "INVALID_KEY_VALUE" => {
                        let kv = key_value.ok_or_else(|| de::Error::missing_field("key_value"))?;
                        Ok(Error::InvalidKeyValue(kv))
                    }
                    "UNRECOGNIZED_ARG" => {
                        let arg = argument.ok_or_else(|| de::Error::missing_field("argument"))?;
                        Ok(Error::UnrecognizedArg(arg))
                    }
                    "INVALID_INT_VALUE" => {
                        let val = value.ok_or_else(|| de::Error::missing_field("value"))?;
                        Ok(Error::InvalidIntValue(val))
                    }
                    "INVALID_BOOL_VALUE" => {
                        let val = value.ok_or_else(|| de::Error::missing_field("value"))?;
                        Ok(Error::InvalidBoolValue(val))
                    }
                    "DEPENDENCY_NOT_MET" => {
                        let arg = argument.ok_or_else(|| de::Error::missing_field("argument"))?;
                        let dep = dependency.ok_or_else(|| de::Error::missing_field("dependency"))?;
                        Ok(Error::DependencyNotMet(arg, dep))
                    }
                    "INVALID_VALUE" => {
                        let arg = argument.ok_or_else(|| de::Error::missing_field("argument"))?;
                        let val = value.ok_or_else(|| de::Error::missing_field("value"))?;
                        Ok(Error::InvalidValue(arg, val))
                    }
                    "DUPLICATE_ARG_NAME" => {
                        let arg = argument.ok_or_else(|| de::Error::missing_field("argument"))?;
                        Ok(Error::DuplicateArgName(arg))
                    }
                    "UNCLOSED_DELIMITER" => {
                        let i = info.ok_or_else(|| de::Error::missing_field("info"))?;
                        Ok(Error::UnclosedDelimiter(i))
                    }
                    "NESTED_BRACKETS" => {
                        let i = info.ok_or_else(|| de::Error::missing_field("info"))?;
                        Ok(Error::NestedBrackets(i))
                    }
                    "INVALID_INPUT" => {
                        let i = info.ok_or_else(|| de::Error::missing_field("info"))?;
                        Ok(Error::InvalidInput(i))
                    }
                    "UNEXPECTED_ERROR" => {
                        let i = info.ok_or_else(|| de::Error::missing_field("info"))?;
                        Ok(Error::UnexpectedError(i))
                    }
                    _ => {
                        // Default to UnexpectedError if code is unknown
                        let msg = message.unwrap_or_else(|| "Unknown error code".to_string());
                        Ok(Error::UnexpectedError(msg))
                    }
                }
            }
        }
        
        deserializer.deserialize_map(ErrorVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_creation() {
        let err = Error::RequiredArgMissing("USER".to_string());
        assert!(matches!(err, Error::RequiredArgMissing(_)));
        
        let err = Error::MutuallyExclusiveArgs("DEBUG".to_string(), "QUIET".to_string());
        assert!(matches!(err, Error::MutuallyExclusiveArgs(_, _)));
    }
    
    #[test]
    fn test_display_trait() {
        let err = Error::RequiredArgMissing("USER".to_string());
        assert_eq!(format!("{}", err), "Required argument missing: USER");
        
        let err = Error::MutuallyExclusiveArgs("DEBUG".to_string(), "QUIET".to_string());
        assert_eq!(format!("{}", err), "Mutually exclusive arguments found: DEBUG and QUIET");
    }
    
    #[test]
    fn test_error_code() {
        let err = Error::RequiredArgMissing("USER".to_string());
        assert_eq!(err.code(), "REQUIRED_ARG_MISSING");
        
        let err = Error::InvalidValue("ALIGN".to_string(), "TOP".to_string());
        assert_eq!(err.code(), "INVALID_VALUE");
    }
    
    #[test]
    fn test_error_details() {
        let err = Error::RequiredArgMissing("USER".to_string());
        assert!(err.details().contains("required argument 'USER' was not provided"));
        
        let err = Error::InvalidBoolValue("MAYBE".to_string());
        assert!(err.details().contains("could not be parsed as a boolean"));
    }
    
    #[test]
    fn test_debug_trait() {
        let err = Error::InvalidValue("ALIGN".to_string(), "TOP".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("InvalidValue"));
        assert!(debug_str.contains("ALIGN"));
        assert!(debug_str.contains("TOP"));
    }
    
    #[test]
    fn test_error_trait() {
        let err = Error::InvalidInput("test".to_string());
        let err_trait: Box<dyn std::error::Error> = Box::new(err);
        assert!(err_trait.to_string().contains("Invalid input"));
    }
    
    #[test]
    fn test_pattern_matching() {
        let err = Error::RequiredArgMissing("USER".to_string());
        
        let message = match err {
            Error::RequiredArgMissing(arg) => format!("Missing: {}", arg),
            _ => "Other error".to_string(),
        };
        
        assert_eq!(message, "Missing: USER");
    }
    
    #[test]
    fn test_result_type() {
        let result: Result<()> = Err(Error::InvalidIntValue("abc".to_string()));
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(matches!(e, Error::InvalidIntValue(_)));
        } else {
            panic!("Expected Err variant");
        }
    }
    
    #[test]
    fn test_clone() {
        let err = Error::DuplicateArgName("FLAG".to_string());
        let err2 = err.clone();
        
        assert_eq!(err, err2);
    }
    
    #[test]
    fn test_partial_eq() {
        let err1 = Error::RequiredArgMissing("USER".to_string());
        let err2 = Error::RequiredArgMissing("USER".to_string());
        let err3 = Error::RequiredArgMissing("PASSWORD".to_string());
        
        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }
}