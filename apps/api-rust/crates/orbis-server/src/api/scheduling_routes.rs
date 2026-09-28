//! Scheduling routes — Rooms, TimeSlots, ScheduledLessons.

use axum::{extract::{State, Path, Query}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;

#[derive(Serialize, ToSchema)]
pub struct RoomDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub room_type: String,
    pub capacity: i32,
    pub org_unit_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateRoomRequest {
    pub name: String,
    pub room_type: String,
    pub capacity: i32,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TimeSlotDto {
    pub id: Uuid,
    pub period_id: Uuid,
    pub day_of_week: i16,
    pub slot_index: i32,
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub start_time: chrono::NaiveTime,
    pub end_time: chrono::NaiveTime,
    pub is_break: bool,
    pub break_type: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateTimeSlotRequest {
    pub period_id: Uuid,
    pub day_of_week: i16,
    pub slot_index: i32,
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub start_time: chrono::NaiveTime,
    pub end_time: chrono::NaiveTime,
    #[serde(default)]
    pub is_break: bool,
    pub break_type: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct ScheduledLessonDto {
    pub id: Uuid,
    pub class_subject_id: Uuid,
    pub time_slot_id: Uuid,
    pub room_id: Option<Uuid>,
    pub teacher_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct ScheduleLessonRequest {
    pub class_subject_id: Uuid,
    pub time_slot_id: Uuid,
    pub room_id: Option<Uuid>,
    pub teacher_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct TimeSlotQuery { pub period_id: Option<Uuid> }

#[derive(Deserialize)]
pub struct LessonQuery { pub class_subject_id: Option<Uuid>, pub period_id: Option<Uuid> }

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/rooms", request_body = CreateRoomRequest,
    responses((status = 201, body = RoomDto)), security(("bearer" = [])), tag = "Scheduling")]
async fn create_room(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<CreateRoomRequest>,
) -> Result<Json<ApiResponse<RoomDto>>, ApiError> {
    let r = state.scheduling_service.create_room(&ctx, ctx.tenant_id, p.name, p.capacity, p.room_type).await?;
    Ok(ApiResponse::ok(RoomDto { id: r.id, org_id: r.org_id, name: r.name, room_type: r.room_type, capacity: r.capacity, org_unit_id: r.org_unit_id, created_at: r.created_at, updated_at: r.updated_at }))
}

#[utoipa::path(get, path = "/api/v1/rooms", responses((status = 200, body = Vec<RoomDto>)),
    security(("bearer" = [])), tag = "Scheduling")]
async fn list_rooms(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<RoomDto>>>, ApiError> {
    let rooms = state.scheduling_service.list_rooms(ctx.tenant_id).await?;
    let dtos = rooms.into_iter().map(|r| RoomDto { id: r.id, org_id: r.org_id, name: r.name, room_type: r.room_type, capacity: r.capacity, org_unit_id: r.org_unit_id, created_at: r.created_at, updated_at: r.updated_at }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/rooms/{id}", params(("id" = Uuid, Path, description = "Room ID")),
    responses((status = 200, body = RoomDto)), security(("bearer" = [])), tag = "Scheduling")]
async fn get_room(
    _ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<RoomDto>>, ApiError> {
    let r = state.scheduling_service.get_room(id).await?.ok_or_else(|| ApiError::not_found("Room not found"))?;
    Ok(ApiResponse::ok(RoomDto { id: r.id, org_id: r.org_id, name: r.name, room_type: r.room_type, capacity: r.capacity, org_unit_id: r.org_unit_id, created_at: r.created_at, updated_at: r.updated_at }))
}

#[utoipa::path(post, path = "/api/v1/time-slots", request_body = CreateTimeSlotRequest,
    responses((status = 201, body = TimeSlotDto)), security(("bearer" = [])), tag = "Scheduling")]
async fn create_time_slot(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<CreateTimeSlotRequest>,
) -> Result<Json<ApiResponse<TimeSlotDto>>, ApiError> {
    let t = state.scheduling_service.create_time_slot(
        &ctx, p.period_id, p.day_of_week, p.slot_index, p.name, p.name_ar, p.start_time, p.end_time, p.is_break, p.break_type,
    ).await?;
    Ok(ApiResponse::ok(TimeSlotDto {
        id: t.id,
        period_id: t.period_id,
        day_of_week: t.day_of_week,
        slot_index: t.slot_index,
        name: t.name,
        name_ar: t.name_ar,
        start_time: t.start_time,
        end_time: t.end_time,
        is_break: t.is_break,
        break_type: t.break_type,
        created_at: t.created_at,
        updated_at: t.updated_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/time-slots", responses((status = 200, body = Vec<TimeSlotDto>)),
    security(("bearer" = [])), tag = "Scheduling")]
async fn list_time_slots(
    _ctx: AuthContext, State(state): State<AppState>, Query(q): Query<TimeSlotQuery>,
) -> Result<Json<ApiResponse<Vec<TimeSlotDto>>>, ApiError> {
    let slots = match q.period_id {
        Some(pid) => state.scheduling_service.list_time_slots(pid).await?,
        None => vec![],
    };
    let dtos = slots.into_iter().map(|t| TimeSlotDto {
        id: t.id,
        period_id: t.period_id,
        day_of_week: t.day_of_week,
        slot_index: t.slot_index,
        name: t.name,
        name_ar: t.name_ar,
        start_time: t.start_time,
        end_time: t.end_time,
        is_break: t.is_break,
        break_type: t.break_type,
        created_at: t.created_at,
        updated_at: t.updated_at,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/scheduled-lessons", request_body = ScheduleLessonRequest,
    responses((status = 201, body = ScheduledLessonDto)), security(("bearer" = [])), tag = "Scheduling")]
async fn schedule_lesson(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<ScheduleLessonRequest>,
) -> Result<Json<ApiResponse<ScheduledLessonDto>>, ApiError> {
    let l = state.scheduling_service.schedule_lesson(&ctx, p.class_subject_id, p.time_slot_id, p.room_id, p.teacher_id).await?;
    Ok(ApiResponse::ok(ScheduledLessonDto { id: l.id, class_subject_id: l.class_subject_id, time_slot_id: l.time_slot_id, room_id: l.room_id, teacher_id: l.teacher_id, created_at: l.created_at, updated_at: l.updated_at }))
}

#[utoipa::path(get, path = "/api/v1/scheduled-lessons", responses((status = 200, body = Vec<ScheduledLessonDto>)),
    security(("bearer" = [])), tag = "Scheduling")]
async fn list_lessons(
    _ctx: AuthContext, State(state): State<AppState>, Query(q): Query<LessonQuery>,
) -> Result<Json<ApiResponse<Vec<ScheduledLessonDto>>>, ApiError> {
    let lessons = if let Some(csid) = q.class_subject_id {
        state.scheduling_service.list_lessons_by_class_subject(csid).await?
    } else if let Some(pid) = q.period_id {
        state.scheduling_service.list_lessons_by_period(pid).await?
    } else { vec![] };
    let dtos = lessons.into_iter().map(|l| ScheduledLessonDto { id: l.id, class_subject_id: l.class_subject_id, time_slot_id: l.time_slot_id, room_id: l.room_id, teacher_id: l.teacher_id, created_at: l.created_at, updated_at: l.updated_at }).collect();
    Ok(ApiResponse::ok(dtos))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/rooms", post(create_room).get(list_rooms))
        .route("/api/v1/rooms/:id", get(get_room))
        .route("/api/v1/time-slots", post(create_time_slot).get(list_time_slots))
        .route("/api/v1/scheduled-lessons", post(schedule_lesson).get(list_lessons))
}
