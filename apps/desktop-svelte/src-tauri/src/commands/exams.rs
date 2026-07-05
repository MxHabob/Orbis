use tauri::State;
use std::sync::Arc;

use core_domain::{
    entities::{
        exam_session, exam_subject, exam_schedule, exam_hall,
        exam_hall_allocation, exam_proctor,
    },
    service::exam_session::{
        CreateExamSessionRequest, UpdateExamSessionRequest,
        CreateExamSubjectRequest, CreateExamHallRequest,
    },
    service::student_ordering::{OrderedStudent, RebuildOrderingResult},
    service::exam_numbers::{GenerationResult, SeatingNumberView},
    service::exam_auto_schedule::AutoScheduleRequest,
    service::schedule_solver::SolveJobResponse,
    service::exam_logistics::{
        AllocationConfig, AllocationResult, ProctorAssignConfig, ProctorAssignResult,
    },
};

use crate::state::AppState;
use super::error::{CmdResult, CommandError};

// ── Exam Sessions ──────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_exam_sessions(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: Option<String>,
) -> CmdResult<Vec<exam_session::Model>> {
    state.service().await
        .get_exam_sessions(&org_id, cycle_id.as_deref())
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_exam_session(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
) -> CmdResult<exam_session::Model> {
    state.service().await
        .get_exam_session(&org_id, &session_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn create_exam_session(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    user_id: String,
    request: CreateExamSessionRequest,
) -> CmdResult<exam_session::Model> {
    state.service().await
        .create_exam_session(&org_id, &user_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn update_exam_session(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
    request: UpdateExamSessionRequest,
) -> CmdResult<exam_session::Model> {
    state.service().await
        .update_exam_session(&org_id, &session_id, request)
        .await
        .map_err(CommandError::from)
}

// ── Exam Subjects ──────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_exam_subjects(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
) -> CmdResult<Vec<exam_subject::Model>> {
    state.service().await
        .get_exam_subjects(&org_id, &session_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn create_exam_subject(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateExamSubjectRequest,
) -> CmdResult<exam_subject::Model> {
    state.service().await
        .create_exam_subject(&org_id, request)
        .await
        .map_err(CommandError::from)
}

// ── Student Ordering ───────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_student_ordering(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
    class_id: Option<String>,
) -> CmdResult<Vec<OrderedStudent>> {
    state.service().await
        .get_ordered_students(&org_id, &cycle_id, class_id.as_deref())
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn rebuild_student_ordering(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
    strategy: String,
) -> CmdResult<RebuildOrderingResult> {
    state.service().await
        .rebuild_ordering(&org_id, &cycle_id, &strategy)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn freeze_student_ordering(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
    user_id: String,
) -> CmdResult<u64> {
    state.service().await
        .freeze_ordering(&org_id, &cycle_id, &user_id)
        .await
        .map_err(CommandError::from)
}

// ── Seating & Secret Numbers ───────────────────────────────────────────────

#[tauri::command]
pub async fn generate_seating_numbers(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
    user_id: String,
) -> CmdResult<GenerationResult> {
    state.service().await
        .generate_seating_numbers(&org_id, &session_id, &user_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn generate_secret_numbers(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
    user_id: String,
) -> CmdResult<GenerationResult> {
    state.service().await
        .generate_secret_numbers(&org_id, &session_id, &user_id)
        .await
        .map_err(CommandError::from)
}

// ── Exam Schedules ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_exam_schedules(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
) -> CmdResult<Vec<exam_schedule::Model>> {
    state.service().await
        .get_exam_schedules(&org_id, &session_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn start_exam_solve(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    user_id: String,
    request: AutoScheduleRequest,
) -> CmdResult<SolveJobResponse> {
    state.service().await
        .start_exam_solve(&org_id, &user_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_exam_solve_job(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    job_id: String,
) -> CmdResult<SolveJobResponse> {
    state.service().await
        .get_exam_solve_job(&org_id, &job_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn list_exam_solve_jobs(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    exam_session_id: Option<String>,
) -> CmdResult<Vec<SolveJobResponse>> {
    state.service().await
        .list_exam_solve_jobs(&org_id, exam_session_id.as_deref())
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn cancel_exam_solve(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    job_id: String,
) -> CmdResult<SolveJobResponse> {
    state.service().await
        .cancel_exam_solve(&org_id, &job_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn apply_exam_solve(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    job_id: String,
) -> CmdResult<SolveJobResponse> {
    state.service().await
        .apply_exam_solve(&org_id, &job_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn allocate_students_to_halls(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
    config: AllocationConfig,
) -> CmdResult<AllocationResult> {
    state.service().await
        .allocate_students_to_halls(&org_id, &session_id, config)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn auto_assign_proctors(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
    schedule_id: String,
    config: ProctorAssignConfig,
) -> CmdResult<ProctorAssignResult> {
    state.service().await
        .auto_assign_proctors(&org_id, &session_id, &schedule_id, config)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_exam_halls(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
) -> CmdResult<Vec<exam_hall::Model>> {
    state.service().await
        .get_exam_halls(&org_id, &session_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn create_exam_hall(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateExamHallRequest,
) -> CmdResult<exam_hall::Model> {
    state.service().await
        .create_exam_hall(&org_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_exam_proctors(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
    schedule_id: Option<String>,
) -> CmdResult<Vec<exam_proctor::Model>> {
    state.service().await
        .get_exam_proctors(&org_id, &session_id, schedule_id.as_deref())
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_hall_students(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
    hall_id: String,
) -> CmdResult<Vec<exam_hall_allocation::Model>> {
    state.service().await
        .get_hall_students(&org_id, &session_id, &hall_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_seating_numbers(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
    class_id: Option<String>,
) -> CmdResult<Vec<SeatingNumberView>> {
    state.service().await
        .get_seating_numbers(&org_id, &session_id, class_id.as_deref())
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn delete_exam_schedule(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    schedule_id: String,
) -> CmdResult<()> {
    state.service().await
        .delete_exam_schedule(&org_id, &schedule_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn update_exam_schedule_status(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    schedule_id: String,
    status: String,
) -> CmdResult<exam_schedule::Model> {
    state.service().await
        .update_exam_schedule_status(&org_id, &schedule_id, &status)
        .await
        .map_err(CommandError::from)
}
