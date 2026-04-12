//! Comprehensive error handling types
//!
//! Provides unified error types for all layers of the application with:
//! - Detailed error messages with context
//! - Error categorization and recovery suggestions
//! - Integration with thiserror for clean error handling
//! - Support for error chaining and source tracing

use std::path::PathBuf;
use thiserror::Error;

/// Main application error type
#[derive(Debug, Error)]
pub enum AnkiTuiError {
    /// Configuration-related errors
    #[error("Configuration error: {message}")]
    Config {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        suggestion: Option<String>,
    },

    /// Data layer errors (TOML/SQLite operations)
    #[error("Data error: {message}")]
    Data {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        file_path: Option<PathBuf>,
        suggestion: Option<String>,
    },

    /// Database operation errors
    #[error("Database error: {message}")]
    Database {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        operation: Option<String>,
        suggestion: Option<String>,
    },

    /// Core logic errors (scheduler, session, etc.)
    #[error("Core error: {message}")]
    Core {
        message: String,
        component: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        suggestion: Option<String>,
    },

    /// TUI/Interface errors
    #[error("Interface error: {message}")]
    Interface {
        message: String,
        component: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        suggestion: Option<String>,
    },

    /// Input/Validation errors
    #[error("Validation error: {message}")]
    Validation {
        message: String,
        field: Option<String>,
        value: Option<String>,
        suggestion: Option<String>,
    },

    /// Filesystem/IO errors
    #[error("Filesystem error: {message}")]
    Filesystem {
        message: String,
        operation: String,
        path: Option<PathBuf>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        suggestion: Option<String>,
    },

    /// Network/External service errors
    #[error("External service error: {message}")]
    External {
        message: String,
        service: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        suggestion: Option<String>,
    },

    /// User input errors
    #[error("Input error: {message}")]
    Input {
        message: String,
        input_type: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        suggestion: Option<String>,
    },

    /// Permission errors
    #[error("Permission error: {message}")]
    Permission {
        message: String,
        resource: String,
        required_permission: String,
        suggestion: Option<String>,
    },

    /// Resource not found errors
    #[error("Not found: {message}")]
    NotFound {
        message: String,
        resource_type: String,
        resource_id: Option<String>,
        suggestion: Option<String>,
    },

    /// Timeout/Performance errors
    #[error("Timeout error: {message}")]
    Timeout {
        message: String,
        operation: String,
        timeout_ms: u64,
        suggestion: Option<String>,
    },

    /// Concurrency/Locking errors
    #[error("Concurrency error: {message}")]
    Concurrency {
        message: String,
        resource: String,
        operation: String,
        suggestion: Option<String>,
    },

    /// Serialization/Deserialization errors
    #[error("Serialization error: {message}")]
    Serialization {
        message: String,
        format: String,
        data_type: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        suggestion: Option<String>,
    },

    /// Generic/internal errors
    #[error("Internal error: {message}")]
    Internal {
        message: String,
        code: Option<String>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
        suggestion: Option<String>,
    },
}

impl AnkiTuiError {
    // Convenience constructors for common error types

    /// Create a configuration error
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config {
            message: message.into(),
            source: None,
            suggestion: None,
        }
    }

    /// Create a configuration error with suggestion
    pub fn config_with_suggestion<S: Into<String>, T: Into<String>>(message: S, suggestion: T) -> Self {
        Self::Config {
            message: message.into(),
            source: None,
            suggestion: Some(suggestion.into()),
        }
    }

    /// Create a data error
    pub fn data<S: Into<String>>(message: S) -> Self {
        Self::Data {
            message: message.into(),
            source: None,
            file_path: None,
            suggestion: None,
        }
    }

    /// Create a data error with file path
    pub fn data_with_path<S: Into<String>, P: Into<PathBuf>>(message: S, path: P) -> Self {
        Self::Data {
            message: message.into(),
            source: None,
            file_path: Some(path.into()),
            suggestion: None,
        }
    }

    /// Create a database error
    pub fn database<S: Into<String>>(message: S) -> Self {
        Self::Database {
            message: message.into(),
            source: None,
            operation: None,
            suggestion: None,
        }
    }

    /// Create a database error with operation
    pub fn database_with_operation<S: Into<String>, T: Into<String>>(message: S, operation: T) -> Self {
        Self::Database {
            message: message.into(),
            source: None,
            operation: Some(operation.into()),
            suggestion: None,
        }
    }

    /// Create a core logic error
    pub fn core<S: Into<String>, T: Into<String>>(message: S, component: T) -> Self {
        Self::Core {
            message: message.into(),
            component: component.into(),
            source: None,
            suggestion: None,
        }
    }

    /// Create an interface error
    pub fn interface<S: Into<String>, T: Into<String>>(message: S, component: T) -> Self {
        Self::Interface {
            message: message.into(),
            component: component.into(),
            source: None,
            suggestion: None,
        }
    }

    /// Create a validation error
    pub fn validation<S: Into<String>>(message: S) -> Self {
        Self::Validation {
            message: message.into(),
            field: None,
            value: None,
            suggestion: None,
        }
    }

    /// Create a validation error with field and value
    pub fn validation_with_field<S: Into<String>, T: Into<String>, U: Into<String>>(
        message: S,
        field: T,
        value: U,
    ) -> Self {
        Self::Validation {
            message: message.into(),
            field: Some(field.into()),
            value: Some(value.into()),
            suggestion: None,
        }
    }

    /// Create a filesystem error
    pub fn filesystem<S: Into<String>, T: Into<String>>(message: S, operation: T) -> Self {
        Self::Filesystem {
            message: message.into(),
            operation: operation.into(),
            path: None,
            source: None,
            suggestion: None,
        }
    }

    /// Create a not found error
    pub fn not_found<S: Into<String>, T: Into<String>>(message: S, resource_type: T) -> Self {
        Self::NotFound {
            message: message.into(),
            resource_type: resource_type.into(),
            resource_id: None,
            suggestion: None,
        }
    }

    /// Create a not found error with resource ID
    pub fn not_found_with_id<S: Into<String>, T: Into<String>, U: Into<String>>(
        message: S,
        resource_type: T,
        resource_id: U,
    ) -> Self {
        Self::NotFound {
            message: message.into(),
            resource_type: resource_type.into(),
            resource_id: Some(resource_id.into()),
            suggestion: None,
        }
    }

    /// Create a serialization error
    pub fn serialization<S: Into<String>, T: Into<String>, U: Into<String>>(
        message: S,
        format: T,
        data_type: U,
    ) -> Self {
        Self::Serialization {
            message: message.into(),
            format: format.into(),
            data_type: data_type.into(),
            source: None,
            suggestion: None,
        }
    }

    /// Create an internal error
    pub fn internal<S: Into<String>>(message: S) -> Self {
        Self::Internal {
            message: message.into(),
            code: None,
            source: None,
            suggestion: None,
        }
    }

    /// Create an internal error with error code
    pub fn internal_with_code<S: Into<String>, T: Into<String>>(message: S, code: T) -> Self {
        Self::Internal {
            message: message.into(),
            code: Some(code.into()),
            source: None,
            suggestion: None,
        }
    }

    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::Validation { .. }
            | Self::Config { .. }
            | Self::Input { .. }
            | Self::Timeout { .. }
            | Self::External { .. } => true,
            Self::Filesystem {
                suggestion: Some(s), ..
            } if s.contains("permission") => false,
            Self::Permission { .. } => false,
            _ => false,
        }
    }

    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Self::Validation { .. } | Self::Input { .. } => ErrorSeverity::Warning,
            Self::Config { .. }
            | Self::Data { .. }
            | Self::Interface { .. }
            | Self::Timeout { .. }
            | Self::Concurrency { .. } => ErrorSeverity::Error,
            Self::Database { .. }
            | Self::Core { .. }
            | Self::Filesystem { .. }
            | Self::Permission { .. }
            | Self::External { .. }
            | Self::Internal { .. } => ErrorSeverity::Critical,
            Self::Serialization { .. } | Self::NotFound { .. } => ErrorSeverity::Info,
        }
    }

    /// Get recovery suggestion
    pub fn suggestion(&self) -> Option<&str> {
        match self {
            Self::Config { suggestion, .. }
            | Self::Data { suggestion, .. }
            | Self::Database { suggestion, .. }
            | Self::Core { suggestion, .. }
            | Self::Interface { suggestion, .. }
            | Self::Validation { suggestion, .. }
            | Self::Filesystem { suggestion, .. }
            | Self::External { suggestion, .. }
            | Self::Input { suggestion, .. }
            | Self::Permission { suggestion, .. }
            | Self::NotFound { suggestion, .. }
            | Self::Timeout { suggestion, .. }
            | Self::Concurrency { suggestion, .. }
            | Self::Serialization { suggestion, .. }
            | Self::Internal { suggestion, .. } => suggestion.as_deref(),
        }
    }

    /// Get error category for logging/analytics
    pub fn category(&self) -> &'static str {
        match self {
            Self::Config { .. } => "config",
            Self::Data { .. } => "data",
            Self::Database { .. } => "database",
            Self::Core { .. } => "core",
            Self::Interface { .. } => "interface",
            Self::Validation { .. } => "validation",
            Self::Filesystem { .. } => "filesystem",
            Self::External { .. } => "external",
            Self::Input { .. } => "input",
            Self::Permission { .. } => "permission",
            Self::NotFound { .. } => "not_found",
            Self::Timeout { .. } => "timeout",
            Self::Concurrency { .. } => "concurrency",
            Self::Serialization { .. } => "serialization",
            Self::Internal { .. } => "internal",
        }
    }
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Error reporting and formatting utilities
pub mod formatting {
    use super::*;
    use std::fmt::Write;

    /// Format error with full context and suggestions
    pub fn format_error_full(error: &AnkiTuiError) -> String {
        let mut output = String::new();

        // Basic error message
        writeln!(output, "❌ {}", error).unwrap();

        // Error category and severity
        writeln!(
            output,
            "   Category: {} (Severity: {:?})",
            error.category(),
            error.severity()
        )
        .unwrap();

        // Add suggestion if available
        if let Some(suggestion) = error.suggestion() {
            writeln!(output, "💡 Suggestion: {}", suggestion).unwrap();
        }

        // Add recovery information
        if error.is_recoverable() {
            writeln!(output, "✅ This error is recoverable").unwrap();
        } else {
            writeln!(output, "⚠️  This error requires manual intervention").unwrap();
        }

        output
    }

    /// Format error for user display (less technical)
    pub fn format_error_user(error: &AnkiTuiError) -> String {
        let mut output = String::new();

        match error {
            AnkiTuiError::Config {
                message, suggestion, ..
            } => {
                writeln!(output, "Configuration problem: {}", message).unwrap();
                if let Some(s) = suggestion {
                    writeln!(output, "Try: {}", s).unwrap();
                }
            }
            AnkiTuiError::Data {
                message, suggestion, ..
            } => {
                writeln!(output, "Data problem: {}", message).unwrap();
                if let Some(s) = suggestion {
                    writeln!(output, "Try: {}", s).unwrap();
                }
            }
            AnkiTuiError::Validation {
                message, suggestion, ..
            } => {
                writeln!(output, "Invalid input: {}", message).unwrap();
                if let Some(s) = suggestion {
                    writeln!(output, "Hint: {}", s).unwrap();
                }
            }
            AnkiTuiError::NotFound {
                message, suggestion, ..
            } => {
                writeln!(output, "Not found: {}", message).unwrap();
                if let Some(s) = suggestion {
                    writeln!(output, "Check: {}", s).unwrap();
                }
            }
            _ => {
                writeln!(output, "Error: {}", error).unwrap();
                if let Some(s) = error.suggestion() {
                    writeln!(output, "Try: {}", s).unwrap();
                }
            }
        }

        output
    }

    /// Format error for logging (structured)
    pub fn format_error_log(error: &AnkiTuiError) -> String {
        format!(
            "error={{ category='{}', severity='{:?}', message='{}', recoverable={} }}",
            error.category(),
            error.severity(),
            error,
            error.is_recoverable()
        )
    }
}

/// Error recovery utilities
pub mod recovery {
    use super::*;

    /// Attempt to recover from common errors
    pub fn attempt_recovery(error: &AnkiTuiError) -> Result<RecoveryAction, String> {
        match error {
            AnkiTuiError::Config { message, .. } if message.contains("not found") => {
                Ok(RecoveryAction::CreateDefaultConfig)
            }
            AnkiTuiError::Data { message, file_path, .. } if message.contains("corrupted") => {
                if let Some(path) = file_path {
                    Ok(RecoveryAction::RestoreFromBackup(path.clone()))
                } else {
                    Ok(RecoveryAction::ManualIntervention(
                        "Data corruption detected. Please check your data files.".to_string(),
                    ))
                }
            }
            AnkiTuiError::Database { message, .. } if message.contains("locked") => Ok(RecoveryAction::Retry),
            AnkiTuiError::Permission { resource, .. } => Ok(RecoveryAction::RequestPermission(resource.clone())),
            AnkiTuiError::NotFound { resource_type, .. } => Ok(RecoveryAction::CreateResource(resource_type.clone())),
            _ => Ok(RecoveryAction::ManualIntervention(format!(
                "No automatic recovery available for: {}",
                error
            ))),
        }
    }

    /// Recovery actions that can be taken
    #[derive(Debug, Clone)]
    pub enum RecoveryAction {
        /// Retry the operation
        Retry,
        /// Create default configuration
        CreateDefaultConfig,
        /// Restore from backup
        RestoreFromBackup(PathBuf),
        /// Request additional permissions
        RequestPermission(String),
        /// Create missing resource
        CreateResource(String),
        /// Manual intervention required
        ManualIntervention(String),
    }
}

// Implement conversions from common error types
impl From<std::io::Error> for AnkiTuiError {
    fn from(err: std::io::Error) -> Self {
        let message = err.to_string();
        let operation = "io_operation".to_string();

        match err.kind() {
            std::io::ErrorKind::NotFound => Self::NotFound {
                message,
                resource_type: "file".to_string(),
                resource_id: None,
                suggestion: Some("Check if the file path is correct".to_string()),
            },
            std::io::ErrorKind::PermissionDenied => Self::Permission {
                message,
                resource: "file".to_string(),
                required_permission: "read/write".to_string(),
                suggestion: Some("Check file permissions".to_string()),
            },
            std::io::ErrorKind::AlreadyExists => Self::Filesystem {
                message,
                operation,
                path: None,
                source: Some(Box::new(err)),
                suggestion: Some("File already exists".to_string()),
            },
            _ => Self::Filesystem {
                message,
                operation,
                path: None,
                source: Some(Box::new(err)),
                suggestion: None,
            },
        }
    }
}

impl From<toml::de::Error> for AnkiTuiError {
    fn from(err: toml::de::Error) -> Self {
        Self::Serialization {
            message: format!("TOML parsing failed: {}", err),
            format: "TOML".to_string(),
            data_type: "configuration".to_string(),
            source: Some(Box::new(err)),
            suggestion: Some("Check TOML syntax and data types".to_string()),
        }
    }
}

impl From<toml::ser::Error> for AnkiTuiError {
    fn from(err: toml::ser::Error) -> Self {
        Self::Serialization {
            message: format!("TOML serialization failed: {}", err),
            format: "TOML".to_string(),
            data_type: "configuration".to_string(),
            source: Some(Box::new(err)),
            suggestion: Some("Check data types and structure".to_string()),
        }
    }
}

impl From<sqlx::Error> for AnkiTuiError {
    fn from(err: sqlx::Error) -> Self {
        let message = err.to_string();

        match err {
            sqlx::Error::Database(ref db_err) => Self::Database {
                message,
                source: Some(Box::new(err)),
                operation: Some("database_operation".to_string()),
                suggestion: Some("Check database connection and query syntax".to_string()),
            },
            sqlx::Error::RowNotFound => Self::NotFound {
                message,
                resource_type: "database_row".to_string(),
                resource_id: None,
                suggestion: Some("Check if the record exists".to_string()),
            },
            _ => Self::Database {
                message,
                source: Some(Box::new(err)),
                operation: None,
                suggestion: Some("Check database configuration and connectivity".to_string()),
            },
        }
    }
}

impl From<serde_json::Error> for AnkiTuiError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization {
            message: format!("JSON serialization failed: {}", err),
            format: "JSON".to_string(),
            data_type: "data".to_string(),
            source: Some(Box::new(err)),
            suggestion: Some("Check JSON syntax and data types".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = AnkiTuiError::config("Test config error");
        assert_eq!(err.category(), "config");
        assert_eq!(err.severity(), ErrorSeverity::Error);
    }

    #[test]
    fn test_error_with_suggestion() {
        let err = AnkiTuiError::config_with_suggestion("Test error", "Try fixing config");
        assert_eq!(err.suggestion(), Some("Try fixing config"));
    }

    #[test]
    fn test_error_formatting() {
        let err = AnkiTuiError::validation_with_field("Invalid value", "ease_factor", "-1.0");
        let formatted = formatting::format_error_full(&err);
        assert!(formatted.contains("Invalid value"));
        assert!(formatted.contains("validation"));
    }

    #[test]
    fn test_error_recovery() {
        let err = AnkiTuiError::config("Config file not found");
        let recovery = recovery::attempt_recovery(&err).unwrap();
        assert!(matches!(recovery, recovery::RecoveryAction::CreateDefaultConfig));
    }

    #[test]
    fn test_error_severity() {
        let validation_err = AnkiTuiError::validation("Invalid input");
        assert_eq!(validation_err.severity(), ErrorSeverity::Warning);

        let database_err = AnkiTuiError::database("Connection failed");
        assert_eq!(database_err.severity(), ErrorSeverity::Critical);
    }
}
