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
}

