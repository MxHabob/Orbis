use std::sync::Arc;
use std::time::Duration;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tokio::time::sleep;
use tracing::{error, info, warn};

use crate::entities::solve_job;
use crate::service::schedule_solver::SolveJobResponse;
use infra_scheduler::{JobStatus, ExamProblem, ExamTimeSlotDto, ExamAssignmentDto, ScheduleSolver};

/// Request to auto-generate an optimal exam schedule
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AutoScheduleRequest {
    pub exam_session_id: String,
    /// Working days (ISO format: 2026-01-15, 2026-01-16, ...)
    pub available_dates: Vec<String>,
    /// Max exams per day (default: 2)
    pub max_exams_per_day: Option<i32>,
    /// Preferred period: "morning", "afternoon", or "both"
    pub preferred_period: Option<String>,
    /// Default exam duration in minutes (default: 90)
    pub default_duration_minutes: Option<i32>,
    /// Morning period start time (default: "08:00")
    pub morning_start: Option<String>,
    /// Afternoon period start time (default: "13:00")
    pub afternoon_start: Option<String>,
    /// Gap between consecutive exams in minutes (default: 30)
    pub gap_minutes: Option<i32>,
    /// Subjects that should not be on the same day
    pub conflict_groups: Option<Vec<Vec<String>>>,
    /// Priority subjects to schedule first (e.g., core subjects)
    pub priority_subject_ids: Option<Vec<String>>,
}

/// Result of the auto-scheduling operation
#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AutoScheduleResult {
    pub total_subjects: usize,
    pub scheduled_count: usize,
    pub unscheduled: Vec<UnscheduledSubject>,
    pub schedule: Vec<ScheduleEntry>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ScheduleEntry {
    pub exam_schedule_id: String,
    pub subject_id: String,
    pub subject_name: Option<String>,
    pub exam_date: String,
    pub start_time: String,
    pub end_time: String,
    pub period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UnscheduledSubject {
    pub subject_id: String,
    pub reason: String,
}

impl super::Service {
    pub async fn start_exam_solve(
        &self,
        org_id: &str,
        user_id: &str,
        req: AutoScheduleRequest,
    ) -> Result<SolveJobResponse, String> {
        let active = self
            .repo
            .count_active_solve_jobs(org_id, "exam")
            .await
            .map_err(|e| e.to_string())?;
        if active >= 1 {
            return Err(
                "an exam solve job is already running for this organization; wait for it to finish"
                    .to_string(),
            );
        }

        let job_id = Uuid::new_v4().to_string();

        let problem = self.build_exam_problem(org_id, &req, &job_id).await?;
        let snapshot = serde_json::to_string(&problem).map_err(|e| e.to_string())?;

        let now = Utc::now().to_rfc3339();
        let job = solve_job::Model {
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

        self.repo
            .insert_solve_job(job.clone())
            .await
            .map_err(|e| e.to_string())?;

        let scheduler = Arc::clone(&self.scheduler);
        let repo = Arc::clone(&self.repo);

        tokio::spawn(async move {
            if let Err(e) =
                Self::run_exam_solve_job(scheduler, repo, job_id, problem).await
            {
                error!(error = %e, "exam solve job failed");
            }
        });

        Ok(job.into())
    }

    pub async fn get_exam_solve_job(
        &self,
        org_id: &str,
        job_id: &str,
    ) -> Result<SolveJobResponse, String> {
        let job = self
            .repo
            .get_solve_job_by_id(job_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "solve job not found".to_string())?;

        if job.org_id != org_id || job.job_type != "exam" {
            return Err("solve job not found".to_string());
        }

        Ok(job.into())
    }

    pub async fn list_exam_solve_jobs(
        &self,
        org_id: &str,
        exam_session_id: Option<&str>,
    ) -> Result<Vec<SolveJobResponse>, String> {
        let jobs = self
            .repo
            .get_solve_jobs(org_id, "exam", exam_session_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(jobs.into_iter().map(Into::into).collect())
    }

    pub async fn cancel_exam_solve(
        &self,
        org_id: &str,
        job_id: &str,
    ) -> Result<SolveJobResponse, String> {
        let mut job = self
            .repo
            .get_solve_job_by_id(job_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "solve job not found".to_string())?;

        if job.org_id != org_id || job.job_type != "exam" {
            return Err("solve job not found".to_string());
        }

        if matches!(job.status.as_str(), "completed" | "failed" | "cancelled") {
            return Ok(job.into());
        }

        let _ = self.scheduler.cancel_exam(job_id).await;
        job.status = "cancelled".to_string();
        job.completed_at = Some(Utc::now().to_rfc3339());
        job.updated_at = Utc::now().to_rfc3339();

        let job = self.repo.update_solve_job(job).await.map_err(|e| e.to_string())?;
        Ok(job.into())
    }

    pub async fn apply_exam_solve(
        &self,
        org_id: &str,
        job_id: &str,
    ) -> Result<SolveJobResponse, String> {
        let mut job = self
            .repo
            .get_solve_job_by_id(job_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "solve job not found".to_string())?;

        if job.org_id != org_id || job.job_type != "exam" {
            return Err("solve job not found".to_string());
        }
        if job.status != "completed" {
            return Err("solve job is not completed yet".to_string());
        }

        let solution_json = job
            .best_solution
            .as_ref()
            .ok_or_else(|| "no solution available to apply".to_string())?;

        let solution: infra_scheduler::ExamSolution =
            serde_json::from_str(solution_json).map_err(|e| format!("invalid solution: {e}"))?;

        // 1. Clear current schedules for this exam session
        self.repo
            .delete_exam_schedules_for_session(&job.target_id)
            .await
            .map_err(|e| e.to_string())?;

        // 2. Load the problem snapshot
        let problem_json = job
            .problem_snapshot
            .as_ref()
            .ok_or_else(|| "no problem snapshot available".to_string())?;
        let problem: infra_scheduler::ExamProblem =
            serde_json::from_str(problem_json).map_err(|e| format!("invalid problem: {e}"))?;

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

            let exam_subject = self
                .repo
                .get_exam_subjects(&job.target_id)
                .await
                .map_err(|e| e.to_string())?
                .into_iter()
                .find(|s| s.id == assignment.exam_subject_id)
                .ok_or_else(|| format!("Exam subject {} not found", assignment.exam_subject_id))?;

            let duration = exam_subject.duration_minutes;
            let start_time = if slot.period == "morning" {
                "08:00".to_string()
            } else {
                "13:00".to_string()
            };
            let end_time = add_minutes(&start_time, duration);

            let model = crate::entities::exam_schedule::Model {
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

            self.repo
                .insert_exam_schedule(model)
                .await
                .map_err(|e| e.to_string())?;

            created += 1;
        }

        job.items_created = created;
        job.updated_at = Utc::now().to_rfc3339();
        let job = self.repo.update_solve_job(job).await.map_err(|e| e.to_string())?;

        Ok(job.into())
    }

    async fn build_exam_problem(
        &self,
        org_id: &str,
        req: &AutoScheduleRequest,
        job_id: &str,
    ) -> Result<ExamProblem, String> {
        let session = self.get_exam_session(org_id, &req.exam_session_id).await?;
        if session.status != "draft" && session.status != "scheduled" {
            return Err("can only auto-schedule for draft or scheduled sessions".to_string());
        }

        let subjects = self
            .repo
            .get_exam_subjects(&req.exam_session_id)
            .await
            .map_err(|e| e.to_string())?;

        if subjects.is_empty() {
            return Err("no exam subjects found for this session".to_string());
        }

        if req.available_dates.is_empty() {
            return Err("available_dates cannot be empty".to_string());
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
                if let Ok(Some(cs)) = self.repo.get_class_subject_by_id(cs_id).await {
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
        repo: Arc<crate::repository::Repository>,
        job_id: String,
        problem: ExamProblem,
    ) -> Result<(), String> {
        let mut job = repo
            .get_solve_job_by_id(&job_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "job not found".to_string())?;

        job.status = "running".to_string();
        job.started_at = Some(Utc::now().to_rfc3339());
        job.updated_at = Utc::now().to_rfc3339();
        job = repo.update_solve_job(job).await.map_err(|e| e.to_string())?;

        if let Err(e) = scheduler.submit_exam(&problem).await {
            job.status = "failed".to_string();
            job.error_message = Some(e.to_string());
            job.completed_at = Some(Utc::now().to_rfc3339());
            job.updated_at = Utc::now().to_rfc3339();
            repo.update_solve_job(job).await.map_err(|e| e.to_string())?;
            return Err(e.to_string());
        }

        const MAX_POLL_ATTEMPTS: u32 = 300;
        const POLL_INTERVAL: Duration = Duration::from_secs(2);

        for attempt in 0..MAX_POLL_ATTEMPTS {
            sleep(POLL_INTERVAL).await;

            if let Some(j) = repo
                .get_solve_job_by_id(&job_id)
                .await
                .map_err(|e| e.to_string())?
            {
                if j.status == "cancelled" {
                    let _ = scheduler.cancel_exam(&job_id).await;
                    return Ok(());
                }
            }

            match scheduler.status_exam(&job_id).await {
                Ok(status) => {
                    let mut job = repo
                        .get_solve_job_by_id(&job_id)
                        .await
                        .map_err(|e| e.to_string())?
                        .ok_or_else(|| "job not found".to_string())?;

                    job.score = status.score.clone();
                    job.updated_at = Utc::now().to_rfc3339();

                    if let Some(solution) = &status.best_solution {
                        job.best_solution = serde_json::to_string(solution).ok();
                    }

                    match status.status {
                        JobStatus::Completed => {
                            job.status = "completed".to_string();
                            job.completed_at = Some(Utc::now().to_rfc3339());
                            repo.update_solve_job(job).await.map_err(|e| e.to_string())?;
                            info!(job_id = %job_id, "exam solve job completed");
                            return Ok(());
                        }
                        JobStatus::Failed => {
                            job.status = "failed".to_string();
                            job.error_message = status.error;
                            job.completed_at = Some(Utc::now().to_rfc3339());
                            repo.update_solve_job(job).await.map_err(|e| e.to_string())?;
                            return Err("solver reported failure".to_string());
                        }
                        JobStatus::Cancelled => {
                            job.status = "cancelled".to_string();
                            job.completed_at = Some(Utc::now().to_rfc3339());
                            repo.update_solve_job(job).await.map_err(|e| e.to_string())?;
                            return Ok(());
                        }
                        JobStatus::Pending | JobStatus::Running => {
                            repo.update_solve_job(job).await.map_err(|e| e.to_string())?;
                        }
                    }
                }
                Err(e) => warn!(attempt, error = %e, "exam solver poll failed"),
            }

            if attempt == MAX_POLL_ATTEMPTS - 1 {
                let mut job = repo
                    .get_solve_job_by_id(&job_id)
                    .await
                    .map_err(|e| e.to_string())?
                    .ok_or_else(|| "job not found".to_string())?;
                job.status = "failed".to_string();
                job.error_message = Some("solve timed out".to_string());
                job.completed_at = Some(Utc::now().to_rfc3339());
                job.updated_at = Utc::now().to_rfc3339();
                repo.update_solve_job(job).await.map_err(|e| e.to_string())?;
                let _ = scheduler.cancel_exam(&job_id).await;
                return Err("solve timed out".to_string());
            }
        }

        Ok(())
    }

    /// Postpone a scheduled exam to a new date
    pub async fn postpone_exam(
        &self,
        org_id: &str,
        exam_session_id: &str,
        schedule_id: &str,
        new_date: &str,
        reason: &str,
    ) -> Result<crate::entities::exam_schedule::Model, String> {
        let _session = self.get_exam_session(org_id, exam_session_id).await?;

        let schedules = self
            .repo
            .get_exam_schedules(exam_session_id)
            .await
            .map_err(|e| e.to_string())?;

        let schedule = schedules
            .iter()
            .find(|s| s.id == schedule_id)
            .ok_or_else(|| "exam schedule not found".to_string())?;

        let mut updated = schedule.clone();
        updated.status = "postponed".to_string();
        updated.postponed_to = Some(new_date.to_string());
        updated.postponement_reason = Some(reason.to_string());
        updated.updated_at = Utc::now().to_rfc3339();

        // Create new schedule entry for the new date
        let new_schedule = crate::entities::exam_schedule::Model {
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

        self.repo
            .insert_exam_schedule(new_schedule.clone())
            .await
            .map_err(|e| e.to_string())?;

        Ok(new_schedule)
    }
}

/// Add minutes to a time string (HH:MM) and return new time string
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
