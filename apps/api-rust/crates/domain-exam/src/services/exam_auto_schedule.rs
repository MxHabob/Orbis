//! Exam auto-schedule service — schedules exams optimally without student conflicts using infra-scheduler.

use std::sync::Arc;
use std::time::Duration;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tokio::time::sleep;
use tracing::{error, info, warn};

use shared_kernel::errors::AppError;
use crate::entities::exam_schedule;
use crate::repository::{ExamCommandRepo, ExamQueryRepo};
use crate::services::exam_session::ExamSessionService;

use infra_scheduler::{JobStatus, ExamProblem, ExamTimeSlotDto, ExamAssignmentDto, ScheduleSolver};

/// Request to auto-generate an optimal exam schedule
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AutoScheduleRequest {
    pub exam_session_id: String,
    pub available_dates: Vec<String>,
    pub max_exams_per_day: Option<i32>,
    pub preferred_period: Option<String>,
    pub default_duration_minutes: Option<i32>,
    pub morning_start: Option<String>,
    pub afternoon_start: Option<String>,
    pub gap_minutes: Option<i32>,
    pub conflict_groups: Option<Vec<Vec<String>>>,
    pub priority_subject_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, utoipa::ToSchema)]
pub struct SolveJobResponse {
    pub id: String,
    pub org_id: String,
    pub exam_session_id: String,
    pub status: String,
    pub score: Option<String>,
    pub items_created: i32,
    pub error_message: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: String,
}

impl From<domain_scheduling::entities::solve_job::Model> for SolveJobResponse {
    fn from(j: domain_scheduling::entities::solve_job::Model) -> Self {
        Self {
            id: j.id,
            org_id: j.org_id,
            exam_session_id: j.target_id,
            status: j.status,
            score: j.score,
            items_created: j.items_created,
            error_message: j.error_message,
            started_at: j.started_at,
            completed_at: j.completed_at,
            created_at: j.created_at,
        }
    }
}

pub struct ExamAutoScheduleService {
    pub cmd: Arc<ExamCommandRepo>,
    pub query: Arc<ExamQueryRepo>,
    pub session_service: Arc<ExamSessionService>,
    pub scheduler: Arc<dyn ScheduleSolver>,
}

impl ExamAutoScheduleService {
    pub fn new(
        cmd: Arc<ExamCommandRepo>,
        query: Arc<ExamQueryRepo>,
        session_service: Arc<ExamSessionService>,
        scheduler: Arc<dyn ScheduleSolver>,
    ) -> Self {
        Self {
            cmd,
            query,
            session_service,
            scheduler,
        }
    }

    pub async fn start_exam_solve(
        &self,
        org_id: &str,
        user_id: &str,
        req: AutoScheduleRequest,
    ) -> Result<SolveJobResponse, AppError> {
        let active = self.query.count_active_solve_jobs(org_id, "exam").await?;
        if active >= 1 {
            return Err(AppError::Conflict {
                entity: "solve_job".into(),
                details: "An exam solve job is already running for this organization".into(),
            });
        }

        let job_id = Uuid::new_v4().to_string();
        let problem = self.build_exam_problem(org_id, &req, &job_id).await?;
        let snapshot = serde_json::to_string(&problem).map_err(|e| AppError::internal(e.to_string()))?;

        let now = Utc::now().to_rfc3339();
        let job = domain_scheduling::entities::solve_job::Model {
            id: job_id.clone(),
            org_id: org_id.to_string(),
            job_type: "exam".to_string(),
            target_id: req.exam_session_id.clone(),
            mode: None,
            status: "pending".to_string(),
            problem_snapshot: Some(snapshot),
            best_solution: None,
            score: None,
            items_created: 0,
            error_message: None,
            created_by: Some(user_id.to_string()),
            started_at: None,
            completed_at: None,
            created_at: now.clone(),
            updated_at: now,
        };

        self.cmd.insert_solve_job(job.clone()).await?;

        let scheduler = Arc::clone(&self.scheduler);
        let cmd = Arc::clone(&self.cmd);
        let query = Arc::clone(&self.query);

        tokio::spawn(async move {
            if let Err(e) = Self::run_exam_solve_job(scheduler, cmd, query, job_id, problem).await {
                error!(error = %e, "exam solve job failed");
            }
        });

        Ok(job.into())
    }

    pub async fn get_exam_solve_job(
        &self,
        org_id: &str,
        job_id: &str,
    ) -> Result<SolveJobResponse, AppError> {
        let job = self.query.get_solve_job_by_id(job_id).await?
            .ok_or_else(|| AppError::not_found("solve_job", job_id))?;

        if job.org_id != org_id || job.job_type != "exam" {
            return Err(AppError::not_found("solve_job", job_id));
        }

        Ok(job.into())
    }

    pub async fn list_exam_solve_jobs(
        &self,
        org_id: &str,
        exam_session_id: Option<&str>,
    ) -> Result<Vec<SolveJobResponse>, AppError> {
        let jobs = self.query.get_solve_jobs(org_id, "exam", exam_session_id).await?;
        Ok(jobs.into_iter().map(Into::into).collect())
    }

    pub async fn cancel_exam_solve(
        &self,
        org_id: &str,
        job_id: &str,
    ) -> Result<SolveJobResponse, AppError> {
        let mut job = self.query.get_solve_job_by_id(job_id).await?
            .ok_or_else(|| AppError::not_found("solve_job", job_id))?;

        if job.org_id != org_id || job.job_type != "exam" {
            return Err(AppError::not_found("solve_job", job_id));
        }

        if matches!(job.status.as_str(), "completed" | "failed" | "cancelled") {
            return Ok(job.into());
        }

        let _ = self.scheduler.cancel_exam(&job_id).await;
        job.status = "cancelled".to_string();
        job.completed_at = Some(Utc::now().to_rfc3339());
        job.updated_at = Utc::now().to_rfc3339();

        let updated = self.cmd.update_solve_job(job).await?;
        Ok(updated.into())
    }

    pub async fn apply_exam_solve(
        &self,
        org_id: &str,
        job_id: &str,
    ) -> Result<SolveJobResponse, AppError> {
        let mut job = self.query.get_solve_job_by_id(job_id).await?
            .ok_or_else(|| AppError::not_found("solve_job", job_id))?;

        if job.org_id != org_id || job.job_type != "exam" {
            return Err(AppError::not_found("solve_job", job_id));
        }
        if job.status != "completed" {
            return Err(AppError::validation("solve job is not completed yet"));
        }

        let solution_json = job.best_solution.as_ref()
            .ok_or_else(|| AppError::validation("no solution available to apply"))?;

        let solution: infra_scheduler::ExamSolution = serde_json::from_str(solution_json)
            .map_err(|e| AppError::validation(format!("invalid solution: {e}")))?;

        // 1. Clear current schedules for this exam session
        self.cmd.delete_exam_schedules_for_session(&job.target_id).await?;

        // 2. Load the problem snapshot
        let problem_json = job.problem_snapshot.as_ref()
            .ok_or_else(|| AppError::validation("no problem snapshot available"))?;
        let problem: infra_scheduler::ExamProblem = serde_json::from_str(problem_json)
            .map_err(|e| AppError::validation(format!("invalid problem: {e}")))?;

        let slot_map: std::collections::HashMap<String, &infra_scheduler::ExamTimeSlotDto> =
            problem.slots.iter().map(|s| (s.id.clone(), s)).collect();

        let now = Utc::now().to_rfc3339();
        let mut created = 0i32;

        for assignment in &solution.assignments {
            let Some(slot_id) = &assignment.slot_id else {
                continue;
            };
            let Some(slot) = slot_map.get(slot_id) else {
                continue;
            };

            let exam_subject = self.query.get_exam_subjects(&job.target_id).await?
                .into_iter()
                .find(|s| s.id == assignment.exam_subject_id)
                .ok_or_else(|| AppError::validation(format!("Exam subject {} not found", assignment.exam_subject_id)))?;

            let duration = exam_subject.duration_minutes;
            let start_time = if slot.period == "morning" {
                "08:00".to_string()
            } else {
                "13:00".to_string()
            };
            let end_time = add_minutes(&start_time, duration);

            let model = exam_schedule::Model {
                id: Uuid::new_v4().to_string(),
                exam_session_id: job.target_id.clone(),
                exam_subject_id: assignment.exam_subject_id.clone(),
                exam_date: slot.date.clone(),
                start_time,
                end_time,
                period: slot.period.clone(),
                applicable_class_ids: Some(assignment.class_id.clone()),
                room_ids: None,
                status: "scheduled".to_string(),
                postponed_to: None,
                postponement_reason: None,
                notes: Some("auto-generated via solver-service".to_string()),
                created_at: now.clone(),
                updated_at: now.clone(),
            };

            self.cmd.insert_exam_schedule(model).await?;
            created += 1;
        }

        job.items_created = created;
        job.updated_at = Utc::now().to_rfc3339();
        let updated = self.cmd.update_solve_job(job).await?;

        Ok(updated.into())
    }

    async fn build_exam_problem(
        &self,
        org_id: &str,
        req: &AutoScheduleRequest,
        job_id: &str,
    ) -> Result<ExamProblem, AppError> {
        let session = self.session_service.get_exam_session(org_id, &req.exam_session_id).await?;
        if session.status != "draft" && session.status != "scheduled" {
            return Err(AppError::validation("can only auto-schedule for draft or scheduled sessions"));
        }

        let subjects = self.query.get_exam_subjects(&req.exam_session_id).await?;
        if subjects.is_empty() {
            return Err(AppError::validation("no exam subjects found for this session"));
        }

        if req.available_dates.is_empty() {
            return Err(AppError::validation("available_dates cannot be empty"));
        }

        let max_per_day = req.max_exams_per_day.unwrap_or(2);
        let preferred = req.preferred_period.as_deref().unwrap_or("both");
        let conflict_groups = req.conflict_groups.clone().unwrap_or_default();

        let mut slots = Vec::new();
        let mut sorted_dates = req.available_dates.clone();
        sorted_dates.sort();

        for date in &sorted_dates {
            if preferred == "morning" || preferred == "both" {
                slots.push(ExamTimeSlotDto {
                    id: format!("{}_morning", date),
                    date: date.clone(),
                    period: "morning".to_string(),
                });
            }
            if preferred == "afternoon" || preferred == "both" {
                slots.push(ExamTimeSlotDto {
                    id: format!("{}_afternoon", date),
                    date: date.clone(),
                    period: "afternoon".to_string(),
                });
            }
        }

        let mut assignments = Vec::new();
        for s in &subjects {
            if s.is_included != 1 {
                continue;
            }
            let conflict_group_id = conflict_groups
                .iter()
                .position(|group| group.contains(&s.subject_id))
                .map(|idx| idx as i32);

            let mut class_id = "unknown_class".to_string();
            if let Some(cs_id) = &s.class_subject_id {
                let db_cs = domain_academic::entities::class_subject::Entity::find_by_id(cs_id.clone())
                    .one(&self.query.db)
                    .await?;
                if let Some(cs) = db_cs {
                    class_id = cs.class_id;
                }
            }

            assignments.push(ExamAssignmentDto {
                id: Uuid::new_v4().to_string(),
                exam_subject_id: s.id.clone(),
                subject_id: s.subject_id.clone(),
                class_id,
                conflict_group_id,
                slot_id: None,
            });
        }

        Ok(ExamProblem {
            job_id: job_id.to_string(),
            org_id: org_id.to_string(),
            exam_session_id: req.exam_session_id.clone(),
            slots,
            assignments,
            max_exams_per_day: max_per_day,
        })
    }

    async fn run_exam_solve_job(
        scheduler: Arc<dyn ScheduleSolver>,
        cmd: Arc<ExamCommandRepo>,
        query: Arc<ExamQueryRepo>,
        job_id: String,
        problem: ExamProblem,
    ) -> Result<(), AppError> {
        let mut job = query.get_solve_job_by_id(&job_id).await?
            .ok_or_else(|| AppError::not_found("solve_job", &job_id))?;

        job.status = "running".to_string();
        job.started_at = Some(Utc::now().to_rfc3339());
        job.updated_at = Utc::now().to_rfc3339();
        job = cmd.update_solve_job(job).await?;

        if let Err(e) = scheduler.submit_exam(&problem).await {
            job.status = "failed".to_string();
            job.error_message = Some(e.to_string());
            job.completed_at = Some(Utc::now().to_rfc3339());
            job.updated_at = Utc::now().to_rfc3339();
            cmd.update_solve_job(job).await?;
            return Err(AppError::internal(e.to_string()));
        }

        const MAX_POLL_ATTEMPTS: u32 = 300;
        const POLL_INTERVAL: Duration = Duration::from_secs(2);

        for attempt in 0..MAX_POLL_ATTEMPTS {
            sleep(POLL_INTERVAL).await;

            if let Some(j) = query.get_solve_job_by_id(&job_id).await? {
                if j.status == "cancelled" {
                    let _ = scheduler.cancel_exam(&job_id).await;
                    return Ok(());
                }
            }

            match scheduler.status_exam(&job_id).await {
                Ok(status) => {
                    let mut current_job = query.get_solve_job_by_id(&job_id).await?
                        .ok_or_else(|| AppError::not_found("solve_job", &job_id))?;

                    current_job.score = status.score.clone();
                    current_job.updated_at = Utc::now().to_rfc3339();

                    if let Some(solution) = &status.best_solution {
                        current_job.best_solution = serde_json::to_string(solution).ok();
                    }

                    match status.status {
                        JobStatus::Completed => {
                            current_job.status = "completed".to_string();
                            current_job.completed_at = Some(Utc::now().to_rfc3339());
                            cmd.update_solve_job(current_job).await?;
                            info!(job_id = %job_id, "exam solve job completed");
                            return Ok(());
                        }
                        JobStatus::Failed => {
                            current_job.status = "failed".to_string();
                            current_job.error_message = status.error;
                            current_job.completed_at = Some(Utc::now().to_rfc3339());
                            cmd.update_solve_job(current_job).await?;
                            return Err(AppError::internal("solver reported failure"));
                        }
                        JobStatus::Cancelled => {
                            current_job.status = "cancelled".to_string();
                            current_job.completed_at = Some(Utc::now().to_rfc3339());
                            cmd.update_solve_job(current_job).await?;
                            return Ok(());
                        }
                        JobStatus::Pending | JobStatus::Running => {
                            cmd.update_solve_job(current_job).await?;
                        }
                    }
                }
                Err(e) => warn!(attempt, error = %e, "exam solver poll failed"),
            }

            if attempt == MAX_POLL_ATTEMPTS - 1 {
                let mut current_job = query.get_solve_job_by_id(&job_id).await?
                    .ok_or_else(|| AppError::not_found("solve_job", &job_id))?;
                current_job.status = "failed".to_string();
                current_job.error_message = Some("solve timed out".to_string());
                current_job.completed_at = Some(Utc::now().to_rfc3339());
                current_job.updated_at = Utc::now().to_rfc3339();
                cmd.update_solve_job(current_job).await?;
                let _ = scheduler.cancel_exam(&job_id).await;
                return Err(AppError::internal("solve timed out"));
            }
        }

        Ok(())
    }

    /// Postpone a scheduled exam.
    pub async fn postpone_exam(
        &self,
        org_id: &str,
        exam_session_id: &str,
        schedule_id: &str,
        new_date: &str,
        reason: &str,
    ) -> Result<exam_schedule::Model, AppError> {
        let _session = self.session_service.get_exam_session(org_id, exam_session_id).await?;

        let sched_params = ExamScheduleParams { exam_session_id: Some(exam_session_id.to_string()), exam_date: None, status: None };
        let schedules = self.query.get_exam_schedules(exam_session_id, sched_params).await?;
        let schedule = schedules.iter().find(|s| s.id == schedule_id)
            .ok_or_else(|| AppError::not_found("exam_schedule", schedule_id))?;

        let mut updated = schedule.clone();
        updated.status = "postponed".to_string();
        updated.postponed_to = Some(new_date.to_string());
        updated.postponement_reason = Some(reason.to_string());
        updated.updated_at = Utc::now().to_rfc3339();
        self.cmd.update_exam_schedule(updated).await?;

        let new_schedule = exam_schedule::Model {
            id: Uuid::new_v4().to_string(),
            exam_session_id: exam_session_id.to_string(),
            exam_subject_id: schedule.exam_subject_id.clone(),
            exam_date: new_date.to_string(),
            start_time: schedule.start_time.clone(),
            end_time: schedule.end_time.clone(),
            period: schedule.period.clone(),
            applicable_class_ids: schedule.applicable_class_ids.clone(),
            room_ids: schedule.room_ids.clone(),
            status: "scheduled".to_string(),
            postponed_to: None,
            postponement_reason: None,
            notes: Some(format!("postponed from {} — {}", schedule.exam_date, reason)),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        Ok(self.cmd.insert_exam_schedule(new_schedule).await?)
    }

    pub async fn delete_exam_schedule(
        &self,
        org_id: &str,
        schedule_id: &str,
    ) -> Result<(), AppError> {
        let schedule = self.query.get_exam_schedule_by_id(schedule_id).await?
            .ok_or_else(|| AppError::not_found("exam_schedule", schedule_id))?;

        let session = self.session_service.get_exam_session(org_id, &schedule.exam_session_id).await?;
        if session.status != "draft" && session.status != "scheduled" {
            return Err(AppError::validation("can only delete schedules for draft or scheduled sessions"));
        }

        self.cmd.delete_exam_schedule(schedule_id).await?;
        Ok(())
    }

    pub async fn update_exam_schedule_status(
        &self,
        org_id: &str,
        schedule_id: &str,
        status: &str,
    ) -> Result<exam_schedule::Model, AppError> {
        let mut schedule = self.query.get_exam_schedule_by_id(schedule_id).await?
            .ok_or_else(|| AppError::not_found("exam_schedule", schedule_id))?;

        let session = self.session_service.get_exam_session(org_id, &schedule.exam_session_id).await?;
        if session.status != "draft" && session.status != "scheduled" {
            return Err(AppError::validation("can only update schedules for draft or scheduled sessions"));
        }

        schedule.status = status.to_string();
        schedule.updated_at = Utc::now().to_rfc3339();

        Ok(self.cmd.update_exam_schedule(schedule).await?)
    }
}

fn add_minutes(time: &str, minutes: i32) -> String {
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() != 2 {
        return format!("{}:{:02}", minutes / 60, minutes % 60);
    }
    let h: i32 = parts[0].parse().unwrap_or(0);
    let m: i32 = parts[1].parse().unwrap_or(0);
    let total = h * 60 + m + minutes;
    format!("{:02}:{:02}", total / 60, total % 60)
}
