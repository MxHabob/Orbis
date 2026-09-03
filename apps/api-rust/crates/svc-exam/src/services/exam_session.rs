//! Exam session service — manages exam sessions, schedules, subjects, and hall definitions.

use std::sync::Arc;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sea_orm::EntityTrait;

use shared_kernel::errors::AppError;
use crate::entities::{exam_session, exam_subject, exam_schedule, exam_hall};
use crate::params::{ExamSessionParams, ExamScheduleParams, ExamHallParams};
use crate::repository::{ExamCommandRepo, ExamQueryRepo};

use svc_scheduling::entities::room;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateExamSessionRequest {
    pub period_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "type")]
    pub session_type: String,
    pub start_date: String,
    pub end_date: String,
    pub hijri_start_date: Option<String>,
    pub hijri_end_date: Option<String>,
    pub applicable_levels: Option<String>,
    pub applicable_class_ids: Option<String>,
    pub settings: Option<String>,
    pub grading_deadline: Option<String>,
    pub results_publish_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdateExamSessionRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "type")]
    pub session_type: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub hijri_start_date: Option<String>,
    pub hijri_end_date: Option<String>,
    pub applicable_levels: Option<String>,
    pub applicable_class_ids: Option<String>,
    pub settings: Option<String>,
    pub grading_deadline: Option<String>,
    pub results_publish_date: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateExamSubjectRequest {
    pub exam_session_id: String,
    pub subject_id: String,
    pub class_subject_id: Option<String>,
    pub max_score: f64,
    pub passing_score: f64,
    pub duration_minutes: Option<i32>,
    pub exam_type: Option<String>,
    pub weight: Option<f64>,
    pub instructions: Option<String>,
    pub instructions_ar: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateExamScheduleRequest {
    pub exam_session_id: String,
    pub exam_subject_id: String,
    pub exam_date: String,
    pub start_time: String,
    pub end_time: String,
    pub period: Option<String>,
    pub applicable_class_ids: Option<String>,
    pub room_ids: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateExamHallRequest {
    pub exam_session_id: String,
    pub room_id: String,
    pub hall_number: String,
    pub capacity: Option<i32>,
    pub seating_arrangement: Option<String>,
    pub is_accessible: Option<i32>,
    pub notes: Option<String>,
}

const VALID_TRANSITIONS: &[(&str, &str)] = &[
    ("draft", "scheduled"),
    ("scheduled", "active"),
    ("active", "grading"),
    ("grading", "completed"),
    ("completed", "archived"),
    ("scheduled", "draft"),
    ("active", "scheduled"),
];

pub struct ExamSessionService {
    pub cmd: Arc<ExamCommandRepo>,
    pub query: Arc<ExamQueryRepo>,
}

impl ExamSessionService {
    pub fn new(cmd: Arc<ExamCommandRepo>, query: Arc<ExamQueryRepo>) -> Self {
        Self { cmd, query }
    }

    pub async fn get_exam_sessions(
        &self,
        org_id: &str,
        period_id: Option<&str>,
    ) -> Result<Vec<exam_session::Model>, AppError> {
        let params = ExamSessionParams {
            period_id: period_id.map(String::from),
            status: None,
        };
        Ok(self.query.get_exam_sessions(org_id, params).await?)
    }

    pub async fn get_exam_session(
        &self,
        org_id: &str,
        id: &str,
    ) -> Result<exam_session::Model, AppError> {
        let session = self.query.get_exam_session_by_id(id).await?
            .ok_or_else(|| AppError::not_found("exam_session", id))?;

        if session.org_id != org_id {
            return Err(AppError::not_found("exam_session", id));
        }

        Ok(session)
    }

    pub async fn create_exam_session(
        &self,
        org_id: &str,
        user_id: &str,
        req: CreateExamSessionRequest,
    ) -> Result<exam_session::Model, AppError> {
        let valid_types = ["monthly", "midterm", "final", "second_chance", "custom"];
        if !valid_types.contains(&req.session_type.as_str()) {
            return Err(AppError::validation(format!(
                "invalid session type '{}'. Valid types: {:?}",
                req.session_type, valid_types
            )));
        }

        if req.start_date > req.end_date {
            return Err(AppError::validation("start_date must be before end_date"));
        }

        let now = Utc::now().to_rfc3339();
        let model = exam_session::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            period_id: req.period_id,
            name: req.name,
            name_ar: req.name_ar,
            code: req.code,
            session_type: req.session_type,
            status: "draft".to_string(),
            start_date: req.start_date,
            end_date: req.end_date,
            hijri_start_date: req.hijri_start_date,
            hijri_end_date: req.hijri_end_date,
            applicable_levels: req.applicable_levels,
            applicable_class_ids: req.applicable_class_ids,
            settings: req.settings,
            grading_deadline: req.grading_deadline,
            results_publish_date: req.results_publish_date,
            is_results_published: 0,
            created_by: user_id.to_string(),
            approved_by: None,
            approved_at: None,
            created_at: now.clone(),
            updated_at: now,
        };

        Ok(self.cmd.insert_exam_session(model).await?)
    }

    pub async fn update_exam_session(
        &self,
        org_id: &str,
        id: &str,
        req: UpdateExamSessionRequest,
    ) -> Result<exam_session::Model, AppError> {
        let mut session = self.get_exam_session(org_id, id).await?;

        if let Some(ref new_status) = req.status {
            let valid = VALID_TRANSITIONS
                .iter()
                .any(|(from, to)| *from == session.status && *to == new_status.as_str());
            if !valid {
                return Err(AppError::validation(format!(
                    "invalid status transition from '{}' to '{}'",
                    session.status, new_status
                )));
            }
            session.status = new_status.clone();
        }

        if let Some(v) = req.name { session.name = v; }
        if let Some(v) = req.name_ar { session.name_ar = Some(v); }
        if let Some(v) = req.code { session.code = Some(v); }
        if let Some(v) = req.session_type { session.session_type = v; }
        if let Some(v) = req.start_date { session.start_date = v; }
        if let Some(v) = req.end_date { session.end_date = v; }
        if let Some(v) = req.hijri_start_date { session.hijri_start_date = Some(v); }
        if let Some(v) = req.hijri_end_date { session.hijri_end_date = Some(v); }
        if let Some(v) = req.applicable_levels { session.applicable_levels = Some(v); }
        if let Some(v) = req.applicable_class_ids { session.applicable_class_ids = Some(v); }
        if let Some(v) = req.settings { session.settings = Some(v); }
        if let Some(v) = req.grading_deadline { session.grading_deadline = Some(v); }
        if let Some(v) = req.results_publish_date { session.results_publish_date = Some(v); }

        session.updated_at = Utc::now().to_rfc3339();

        Ok(self.cmd.update_exam_session(session).await?)
    }

    pub async fn activate_exam_session(
        &self,
        org_id: &str,
        id: &str,
        user_id: &str,
    ) -> Result<exam_session::Model, AppError> {
        let mut session = self.get_exam_session(org_id, id).await?;

        if session.status != "scheduled" {
            return Err(AppError::validation(format!(
                "cannot activate exam session with status '{}'. Must be 'scheduled'",
                session.status
            )));
        }

        let subjects = self.query.get_exam_subjects(id).await?;
        if subjects.is_empty() {
            return Err(AppError::validation("cannot activate exam session without any subjects"));
        }

        session.status = "active".to_string();
        session.approved_by = Some(user_id.to_string());
        session.approved_at = Some(Utc::now().to_rfc3339());
        session.updated_at = Utc::now().to_rfc3339();

        Ok(self.cmd.update_exam_session(session).await?)
    }

    pub async fn complete_exam_session(
        &self,
        org_id: &str,
        id: &str,
    ) -> Result<exam_session::Model, AppError> {
        let mut session = self.get_exam_session(org_id, id).await?;

        if session.status != "grading" {
            return Err(AppError::validation(format!(
                "cannot complete exam session with status '{}'. Must be 'grading'",
                session.status
            )));
        }

        session.status = "completed".to_string();
        session.updated_at = Utc::now().to_rfc3339();

        Ok(self.cmd.update_exam_session(session).await?)
    }

    // ── Exam Subjects ──

    pub async fn get_exam_subjects(
        &self,
        org_id: &str,
        exam_session_id: &str,
    ) -> Result<Vec<exam_subject::Model>, AppError> {
        let _ = self.get_exam_session(org_id, exam_session_id).await?;
        Ok(self.query.get_exam_subjects(exam_session_id).await?)
    }

    pub async fn create_exam_subject(
        &self,
        org_id: &str,
        req: CreateExamSubjectRequest,
    ) -> Result<exam_subject::Model, AppError> {
        let session = self.get_exam_session(org_id, &req.exam_session_id).await?;

        if session.status != "draft" && session.status != "scheduled" {
            return Err(AppError::validation("can only add subjects to draft or scheduled exam sessions"));
        }

        let now = Utc::now().to_rfc3339();
        let model = exam_subject::Model {
            id: Uuid::new_v4().to_string(),
            exam_session_id: req.exam_session_id,
            subject_id: req.subject_id,
            class_subject_id: req.class_subject_id,
            max_score: req.max_score,
            passing_score: req.passing_score,
            duration_minutes: req.duration_minutes.unwrap_or(90),
            exam_type: req.exam_type.unwrap_or_else(|| "written".to_string()),
            is_included: 1,
            weight: req.weight.unwrap_or(1.0),
            instructions: req.instructions,
            instructions_ar: req.instructions_ar,
            sort_order: req.sort_order,
            created_at: now.clone(),
            updated_at: now,
        };

        Ok(self.cmd.insert_exam_subject(model).await?)
    }

    // ── Exam Schedules ──

    pub async fn get_exam_schedules(
        &self,
        org_id: &str,
        exam_session_id: &str,
    ) -> Result<Vec<exam_schedule::Model>, AppError> {
        let _ = self.get_exam_session(org_id, exam_session_id).await?;
        let params = ExamScheduleParams { exam_session_id: Some(exam_session_id.to_string()), exam_date: None, status: None };
        Ok(self.query.get_exam_schedules(exam_session_id, params).await?)
    }

    pub async fn create_exam_schedule(
        &self,
        org_id: &str,
        req: CreateExamScheduleRequest,
    ) -> Result<exam_schedule::Model, AppError> {
        let session = self.get_exam_session(org_id, &req.exam_session_id).await?;

        if session.status == "completed" || session.status == "archived" {
            return Err(AppError::validation("cannot schedule exams in a completed or archived session"));
        }

        let subject = self.query.get_exam_subject_by_id(&req.exam_subject_id).await?
            .ok_or_else(|| AppError::not_found("exam_subject", &req.exam_subject_id))?;

        if subject.exam_session_id != req.exam_session_id {
            return Err(AppError::validation("exam_subject does not belong to this session"));
        }

        let params = ExamScheduleParams { exam_session_id: Some(req.exam_session_id.clone()), exam_date: None, status: None };
        let existing_schedules = self.query.get_exam_schedules(&req.exam_session_id, params).await?;

        for existing in &existing_schedules {
            if existing.exam_date == req.exam_date
                && existing.period == req.period.as_deref().unwrap_or("morning")
                && existing.status != "cancelled"
            {
                if req.start_time < existing.end_time && req.end_time > existing.start_time {
                    return Err(AppError::validation(format!(
                        "scheduling conflict: overlaps with existing exam on {} during {} period",
                        existing.exam_date,
                        existing.period
                    )));
                }
            }
        }

        let now = Utc::now().to_rfc3339();
        let model = exam_schedule::Model {
            id: Uuid::new_v4().to_string(),
            exam_session_id: req.exam_session_id,
            exam_subject_id: req.exam_subject_id,
            exam_date: req.exam_date,
            start_time: req.start_time,
            end_time: req.end_time,
            period: req.period.unwrap_or_else(|| "morning".to_string()),
            applicable_class_ids: req.applicable_class_ids,
            room_ids: req.room_ids,
            status: "scheduled".to_string(),
            postponed_to: None,
            postponement_reason: None,
            notes: req.notes,
            created_at: now.clone(),
            updated_at: now,
        };

        Ok(self.cmd.insert_exam_schedule(model).await?)
    }

    // ── Exam Halls ──

    pub async fn get_exam_halls(
        &self,
        org_id: &str,
        exam_session_id: &str,
    ) -> Result<Vec<exam_hall::Model>, AppError> {
        let _ = self.get_exam_session(org_id, exam_session_id).await?;
        let params = ExamHallParams { exam_session_id: Some(exam_session_id.to_string()), room_id: None };
        Ok(self.query.get_exam_halls(exam_session_id, params).await?)
    }

    pub async fn create_exam_hall(
        &self,
        org_id: &str,
        req: CreateExamHallRequest,
    ) -> Result<exam_hall::Model, AppError> {
        let _ = self.get_exam_session(org_id, &req.exam_session_id).await?;

        let room = room::Entity::find_by_id(req.room_id.clone())
            .one(&self.query.db)
            .await?
            .ok_or_else(|| AppError::not_found("room", &req.room_id))?;

        let now = Utc::now().to_rfc3339();
        let model = exam_hall::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            exam_session_id: req.exam_session_id,
            room_id: req.room_id,
            hall_number: req.hall_number,
            capacity: req.capacity.unwrap_or(room.capacity.unwrap_or(30)),
            allocated_count: 0,
            seating_arrangement: req.seating_arrangement,
            floor_number: room.floor,
            building: room.building,
            is_accessible: req.is_accessible.unwrap_or(0),
            status: "active".to_string(),
            notes: req.notes,
            created_at: now.clone(),
            updated_at: now,
        };

        Ok(self.cmd.insert_exam_hall(model).await?)
    }
}
