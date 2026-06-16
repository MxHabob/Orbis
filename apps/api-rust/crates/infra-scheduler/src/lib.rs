mod client;
mod error;
mod noop;
pub mod types;

pub use client::RemoteTimefoldSolver;
pub use error::SchedulerError;
pub use noop::NoOpScheduleSolver;
pub use types::*;

use async_trait::async_trait;

/// Abstraction over the external Timefold solver service.
/// Keeps core-domain free of HTTP/Timefold specifics.
#[async_trait]
pub trait ScheduleSolver: Send + Sync {
    async fn submit(&self, problem: &TimetableProblem) -> Result<(), SchedulerError>;
    async fn status(&self, job_id: &str) -> Result<SolveJobStatus, SchedulerError>;
    async fn cancel(&self, job_id: &str) -> Result<(), SchedulerError>;
}
