//! Tauri IPC commands — Scheduling (Rooms, TimeSlots, Lessons).

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use chrono::NaiveTime;
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct RoomDto {
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub room_type: String,
    pub capacity: i32,
}

#[derive(Deserialize)]
pub struct CreateRoomInput {
    pub name: String,
    pub capacity: i32,
    #[serde(rename = "roomType", alias = "room_type", default = "default_room_type")]
    pub room_type: String,
}

fn default_room_type() -> String {
    "classroom".to_string()
}

#[derive(Serialize)]
pub struct TimeSlotDto {
    pub id: String,
    pub period_id: String,
    pub day_of_week: i16,
    pub slot_index: i32,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Deserialize)]
pub struct CreateTimeSlotInput {
    #[serde(rename = "periodId")]
    pub period_id: String,
    #[serde(rename = "dayOfWeek")]
    pub day_of_week: i16,
    #[serde(rename = "slotIndex")]
    pub slot_index: i32,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
}

#[derive(Serialize)]
pub struct ScheduledLessonDto {
    pub id: String,
    pub class_subject_id: String,
    pub time_slot_id: String,
    pub room_id: Option<String>,
    pub teacher_id: Option<String>,
}

#[derive(Deserialize)]
pub struct ScheduleLessonInput {
    #[serde(rename = "classSubjectId")]
    pub class_subject_id: String,
    #[serde(rename = "timeSlotId")]
    pub time_slot_id: String,
    #[serde(rename = "roomId")]
    pub room_id: Option<String>,
    #[serde(rename = "teacherId")]
    pub teacher_id: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct LessonFilter {
    #[serde(rename = "classSubjectId")]
    pub class_subject_id: Option<String>,
    #[serde(rename = "periodId")]
    pub period_id: Option<String>,
}

#[tauri::command]
pub async fn scheduling_create_room(request: CreateRoomInput, state: State<'_, AppState>) -> Result<ApiResponse<RoomDto>, String> {
    let ctx = make_ctx(&state);
    let r = state.scheduling_service.create_room(&ctx, ctx.tenant_id, request.name, request.capacity, request.room_type)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(RoomDto {
        id: r.id.to_string(),
        org_id: r.org_id.to_string(),
        name: r.name,
        room_type: r.room_type,
        capacity: r.capacity,
    }))
}

#[tauri::command]
pub async fn scheduling_list_rooms(state: State<'_, AppState>) -> Result<ApiResponse<Vec<RoomDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.scheduling_service.list_rooms(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|r| RoomDto {
        id: r.id.to_string(),
        org_id: r.org_id.to_string(),
        name: r.name,
        room_type: r.room_type,
        capacity: r.capacity,
    }).collect()))
}

#[tauri::command]
pub async fn scheduling_get_room(id: String, state: State<'_, AppState>) -> Result<ApiResponse<RoomDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let r = state.scheduling_service.get_room(uid).await.map_err(|e| e.to_string())?.ok_or("Room not found")?;
    Ok(ok(RoomDto {
        id: r.id.to_string(),
        org_id: r.org_id.to_string(),
        name: r.name,
        room_type: r.room_type,
        capacity: r.capacity,
    }))
}

#[tauri::command]
pub async fn scheduling_create_time_slot(request: CreateTimeSlotInput, state: State<'_, AppState>) -> Result<ApiResponse<TimeSlotDto>, String> {
    let ctx = make_ctx(&state);
    let period_id = Uuid::parse_str(&request.period_id).map_err(|_| "Invalid periodId")?;
    let start_time = NaiveTime::parse_from_str(&request.start_time, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(&request.start_time, "%H:%M"))
        .map_err(|_| "Invalid start_time format (expected HH:MM:SS or HH:MM)")?;
    let end_time = NaiveTime::parse_from_str(&request.end_time, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(&request.end_time, "%H:%M"))
        .map_err(|_| "Invalid end_time format (expected HH:MM:SS or HH:MM)")?;

    let t = state.scheduling_service.create_time_slot(&ctx, period_id, request.day_of_week, start_time, end_time, request.slot_index)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(TimeSlotDto {
        id: t.id.to_string(),
        period_id: t.period_id.to_string(),
        day_of_week: t.day_of_week,
        slot_index: t.slot_index,
        start_time: t.start_time.to_string(),
        end_time: t.end_time.to_string(),
    }))
}

#[tauri::command]
pub async fn scheduling_list_time_slots(period_id: String, state: State<'_, AppState>) -> Result<ApiResponse<Vec<TimeSlotDto>>, String> {
    let pid = Uuid::parse_str(&period_id).map_err(|_| "Invalid period ID")?;
    let items = state.scheduling_service.list_time_slots(pid).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|t| TimeSlotDto {
        id: t.id.to_string(),
        period_id: t.period_id.to_string(),
        day_of_week: t.day_of_week,
        slot_index: t.slot_index,
        start_time: t.start_time.to_string(),
        end_time: t.end_time.to_string(),
    }).collect()))
}

#[tauri::command]
pub async fn scheduling_schedule_lesson(request: ScheduleLessonInput, state: State<'_, AppState>) -> Result<ApiResponse<ScheduledLessonDto>, String> {
    let ctx = make_ctx(&state);
    let class_subject_id = Uuid::parse_str(&request.class_subject_id).map_err(|_| "Invalid classSubjectId")?;
    let time_slot_id = Uuid::parse_str(&request.time_slot_id).map_err(|_| "Invalid timeSlotId")?;
    let room_id = request.room_id.and_then(|r| Uuid::parse_str(&r).ok());
    let teacher_id = request.teacher_id.and_then(|t| Uuid::parse_str(&t).ok());

    let l = state.scheduling_service.schedule_lesson(&ctx, class_subject_id, time_slot_id, room_id, teacher_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(ScheduledLessonDto {
        id: l.id.to_string(),
        class_subject_id: l.class_subject_id.to_string(),
        time_slot_id: l.time_slot_id.to_string(),
        room_id: l.room_id.map(|r| r.to_string()),
        teacher_id: l.teacher_id.map(|t| t.to_string()),
    }))
}

#[tauri::command]
pub async fn scheduling_list_lessons(filter: Option<LessonFilter>, state: State<'_, AppState>) -> Result<ApiResponse<Vec<ScheduledLessonDto>>, String> {
    let f = filter.unwrap_or_default();
    let items = if let Some(csid) = f.class_subject_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        state.scheduling_service.list_lessons_by_class_subject(csid).await
    } else if let Some(pid) = f.period_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        state.scheduling_service.list_lessons_by_period(pid).await
    } else {
        Ok(vec![])
    }.map_err(|e| e.to_string())?;

    Ok(ok(items.into_iter().map(|l| ScheduledLessonDto {
        id: l.id.to_string(),
        class_subject_id: l.class_subject_id.to_string(),
        time_slot_id: l.time_slot_id.to_string(),
        room_id: l.room_id.map(|r| r.to_string()),
        teacher_id: l.teacher_id.map(|t| t.to_string()),
    }).collect()))
}

// ═════════════════════════════════════════════════════════════════════════════
// ── Solver IPC Handlers (Timefold Sidecar Integration) ──────────────────────
// ═════════════════════════════════════════════════════════════════════════════

use orbis_app::ports::gateways::SchedulingSolverPort;
use orbis_domain::entities::solver::{
    ExamProblem, ExamSolveJobStatus, JobStatus, SolveJobStatus, TimetableProblem,
};
use crate::features::solver_sidecar::SidecarStatus;

#[tauri::command]
pub async fn solver_get_sidecar_status(state: State<'_, AppState>) -> Result<ApiResponse<SidecarStatus>, String> {
    let status = state.solver_sidecar.status().await;
    Ok(ok(status))
}

#[tauri::command]
pub async fn solver_stop_sidecar(state: State<'_, AppState>) -> Result<ApiResponse<SidecarStatus>, String> {
    state.solver_sidecar.stop().await;
    let status = state.solver_sidecar.status().await;
    Ok(ok(status))
}

#[tauri::command]
pub async fn solver_submit_timetable(
    problem: TimetableProblem,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let ctx = make_ctx(&state);
    state.solver_sidecar.ensure_running().await.map_err(|e| e.to_string())?;
    state.solver_sidecar.increment_active_jobs();

    state.integration_resolver
        .submit_timetable(&ctx, &problem)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(()))
}

#[tauri::command]
pub async fn solver_get_timetable_status(
    job_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<SolveJobStatus>, String> {
    let ctx = make_ctx(&state);
    state.solver_sidecar.mark_activity();

    let status = state.integration_resolver
        .get_timetable_status(&ctx, &job_id)
        .await
        .map_err(|e| e.to_string())?;

    if status.status == JobStatus::Completed || status.status == JobStatus::Failed || status.status == JobStatus::Cancelled {
        state.solver_sidecar.decrement_active_jobs();
    }

    Ok(ok(status))
}

#[tauri::command]
pub async fn solver_submit_exam(
    problem: ExamProblem,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let ctx = make_ctx(&state);
    state.solver_sidecar.ensure_running().await.map_err(|e| e.to_string())?;
    state.solver_sidecar.increment_active_jobs();

    state.integration_resolver
        .submit_exam(&ctx, &problem)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(()))
}

#[tauri::command]
pub async fn solver_get_exam_status(
    job_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<ExamSolveJobStatus>, String> {
    let ctx = make_ctx(&state);
    state.solver_sidecar.mark_activity();

    let status = state.integration_resolver
        .get_exam_status(&ctx, &job_id)
        .await
        .map_err(|e| e.to_string())?;

    if status.status == JobStatus::Completed || status.status == JobStatus::Failed || status.status == JobStatus::Cancelled {
        state.solver_sidecar.decrement_active_jobs();
    }

    Ok(ok(status))
}


