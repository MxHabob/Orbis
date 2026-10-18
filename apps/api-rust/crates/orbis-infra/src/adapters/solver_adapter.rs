use orbis_domain::errors::DomainError;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, warn};

pub use orbis_domain::entities::solver::*;

const HTTP_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitResponse {
    pub job_id: String,
    pub accepted: bool,
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

    pub async fn check_health(&self) -> Result<(), DomainError> {
        let url = format!("{}/health", self.base_url);
        let response = self
            .client
            .get(url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
            .map_err(|error| {
                DomainError::Validation(format!("Solver connection check failed: {error}"))
            })?;
        if !response.status().is_success() {
            return Err(DomainError::Validation(format!(
                "Solver health check returned HTTP {}",
                response.status().as_u16()
            )));
        }
        let health: serde_json::Value = response.json().await.map_err(|error| {
            DomainError::Validation(format!("Solver health response was invalid: {error}"))
        })?;
        if health.get("status").and_then(serde_json::Value::as_str) != Some("healthy")
            || health.get("service").and_then(serde_json::Value::as_str) != Some("orbis-solver")
        {
            return Err(DomainError::Validation(
                "The endpoint did not identify itself as a healthy Orbis Solver service".into(),
            ));
        }
        Ok(())
    }

    pub async fn submit(&self, problem: &TimetableProblem) -> Result<(), DomainError> {
        let url = format!("{}/solve", self.base_url);
        debug!(job_id = %problem.job_id, "submitting timetable problem to Timefold solver service");

        let resp = self
            .client
            .post(&url)
            .json(problem)
            .send()
            .await
            .map_err(|e| DomainError::Internal(format!("Solver service request failed: {e}")))?;

        if resp.status() == StatusCode::ACCEPTED || resp.status() == StatusCode::OK {
            let body: SubmitResponse = resp.json().await.map_err(|e| {
                DomainError::Internal(format!("Failed to parse solver response: {e}"))
            })?;
            if body.accepted {
                return Ok(());
            }
            return Err(DomainError::Internal(
                "Timefold solver rejected problem submission".into(),
            ));
        }

        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        warn!(%status, %body, "solver submit failed");
        Err(DomainError::Internal(format!(
            "Solver error {status}: {body}"
        )))
    }

    pub async fn status(&self, job_id: &str) -> Result<SolveJobStatus, DomainError> {
        let url = format!("{}/solve/{job_id}", self.base_url);

        let resp = self.client.get(&url).send().await.map_err(|e| {
            DomainError::Internal(format!("Solver service status request failed: {e}"))
        })?;

        if resp.status() == StatusCode::NOT_FOUND {
            return Err(DomainError::NotFound(format!(
                "Solve job {job_id} not found"
            )));
        }

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(DomainError::Internal(format!(
                "Solver service error {status}: {body}"
            )));
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
            Err(DomainError::Internal(format!(
                "Solver cancel error {status}: {body}"
            )))
        }
    }

    pub async fn submit_exam(&self, problem: &ExamProblem) -> Result<(), DomainError> {
        let url = format!("{}/solve/exam", self.base_url);
        debug!(job_id = %problem.job_id, "submitting exam problem to Timefold solver service");

        let resp = self
            .client
            .post(&url)
            .json(problem)
            .send()
            .await
            .map_err(|e| DomainError::Internal(format!("Exam solver request failed: {e}")))?;

        if resp.status() == StatusCode::ACCEPTED || resp.status() == StatusCode::OK {
            let body: SubmitResponse = resp.json().await.map_err(|e| {
                DomainError::Internal(format!("Failed to parse exam solver response: {e}"))
            })?;
            if body.accepted {
                return Ok(());
            }
            return Err(DomainError::Internal(
                "Exam solver rejected problem submission".into(),
            ));
        }

        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        warn!(%status, %body, "exam solver submit failed");
        Err(DomainError::Internal(format!(
            "Exam solver error {status}: {body}"
        )))
    }

    pub async fn status_exam(&self, job_id: &str) -> Result<ExamSolveJobStatus, DomainError> {
        let url = format!("{}/solve/exam/{job_id}", self.base_url);

        let resp = self.client.get(&url).send().await.map_err(|e| {
            DomainError::Internal(format!("Exam solver status request failed: {e}"))
        })?;

        if resp.status() == StatusCode::NOT_FOUND {
            return Err(DomainError::NotFound(format!(
                "Exam solve job {job_id} not found"
            )));
        }

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(DomainError::Internal(format!(
                "Exam solver error {status}: {body}"
            )));
        }

        resp.json()
            .await
            .map_err(|e| DomainError::Internal(format!("Failed to parse exam solver status: {e}")))
    }

    pub async fn cancel_exam(&self, job_id: &str) -> Result<(), DomainError> {
        let url = format!("{}/solve/exam/{job_id}", self.base_url);

        let resp = self.client.delete(&url).send().await.map_err(|e| {
            DomainError::Internal(format!("Exam solver cancel request failed: {e}"))
        })?;

        if resp.status().is_success() || resp.status() == StatusCode::NOT_FOUND {
            Ok(())
        } else {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            Err(DomainError::Internal(format!(
                "Exam solver cancel error {status}: {body}"
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timetable_problem_serialization() {
        let problem = TimetableProblem {
            job_id: "test-job-1".to_string(),
            org_id: "org-1".to_string(),
            period_id: "period-1".to_string(),
            mode: SolveMode::Full,
            time_slots: vec![TimeSlotDto {
                id: "slot-1".to_string(),
                slot_number: 1,
                start_time: "08:00".to_string(),
                end_time: "08:45".to_string(),
                is_break: false,
                applicable_days: None,
            }],
            rooms: vec![RoomDto {
                id: "room-101".to_string(),
                name: "Room 101".to_string(),
                capacity: Some(30),
                room_type: "Classroom".to_string(),
            }],
            assignments: vec![LessonAssignmentDto {
                id: "lesson-1".to_string(),
                section_subject_id: "cs-1".to_string(),
                section_id: "class-1".to_string(),
                subject_id: "sub-math".to_string(),
                teacher_id: "teacher-1".to_string(),
                hour_index: 0,
                time_slot_id: None,
                day_of_week: None,
                room_id: None,
            }],
            pinned_lessons: vec![],
            unavailable_slots: vec![],
        };

        let json = serde_json::to_string(&problem).expect("Must serialize to JSON");
        assert!(json.contains("test-job-1"));
        assert!(json.contains("sub-math"));
        assert!(json.contains("\"cycle_id\":\"period-1\""));
        assert!(json.contains("\"class_subject_id\":\"cs-1\""));
        assert!(json.contains("\"class_id\":\"class-1\""));
        assert!(!json.contains("\"period_id\""));
        assert!(!json.contains("\"section_subject_id\""));
    }

    #[test]
    fn test_timetable_problem_accepts_desktop_ipc_field_names() {
        let problem: TimetableProblem = serde_json::from_value(serde_json::json!({
            "job_id": "test-job-2",
            "org_id": "org-1",
            "period_id": "period-1",
            "mode": "incremental",
            "time_slots": [],
            "rooms": [],
            "assignments": [{
                "id": "lesson-1",
                "section_subject_id": "cs-1",
                "section_id": "class-1",
                "subject_id": "sub-math",
                "teacher_id": "teacher-1",
                "hour_index": 0
            }],
            "pinned_lessons": [],
            "unavailable_slots": []
        }))
        .expect("Desktop IPC field names must deserialize");

        assert_eq!(problem.period_id, "period-1");
        assert_eq!(problem.assignments[0].section_subject_id, "cs-1");
        assert_eq!(problem.assignments[0].section_id, "class-1");
    }
}
