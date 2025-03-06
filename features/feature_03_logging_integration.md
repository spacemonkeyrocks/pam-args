# Feature 3: Logging Integration

## Module Type
**Utility**: This component provides structured logging functionality used throughout the library. While primarily an internal implementation detail, it includes public initialization functions to give users control over logging destinations and formats.

## Feature Information

**Feature Name**: Logging Integration

**Description**: Implements a structured logging system that integrates with the standard Rust `log` crate. This component provides consistent logging patterns, appropriate log levels, and contextual information across all library components, enhancing debuggability and observability without adding overhead in production environments.

**Priority**: High

**Dependencies**: 
- [Feature 1: Result & Error Types](error-result-types.md)
- [Feature 2: Utility Functions](utility-functions.md)

## Requirements

### Functional Requirements
1. Integrate with the standard Rust `log` crate for flexibility in logging backends
2. Provide consistent logging patterns across different library components
3. Support multiple log levels (trace, debug, info, warn, error)
4. Include contextual information in log messages (component, operation, argument values)
5. Ensure log messages are informative without exposing sensitive data
6. Allow for conditional logging based on compilation features
7. Support structured logging for machine parsability
8. Provide configurable logging destinations (syslog, terminal, or both)
9. Support custom logging backends through the standard `log` crate interface

### API Requirements
- Provide simple, ergonomic macros/functions for logging at different levels
- Ensure logging has minimal performance impact when disabled
- Avoid exposing logging implementation details to library users
- Support both human-readable and structured log formats
- Enable logging customization without requiring code changes
- Provide public initialization functions for common logging destinations
- Allow PAM modules to easily log to syslog (the standard PAM logging destination)
- Support development workflows with terminal logging options

### Performance Requirements
- Logging should have zero cost when disabled at compile time
- Log message formatting should be deferred until needed
- Avoid heap allocations in hot logging paths
- Minimize impact on critical code paths

## Design

### Data Structures
```rust
/// Represents a component within the library for logging purposes
#[derive(Debug, Clone, Copy)]
pub(crate) enum LogComponent {
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
    pub(crate) fn as_str(&self) -> &'static str {
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
pub(crate) enum LogOperation {
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
    pub(crate) fn as_str(&self) -> &'static str {
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
pub(crate) struct LogConfig {
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
            syslog_identifier: Some("pam_args-rs".to_string()),
        }
    }
}
```

### Function Signatures

```rust
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
    /// use pam_args_rs::logging::{LogOptions, LogDestination};
    /// use log::LevelFilter;
    ///
    /// // Initialize logging to both syslog and terminal with debug level
    /// let options = LogOptions {
    ///     destination: LogDestination::Both,
    ///     level: LevelFilter::Debug,
    ///     ..Default::default()
    /// };
    ///
    /// pam_args_rs::logging::init::with_options(&options)?;
    /// # Ok::<(), pam_args_rs::error::Error>(())
    /// ```
    pub fn with_options(options: &LogOptions) -> Result<()> {
        // Initialize the logger configuration
        super::logger::init(options.config.clone());
        
        match options.destination {
            LogDestination::Syslog => {
                init_syslog(
                    options.level,
                    options.syslog_facility.unwrap_or(syslog::Facility::LOG_AUTH),
                    options.syslog_identifier.clone().unwrap_or_else(|| "pam_args-rs".to_string()),
                )?;
            }
            LogDestination::Terminal => {
                init_terminal(options.level)?;
            }
            LogDestination::Both => {
                init_both(
                    options.level,
                    options.syslog_facility.unwrap_or(syslog::Facility::LOG_AUTH),
                    options.syslog_identifier.clone().unwrap_or_else(|| "pam_args-rs".to_string()),
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
    /// use pam_args_rs::logging::init;
    /// use log::LevelFilter;
    ///
    /// // In a PAM module's initialization code
    /// init::for_pam("pam_mymodule", LevelFilter::Info)?;
    /// # Ok::<(), pam_args_rs::error::Error>(())
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
    /// use pam_args_rs::logging::init;
    /// use log::LevelFilter;
    ///
    /// // During development or in tests
    /// init::for_development(LevelFilter::Debug)?;
    /// # Ok::<(), pam_args_rs::error::Error>(())
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
    /// use pam_args_rs::logging::init;
    /// use log::LevelFilter;
    ///
    /// // For debugging a PAM module
    /// init::dual_output("pam_mymodule", LevelFilter::Debug)?;
    /// # Ok::<(), pam_args_rs::error::Error>(())
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
        let formatter = syslog::Formatter3164::new(identifier);
        
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
        
        // For simplicity, we'll use the log-mux crate
        // In a real implementation, you'd want to add proper error handling
        
        #[cfg(feature = "log-mux")]
        {
            use log_mux::MuxLogger;
            
            // Set up syslog logger
            let formatter = syslog::Formatter3164::new(identifier);
            let syslog_conn = syslog::unix(formatter)
                .map_err(|e| Error::UnexpectedError(format!("Failed to connect to syslog: {}", e)))?;
            let syslog_logger = Box::new(syslog::BasicLogger::new(syslog_conn));
            
            // Set up terminal logger
            let mut builder = env_logger::Builder::new();
            builder.filter_level(level);
            builder.format_timestamp_secs();
            let terminal_logger = Box::new(builder.build());
            
            // Combine loggers
            let mux_logger = MuxLogger::new()
                .logger(syslog_logger)
                .logger(terminal_logger);
            
            // Set the combined logger
            log::set_boxed_logger(Box::new(mux_logger))
                .map_err(|e| Error::UnexpectedError(format!("Failed to set logger: {}", e)))?;
            log::set_max_level(level);
            
            Ok(())
        }
        
        #[cfg(not(feature = "log-mux"))]
        {
            // If log-mux feature is not enabled, fall back to just syslog
            init_syslog(level, facility, identifier)?;
            
            // And print a warning
            eprintln!("Warning: Dual output logging requires the 'log-mux' feature. Falling back to syslog only.");
            
            Ok(())
        }
    }
}

```rust
/// Main logging module providing functions for different log levels
pub(crate) mod logger {
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
    pub(crate) fn init(config: LogConfig) {
        let _ = LOG_CONFIG.set(config);
    }
    
    /// Get the current logger configuration
    ///
    /// # Returns
    ///
    /// The current logging configuration
    pub(crate) fn config() -> &'static LogConfig {
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
    pub(crate) fn trace_log<D: Debug>(
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
    pub(crate) fn debug_log<D: Debug>(
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
    pub(crate) fn info_log<D: Debug>(
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
    pub(crate) fn warn_log<D: Debug>(
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
    pub(crate) fn error_log(
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
    pub(crate) fn parser_event(message: &str, args: &[String]) {
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
    pub(crate) fn tokenizer_event(message: &str, input: &str, tokens: Option<&[String]>) {
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
    pub(crate) fn validation_event<D: Debug>(message: &str, args: Option<D>) {
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
    pub(crate) fn error_event(
        component: LogComponent,
        operation: LogOperation,
        message: &str,
        error: &Error,
    ) {
        error_log(component, operation, message, Some(error));
    }
}

/// Logging macros to make usage more ergonomic
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
```

### Implementation Approach

#### 1. Component-based Logging Structure

The logging system uses a component-based approach that categorizes log messages by their source component and operation:

```rust
#[derive(Debug, Clone, Copy)]
pub(crate) enum LogComponent {
    General,
    Parser,
    Tokenizer,
    KeyValueStore,
    Validator,
    FieldBinding,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum LogOperation {
    Init,
    Config,
    Parse,
    Tokenize,
    Validate,
    Error,
    Operation,
}
```

This approach:
- Provides clear organization of log messages
- Enables filtering by component or operation
- Establishes a consistent structure across the library
- Makes logs more searchable and categorizable

#### 1.1 Flexible Logging Destinations

The library supports multiple logging destinations through the `LogDestination` enum:

```rust
#[derive(Debug, Clone, Copy)]
pub enum LogDestination {
    Syslog,      // Standard for PAM modules
    Terminal,    // Useful during development
    Both,        // Helpful for debugging PAM modules
    None,        // Use existing logger or none
}
```

This provides:
- Compatibility with PAM system expectations (syslog)
- Developer-friendly options for debugging
- Flexibility to adapt to different usage scenarios
- Zero-cost abstraction when logging is disabled

#### 2. Integration with `log` Crate

The logging module integrates with Rust's standard `log` crate:

```rust
use log::{debug, error, info, trace, warn};

pub(crate) fn debug_log<D: Debug>(
    component: LogComponent,
    operation: LogOperation,
    message: &str,
    args: Option<D>,
) {
    if !log::log_enabled!(log::Level::Debug) {
        return;
    }
    
    // Format and emit log
    debug!("{}{} {}", comp_str, op_str, message);
}
```

Benefits of this approach:
- Leverages the established Rust logging ecosystem
- Allows users to plug in their preferred logging implementation
- Enables filtering by log level
- Provides zero-cost abstractions when disabled

#### 3. Structured Logging Support

The logging system supports both traditional and structured logging formats:

```rust
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
```

This design:
- Supports machine-parsable logs with JSON format
- Allows for human-readable logs with configurable formatting
- Includes consistent fields across all log messages
- Enables integration with log analysis tools

#### 4. Ergonomic Macros

The module provides macros for more ergonomic logging:

```rust
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
```

Benefits of macros:
- More concise logging calls
- Early checking of log level to avoid unnecessary work
- Type-safe logging with compile-time checking
- Consistent logging patterns across the codebase

#### 5. Specialized Logging Functions

The module includes specialized functions for common logging scenarios:

```rust
pub(crate) fn parser_event(message: &str, args: &[String]) {
    debug_log(
        LogComponent::Parser,
        LogOperation::Parse,
        message,
        Some(args),
    );
}

pub(crate) fn error_event(
    component: LogComponent,
    operation: LogOperation,
    message: &str,
    error: &Error,
) {
    error_log(component, operation, message, Some(error));
}
```

These functions:
- Reduce boilerplate for common logging patterns
- Enforce consistency in log format and content
- Provide semantic clarity about the logging purpose
- Encapsulate logging implementation details

### Logging Configuration

The logging system is configurable through a global configuration:

```rust
#[derive(Debug, Clone)]
pub(crate) struct LogConfig {
    pub include_timestamps: bool,
    pub include_component: bool,
    pub include_operation: bool,
    pub json_format: bool,
}

static LOG_CONFIG: std::sync::OnceLock<LogConfig> = std::sync::OnceLock::new();

pub(crate) fn init(config: LogConfig) {
    let _ = LOG_CONFIG.set(config);
}

pub(crate) fn config() -> &'static LogConfig {
    LOG_CONFIG.get_or_init(LogConfig::default)
}
```

This approach:
- Uses `OnceLock` for thread-safe lazy initialization
- Provides global access to logging configuration
- Allows for runtime configuration of logging format
- Has sensible defaults for common use cases

### Logging Destination Implementation

The implementation uses conditional compilation to support different logging backends:

```rust
#[cfg(feature = "log-mux")]
fn init_both(level: LevelFilter, facility: syslog::Facility, identifier: String) -> Result<()> {
    use log_mux::MuxLogger;
    
    // Set up syslog logger
    let formatter = syslog::Formatter3164::new(identifier);
    let syslog_conn = syslog::unix(formatter)?;
    let syslog_logger = Box::new(syslog::BasicLogger::new(syslog_conn));
    
    // Set up terminal logger
    let mut builder = env_logger::Builder::new();
    builder.filter_level(level);
    builder.format_timestamp_secs();
    let terminal_logger = Box::new(builder.build());
    
    // Combine loggers
    let mux_logger = MuxLogger::new()
        .logger(syslog_logger)
        .logger(terminal_logger);
    
    // Set the combined logger
    log::set_boxed_logger(Box::new(mux_logger))?;
    log::set_max_level(level);
    
    Ok(())
}
```

This design:
- Leverages feature flags for optional dependencies
- Provides graceful fallbacks when features aren't available
- Integrates with common Rust logging implementations
- Makes PAM-specific logging configuration simple

## Integration

### Integration with Other Components

The logging module integrates with other components as follows:

1. **Tokenizer**: Uses logging to trace token processing and error conditions
2. **Parser**: Logs argument parsing events and decisions
3. **Validation**: Records validation checks and constraint enforcement
4. **Error Handling**: Integrates with error system to log errors with context
5. **Public API**: Provides diagnostic information without exposing implementation details

### Usage Examples

```rust
use crate::logging::{LogComponent, LogOperation};
use crate::error::Error;

// Direct function usage
fn process_arguments(args: &[String]) -> Result<(), Error> {
    // Log at the beginning of processing
    crate::logging::logger::parser_event("Processing arguments", args);
    
    // Processing logic...
    let arg = args.get(0).ok_or_else(|| Error::InvalidInput("No arguments provided".to_string()))?;
    
    // Log successful completion
    crate::logging::logger::debug_log(
        LogComponent::Parser,
        LogOperation::Parse,
        "Successfully processed arguments",
        Some(args.len()),
    );
    
    Ok(())
}

// Macro usage
fn tokenize_input(input: &str) -> Result<Vec<String>, Error> {
    // Log at trace level
    log_trace!(LogComponent::Tokenizer, LogOperation::Tokenize, 
        "Starting tokenization", input);
    
    // Tokenization logic...
    let tokens = vec![input.to_string()]; // Simplified for example
    
    // Log the result
    log_tokenizer!("Tokenization complete", input, &tokens);
    
    Ok(tokens)
}

// Error logging
fn validate_argument(arg: &str) -> Result<(), Error> {
    if arg.is_empty() {
        let error = Error::InvalidInput("Empty argument".to_string());
        log_error_ctx!(
            LogComponent::Validator,
            LogOperation::Validate,
            "Argument validation failed",
            &error
        );
        return Err(error);
    }
    
    // Log successful validation
    log_validation!("Argument validated successfully", arg);
    
    Ok(())
}
```

## Testing Strategy

### Unit Test Cases

| # | Category | Input | Expected Output | Notes |
|---|----------|-------|-----------------|-------|
| 1 | Logger Init | `logger::init(LogConfig::default())` | Successfully initialized | Test basic initialization |
| 2 | Log Config | `logger::config()` | Default config values | Test configuration access |
| 3 | Log Levels | Call each level function with log disabled | No output | Test early return on disabled level |
| 4 | Log Level Enabled | Call trace_log with trace enabled | Correctly formatted output | Test basic logging |
| 5 | Component Formatting | Log with component enabled | Output contains component tag | Test component inclusion |
| 6 | Operation Formatting | Log with operation enabled | Output contains operation tag | Test operation inclusion |
| 7 | Custom Formatting | Log with custom format settings | Output matches expected format | Test format customization |
| 8 | JSON Format | Log with JSON format enabled | Valid JSON output | Test structured logging |
| 9 | Debug Data | Log with complex debug data | Output includes formatted data | Test data serialization |
| 10 | Error Logging | Log with Error object | Output includes error details | Test error formatting |
| 11 | Macro Usage | Use each macro variant | Correct log output | Test macro correctness |
| 12 | Specialized Logging | Use each specialized function | Correct component/operation | Test specialized functions |
| 13 | Thread Safety | Log from multiple threads | All logs captured without data races | Test thread safety |
| 14 | Performance | Log in tight loop | Minimal CPU impact | Test logging performance |
| 15 | Log Ordering | Log sequence of events | Correct event order | Test log sequencing |
| 16 | Syslog Init | `init::for_pam("test", LevelFilter::Info)` | Successful syslog connection | Test syslog logging |
| 17 | Terminal Init | `init::for_development(LevelFilter::Debug)` | Successful terminal logger | Test terminal logging |
| 18 | Dual Output | `init::dual_output("test", LevelFilter::Debug)` | Both loggers initialized | Test combined logging |
| 19 | Custom Options | `init::with_options(&custom_options)` | Logger configured per options | Test custom options |
| 20 | Feature Control | Enable/disable the log-mux feature | Appropriate fallback behavior | Test feature flags |



The logging system should be tested in integration with other components to ensure correct end-to-end behavior. Integration tests should focus on:

1. **Tokenizer Integration**
   - Test logging during tokenization process
   - Verify appropriate log levels for different tokenizer events
   - Test error logging for tokenizer errors

2. **Parser Integration**
   - Test logging during argument parsing
   - Verify appropriate log levels for different parser events
   - Test integration with validation logging

3. **Error Handling Integration**
   - Test how errors propagate through the logging system
   - Verify error context is properly included in logs
   - Test error logging at different stages of processing

### Testing Focus Areas

1. **Log Level Filtering**
   - Test that logs are only emitted at appropriate levels
   - Verify that disabled levels have no performance impact
   - Test filtering by component and operation

2. **Format Correctness**
   - Verify log messages have consistent format
   - Test JSON format validity
   - Verify component and operation tags are correctly included

3. **Thread Safety**
   - Test logging from multiple threads
   - Verify no data races or corruption
   - Test with high concurrency

4. **Performance Impact**
   - Measure performance with and without logging
   - Test impact of different log levels
   - Verify minimal overhead in hot paths

5. **Destination Correctness**
   - Verify logs go to the correct destination(s)
   - Test destination-specific formatting (syslog vs terminal)
   - Ensure consistent formatting across destinations

6. **PAM-Specific Requirements**
   - Test AUTH facility usage for PAM modules
   - Verify compatibility with system syslog expectations
   - Test with realistic PAM module scenarios

7. **Initialization Robustness**
   - Test initialization in different environments
   - Verify graceful handling of initialization failures
   - Test multiple initialization attempts

## Performance Considerations

### Memory Efficiency
- Early check of log level to avoid unnecessary string formatting
- Use of macros to prevent evaluation of arguments when logging is disabled
- Minimized heap allocations in logging paths
- Reuse of static strings for component and operation names
- Efficient handling of logger initialization using OnceLock

### Time Complexity
- Configuration lookup is O(1) using OnceLock
- Log level check is O(1)
- String formatting is O(n) in message size but only occurs when necessary
- Initialization is O(1) and happens only once per process

### Optimizations
- Conditional compilation with feature flags
- Zero-cost when logging is disabled at compile time
- Use of macros for compile-time optimization
- Static dispatch for all logging functions
- Lazy formatting to defer expensive operations

### Destination-Specific Considerations

#### Syslog Performance
- Syslog connections are established only once at initialization
- Syslog format is optimized for standard PAM module expectations
- Rate-limiting is handled by the system syslog daemon
- Uses native syslog implementations for specific platforms

#### Terminal Performance
- Uses env_logger's optimized terminal formatting
- Minimizes standard error flushes for better performance
- Provides efficient timestamp formatting options
- Avoids terminal-specific formatting when not needed

#### Dual Output Performance
- Optional through feature flag to avoid dependencies when not needed
- Uses efficient multiplexing to avoid duplicating format operations
- Implements early returns for performance-critical paths
- Provides fallback options for when optimal configuration isn't available

## Documentation

### Internal Developer Documentation
```rust
//! # Logging System for pam_args-rs
//!
//! This module provides a structured logging system that integrates with
//! the standard Rust `log` crate. It offers consistent logging patterns,
//! appropriate log levels, and contextual information across all library
//! components.
//!
//! ## Usage Examples
//!
//! ### Initializing Logging
//!
//! For a PAM module:
//!
//! ```rust
//! use pam_args_rs::logging::init;
//! use log::LevelFilter;
//!
//! // In your PAM module initialization
//! fn pam_sm_authenticate(pamh: &PamHandle, args: Vec<String>, flags: u32) -> PamResultCode {
//!     // Initialize logging with the PAM module name
//!     if let Err(e) = init::for_pam("pam_mymodule", LevelFilter::Info) {
//!         // Fall back to stderr if logging initialization fails
//!         eprintln!("Warning: Failed to initialize logging: {}", e);
//!     }
//!
//!     // Rest of your PAM module...
//! }
//! ```
//!
//! For development:
//!
//! ```rust
//! use pam_args_rs::logging::init;
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
//! ### Using the Logging Macros
//!
//! ```rust
//! use pam_args_rs::{log_debug, log_error, log_info};
//! use pam_args_rs::logging::{LogComponent, LogOperation};
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
//! ```
//!
//! ### Specialized Logging Functions
//!
//! ```rust
//! use pam_args_rs::logging::logger;
//!
//! // Log tokenization events
//! let input = "[DEBUG,USER=admin]";
//! let tokens = vec!["DEBUG".to_string(), "USER=admin".to_string()];
//! logger::tokenizer_event("Tokenized bracketed content", input, Some(&tokens));
//!
//! // Log validation events
//! logger::validation_event("Required argument present", Some("USER"));
//!
//! // Log parser events
//! let args = vec!["DEBUG".to_string(), "USER=admin".to_string()];
//! logger::parser_event("Processing command-line arguments", &args);
//! ```
//!
//! ## Configuring Log Format
//!
//! ```rust
//! use pam_args_rs::logging::{init, LogOptions, LogDestination, LogConfig};
//! use log::LevelFilter;
//!
//! // Create custom logging options with JSON formatting
//! let options = LogOptions {
//!     destination: LogDestination::Terminal,
//!     level: LevelFilter::Debug,
//!     config: LogConfig {
//!         include_timestamps: true,
//!         include_component: true,
//!         include_operation: true,
//!         json_format: true,
//!     },
//!     ..Default::default()
//! };
//!
//! // Initialize with custom options
//! init::with_options(&options).expect("Failed to initialize logging");
//! ```
//!
//! ## Notes on Performance
//!
//! - Log macros check log level first to avoid unnecessary work
//! - Disabled log levels have zero cost
//! - Use appropriate log levels to minimize performance impact in production
//! - Consider using the `json_format` option for machine-parsable logs in production
//!
//!