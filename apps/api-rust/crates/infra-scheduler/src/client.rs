use std::time::Duration;

use async_trait::async_trait;
use reqwest::StatusCode;
use tracing::{debug, warn};

use crate::{
    ScheduleSolver, SchedulerError, SolveJobStatus, SubmitResponse, TimetableProblem,
};

const HTTP_TIMEOUT: Duration = Duration::from_secs(30);

pub struct RemoteTimefoldSolver {
    base_url: String,
    client: reqwest::Client,
}

impl RemoteTimefoldSolver {
    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(HTTP_TIMEOUT)
            .build()
            .expect("failed to build HTTP client");

        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client,
        }
    }
}

#[async_trait]
impl ScheduleSolver for RemoteTimefoldSolver {
    async fn submit(&self, problem: &TimetableProblem) -> Result<(), SchedulerError> {
        let url = format!("{}/solve", self.base_url);
        debug!(job_id = %problem.job_id, "submitting timetable to solver");

        let resp = self
            .client
            .post(&url)
            .json(problem)
            .send()
            .await
            .map_err(|e| SchedulerError::RequestFailed(e.to_string()))?;

        if resp.status() == StatusCode::ACCEPTED || resp.status() == StatusCode::OK {
            let body: SubmitResponse = resp
                .json()
                .await
                .map_err(|e| SchedulerError::RequestFailed(e.to_string()))?;
            if body.accepted {
                return Ok(());
            }
            return Err(SchedulerError::SolverError("solver rejected job".into()));
        }

        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        warn!(%status, %body, "solver submit failed");
        Err(SchedulerError::SolverError(format!("{status}: {body}")))
    }

    async fn status(&self, job_id: &str) -> Result<SolveJobStatus, SchedulerError> {
        let url = format!("{}/solve/{job_id}", self.base_url);

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| SchedulerError::RequestFailed(e.to_string()))?;

        if resp.status() == StatusCode::NOT_FOUND {
            return Err(SchedulerError::JobNotFound(job_id.to_string()));
        }

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(SchedulerError::SolverError(format!("{status}: {body}")));
        }

        resp.json()
            .await
            .map_err(|e| SchedulerError::RequestFailed(e.to_string()))
    }

    async fn cancel(&self, job_id: &str) -> Result<(), SchedulerError> {
        let url = format!("{}/solve/{job_id}", self.base_url);

        let resp = self
            .client
            .delete(&url)
            .send()
            .await
            .map_err(|e| SchedulerError::RequestFailed(e.to_string()))?;

        if resp.status().is_success() || resp.status() == StatusCode::NOT_FOUND {
            Ok(())
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            Err(SchedulerError::SolverError(format!("{status}: {body}")))
        }
    }
}
