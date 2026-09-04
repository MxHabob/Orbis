use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchedulerError {
    #[error("solver service not configured")]
    NotConfigured,
    #[error("solver request failed: {0}")]
    RequestFailed(String),
    #[error("solver returned error: {0}")]
    SolverError(String),
    #[error("job not found: {0}")]
    JobNotFound(String),
}
