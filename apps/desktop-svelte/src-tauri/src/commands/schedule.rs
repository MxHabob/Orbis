use tauri::State;
use std::sync::Arc;
use core_domain::service::schedule::{
    CreateTimeSlotRequest, UpdateTimeSlotRequest,
    CreateClassSubjectRequest, UpdateClassSubjectRequest,
    CreateLessonRequest, UpdateLessonRequest, ScheduledLessonView,
};
use core_domain::service::room::{CreateRoomRequest, UpdateRoomRequest};
use crate::state::AppState;
use super::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn get_time_slots(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
) -> CmdResult<Vec<core_domain::entities::time_slot::Model>> {
    state.service.get_time_slots(&org_id, &cycle_id).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn create_time_slot(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateTimeSlotRequest,
) -> CmdResult<core_domain::entities::time_slot::Model> {
    state.service.create_time_slot(&org_id, request).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn update_time_slot(
    state: State<'_, Arc<AppState>>,
    id: String,
    request: UpdateTimeSlotRequest,
) -> CmdResult<core_domain::entities::time_slot::Model> {
    state.service.update_time_slot(&id, request).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn delete_time_slot(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<()> {
    state.service.delete_time_slot(&id).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn get_class_subjects(
    state: State<'_, Arc<AppState>>,
    class_id: Option<String>,
    cycle_id: Option<String>,
) -> CmdResult<Vec<core_domain::entities::class_subject::Model>> {
    state.service.get_class_subjects(
        class_id.as_deref(),
        cycle_id.as_deref(),
    ).await.map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn create_class_subject(
    state: State<'_, Arc<AppState>>,
    request: CreateClassSubjectRequest,
) -> CmdResult<core_domain::entities::class_subject::Model> {
    state.service.create_class_subject(request).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn update_class_subject(
    state: State<'_, Arc<AppState>>,
    id: String,
    request: UpdateClassSubjectRequest,
) -> CmdResult<core_domain::entities::class_subject::Model> {
    state.service.update_class_subject(&id, request).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn delete_class_subject(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<()> {
    state.service.delete_class_subject(&id).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn get_class_schedule(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
    class_id: String,
) -> CmdResult<Vec<ScheduledLessonView>> {
    state.service.get_class_schedule(&org_id, &cycle_id, &class_id).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn get_teacher_schedule(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
    teacher_id: String,
) -> CmdResult<Vec<ScheduledLessonView>> {
    state.service.get_teacher_schedule(&org_id, &cycle_id, &teacher_id).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn get_lesson_by_id(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<ScheduledLessonView> {
    state.service.get_lesson_by_id(&id).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn create_lesson(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateLessonRequest,
) -> CmdResult<ScheduledLessonView> {
    state.service.create_lesson(&org_id, request).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn update_lesson(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
    request: UpdateLessonRequest,
) -> CmdResult<ScheduledLessonView> {
    state.service.update_lesson(&org_id, &id, request).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn delete_lesson(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<()> {
    state.service.delete_lesson(&id).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn check_schedule_conflicts(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
    time_slot_id: String,
    day_of_week: i32,
    teacher_id: String,
    class_id: String,
    room_id: Option<String>,
    exclude_id: Option<String>,
) -> CmdResult<Vec<core_domain::service::schedule::ScheduleConflict>> {
    state.service.check_schedule_conflicts(
        &org_id, &cycle_id, &time_slot_id, day_of_week,
        exclude_id.as_deref(), &teacher_id, &class_id, room_id.as_deref(),
    ).await.map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn start_schedule_solve(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    user_id: String,
    req: core_domain::service::schedule_solver::StartSolveRequest,
) -> CmdResult<core_domain::service::schedule_solver::SolveJobResponse> {
    state.service.start_schedule_solve(&org_id, &user_id, req).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn get_schedule_solve_job(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    job_id: String,
) -> CmdResult<core_domain::service::schedule_solver::SolveJobResponse> {
    state.service.get_schedule_solve_job(&org_id, &job_id).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn list_schedule_solve_jobs(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: Option<String>,
) -> CmdResult<Vec<core_domain::service::schedule_solver::SolveJobResponse>> {
    state.service.list_schedule_solve_jobs(&org_id, cycle_id.as_deref()).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn cancel_schedule_solve(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    job_id: String,
) -> CmdResult<core_domain::service::schedule_solver::SolveJobResponse> {
    state.service.cancel_schedule_solve(&org_id, &job_id).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn apply_schedule_solve(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    job_id: String,
) -> CmdResult<core_domain::service::schedule_solver::SolveJobResponse> {
    state.service.apply_schedule_solve(&org_id, &job_id).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn get_rooms(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<core_domain::entities::room::Model>> {
    state.service.get_rooms(&org_id).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn create_room(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateRoomRequest,
) -> CmdResult<core_domain::entities::room::Model> {
    state.service.create_room(&org_id, request).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn update_room(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
    request: UpdateRoomRequest,
) -> CmdResult<core_domain::entities::room::Model> {
    state.service.update_room(&org_id, &id, request).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn delete_room(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
) -> CmdResult<()> {
    state.service.delete_room(&org_id, &id).await
        .map_err(|e| CommandError::new(e))
}

