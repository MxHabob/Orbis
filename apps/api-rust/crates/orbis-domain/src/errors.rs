use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct PeriodOverrideTarget {
    pub override_type: &'static str,
    pub entity_type: &'static str,
    pub entity_id: Uuid,
    pub period_id: Uuid,
}

impl PeriodOverrideTarget {
    pub fn grade(period_id: Uuid, section_subject_id: Uuid) -> Self {
        Self {
            override_type: "grade_change",
            entity_type: "section_subject",
            entity_id: section_subject_id,
            period_id,
        }
    }

    pub fn enrollment(period_id: Uuid, section_id: Option<Uuid>) -> Self {
        let (entity_type, entity_id) =
            section_id.map_or(("academic_period", period_id), |id| ("section", id));
        Self {
            override_type: "late_enrollment",
            entity_type,
            entity_id,
            period_id,
        }
    }
}

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Validation failed: {0}")]
    Validation(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Business rule violation: {0}")]
    BusinessRule(String),
    #[error("The academic period is closed for this operation")]
    PeriodOverrideRequired(Vec<PeriodOverrideTarget>),
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Organization already initialized")]
    AlreadyInitialized,
    #[error("Bootstrap failed: {0}")]
    BootstrapFailed(String),
    #[error("Concurrency conflict: {0}")]
    ConcurrencyConflict(String),
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("License required: {0}")]
    LicenseRequired(String),
    #[error("The application key must be claimed before activation")]
    LicenseKeyRequiresClaim,
}

impl DomainError {
    pub fn period_override_targets(&self) -> Option<&[PeriodOverrideTarget]> {
        match self {
            Self::PeriodOverrideRequired(targets) => Some(targets),
            _ => None,
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum StorageError {
    #[error("Storage provider not configured: {0}")]
    NotConfigured(String),
    #[error("Storage connection error: {0}")]
    Connection(String),
    #[error("Object not found: {0}")]
    NotFound(String),
    #[error("Storage I/O error: {0}")]
    Io(String),
    #[error("Security or path traversal violation: {0}")]
    SecurityViolation(String),
    #[error("File size exceeded limit: {0} bytes (max: {1} bytes)")]
    QuotaExceeded(u64, u64),
    #[error("Invalid MIME or magic bytes: {0}")]
    InvalidMime(String),
    #[error("Operation not supported by provider: {0}")]
    Unsupported(String),
}

impl From<StorageError> for DomainError {
    fn from(err: StorageError) -> Self {
        DomainError::Storage(err.to_string())
    }
}
