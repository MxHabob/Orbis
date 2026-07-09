use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use infra_scheduler::{
    JobStatus, LessonAssignmentDto, PinnedLessonDto, ScheduleSolver, SolveMode,
    TeacherUnavailableDto, TimeSlotDto, TimetableProblem,
};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::entities::solve_job;
use crate::service::schedule_common::LessonPlacement;

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

impl super::Service {
    pub async fn start_schedule_solve(
        &self,
        org_id: &str,
        user_id: &str,
        req: StartSolveRequest,
    ) -> Result<SolveJobResponse, String> {
        let active = self
            .repo
            .count_active_solve_jobs(org_id, "timetable")
            .await
            .map_err(|e| e.to_string())?;
        if active >= MAX_CONCURRENT_JOBS_PER_ORG {
            return Err(
                "a solve job is already running for this organization; wait for it to finish"
                    .to_string(),
            );
        }

        let mode = req.mode.clone();
        let job_id = Uuid::new_v4().to_string();

        let mut problem = self
            .build_timetable_problem(org_id, &req.cycle_id, mode.clone())
            .await?;
        problem.job_id = job_id.clone();

        if problem.assignments.is_empty() && problem.pinned_lessons.is_empty() {
            return Err(
                "no lesson assignments to solve; configure class subjects with weekly hours"
                    .to_string(),
            );
        }

        let now = Utc::now().to_rfc3339();
        let snapshot = serde_json::to_string(&problem).map_err(|e| e.to_string())?;

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

        self.repo
            .insert_solve_job(job.clone())
            .await
            .map_err(|e| e.to_string())?;

        let scheduler = Arc::clone(&self.scheduler);
        let repo = Arc::clone(&self.repo);
        let open_ids: HashSet<String> =
            problem.assignments.iter().map(|a| a.id.clone()).collect();

        tokio::spawn(async move {
            if let Err(e) =
                Self::run_solve_job(scheduler, repo, job_id, problem, open_ids).await
            {
                error!(error = %e, "solve job failed");
            }
        });

        Ok(job.into())
    }

    pub async fn get_schedule_solve_job(
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

        if job.org_id != org_id {
            return Err("solve job not found".to_string());
        }

        Ok(job.into())
    }

    pub async fn list_schedule_solve_jobs(
        &self,
        org_id: &str,
        cycle_id: Option<&str>,
    ) -> Result<Vec<SolveJobResponse>, String> {
        let jobs = self
            .repo
            .get_solve_jobs(org_id, "timetable", cycle_id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(jobs.into_iter().map(Into::into).collect())
    }

    pub async fn cancel_schedule_solve(
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

        if job.org_id != org_id {
            return Err("solve job not found".to_string());
        }

        if matches!(job.status.as_str(), "completed" | "failed" | "cancelled") {
            return Ok(job.into());
        }

        let _ = self.scheduler.cancel(job_id).await;
        job.status = "cancelled".to_string();
        job.completed_at = Some(Utc::now().to_rfc3339());
        job.updated_at = Utc::now().to_rfc3339();

        let job = self.repo.update_solve_job(job).await.map_err(|e| e.to_string())?;
        Ok(job.into())
    }

    pub async fn apply_schedule_solve(
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

        if job.org_id != org_id {
            return Err("solve job not found".to_string());
        }
        if job.status != "completed" {
            return Err("solve job is not completed yet".to_string());
        }

        let solution_json = job
            .best_solution
            .as_ref()
            .ok_or_else(|| "no solution available to apply".to_string())?;

        let solution: StoredSolution =
            serde_json::from_str(solution_json).map_err(|e| format!("invalid solution: {e}"))?;

        if job.mode.as_deref() == Some("full") {
            self.repo
                .cancel_lessons_for_cycle(org_id, &job.target_id)
                .await
                .map_err(|e| e.to_string())?;
        }

        let mut created = 0i32;
        for assignment in &solution.assignments {
            let Some(placement) = LessonPlacement::from_solver_assignment(
                &job.target_id,
                assignment,
                &job.id,
            ) else {
                continue;
            };

            self.persist_lesson(org_id, placement, None).await?;
            created += 1;
        }

        job.items_created = created;
        job.updated_at = Utc::now().to_rfc3339();
        let job = self.repo.update_solve_job(job).await.map_err(|e| e.to_string())?;

        info!(job_id = %job.id, created, "applied timetable solution");
        Ok(job.into())
    }

    async fn build_timetable_problem(
        &self,
        org_id: &str,
        cycle_id: &str,
        mode: SolveMode,
    ) -> Result<TimetableProblem, String> {
        let slots = self
            .repo
            .get_time_slots(org_id, cycle_id)
            .await
            .map_err(|e| e.to_string())?;

        if slots.is_empty() {
            return Err("no time slots configured for this cycle".to_string());
        }

        let time_slots: Vec<TimeSlotDto> = slots
            .iter()
            .map(|s| TimeSlotDto {
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
            })
            .collect();

        let rooms = self.get_rooms_for_solver(org_id).await?;
        let unavailable_slots = self.load_teacher_unavailability(org_id, cycle_id).await?;

        let classes = self
            .repo
            .get_classes(
                org_id,
                crate::params::ClassParams {
                    q: None,
                    is_active: Some(1),
                },
            )
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .filter(|c| c.cycle_id == cycle_id)
            .collect::<Vec<_>>();

        let class_ids: Vec<String> = classes.iter().map(|c| c.id.clone()).collect();
        let class_subjects = self
            .repo
            .get_class_subjects_for_classes(&class_ids, cycle_id)
            .await
            .map_err(|e| e.to_string())?;

        let existing = self
            .repo
            .get_all_scheduled_lessons_for_cycle(org_id, cycle_id)
            .await
            .map_err(|e| e.to_string())?;

        let mut lessons_per_subject: std::collections::HashMap<String, u32> =
            std::collections::HashMap::new();
        for lesson in &existing {
            *lessons_per_subject
                .entry(lesson.class_subject_id.clone())
                .or_insert(0) += 1;
        }

        let pinned_lessons: Vec<PinnedLessonDto> = existing
            .iter()
            .map(|l| PinnedLessonDto {
                id: l.id.clone(),
                class_id: l.class_id.clone(),
                teacher_id: l.teacher_id.clone(),
                subject_id: l.subject_id.clone(),
                class_subject_id: l.class_subject_id.clone(),
                time_slot_id: l.time_slot_id.clone(),
                day_of_week: l.day_of_week,
                room_id: l.room_id.clone(),
            })
            .collect();

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

    async fn load_teacher_unavailability(
        &self,
        org_id: &str,
        cycle_id: &str,
    ) -> Result<Vec<TeacherUnavailableDto>, String> {
        let records = self
            .repo
            .get_teacher_unavailability(org_id, cycle_id)
            .await
            .map_err(|e| e.to_string())?;

        Ok(records
            .into_iter()
            .map(|r| TeacherUnavailableDto {
                teacher_id: r.teacher_id,
                day_of_week: r.day_of_week,
                time_slot_id: r.time_slot_id,
            })
            .collect())
    }

    async fn run_solve_job(
        scheduler: Arc<dyn ScheduleSolver>,
        repo: Arc<crate::repository::Repository>,
        job_id: String,
        mut problem: TimetableProblem,
        open_ids: HashSet<String>,
    ) -> Result<(), String> {
        problem.job_id = job_id.clone();

        let mut job = repo
            .get_solve_job_by_id(&job_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "job not found".to_string())?;

        job.status = "running".to_string();
        job.started_at = Some(Utc::now().to_rfc3339());
        job.updated_at = Utc::now().to_rfc3339();
        job = repo.update_solve_job(job).await.map_err(|e| e.to_string())?;

        if let Err(e) = scheduler.submit(&problem).await {
            job.status = "failed".to_string();
            job.error_message = Some(e.to_string());
            job.completed_at = Some(Utc::now().to_rfc3339());
            job.updated_at = Utc::now().to_rfc3339();
            repo.update_solve_job(job).await.map_err(|e| e.to_string())?;
            return Err(e.to_string());
        }

        for attempt in 0..MAX_POLL_ATTEMPTS {
            sleep(POLL_INTERVAL).await;

            if let Some(j) = repo
                .get_solve_job_by_id(&job_id)
                .await
                .map_err(|e| e.to_string())?
            {
                if j.status == "cancelled" {
                    let _ = scheduler.cancel(&job_id).await;
                    return Ok(());
                }
            }

            match scheduler.status(&job_id).await {
                Ok(status) => {
                    let mut job = repo
                        .get_solve_job_by_id(&job_id)
                        .await
                        .map_err(|e| e.to_string())?
                        .ok_or_else(|| "job not found".to_string())?;

                    job.score = status.score.clone();
                    job.updated_at = Utc::now().to_rfc3339();

                    if let Some(solution) = &status.best_solution {
                        let filtered =
                            filter_open_assignments(solution.assignments.clone(), &open_ids);
                        job.best_solution = serde_json::to_string(&StoredSolution {
                            assignments: filtered,
                        })
                        .ok();
                    }

                    match status.status {
                        JobStatus::Completed => {
                            job.status = "completed".to_string();
                            job.completed_at = Some(Utc::now().to_rfc3339());
                            repo.update_solve_job(job).await.map_err(|e| e.to_string())?;
                            info!(job_id = %job_id, "solve job completed");
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
                Err(e) => warn!(attempt, error = %e, "solver poll failed"),
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
                let _ = scheduler.cancel(&job_id).await;
                return Err("solve timed out".to_string());
            }
        }

        Ok(())
    }
}
