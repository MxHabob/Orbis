use thiserror::Error;

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
