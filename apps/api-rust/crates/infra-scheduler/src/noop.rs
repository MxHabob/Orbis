use async_trait::async_trait;

use crate::{ScheduleSolver, SchedulerError, TimetableProblem, SolveJobStatus, ExamProblem, ExamSolveJobStatus};

pub struct NoOpScheduleSolver;

#[async_trait]
impl ScheduleSolver for NoOpScheduleSolver {
    async fn submit(&self, _problem: &TimetableProblem) -> Result<(), SchedulerError> {
        Err(SchedulerError::NotConfigured)
    }

    async fn status(&self, _job_id: &str) -> Result<SolveJobStatus, SchedulerError> {
        Err(SchedulerError::NotConfigured)
    }

    async fn cancel(&self, _job_id: &str) -> Result<(), SchedulerError> {
        Err(SchedulerError::NotConfigured)
    }

    async fn submit_exam(&self, _problem: &ExamProblem) -> Result<(), SchedulerError> {
        Err(SchedulerError::NotConfigured)
    }

    async fn status_exam(&self, _job_id: &str) -> Result<ExamSolveJobStatus, SchedulerError> {
        Err(SchedulerError::NotConfigured)
    }

    async fn cancel_exam(&self, _job_id: &str) -> Result<(), SchedulerError> {
        Err(SchedulerError::NotConfigured)
    }
}
