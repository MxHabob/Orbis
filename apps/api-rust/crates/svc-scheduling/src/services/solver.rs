//! Timetable Solver coordinator service — schedules lessons automatically using infra-scheduler.

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use chrono::Utc;
use uuid::Uuid;
use tokio::time::sleep;
use tracing::{error, info, warn};
use serde::{Deserialize, Serialize};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

use shared_kernel::errors::AppError;
use crate::entities::solve_job;
use crate::params::RoomParams;
use crate::repository::{SchedulingCommandRepo, SchedulingQueryRepo};
use crate::services::schedule::{LessonPlacement, ScheduleService};

use svc_academic::entities::{class, class_subject};


use infra_scheduler::{
    JobStatus, LessonAssignmentDto, PinnedLessonDto, ScheduleSolver, SolveMode,
    TeacherUnavailableDto, TimeSlotDto, TimetableProblem, RoomDto,
};

const MAX_CONCURRENT_JOBS_PER_ORG: u64 = 1;
const POLL_INTERVAL: Duration = Duration::from_secs(2);
const MAX_POLL_ATTEMPTS: u32 = 300;

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct StartSolveRequest {
    pub cycle_id: String,
    #[serde(default)]
    pub mode: SolveMode,
}

#[derive(serde::Serialize, utoipa::ToSchema, Clone, Debug)]
pub struct SolveJobResponse {
    pub id: String,
    pub org_id: String,
    pub cycle_id: String,
    pub mode: String,
    pub status: String,
    pub score: Option<String>,
    pub lessons_created: i32,
    pub error_message: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: String,
}

impl From<solve_job::Model> for SolveJobResponse {
    fn from(j: solve_job::Model) -> Self {
        Self {
            id: j.id,
            org_id: j.org_id,
            cycle_id: j.target_id,
            mode: j.mode.unwrap_or_default(),
            status: j.status,
            score: j.score,
            lessons_created: j.items_created,
            error_message: j.error_message,
            started_at: j.started_at,
            completed_at: j.completed_at,
            created_at: j.created_at,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct StoredSolution {
    assignments: Vec<LessonAssignmentDto>,
}

fn solve_mode_label(mode: &SolveMode) -> &'static str {
    match mode {
        SolveMode::Full => "full",
        SolveMode::Incremental => "incremental",
    }
}

fn filter_open_assignments(
    assignments: Vec<LessonAssignmentDto>,
    open_ids: &HashSet<String>,
) -> Vec<LessonAssignmentDto> {
    assignments
        .into_iter()
        .filter(|a| open_ids.contains(&a.id))
        .collect()
}

pub struct TimetableSolverService {
    pub cmd: Arc<SchedulingCommandRepo>,
    pub query: Arc<SchedulingQueryRepo>,
    pub schedule_service: Arc<ScheduleService>,
    pub scheduler: Arc<dyn ScheduleSolver>,
}

impl TimetableSolverService {
    pub fn new(
        cmd: Arc<SchedulingCommandRepo>,
        query: Arc<SchedulingQueryRepo>,
        schedule_service: Arc<ScheduleService>,
        scheduler: Arc<dyn ScheduleSolver>,
    ) -> Self {
        Self {
            cmd,
            query,
            schedule_service,
            scheduler,
        }
    }

    /// Start a timetable auto-scheduling job.
    pub async fn start_schedule_solve(
        &self,
        org_id: &str,
        user_id: &str,
        req: StartSolveRequest,
    ) -> Result<SolveJobResponse, AppError> {
        let active = self.query.count_active_solve_jobs(org_id, "timetable").await?;
        if active >= MAX_CONCURRENT_JOBS_PER_ORG {
            return Err(AppError::conflict("A solve job is already running for this organization"));
        }

        let mode = req.mode.clone();
        let job_id = Uuid::new_v4().to_string();

        let mut problem = self.build_timetable_problem(org_id, &req.cycle_id, mode.clone()).await?;
        problem.job_id = job_id.clone();

        if problem.assignments.is_empty() && problem.pinned_lessons.is_empty() {
            return Err(AppError::validation(
                "No lesson assignments to solve. Configure class subjects with weekly hours first."
            ));
        }

        let now = Utc::now().to_rfc3339();
        let snapshot = serde_json::to_string(&problem).map_err(|e| AppError::internal(e.to_string()))?;

        let job = solve_job::Model {
            id: job_id.clone(),
            org_id: org_id.to_string(),
            job_type: "timetable".to_string(),
            target_id: req.cycle_id.clone(),
            mode: Some(solve_mode_label(&mode).to_string()),
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

        self.cmd.create_solve_job(job.clone()).await?;

        let scheduler = Arc::clone(&self.scheduler);
        let cmd = Arc::clone(&self.cmd);
        let query = Arc::clone(&self.query);
        let open_ids: HashSet<String> = problem.assignments.iter().map(|a| a.id.clone()).collect();

        tokio::spawn(async move {
            if let Err(e) = Self::run_solve_job(scheduler, cmd, query, job_id, problem, open_ids).await {
                error!(error = %e, "solve job failed");
            }
        });

        Ok(job.into())
    }

    pub async fn get_schedule_solve_job(
        &self,
        org_id: &str,
        job_id: &str,
    ) -> Result<SolveJobResponse, AppError> {
        let job = self.query.get_solve_job_by_id(job_id).await?
            .ok_or_else(|| AppError::not_found("solve_job", job_id))?;

        if job.org_id != org_id {
            return Err(AppError::not_found("solve_job", job_id));
        }

        Ok(job.into())
    }

    pub async fn list_schedule_solve_jobs(
        &self,
        org_id: &str,
        cycle_id: Option<&str>,
    ) -> Result<Vec<SolveJobResponse>, AppError> {
        let jobs = self.query.get_solve_jobs(org_id, "timetable", cycle_id).await?;
        Ok(jobs.into_iter().map(Into::into).collect())
    }

    pub async fn cancel_schedule_solve(
        &self,
        org_id: &str,
        job_id: &str,
    ) -> Result<SolveJobResponse, AppError> {
        let mut job = self.query.get_solve_job_by_id(job_id).await?
            .ok_or_else(|| AppError::not_found("solve_job", job_id))?;

        if job.org_id != org_id {
            return Err(AppError::not_found("solve_job", job_id));
        }

        if matches!(job.status.as_str(), "completed" | "failed" | "cancelled") {
            return Ok(job.into());
        }

        let _ = self.scheduler.cancel(job_id).await;
        job.status = "cancelled".to_string();
        job.completed_at = Some(Utc::now().to_rfc3339());
        job.updated_at = Utc::now().to_rfc3339();

        let updated = self.cmd.update_solve_job(job).await?;
        Ok(updated.into())
    }

    pub async fn apply_schedule_solve(
        &self,
        org_id: &str,
        job_id: &str,
    ) -> Result<SolveJobResponse, AppError> {
        let mut job = self.query.get_solve_job_by_id(job_id).await?
            .ok_or_else(|| AppError::not_found("solve_job", job_id))?;

        if job.org_id != org_id {
            return Err(AppError::not_found("solve_job", job_id));
        }
        if job.status != "completed" {
            return Err(AppError::validation("Solve job is not completed yet"));
        }

        let solution_json = job.best_solution.as_ref()
            .ok_or_else(|| AppError::validation("No solution available to apply"))?;

        let solution: StoredSolution = serde_json::from_str(solution_json)
            .map_err(|e| AppError::validation(format!("Invalid solution: {e}")))?;

        if job.mode.as_deref() == Some("full") {
            self.cmd.cancel_lessons_for_cycle(org_id, &job.target_id).await?;
        }

        let mut created = 0i32;
        for assignment in &solution.assignments {
            let time_slot_id = match &assignment.time_slot_id {
                Some(id) => id.clone(),
                None => continue,
            };
            let day_of_week = match assignment.day_of_week {
                Some(d) => d,
                None => continue,
            };

            let placement = LessonPlacement {
                cycle_id: job.target_id.clone(),
                class_id: assignment.class_id.clone(),
                class_subject_id: assignment.class_subject_id.clone(),
                subject_id: assignment.subject_id.clone(),
                teacher_id: assignment.teacher_id.clone(),
                time_slot_id,
                day_of_week,
                room_id: assignment.room_id.clone(),
                topic: None,
                topic_ar: None,
                homework: None,
                notes: Some(format!("Auto-generated by solve job {}", job.id)),
            };

            self.schedule_service.persist_lesson(org_id, placement, None).await?;
            created += 1;
        }

        job.items_created = created;
        job.updated_at = Utc::now().to_rfc3339();
        let updated = self.cmd.update_solve_job(job).await?;

        info!(job_id = %updated.id, created, "Applied timetable solution");
        Ok(updated.into())
    }

    pub async fn delete_schedule_solve_job(
        &self,
        org_id: &str,
        job_id: &str,
    ) -> Result<(), AppError> {
        let job = self.query.get_solve_job_by_id(job_id).await?
            .ok_or_else(|| AppError::not_found("solve_job", job_id))?;

        if job.org_id != org_id {
            return Err(AppError::not_found("solve_job", job_id));
        }

        // If active (pending or running), cancel solver execution first
        if job.status == "pending" || job.status == "running" {
            let _ = self.scheduler.cancel(job_id).await;
        }

        self.cmd.delete_solve_job(job_id).await?;
        Ok(())
    }


    async fn build_timetable_problem(
        &self,
        org_id: &str,
        cycle_id: &str,
        mode: SolveMode,
    ) -> Result<TimetableProblem, AppError> {
        let slots = self.query.get_time_slots(org_id, cycle_id).await?;
        if slots.is_empty() {
            return Err(AppError::validation("No time slots configured for this cycle"));
        }

        let time_slots: Vec<TimeSlotDto> = slots.iter().map(|s| TimeSlotDto {
            id: s.id.clone(),
            slot_number: s.slot_number,
            start_time: s.start_time.clone(),
            end_time: s.end_time.clone(),
            is_break: s.is_break != 0,
            applicable_days: s.applicable_days.as_ref().map(|d| {
                d.split(',')
                    .filter_map(|v| v.trim().parse().ok())
                    .collect()
            }),
        }).collect();

        let rooms_list = self.query.get_rooms(org_id, RoomParams { building: None, room_type: None, is_active: Some(1) }).await?;
        let rooms: Vec<RoomDto> = rooms_list.iter().map(|r| RoomDto {
            id: r.id.clone(),
            name: r.name.clone(),
            capacity: r.capacity,
            room_type: r.room_type.clone(),
        }).collect();

        let unavailability_list = self.query.get_teacher_unavailability(org_id, cycle_id).await?;
        let unavailable_slots: Vec<TeacherUnavailableDto> = unavailability_list.into_iter().map(|r| TeacherUnavailableDto {
            teacher_id: r.teacher_id,
            day_of_week: r.day_of_week,
            time_slot_id: r.time_slot_id,
        }).collect();

        let classes = class::Entity::find()
            .filter(class::Column::OrgId.eq(org_id))
            .filter(class::Column::CycleId.eq(cycle_id))
            .filter(class::Column::IsActive.eq(1))
            .all(&self.query.db)
            .await?;

        let class_ids: Vec<String> = classes.iter().map(|c| c.id.clone()).collect();
        if class_ids.is_empty() {
            return Err(AppError::validation("No classes found for this cycle"));
        }

        let class_subjects = class_subject::Entity::find()
            .filter(class_subject::Column::CycleId.eq(cycle_id))
            .filter(class_subject::Column::IsActive.eq(1))
            .filter(class_subject::Column::ClassId.is_in(class_ids))
            .all(&self.query.db)
            .await?;

        let existing = self.query.get_all_scheduled_lessons_for_cycle(org_id, cycle_id).await?;

        let mut lessons_per_subject: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
        for lesson in &existing {
            *lessons_per_subject.entry(lesson.class_subject_id.clone()).or_insert(0) += 1;
        }

        let pinned_lessons: Vec<PinnedLessonDto> = existing.iter().map(|l| PinnedLessonDto {
            id: l.id.clone(),
            class_id: l.class_id.clone(),
            teacher_id: l.teacher_id.clone(),
            subject_id: l.subject_id.clone(),
            class_subject_id: l.class_subject_id.clone(),
            time_slot_id: l.time_slot_id.clone(),
            day_of_week: l.day_of_week,
            room_id: l.room_id.clone(),
        }).collect();

        let mut assignments = Vec::new();
        for cs in &class_subjects {
            let target = cs.weekly_hours.unwrap_or(1).max(0) as u32;
            let existing_count = lessons_per_subject.get(&cs.id).copied().unwrap_or(0);

            let to_create = match mode {
                SolveMode::Full => target,
                SolveMode::Incremental => target.saturating_sub(existing_count),
            };

            for hour in 0..to_create {
                assignments.push(LessonAssignmentDto {
                    id: Uuid::new_v4().to_string(),
                    class_subject_id: cs.id.clone(),
                    class_id: cs.class_id.clone(),
                    subject_id: cs.subject_id.clone(),
                    teacher_id: cs.primary_teacher_id.clone(),
                    hour_index: (hour + 1) as i32,
                    time_slot_id: None,
                    day_of_week: None,
                    room_id: None,
                });
            }
        }

        let pinned = match &mode {
            SolveMode::Incremental => pinned_lessons,
            SolveMode::Full => vec![],
        };

        Ok(TimetableProblem {
            job_id: String::new(),
            org_id: org_id.to_string(),
            cycle_id: cycle_id.to_string(),
            mode,
            time_slots,
            rooms,
            assignments,
            pinned_lessons: pinned,
            unavailable_slots,
        })
    }

    async fn run_solve_job(
        scheduler: Arc<dyn ScheduleSolver>,
        cmd: Arc<SchedulingCommandRepo>,
        query: Arc<SchedulingQueryRepo>,
        job_id: String,
        mut problem: TimetableProblem,
        open_ids: HashSet<String>,
    ) -> Result<(), AppError> {
        problem.job_id = job_id.clone();

        let mut job = query.get_solve_job_by_id(&job_id).await?
            .ok_or_else(|| AppError::not_found("solve_job", &job_id))?;

        job.status = "running".to_string();
        job.started_at = Some(Utc::now().to_rfc3339());
        job.updated_at = Utc::now().to_rfc3339();
        job = cmd.update_solve_job(job).await?;

        if let Err(e) = scheduler.submit(&problem).await {
            job.status = "failed".to_string();
            job.error_message = Some(e.to_string());
            job.completed_at = Some(Utc::now().to_rfc3339());
            job.updated_at = Utc::now().to_rfc3339();
            cmd.update_solve_job(job).await?;
            return Err(AppError::internal(e.to_string()));
        }

        for attempt in 0..MAX_POLL_ATTEMPTS {
            sleep(POLL_INTERVAL).await;

            if let Some(j) = query.get_solve_job_by_id(&job_id).await? {
                if j.status == "cancelled" {
                    let _ = scheduler.cancel(&job_id).await;
                    return Ok(());
                }
            }

            match scheduler.status(&job_id).await {
                Ok(status) => {
                    let mut current_job = query.get_solve_job_by_id(&job_id).await?
                        .ok_or_else(|| AppError::not_found("solve_job", &job_id))?;

                    current_job.score = status.score.clone();
                    current_job.updated_at = Utc::now().to_rfc3339();

                    if let Some(solution) = &status.best_solution {
                        let filtered = filter_open_assignments(solution.assignments.clone(), &open_ids);
                        current_job.best_solution = serde_json::to_string(&StoredSolution {
                            assignments: filtered,
                        }).ok();
                    }

                    match status.status {
                        JobStatus::Completed => {
                            current_job.status = "completed".to_string();
                            current_job.completed_at = Some(Utc::now().to_rfc3339());
                            cmd.update_solve_job(current_job).await?;
                            info!(job_id = %job_id, "Solve job completed successfully");
                            return Ok(());
                        }
                        JobStatus::Failed => {
                            current_job.status = "failed".to_string();
                            current_job.error_message = status.error;
                            current_job.completed_at = Some(Utc::now().to_rfc3339());
                            cmd.update_solve_job(current_job).await?;
                            return Err(AppError::internal("Solver reported job failure"));
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
                Err(e) => warn!(attempt, error = %e, "Solver poll failed"),
            }

            if attempt == MAX_POLL_ATTEMPTS - 1 {
                let mut current_job = query.get_solve_job_by_id(&job_id).await?
                    .ok_or_else(|| AppError::not_found("solve_job", &job_id))?;
                current_job.status = "failed".to_string();
                current_job.error_message = Some("Solve timed out".to_string());
                current_job.completed_at = Some(Utc::now().to_rfc3339());
                current_job.updated_at = Utc::now().to_rfc3339();
                cmd.update_solve_job(current_job).await?;
                let _ = scheduler.cancel(&job_id).await;
                return Err(AppError::internal("Solve job timed out"));
            }
        }

        Ok(())
    }
}
