//! Tauri IPC commands — Scheduling (Rooms, TimeSlots, Lessons).

use super::make_ctx;
use super::org_cmds::ApiResponse;
use crate::state::AppState;
use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse { data }
}

#[derive(Serialize)]
pub struct RoomDto {
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub room_type: String,
    pub capacity: i32,
    pub floor: Option<i32>,
    pub org_unit_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct CreateRoomInput {
    pub name: String,
    pub name_ar: Option<String>,
    pub capacity: i32,
    #[serde(default = "default_room_type")]
    pub room_type: String,
    pub floor: Option<i32>,
    pub org_unit_id: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateRoomInput {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub capacity: Option<i32>,
    pub room_type: Option<String>,
    pub floor: Option<i32>,
    pub org_unit_id: Option<String>,
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
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub is_break: bool,
    pub break_type: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct CreateTimeSlotInput {
    pub period_id: String,
    pub day_of_week: i16,
    pub slot_index: i32,
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub start_time: String,
    pub end_time: String,
    #[serde(default)]
    pub is_break: bool,
    pub break_type: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateTimeSlotItemInput {
    pub day_of_week: i16,
    pub slot_index: i32,
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub start_time: String,
    pub end_time: String,
    #[serde(default)]
    pub is_break: bool,
    pub break_type: Option<String>,
}

#[derive(Deserialize)]
pub struct BatchCreateTimeSlotsInput {
    pub period_id: String,
    pub slots: Vec<CreateTimeSlotItemInput>,
}

#[derive(Deserialize)]
pub struct UpdateTimeSlotInput {
    pub day_of_week: Option<i16>,
    pub slot_index: Option<i32>,
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub is_break: Option<bool>,
    pub break_type: Option<String>,
}

#[derive(Serialize)]
pub struct ScheduledLessonDto {
    pub id: String,
    pub section_subject_id: String,
    pub time_slot_id: String,
    pub room_id: Option<String>,
    pub teacher_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct ScheduleLessonInput {
    pub section_subject_id: String,
    pub time_slot_id: String,
    pub room_id: Option<String>,
    pub teacher_id: Option<String>,
}

#[derive(Deserialize)]
pub struct BulkScheduleLessonsInput {
    pub lessons: Vec<ScheduleLessonInput>,
}

#[derive(Deserialize, Default)]
pub struct LessonFilter {
    pub section_subject_id: Option<String>,
    pub period_id: Option<String>,
    pub class_id: Option<String>,
    pub teacher_id: Option<String>,
}

// ── Helpers ─────────────────────────────────────────────────────

fn room_dto(r: orbis_domain::entities::room::Room) -> RoomDto {
    RoomDto {
        id: r.id.to_string(),
        org_id: r.org_id.to_string(),
        name: r.name,
        name_ar: r.name_ar,
        room_type: r.room_type,
        capacity: r.capacity,
        floor: r.floor,
        org_unit_id: r.org_unit_id.map(|u| u.to_string()),
        created_at: r.created_at.to_rfc3339(),
        updated_at: r.updated_at.to_rfc3339(),
    }
}

fn time_slot_dto(t: orbis_domain::entities::time_slot::TimeSlot) -> TimeSlotDto {
    TimeSlotDto {
        id: t.id.to_string(),
        period_id: t.period_id.to_string(),
        day_of_week: t.day_of_week,
        slot_index: t.slot_index,
        name: t.name,
        name_ar: t.name_ar,
        start_time: t.start_time.to_string(),
        end_time: t.end_time.to_string(),
        is_break: t.is_break,
        break_type: t.break_type,
        created_at: t.created_at.to_rfc3339(),
        updated_at: t.updated_at.to_rfc3339(),
    }
}

fn lesson_dto(l: orbis_domain::entities::scheduled_lesson::ScheduledLesson) -> ScheduledLessonDto {
    ScheduledLessonDto {
        id: l.id.to_string(),
        section_subject_id: l.section_subject_id.to_string(),
        time_slot_id: l.time_slot_id.to_string(),
        room_id: l.room_id.map(|r| r.to_string()),
        teacher_id: l.teacher_id.map(|t| t.to_string()),
        created_at: l.created_at.to_rfc3339(),
        updated_at: l.updated_at.to_rfc3339(),
    }
}

#[tauri::command]
pub async fn scheduling_create_room(
    request: CreateRoomInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<RoomDto>, String> {
    let ctx = make_ctx(&state);
    let org_unit_id = request.org_unit_id.and_then(|u| Uuid::parse_str(&u).ok());
    let r = state
        .scheduling_service
        .create_room(
            &ctx,
            ctx.tenant_id,
            request.name,
            request.name_ar,
            request.capacity,
            request.room_type,
            request.floor,
            org_unit_id,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(room_dto(r)))
}

#[tauri::command]
pub async fn scheduling_update_room(
    id: String,
    request: UpdateRoomInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<RoomDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid room ID")?;
    let org_unit_id = request.org_unit_id.map(|u| Uuid::parse_str(&u).ok());
    let r = state
        .scheduling_service
        .update_room(
            &ctx,
            uid,
            request.name,
            request.name_ar,
            request.capacity,
            request.room_type,
            request.floor,
            org_unit_id,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(room_dto(r)))
}

#[tauri::command]
pub async fn scheduling_delete_room(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<String>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid room ID")?;
    state
        .scheduling_service
        .delete_room(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok("Room deleted successfully".to_string()))
}

#[tauri::command]
pub async fn scheduling_list_rooms(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<RoomDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state
        .scheduling_service
        .list_rooms(ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(room_dto).collect()))
}

#[tauri::command]
pub async fn scheduling_get_room(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<RoomDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let r = state
        .scheduling_service
        .get_room(uid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Room not found")?;
    Ok(ok(room_dto(r)))
}

#[tauri::command]
pub async fn scheduling_create_time_slot(
    request: CreateTimeSlotInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<TimeSlotDto>, String> {
    let ctx = make_ctx(&state);
    let period_id = Uuid::parse_str(&request.period_id).map_err(|_| "Invalid periodId")?;
    let start_time = NaiveTime::parse_from_str(&request.start_time, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(&request.start_time, "%H:%M"))
        .map_err(|_| "Invalid start_time format (expected HH:MM:SS or HH:MM)")?;
    let end_time = NaiveTime::parse_from_str(&request.end_time, "%H:%M:%S")
        .or_else(|_| NaiveTime::parse_from_str(&request.end_time, "%H:%M"))
        .map_err(|_| "Invalid end_time format (expected HH:MM:SS or HH:MM)")?;

    let t = state
        .scheduling_service
        .create_time_slot(
            &ctx,
            period_id,
            request.day_of_week,
            request.slot_index,
            request.name,
            request.name_ar,
            start_time,
            end_time,
            request.is_break,
            request.break_type,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(time_slot_dto(t)))
}

#[tauri::command]
pub async fn scheduling_batch_create_time_slots(
    request: BatchCreateTimeSlotsInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<TimeSlotDto>>, String> {
    let ctx = make_ctx(&state);
    let period_id = Uuid::parse_str(&request.period_id).map_err(|_| "Invalid periodId")?;
    let mut items = Vec::with_capacity(request.slots.len());
    for s in request.slots {
        let start_time = NaiveTime::parse_from_str(&s.start_time, "%H:%M:%S")
            .or_else(|_| NaiveTime::parse_from_str(&s.start_time, "%H:%M"))
            .map_err(|_| "Invalid start_time format")?;
        let end_time = NaiveTime::parse_from_str(&s.end_time, "%H:%M:%S")
            .or_else(|_| NaiveTime::parse_from_str(&s.end_time, "%H:%M"))
            .map_err(|_| "Invalid end_time format")?;
        items.push((
            s.day_of_week,
            s.slot_index,
            s.name,
            s.name_ar,
            start_time,
            end_time,
            s.is_break,
            s.break_type,
        ));
    }

    let slots = state
        .scheduling_service
        .batch_create_time_slots(&ctx, period_id, items)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(slots.into_iter().map(time_slot_dto).collect()))
}

#[tauri::command]
pub async fn scheduling_get_time_slot(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<TimeSlotDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid time slot ID")?;
    let t = state
        .scheduling_service
        .get_time_slot(uid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Time slot not found")?;
    Ok(ok(time_slot_dto(t)))
}

#[tauri::command]
pub async fn scheduling_update_time_slot(
    id: String,
    request: UpdateTimeSlotInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<TimeSlotDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid time slot ID")?;
    let start_time = if let Some(st) = request.start_time {
        Some(
            NaiveTime::parse_from_str(&st, "%H:%M:%S")
                .or_else(|_| NaiveTime::parse_from_str(&st, "%H:%M"))
                .map_err(|_| "Invalid start_time format")?,
        )
    } else {
        None
    };
    let end_time = if let Some(et) = request.end_time {
        Some(
            NaiveTime::parse_from_str(&et, "%H:%M:%S")
                .or_else(|_| NaiveTime::parse_from_str(&et, "%H:%M"))
                .map_err(|_| "Invalid end_time format")?,
        )
    } else {
        None
    };

    let t = state
        .scheduling_service
        .update_time_slot(
            &ctx,
            uid,
            request.day_of_week,
            request.slot_index,
            request.name.map(Some),
            request.name_ar.map(Some),
            start_time,
            end_time,
            request.is_break,
            request.break_type.map(Some),
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(time_slot_dto(t)))
}

#[tauri::command]
pub async fn scheduling_delete_time_slot(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<String>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid time slot ID")?;
    state
        .scheduling_service
        .delete_time_slot(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok("Time slot deleted successfully".to_string()))
}

#[tauri::command]
pub async fn scheduling_list_time_slots(
    period_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<TimeSlotDto>>, String> {
    let pid = Uuid::parse_str(&period_id).map_err(|_| "Invalid period ID")?;
    let items = state
        .scheduling_service
        .list_time_slots(pid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(time_slot_dto).collect()))
}

#[tauri::command]
pub async fn scheduling_schedule_lesson(
    request: ScheduleLessonInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<ScheduledLessonDto>, String> {
    let ctx = make_ctx(&state);
    let section_subject_id =
        Uuid::parse_str(&request.section_subject_id).map_err(|_| "Invalid SectionSubjectId")?;
    let time_slot_id = Uuid::parse_str(&request.time_slot_id).map_err(|_| "Invalid timeSlotId")?;
    let room_id = request.room_id.and_then(|r| Uuid::parse_str(&r).ok());
    let teacher_id = request.teacher_id.and_then(|t| Uuid::parse_str(&t).ok());

    let l = state
        .scheduling_service
        .schedule_lesson(&ctx, section_subject_id, time_slot_id, room_id, teacher_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(lesson_dto(l)))
}

#[tauri::command]
pub async fn scheduling_bulk_schedule_lessons(
    request: BulkScheduleLessonsInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<ScheduledLessonDto>>, String> {
    let ctx = make_ctx(&state);
    let mut items = Vec::with_capacity(request.lessons.len());
    for l in request.lessons {
        let section_subject_id =
            Uuid::parse_str(&l.section_subject_id).map_err(|_| "Invalid SectionSubjectId")?;
        let time_slot_id = Uuid::parse_str(&l.time_slot_id).map_err(|_| "Invalid timeSlotId")?;
        let room_id = l.room_id.and_then(|r| Uuid::parse_str(&r).ok());
        let teacher_id = l.teacher_id.and_then(|t| Uuid::parse_str(&t).ok());
        items.push((section_subject_id, time_slot_id, room_id, teacher_id));
    }

    let lessons = state
        .scheduling_service
        .bulk_schedule_lessons(&ctx, items)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(lessons.into_iter().map(lesson_dto).collect()))
}

#[tauri::command]
pub async fn scheduling_delete_scheduled_lesson(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<String>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid scheduled lesson ID")?;
    state
        .scheduling_service
        .delete_scheduled_lesson(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok("Scheduled lesson deleted successfully".to_string()))
}

#[tauri::command]
pub async fn scheduling_list_lessons(
    filter: Option<LessonFilter>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<ScheduledLessonDto>>, String> {
    let f = filter.unwrap_or_default();
    let items = if let Some(csid) = f
        .section_subject_id
        .as_deref()
        .and_then(|s| Uuid::parse_str(s).ok())
    {
        state
            .scheduling_service
            .list_lessons_by_section_subject(csid)
            .await
    } else if let Some(cid) = f.class_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        state.scheduling_service.list_lessons_by_class(cid).await
    } else if let Some(tid) = f
        .teacher_id
        .as_deref()
        .and_then(|s| Uuid::parse_str(s).ok())
    {
        state.scheduling_service.list_lessons_by_teacher(tid).await
    } else if let Some(pid) = f.period_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        state.scheduling_service.list_lessons_by_period(pid).await
    } else {
        Ok(vec![])
    }
    .map_err(|e| e.to_string())?;

    Ok(ok(items.into_iter().map(lesson_dto).collect()))
}

// ═════════════════════════════════════════════════════════════════════════════
// ── Solver IPC Handlers (Timefold Sidecar Integration) ──────────────────────
// ═════════════════════════════════════════════════════════════════════════════

use crate::features::solver_sidecar::SidecarStatus;
use orbis_app::ports::gateways::SchedulingSolverPort;
use orbis_domain::entities::solver::{
    ExamProblem, ExamSolveJobStatus, JobStatus, SolveJobStatus, TimetableProblem,
};

#[tauri::command]
pub async fn solver_get_sidecar_status(
    state: State<'_, AppState>,
) -> Result<ApiResponse<SidecarStatus>, String> {
    let status = state.solver_sidecar.status().await;
    Ok(ok(status))
}

#[tauri::command]
pub async fn solver_stop_sidecar(
    state: State<'_, AppState>,
) -> Result<ApiResponse<SidecarStatus>, String> {
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
    state
        .solver_sidecar
        .ensure_running()
        .await
        .map_err(|e| e.to_string())?;
    state.solver_sidecar.increment_active_jobs();

    if let Err(error) = state
        .integration_resolver
        .submit_timetable(&ctx, &problem)
        .await
    {
        state.solver_sidecar.decrement_active_jobs();
        return Err(error.to_string());
    }

    Ok(ok(()))
}

#[tauri::command]
pub async fn solver_get_timetable_status(
    job_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<SolveJobStatus>, String> {
    let ctx = make_ctx(&state);
    state.solver_sidecar.mark_activity();

    let status = state
        .integration_resolver
        .get_timetable_status(&ctx, &job_id)
        .await
        .map_err(|e| e.to_string())?;

    if status.status == JobStatus::Completed
        || status.status == JobStatus::Failed
        || status.status == JobStatus::Cancelled
    {
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
    state
        .solver_sidecar
        .ensure_running()
        .await
        .map_err(|e| e.to_string())?;
    state.solver_sidecar.increment_active_jobs();

    state
        .integration_resolver
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

    let status = state
        .integration_resolver
        .get_exam_status(&ctx, &job_id)
        .await
        .map_err(|e| e.to_string())?;

    if status.status == JobStatus::Completed
        || status.status == JobStatus::Failed
        || status.status == JobStatus::Cancelled
    {
        state.solver_sidecar.decrement_active_jobs();
    }

    Ok(ok(status))
}
