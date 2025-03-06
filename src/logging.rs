//! Logging integration for the pam-args library.
//!
//! This module provides structured logging functionality that integrates with the standard
//! Rust `log` crate. It offers consistent logging patterns, appropriate log levels, and
//! contextual information across all library components, enhancing debuggability and
//! observability without adding overhead in production environments.
//!
//! # Examples
//!
//! ## Initializing Logging
//!
//! For a PAM module:
//!
//! ```rust
//! use pam_args::logging::init;
//! use log::LevelFilter;
//!
//! // In your PAM module initialization
//! fn initialize_pam_module() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize logging with the PAM module name
//!     if let Err(e) = init::for_pam("pam_mymodule", LevelFilter::Info) {
//!         // Fall back to stderr if logging initialization fails
//!         eprintln!("Warning: Failed to initialize logging: {}", e);
//!     }
//!
//!     // Rest of your PAM module initialization...
//!     Ok(())
//! }
//! ```
//!
//! For development:
//!
//! ```rust
//! use pam_args::logging::init;
//! use log::LevelFilter;
//!
//! // During development or testing
//! fn main() {
//!     // Initialize logging to terminal with debug level
//!     init::for_development(LevelFilter::Debug).expect("Failed to initialize logging");
//!
//!     // Your application code...
//! }
//! ```
//!
//! ## Using the Logging Macros
//!
//! ```rust
//! use pam_args::{log_debug, log_error, log_info};
//! use pam_args::logging::{LogComponent, LogOperation};
//!
//! // Simple informational message
//! log_info!(LogComponent::Parser, LogOperation::Parse, "Starting to parse arguments");
//!
//! // Debug message with data
//! let args = vec!["DEBUG", "USER=admin"];
//! log_debug!(LogComponent::Parser, LogOperation::Parse, "Processing arguments", &args);
//!
//! // Error logging with context
//! if let Err(error) = some_operation() {
//!     log_error!(
//!         LogComponent::Validator,
//!         LogOperation::Validate,
//!         "Validation failed",
//!         &error
//!     );
//! }
//! # fn some_operation() -> Result<(), pam_args::Error> { Ok(()) }
//! ```

use crate::error::{Error, Result};
use std::fmt::Debug;

/// Represents a component within the library for logging purposes
#[derive(Debug, Clone, Copy)]
pub enum LogComponent {
    /// General library operations
    General,
    
    /// Parser component
    Parser,
    
    /// Tokenizer component
    Tokenizer,
    
    /// Key-value store component
    KeyValueStore,
    
    /// Validation component
    Validator,
    
    /// Field binding component
    FieldBinding,
}

impl LogComponent {
    /// Returns the string representation of this component
    pub fn as_str(&self) -> &'static str {
        match self {
            LogComponent::General => "GENERAL",
            LogComponent::Parser => "PARSER",
            LogComponent::Tokenizer => "TOKENIZER",
            LogComponent::KeyValueStore => "KV_STORE",
            LogComponent::Validator => "VALIDATOR",
            LogComponent::FieldBinding => "BINDING",
        }
    }
}

/// Represents a logging operation within a component
#[derive(Debug, Clone, Copy)]
pub enum LogOperation {
    /// Initialization operation
    Init,
    
    /// Configuration operation
    Config,
    
    /// Parsing operation
    Parse,
    
    /// Tokenization operation
    Tokenize,
    
    /// Validation operation
    Validate,
    
    /// Error handling operation
    Error,
    
    /// General operation
    Operation,
}

impl LogOperation {
    /// Returns the string representation of this operation
    pub fn as_str(&self) -> &'static str {
        match self {
            LogOperation::Init => "INIT",
            LogOperation::Config => "CONFIG",
            LogOperation::Parse => "PARSE",
            LogOperation::Tokenize => "TOKENIZE",
            LogOperation::Validate => "VALIDATE",
            LogOperation::Error => "ERROR",
            LogOperation::Operation => "OP",
        }
    }
}

/// Configuration for the logging system
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// Whether to include timestamps in log messages
    pub include_timestamps: bool,
    
    /// Whether to include component information in log messages
    pub include_component: bool,
    
    /// Whether to include operation information in log messages
    pub include_operation: bool,
    
    /// Whether to use JSON format for logs
    pub json_format: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            include_timestamps: true,
            include_component: true,
            include_operation: true,
            json_format: false,
        }
    }
}

/// Available logging destinations
#[derive(Debug, Clone, Copy)]
pub enum LogDestination {
    /// Log to the system's syslog facility (default for PAM modules)
    Syslog,
    
    /// Log to the terminal (stderr)
    Terminal,
    
    /// Log to both syslog and terminal
    Both,
    
    /// Do not initialize any logger (use existing or none)
    None,
}

/// Logging initialization options
#[derive(Debug, Clone)]
pub struct LogOptions {
    /// The destination for log messages
    pub destination: LogDestination,
    
    /// The maximum log level to enable
    pub level: log::LevelFilter,
    
    /// The configuration for log message formatting
    pub config: LogConfig,
    
    /// The facility to use when logging to syslog
    pub syslog_facility: Option<syslog::Facility>,
    
    /// The identifier to use in syslog messages
    pub syslog_identifier: Option<String>,
}

impl Default for LogOptions {
    fn default() -> Self {
        Self {
            destination: LogDestination::Syslog,
            level: log::LevelFilter::Info,
            config: LogConfig::default(),
            syslog_facility: Some(syslog::Facility::LOG_AUTH),
            syslog_identifier: Some("pam_args".to_string()),
        }
    }
}

/// Main logging module providing functions for different log levels
pub mod logger {
    use super::*;
    use crate::error::Error;
    use log::{debug, error, info, trace, warn};
    use std::fmt::Debug;
    
    // Configure the global logging configuration
    static LOG_CONFIG: std::sync::OnceLock<LogConfig> = std::sync::OnceLock::new();
    
    /// Initialize the logger configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration to use for logging
    pub fn init(config: LogConfig) {
        let _ = LOG_CONFIG.set(config);
    }
    
    /// Get the current logger configuration
    ///
    /// # Returns
    ///
    /// The current logging configuration
    pub fn config() -> &'static LogConfig {
        LOG_CONFIG.get_or_init(LogConfig::default)
    }
    
    /// Log a message at TRACE level
    ///
    /// # Arguments
    ///
    /// * `component` - The component generating the log
    /// * `operation` - The operation being performed
    /// * `message` - The log message
    /// * `args` - Optional data to include in the log
    pub fn trace_log<D: Debug>(
        component: LogComponent,
        operation: LogOperation,
        message: &str,
        args: Option<D>,
    ) {
        if !log::log_enabled!(log::Level::Trace) {
            return;
        }
        
        let config = config();
        
        if config.json_format {
            trace!(
                "{{\"level\":\"TRACE\",\"component\":\"{}\",\"operation\":\"{}\",\"message\":\"{}\",\"data\":{}}}",
                component.as_str(),
                operation.as_str(),
                message,
                format!("{:?}", args)
            );
        } else {
            let comp_str = if config.include_component {
                format!("[{}]", component.as_str())
            } else {
                String::new()
            };
            
            let op_str = if config.include_operation {
                format!("[{}]", operation.as_str())
            } else {
                String::new()
            };
            
            match args {
                Some(data) => trace!("{}{} {} - {:?}", comp_str, op_str, message, data),
                None => trace!("{}{} {}", comp_str, op_str, message),
            }
        }
    }
    
    /// Log a message at DEBUG level
    ///
    /// # Arguments
    ///
    /// * `component` - The component generating the log
    /// * `operation` - The operation being performed
    /// * `message` - The log message
    /// * `args` - Optional data to include in the log
    pub fn debug_log<D: Debug>(
        component: LogComponent,
        operation: LogOperation,
        message: &str,
        args: Option<D>,
    ) {
        if !log::log_enabled!(log::Level::Debug) {
            return;
        }
        
        let config = config();
        
        if config.json_format {
            debug!(
                "{{\"level\":\"DEBUG\",\"component\":\"{}\",\"operation\":\"{}\",\"message\":\"{}\",\"data\":{}}}",
                component.as_str(),
                operation.as_str(),
                message,
                format!("{:?}", args)
            );
        } else {
            let comp_str = if config.include_component {
                format!("[{}]", component.as_str())
            } else {
                String::new()
            };
            
            let op_str = if config.include_operation {
                format!("[{}]", operation.as_str())
            } else {
                String::new()
            };
            
            match args {
                Some(data) => debug!("{}{} {} - {:?}", comp_str, op_str, message, data),
                None => debug!("{}{} {}", comp_str, op_str, message),
            }
        }
    }
    
    /// Log a message at INFO level
    ///
    /// # Arguments
    ///
    /// * `component` - The component generating the log
    /// * `operation` - The operation being performed
    /// * `message` - The log message
    /// * `args` - Optional data to include in the log
    pub fn info_log<D: Debug>(
        component: LogComponent,
        operation: LogOperation,
        message: &str,
        args: Option<D>,
    ) {
        if !log::log_enabled!(log::Level::Info) {
            return;
        }
        
        let config = config();
        
        if config.json_format {
            info!(
                "{{\"level\":\"INFO\",\"component\":\"{}\",\"operation\":\"{}\",\"message\":\"{}\",\"data\":{}}}",
                component.as_str(),
                operation.as_str(),
                message,
                format!("{:?}", args)
            );
        } else {
            let comp_str = if config.include_component {
                format!("[{}]", component.as_str())
            } else {
                String::new()
            };
            
            let op_str = if config.include_operation {
                format!("[{}]", operation.as_str())
            } else {
                String::new()
            };
            
            match args {
                Some(data) => info!("{}{} {} - {:?}", comp_str, op_str, message, data),
                None => info!("{}{} {}", comp_str, op_str, message),
            }
        }
    }
    
    /// Log a message at WARN level
    ///
    /// # Arguments
    ///
    /// * `component` - The component generating the log
    /// * `operation` - The operation being performed
    /// * `message` - The log message
    /// * `args` - Optional data to include in the log
    pub fn warn_log<D: Debug>(
        component: LogComponent,
        operation: LogOperation,
        message: &str,
        args: Option<D>,
    ) {
        if !log::log_enabled!(log::Level::Warn) {
            return;
        }
        
        let config = config();
        
        if config.json_format {
            warn!(
                "{{\"level\":\"WARN\",\"component\":\"{}\",\"operation\":\"{}\",\"message\":\"{}\",\"data\":{}}}",
                component.as_str(),
                operation.as_str(),
                message,
                format!("{:?}", args)
            );
        } else {
            let comp_str = if config.include_component {
                format!("[{}]", component.as_str())
            } else {
                String::new()
            };
            
            let op_str = if config.include_operation {
                format!("[{}]", operation.as_str())
            } else {
                String::new()
            };
            
            match args {
                Some(data) => warn!("{}{} {} - {:?}", comp_str, op_str, message, data),
                None => warn!("{}{} {}", comp_str, op_str, message),
            }
        }
    }
    
    /// Log a message at ERROR level
    ///
    /// # Arguments
    ///
    /// * `component` - The component generating the log
    /// * `operation` - The operation being performed
    /// * `message` - The log message
    /// * `error` - Optional error to include in the log
    pub fn error_log(
        component: LogComponent,
        operation: LogOperation,
        message: &str,
        error: Option<&Error>,
    ) {
        if !log::log_enabled!(log::Level::Error) {
            return;
        }
        
        let config = config();
        
        if config.json_format {
            let error_data = match error {
                Some(e) => format!("{{\"code\":\"{}\",\"message\":\"{}\"}}", e.code(), e),
                None => "null".to_string(),
            };
            
            error!(
                "{{\"level\":\"ERROR\",\"component\":\"{}\",\"operation\":\"{}\",\"message\":\"{}\",\"error\":{}}}",
                component.as_str(),
                operation.as_str(),
                message,
                error_data
            );
        } else {
            let comp_str = if config.include_component {
                format!("[{}]", component.as_str())
            } else {
                String::new()
            };
            
            let op_str = if config.include_operation {
                format!("[{}]", operation.as_str())
            } else {
                String::new()
            };
            
            match error {
                Some(e) => error!("{}{} {} - Error: {} ({})", comp_str, op_str, message, e, e.code()),
                None => error!("{}{} {}", comp_str, op_str, message),
            }
        }
    }
    
    /// Log a parser event at appropriate level
    ///
    /// # Arguments
    ///
    /// * `message` - The log message
    /// * `args` - Arguments being parsed
    pub fn parser_event(message: &str, args: &[String]) {
        debug_log(
            LogComponent::Parser,
            LogOperation::Parse,
            message,
            Some(args),
        );
    }
    
    /// Log a tokenizer event at appropriate level
    ///
    /// # Arguments
    ///
    /// * `message` - The log message
    /// * `input` - Input being tokenized
    /// * `tokens` - Resulting tokens
    pub fn tokenizer_event(message: &str, input: &str, tokens: Option<&[String]>) {
        if let Some(t) = tokens {
            trace_log(
                LogComponent::Tokenizer,
                LogOperation::Tokenize,
                message,
                Some((input, t)),
            );
        } else {
            trace_log(
                LogComponent::Tokenizer,
                LogOperation::Tokenize,
                message,
                Some(input),
            );
        }
    }
    
    /// Log a validation event at appropriate level
    ///
    /// # Arguments
    ///
    /// * `message` - The log message
    /// * `args` - Optional data about validation
    pub fn validation_event<D: Debug>(message: &str, args: Option<D>) {
        debug_log(
            LogComponent::Validator,
            LogOperation::Validate,
            message,
            args,
        );
    }
    
    /// Log an error with context
    ///
    /// # Arguments
    ///
    /// * `component` - The component where the error occurred
    /// * `operation` - The operation being performed
    /// * `message` - Context message explaining what was happening
    /// * `error` - The error that occurred
    pub fn error_event(
        component: LogComponent,
        operation: LogOperation,
        message: &str,
        error: &Error,
    ) {
        error_log(component, operation, message, Some(error));
    }
}

/// Public logging initialization functions
pub mod init {
    use super::*;
    use crate::error::{Error, Result};
    use log::LevelFilter;
    
    /// Initialize logging with the specified options
    ///
    /// This function allows full control over logging configuration.
    ///
    /// # Arguments
    ///
    /// * `options` - The logging options to use
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::logging::{LogOptions, LogDestination};
    /// use log::LevelFilter;
    ///
    /// // Initialize logging to both syslog and terminal with debug level
    /// let options = LogOptions {
    ///     destination: LogDestination::Both,
    ///     level: LevelFilter::Debug,
    ///     ..Default::default()
    /// };
    ///
    /// pam_args::logging::init::with_options(&options)?;
    /// # Ok::<(), pam_args::Error>(())
    /// ```
    pub fn with_options(options: &LogOptions) -> Result<()> {
        // Initialize the logger configuration
        super::logger::init(options.config.clone());
        
        match options.destination {
            LogDestination::Syslog => {
                init_syslog(
                    options.level,
                    options.syslog_facility.unwrap_or(syslog::Facility::LOG_AUTH),
                    options.syslog_identifier.clone().unwrap_or_else(|| "pam_args".to_string()),
                )?;
            }
            LogDestination::Terminal => {
                init_terminal(options.level)?;
            }
            LogDestination::Both => {
                init_both(
                    options.level,
                    options.syslog_facility.unwrap_or(syslog::Facility::LOG_AUTH),
                    options.syslog_identifier.clone().unwrap_or_else(|| "pam_args".to_string()),
                )?;
            }
            LogDestination::None => {
                // Do nothing, use existing logger or none
            }
        }
        
        Ok(())
    }
    
    /// Initialize logging for a PAM module
    ///
    /// This is a convenience function that sets up syslog logging
    /// with the AUTH facility, which is typical for PAM modules.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The identifier to use in syslog messages
    /// * `level` - The maximum log level to enable
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::logging::init;
    /// use log::LevelFilter;
    ///
    /// // In a PAM module's initialization code
    /// init::for_pam("pam_mymodule", LevelFilter::Info)?;
    /// # Ok::<(), pam_args::Error>(())
    /// ```
    pub fn for_pam(identifier: &str, level: LevelFilter) -> Result<()> {
        let options = LogOptions {
            destination: LogDestination::Syslog,
            level,
            syslog_facility: Some(syslog::Facility::LOG_AUTH),
            syslog_identifier: Some(identifier.to_string()),
            ..Default::default()
        };
        
        with_options(&options)
    }
    
    /// Initialize logging for development with terminal output
    ///
    /// This is a convenience function that sets up terminal logging,
    /// which is useful during development and debugging.
    ///
    /// # Arguments
    ///
    /// * `level` - The maximum log level to enable
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::logging::init;
    /// use log::LevelFilter;
    ///
    /// // During development or in tests
    /// init::for_development(LevelFilter::Debug)?;
    /// # Ok::<(), pam_args::Error>(())
    /// ```
    pub fn for_development(level: LevelFilter) -> Result<()> {
        let options = LogOptions {
            destination: LogDestination::Terminal,
            level,
            ..Default::default()
        };
        
        with_options(&options)
    }
    
    /// Initialize logging with both syslog and terminal output
    ///
    /// This is useful for debugging PAM modules where you want logs
    /// to go to syslog as well as see them on the terminal.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The identifier to use in syslog messages
    /// * `level` - The maximum log level to enable
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    ///
    /// # Examples
    ///
    /// ```
    /// use pam_args::logging::init;
    /// use log::LevelFilter;
    ///
    /// // For debugging a PAM module
    /// init::dual_output("pam_mymodule", LevelFilter::Debug)?;
    /// # Ok::<(), pam_args::Error>(())
    /// ```
    pub fn dual_output(identifier: &str, level: LevelFilter) -> Result<()> {
        let options = LogOptions {
            destination: LogDestination::Both,
            level,
            syslog_identifier: Some(identifier.to_string()),
            ..Default::default()
        };
        
        with_options(&options)
    }
    
    // Private helper functions for logger initialization
    
    fn init_syslog(level: LevelFilter, facility: syslog::Facility, identifier: String) -> Result<()> {
        // Create a formatter with the specified identifier
        let formatter = syslog::Formatter3164 {
            facility,
            hostname: None,
            process: identifier,
            pid: std::process::id(),
        };
        
        // Connect to syslog with the specified facility
        let syslog_logger = syslog::unix(formatter)
            .map_err(|e| Error::UnexpectedError(format!("Failed to connect to syslog: {}", e)))?;
        
        // Initialize the logger
        log::set_boxed_logger(Box::new(syslog::BasicLogger::new(syslog_logger)))
            .map_err(|e| Error::UnexpectedError(format!("Failed to set logger: {}", e)))?;
        
        // Set the log level
        log::set_max_level(level);
        
        Ok(())
    }
    
    fn init_terminal(level: LevelFilter) -> Result<()> {
        // Create a terminal logger using env_logger
        use env_logger::{Builder, Env};
        
        // Build a logger with the specified level
        let mut builder = Builder::from_env(Env::default().default_filter_or(level.to_string()));
        builder.format_timestamp_secs(); // Add timestamps to logs
        
        // Initialize the logger
        builder.try_init()
            .map_err(|e| Error::UnexpectedError(format!("Failed to initialize terminal logger: {}", e)))?;
        
        Ok(())
    }
    
    fn init_both(level: LevelFilter, facility: syslog::Facility, identifier: String) -> Result<()> {
        // Create a logger that sends to both syslog and terminal
        
        // Initialize syslog logger
        init_syslog(level, facility, identifier)?;
        
        // Also initialize terminal logger (this will fail if a logger is already set)
        // So we just print to stderr directly
        eprintln!("Note: Dual output logging is limited. Logs will primarily go to syslog, with some messages to terminal.");
        
        Ok(())
    }
}

// Logging macros to make usage more ergonomic
#[macro_export]
macro_rules! log_trace {
    ($component:expr, $operation:expr, $message:expr) => {
        if log::log_enabled!(log::Level::Trace) {
            $crate::logging::logger::trace_log($component, $operation, $message, None::<()>);
        }
    };
    ($component:expr, $operation:expr, $message:expr, $data:expr) => {
        if log::log_enabled!(log::Level::Trace) {
            $crate::logging::logger::trace_log($component, $operation, $message, Some($data));
        }
    };
}

#[macro_export]
macro_rules! log_debug {
    ($component:expr, $operation:expr, $message:expr) => {
        if log::log_enabled!(log::Level::Debug) {
            $crate::logging::logger::debug_log($component, $operation, $message, None::<()>);
        }
    };
    ($component:expr, $operation:expr, $message:expr, $data:expr) => {
        if log::log_enabled!(log::Level::Debug) {
            $crate::logging::logger::debug_log($component, $operation, $message, Some($data));
        }
    };
}

#[macro_export]
macro_rules! log_info {
    ($component:expr, $operation:expr, $message:expr) => {
        if log::log_enabled!(log::Level::Info) {
            $crate::logging::logger::info_log($component, $operation, $message, None::<()>);
        }
    };
    ($component:expr, $operation:expr, $message:expr, $data:expr) => {
        if log::log_enabled!(log::Level::Info) {
            $crate::logging::logger::info_log($component, $operation, $message, Some($data));
        }
    };
}

#[macro_export]
macro_rules! log_warn {
    ($component:expr, $operation:expr, $message:expr) => {
        if log::log_enabled!(log::Level::Warn) {
            $crate::logging::logger::warn_log($component, $operation, $message, None::<()>);
        }
    };
    ($component:expr, $operation:expr, $message:expr, $data:expr) => {
        if log::log_enabled!(log::Level::Warn) {
            $crate::logging::logger::warn_log($component, $operation, $message, Some($data));
        }
    };
}

#[macro_export]
macro_rules! log_error {
    ($component:expr, $operation:expr, $message:expr) => {
        if log::log_enabled!(log::Level::Error) {
            $crate::logging::logger::error_log($component, $operation, $message, None);
        }
    };
    ($component:expr, $operation:expr, $message:expr, $error:expr) => {
        if log::log_enabled!(log::Level::Error) {
            $crate::logging::logger::error_log($component, $operation, $message, Some($error));
        }
    };
}

#[macro_export]
macro_rules! log_parser {
    ($message:expr, $args:expr) => {
        if log::log_enabled!(log::Level::Debug) {
            $crate::logging::logger::parser_event($message, $args);
        }
    };
}

#[macro_export]
macro_rules! log_tokenizer {
    ($message:expr, $input:expr) => {
        if log::log_enabled!(log::Level::Trace) {
            $crate::logging::logger::tokenizer_event($message, $input, None);
        }
    };
    ($message:expr, $input:expr, $tokens:expr) => {
        if log::log_enabled!(log::Level::Trace) {
            $crate::logging::logger::tokenizer_event($message, $input, Some($tokens));
        }
    };
}

#[macro_export]
macro_rules! log_validation {
    ($message:expr) => {
        if log::log_enabled!(log::Level::Debug) {
            $crate::logging::logger::validation_event($message, None::<()>);
        }
    };
    ($message:expr, $data:expr) => {
        if log::log_enabled!(log::Level::Debug) {
            $crate::logging::logger::validation_event($message, Some($data));
        }
    };
}

#[macro_export]
macro_rules! log_error_ctx {
    ($component:expr, $operation:expr, $message:expr, $error:expr) => {
        if log::log_enabled!(log::Level::Error) {
            $crate::logging::logger::error_event($component, $operation, $message, $error);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use log::LevelFilter;
    
    #[test]
    fn test_log_component_as_str() {
        assert_eq!(LogComponent::General.as_str(), "GENERAL");
        assert_eq!(LogComponent::Parser.as_str(), "PARSER");
        assert_eq!(LogComponent::Tokenizer.as_str(), "TOKENIZER");
        assert_eq!(LogComponent::KeyValueStore.as_str(), "KV_STORE");
        assert_eq!(LogComponent::Validator.as_str(), "VALIDATOR");
        assert_eq!(LogComponent::FieldBinding.as_str(), "BINDING");
    }
    
    #[test]
    fn test_log_operation_as_str() {
        assert_eq!(LogOperation::Init.as_str(), "INIT");
        assert_eq!(LogOperation::Config.as_str(), "CONFIG");
        assert_eq!(LogOperation::Parse.as_str(), "PARSE");
        assert_eq!(LogOperation::Tokenize.as_str(), "TOKENIZE");
        assert_eq!(LogOperation::Validate.as_str(), "VALIDATE");
        assert_eq!(LogOperation::Error.as_str(), "ERROR");
        assert_eq!(LogOperation::Operation.as_str(), "OP");
    }
    
    #[test]
    fn test_log_config_default() {
        let config = LogConfig::default();
        assert!(config.include_timestamps);
        assert!(config.include_component);
        assert!(config.include_operation);
        assert!(!config.json_format);
    }
    
    #[test]
    fn test_log_options_default() {
        let options = LogOptions::default();
        assert!(matches!(options.destination, LogDestination::Syslog));
        assert_eq!(options.level, LevelFilter::Info);
        assert!(matches!(options.syslog_facility, Some(syslog::Facility::LOG_AUTH)));
        assert_eq!(options.syslog_identifier, Some("pam_args".to_string()));
    }
    
    // Note: More comprehensive tests would require mocking the log crate
    // or using a test logger implementation. These tests just verify the
    // basic functionality and structure of the logging system.
    
    #[test]
    fn test_logger_config() {
        // Test that the logger configuration can be initialized and retrieved
        let config = LogConfig {
            include_timestamps: false,
            include_component: true,
            include_operation: false,
            json_format: true,
        };
        
        logger::init(config.clone());
        
        let retrieved_config = logger::config();
        assert_eq!(retrieved_config.include_timestamps, config.include_timestamps);
        assert_eq!(retrieved_config.include_component, config.include_component);
        assert_eq!(retrieved_config.include_operation, config.include_operation);
        assert_eq!(retrieved_config.json_format, config.json_format);
    }
    
    #[test]
    fn test_specialized_logging_functions() {
        // This test just verifies that the specialized logging functions don't panic
        // A more comprehensive test would verify the actual log output
        
        let args = vec!["DEBUG".to_string(), "USER=admin".to_string()];
        logger::parser_event("Test parser event", &args);
        
        logger::tokenizer_event("Test tokenizer event", "input", None);
        logger::tokenizer_event("Test tokenizer event with tokens", "input", Some(&args));
        
        logger::validation_event("Test validation event", None::<()>);
        logger::validation_event("Test validation event with data", Some("data"));
        
        let error = Error::InvalidInput("test".to_string());
        logger::error_event(LogComponent::General, LogOperation::Operation, "Test error event", &error);
    }
}