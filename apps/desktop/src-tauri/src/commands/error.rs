// â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
// Error helper: converts any Result<T, String> into
// a Tauri-serializable error so the frontend receives
// a typed error object instead of a raw string.
// â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandError {
    pub message: String,
    pub code: Option<String>,
    pub field: Option<String>,
}

impl CommandError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: None,
            field: None,
        }
    }

    pub fn with_code(message: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: Some(code.into()),
            field: None,
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: Some("NOT_FOUND".to_string()),
            field: None,
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: Some("UNAUTHORIZED".to_string()),
            field: None,
        }
    }
}

impl From<String> for CommandError {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for CommandError {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<sea_orm::DbErr> for CommandError {
    fn from(err: sea_orm::DbErr) -> Self {
        Self {
            message: err.to_string(),
            code: Some("DATABASE_ERROR".to_string()),
            field: None,
        }
    }
}

impl From<shared_kernel::errors::AppError> for CommandError {
    fn from(err: shared_kernel::errors::AppError) -> Self {
        use shared_kernel::errors::AppError;
        match err {
            AppError::NotFound { entity_type, id } => Self {
                message: format!("{} not found: {}", entity_type, id),
                code: Some("NOT_FOUND".to_string()),
                field: None,
            },
            AppError::Validation { message, field } => Self {
                message,
                code: Some("VALIDATION_ERROR".to_string()),
                field,
            },
            AppError::Unauthorized { reason } => Self {
                message: reason,
                code: Some("UNAUTHORIZED".to_string()),
                field: None,
            },
            AppError::Conflict { message } => Self {
                message,
                code: Some("CONFLICT".to_string()),
                field: None,
            },
            AppError::BusinessRule { rule, details } => Self {
                message: match details {
                    Some(d) => format!("{}: {}", rule, d),
                    None => rule,
                },
                code: Some("BUSINESS_RULE_VIOLATION".to_string()),
                field: None,
            },
            AppError::Database(msg) => Self {
                message: msg,
                code: Some("DATABASE_ERROR".to_string()),
                field: None,
            },
            AppError::Internal(msg) => Self {
                message: msg,
                code: Some("INTERNAL_ERROR".to_string()),
                field: None,
            },
            other => Self {
                message: other.to_string(),
                code: Some("UNKNOWN_ERROR".to_string()),
                field: None,
            },
        }
    }
}

/// Convenience type alias used by every Tauri command.
pub type CmdResult<T> = Result<T, CommandError>;

