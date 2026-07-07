//! Unified error type for the entire Orbis system.
//!
//! `AppError` replaces the scattered `Result<T, String>` pattern with structured,
//! machine-readable errors. Each variant carries context for debugging and for
//! AI agents to handle errors intelligently.
//!
//! # Tauri IPC Compatibility
//!
//! `AppError` implements `Into<String>` so existing Tauri commands that return
//! `Result<T, String>` continue to work during migration. New code should use
//! `Result<T, AppError>` directly.

use serde::Serialize;

/// Structured error type for the Orbis system.
///
/// Categorized by HTTP-like semantics for consistency:
/// - `NotFound` → 404
/// - `Validation` → 400
/// - `Unauthorized` → 403
/// - `Conflict` → 409
/// - `Internal` → 500
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // ── Client Errors ───────────────────────────────────────────

    /// The requested entity does not exist.
    #[error("{entity_type} not found: {id}")]
    NotFound {
        entity_type: &'static str,
        id: String,
    },

    /// Input validation failed.
    #[error("Validation failed: {message}")]
    Validation {
        message: String,
        field: Option<String>,
    },

    /// The caller does not have permission to perform this action.
    #[error("Access denied: {reason}")]
    Unauthorized {
        reason: String,
    },

    /// The operation conflicts with the current state (e.g., duplicate, locked).
    #[error("Conflict: {message}")]
    Conflict {
        message: String,
    },

    /// A business rule prevents this operation.
    #[error("Business rule violation: {rule}")]
    BusinessRule {
        rule: String,
        details: Option<String>,
    },

    // ── AI-Specific Errors ──────────────────────────────────────

    /// The requested action is not registered in the action catalog.
    #[error("Action not found: {code}")]
    ActionNotFound {
        code: String,
    },

    /// The action's risk level exceeds the agent session's maximum.
    #[error("Risk level exceeded: action requires {action_risk}, session allows {session_max}")]
    RiskExceeded {
        action_risk: String,
        session_max: String,
    },

    /// The action has already been executed (idempotency guard).
    #[error("Action already executed (idempotency key exists)")]
    AlreadyExecuted,

    /// The action requires human approval before execution.
    #[error("Approval required for action: {code}")]
    ApprovalRequired {
        code: String,
        approval_id: String,
    },

    // ── Infrastructure Errors ───────────────────────────────────

    /// Database error from SeaORM/SQLite.
    #[error("Database error: {0}")]
    Database(String),

    /// General internal error — use as last resort.
    #[error("Internal error: {0}")]
    Internal(String),
}

// ── Construction Helpers ────────────────────────────────────────

impl AppError {
    pub fn not_found(entity_type: &'static str, id: impl Into<String>) -> Self {
        Self::NotFound { entity_type, id: id.into() }
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation { message: message.into(), field: None }
    }

    pub fn validation_field(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation { message: message.into(), field: Some(field.into()) }
    }

    pub fn unauthorized(reason: impl Into<String>) -> Self {
        Self::Unauthorized { reason: reason.into() }
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict { message: message.into() }
    }

    pub fn business_rule(rule: impl Into<String>) -> Self {
        Self::BusinessRule { rule: rule.into(), details: None }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }

    /// Returns a machine-readable error code for categorization.
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::NotFound { .. } => "NOT_FOUND",
            Self::Validation { .. } => "VALIDATION_ERROR",
            Self::Unauthorized { .. } => "UNAUTHORIZED",
            Self::Conflict { .. } => "CONFLICT",
            Self::BusinessRule { .. } => "BUSINESS_RULE",
            Self::ActionNotFound { .. } => "ACTION_NOT_FOUND",
            Self::RiskExceeded { .. } => "RISK_EXCEEDED",
            Self::AlreadyExecuted => "ALREADY_EXECUTED",
            Self::ApprovalRequired { .. } => "APPROVAL_REQUIRED",
            Self::Database(_) => "DATABASE_ERROR",
            Self::Internal(_) => "INTERNAL_ERROR",
        }
    }
}

// ── SeaORM Integration ──────────────────────────────────────────

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        Self::Database(err.to_string())
    }
}

// ── Backward Compatibility ──────────────────────────────────────
// Existing Tauri commands use `Result<T, String>`. This allows gradual migration.

impl From<AppError> for String {
    fn from(err: AppError) -> String {
        err.to_string()
    }
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        Self::Internal(s)
    }
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        Self::Internal(s.to_string())
    }
}

// ── Serialization for API responses ─────────────────────────────

/// Serializable error response for API and Tauri IPC.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: &'static str,
    pub message: String,
    pub field: Option<String>,
}

impl From<&AppError> for ErrorResponse {
    fn from(err: &AppError) -> Self {
        Self {
            code: err.error_code(),
            message: err.to_string(),
            field: match err {
                AppError::Validation { field, .. } => field.clone(),
                _ => None,
            },
        }
    }
}
