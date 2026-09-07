use std::time::Duration;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};
use orbis_domain::errors::DomainError;

const HTTP_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SolveMode {
    #[default]
    Full,
    Incremental,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimetableProblem {
    pub job_id: String,
    pub org_id: String,
    pub period_id: String,
    pub mode: SolveMode,
    pub time_slots: Vec<TimeSlotDto>,
    pub rooms: Vec<RoomDto>,
    pub assignments: Vec<LessonAssignmentDto>,
    pub pinned_lessons: Vec<PinnedLessonDto>,
    #[serde(default)]
    pub unavailable_slots: Vec<TeacherUnavailableDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeacherUnavailableDto {
    pub teacher_id: String,
    pub day_of_week: i32,
    pub time_slot_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlotDto {
    pub id: String,
    pub slot_number: i32,
    pub start_time: String,
    pub end_time: String,
    pub is_break: bool,
    pub applicable_days: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomDto {
    pub id: String,
    pub name: String,
    pub capacity: Option<i32>,
    pub room_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonAssignmentDto {
    pub id: String,
    pub class_subject_id: String,
    pub class_id: String,
    pub subject_id: String,
    pub teacher_id: String,
    pub hour_index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_slot_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinnedLessonDto {
    pub id: String,
    pub class_id: String,
    pub teacher_id: String,
    pub subject_id: String,
    pub class_subject_id: String,
    pub time_slot_id: String,
    pub day_of_week: i32,
    pub room_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimetableSolution {
    pub job_id: String,
    pub assignments: Vec<LessonAssignmentDto>,
    pub score: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolveJobStatus {
    pub job_id: String,
    pub status: JobStatus,
    pub score: Option<String>,
    pub best_solution: Option<TimetableSolution>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitResponse {
    pub job_id: String,
    pub accepted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamTimeSlotDto {
    pub id: String,
    pub date: String,
    pub period: String, // "morning" or "afternoon"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamAssignmentDto {
    pub id: String,
    pub exam_subject_id: String,
    pub subject_id: String,
    pub class_id: String,
    pub conflict_group_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamProblem {
    pub job_id: String,
    pub org_id: String,
    pub exam_session_id: String,
    pub slots: Vec<ExamTimeSlotDto>,
    pub assignments: Vec<ExamAssignmentDto>,
    pub max_exams_per_day: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamSolution {
    pub job_id: String,
    pub assignments: Vec<ExamAssignmentDto>,
    pub score: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamSolveJobStatus {
    pub job_id: String,
    pub status: JobStatus,
    pub score: Option<String>,
    pub best_solution: Option<ExamSolution>,
    pub error: Option<String>,
}

pub struct RemoteTimefoldSolver {
    base_url: String,
    client: reqwest::Client,
}

impl RemoteTimefoldSolver {
    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(HTTP_TIMEOUT)
            .build()
            .unwrap_or_default();

        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client,
        }
    }

    pub async fn submit(&self, problem: &TimetableProblem) -> Result<(), DomainError> {
        let url = format!("{}/solve", self.base_url);
        debug!(job_id = %problem.job_id, "submitting timetable problem to Timefold solver sidecar");

        let resp = self
            .client
            .post(&url)
            .json(problem)
            .send()
            .await
            .map_err(|e| DomainError::Internal(format!("Solver sidecar request failed: {e}")))?;

        if resp.status() == StatusCode::ACCEPTED || resp.status() == StatusCode::OK {
            let body: SubmitResponse = resp
                .json()
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to parse solver response: {e}")))?;
            if body.accepted {
                return Ok(());
            }
            return Err(DomainError::Internal("Timefold solver rejected problem submission".into()));
        }

        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        warn!(%status, %body, "solver submit failed");
        Err(DomainError::Internal(format!("Solver error {status}: {body}")))
    }

    pub async fn status(&self, job_id: &str) -> Result<SolveJobStatus, DomainError> {
        let url = format!("{}/solve/{job_id}", self.base_url);

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| DomainError::Internal(format!("Solver sidecar status request failed: {e}")))?;

        if resp.status() == StatusCode::NOT_FOUND {
            return Err(DomainError::NotFound(format!("Solve job {job_id} not found")));
        }

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(DomainError::Internal(format!("Solver sidecar error {status}: {body}")));
        }

        resp.json()
            .await
            .map_err(|e| DomainError::Internal(format!("Failed to parse solver status: {e}")))
    }

    pub async fn cancel(&self, job_id: &str) -> Result<(), DomainError> {
        let url = format!("{}/solve/{job_id}", self.base_url);

        let resp = self
            .client
            .delete(&url)
            .send()
            .await
            .map_err(|e| DomainError::Internal(format!("Solver cancel request failed: {e}")))?;

        if resp.status().is_success() || resp.status() == StatusCode::NOT_FOUND {
            Ok(())
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            Err(DomainError::Internal(format!("Solver cancel error {status}: {body}")))
        }
    }

    pub async fn submit_exam(&self, problem: &ExamProblem) -> Result<(), DomainError> {
        let url = format!("{}/solve/exam", self.base_url);
        debug!(job_id = %problem.job_id, "submitting exam problem to Timefold solver sidecar");

        let resp = self
            .client
            .post(&url)
            .json(problem)
            .send()
            .await
            .map_err(|e| DomainError::Internal(format!("Exam solver request failed: {e}")))?;

        if resp.status() == StatusCode::ACCEPTED || resp.status() == StatusCode::OK {
            let body: SubmitResponse = resp
                .json()
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to parse exam solver response: {e}")))?;
            if body.accepted {
                return Ok(());
            }
            return Err(DomainError::Internal("Exam solver rejected problem submission".into()));
        }

        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        warn!(%status, %body, "exam solver submit failed");
        Err(DomainError::Internal(format!("Exam solver error {status}: {body}")))
    }

    pub async fn status_exam(&self, job_id: &str) -> Result<ExamSolveJobStatus, DomainError> {
        let url = format!("{}/solve/exam/{job_id}", self.base_url);

        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| DomainError::Internal(format!("Exam solver status request failed: {e}")))?;

        if resp.status() == StatusCode::NOT_FOUND {
            return Err(DomainError::NotFound(format!("Exam solve job {job_id} not found")));
        }

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(DomainError::Internal(format!("Exam solver error {status}: {body}")));
        }

        resp.json()
            .await
            .map_err(|e| DomainError::Internal(format!("Failed to parse exam solver status: {e}")))
    }

    pub async fn cancel_exam(&self, job_id: &str) -> Result<(), DomainError> {
        let url = format!("{}/solve/exam/{job_id}", self.base_url);

        let resp = self
            .client
            .delete(&url)
            .send()
            .await
            .map_err(|e| DomainError::Internal(format!("Exam solver cancel request failed: {e}")))?;

        if resp.status().is_success() || resp.status() == StatusCode::NOT_FOUND {
            Ok(())
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            Err(DomainError::Internal(format!("Exam solver cancel error {status}: {body}")))
        }
    }
}
