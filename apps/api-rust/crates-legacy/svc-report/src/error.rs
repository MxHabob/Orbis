//! Report error types.

use thiserror::Error;

/// Errors that can occur during report operations.
#[derive(Debug, Error)]
pub enum ReportError {
    /// The requested report code was not found in the registry.
    #[error("Report not found: {0}")]
    NotFound(String),

    /// A required parameter was missing from the request.
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    /// A parameter value did not match the expected type or constraints.
    #[error("Invalid parameter '{key}': {reason}")]
    InvalidParameter {
        key: String,
        reason: String,
    },

    /// The requesting user lacks a required permission.
    #[error("Insufficient permissions for report '{report_code}': requires {permission}")]
    PermissionDenied {
        report_code: String,
        permission: String,
    },

    /// The requested output format is not supported by this report.
    #[error("Unsupported output format '{format}' for report '{report_code}'")]
    UnsupportedFormat {
        report_code: String,
        format: String,
    },

    /// Error from the data provider (domain query failure).
    #[error("Data provider error: {0}")]
    DataProviderError(String),

    /// Database error.
    #[error("Database error: {0}")]
    DatabaseError(String),

    /// Generic internal error.
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<sea_orm::DbErr> for ReportError {
    fn from(err: sea_orm::DbErr) -> Self {
        ReportError::DatabaseError(err.to_string())
    }
}
