//! Scheduling routes — Rooms, TimeSlots, ScheduledLessons.

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;

#[derive(Serialize, ToSchema)]
pub struct RoomDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub room_type: String,
    pub capacity: i32,
    pub floor: Option<i32>,
    pub org_unit_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateRoomRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub room_type: String,
    pub capacity: i32,
    pub floor: Option<i32>,
    pub org_unit_id: Option<Uuid>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateRoomRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub room_type: Option<String>,
    pub capacity: Option<i32>,
    pub floor: Option<i32>,
    pub org_unit_id: Option<Uuid>,
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

#[derive(Deserialize, ToSchema)]
pub struct CreateTimeSlotItemRequest {
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

#[derive(Deserialize, ToSchema)]
pub struct BatchCreateTimeSlotsRequest {
    pub period_id: Uuid,
    pub slots: Vec<CreateTimeSlotItemRequest>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateTimeSlotRequest {
    pub day_of_week: Option<i16>,
    pub slot_index: Option<i32>,
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub start_time: Option<chrono::NaiveTime>,
    pub end_time: Option<chrono::NaiveTime>,
    pub is_break: Option<bool>,
    pub break_type: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct ScheduledLessonDto {
    pub id: Uuid,
    pub section_subject_id: Uuid,
    pub time_slot_id: Uuid,
    pub room_id: Option<Uuid>,
    pub teacher_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct ScheduleLessonRequest {
    pub section_subject_id: Uuid,
    pub time_slot_id: Uuid,
    pub room_id: Option<Uuid>,
    pub teacher_id: Option<Uuid>,
}

#[derive(Deserialize, ToSchema)]
pub struct BulkScheduleLessonsRequest {
    pub lessons: Vec<ScheduleLessonRequest>,
}

#[derive(Deserialize)]
pub struct TimeSlotQuery {
    pub period_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct LessonQuery {
    pub section_subject_id: Option<Uuid>,
    pub period_id: Option<Uuid>,
    pub section_id: Option<Uuid>,
    pub teacher_id: Option<Uuid>,
}

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/rooms", request_body = CreateRoomRequest,
    responses((status = 201, body = RoomDto)), security(("bearer" = [])), tag = "Scheduling")]
async fn create_room(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<CreateRoomRequest>,
) -> Result<Json<ApiResponse<RoomDto>>, ApiError> {
    let r = state
        .scheduling_service
        .create_room(
            &ctx,
            ctx.tenant_id,
            p.name,
            p.name_ar,
            p.capacity,
            p.room_type,
            p.floor,
            p.org_unit_id,
        )
        .await?;
    Ok(ApiResponse::ok(RoomDto {
        id: r.id,
        org_id: r.org_id,
        name: r.name,
        name_ar: r.name_ar,
        room_type: r.room_type,
        capacity: r.capacity,
        floor: r.floor,
        org_unit_id: r.org_unit_id,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/rooms", responses((status = 200, body = Vec<RoomDto>)),
    security(("bearer" = [])), tag = "Scheduling")]
async fn list_rooms(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<RoomDto>>>, ApiError> {
    let rooms = state.scheduling_service.list_rooms(ctx.tenant_id).await?;
    let dtos = rooms
        .into_iter()
        .map(|r| RoomDto {
            id: r.id,
            org_id: r.org_id,
            name: r.name,
            name_ar: r.name_ar,
            room_type: r.room_type,
            capacity: r.capacity,
            floor: r.floor,
            org_unit_id: r.org_unit_id,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/rooms/{id}", params(("id" = Uuid, Path, description = "Room ID")),
    responses((status = 200, body = RoomDto)), security(("bearer" = [])), tag = "Scheduling")]
async fn get_room(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<RoomDto>>, ApiError> {
    let r = state
        .scheduling_service
        .get_room(id)
        .await?
        .ok_or_else(|| ApiError::not_found("Room not found"))?;
    Ok(ApiResponse::ok(RoomDto {
        id: r.id,
        org_id: r.org_id,
        name: r.name,
        name_ar: r.name_ar,
        room_type: r.room_type,
        capacity: r.capacity,
        floor: r.floor,
        org_unit_id: r.org_unit_id,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

#[utoipa::path(patch, path = "/api/v1/rooms/{id}", params(("id" = Uuid, Path, description = "Room ID")),
    request_body = UpdateRoomRequest, responses((status = 200, body = RoomDto)), security(("bearer" = [])), tag = "Scheduling")]
async fn update_room(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateRoomRequest>,
) -> Result<Json<ApiResponse<RoomDto>>, ApiError> {
    let r = state
        .scheduling_service
        .update_room(
            &ctx,
            id,
            p.name,
            p.name_ar,
            p.capacity,
            p.room_type,
            p.floor,
            p.org_unit_id.map(Some),
        )
        .await?;
    Ok(ApiResponse::ok(RoomDto {
        id: r.id,
        org_id: r.org_id,
        name: r.name,
        name_ar: r.name_ar,
        room_type: r.room_type,
        capacity: r.capacity,
        floor: r.floor,
        org_unit_id: r.org_unit_id,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

#[utoipa::path(delete, path = "/api/v1/rooms/{id}", params(("id" = Uuid, Path, description = "Room ID")),
    responses((status = 200, body = String)), security(("bearer" = [])), tag = "Scheduling")]
async fn delete_room(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    state.scheduling_service.delete_room(&ctx, id).await?;
    Ok(ApiResponse::ok("Room deleted successfully".to_string()))
}

#[utoipa::path(post, path = "/api/v1/time-slots", request_body = CreateTimeSlotRequest,
    responses((status = 201, body = TimeSlotDto)), security(("bearer" = [])), tag = "Scheduling")]
async fn create_time_slot(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<CreateTimeSlotRequest>,
) -> Result<Json<ApiResponse<TimeSlotDto>>, ApiError> {
    let t = state
        .scheduling_service
        .create_time_slot(
            &ctx,
            p.period_id,
            p.day_of_week,
            p.slot_index,
            p.name,
            p.name_ar,
            p.start_time,
            p.end_time,
            p.is_break,
            p.break_type,
        )
        .await?;
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

#[utoipa::path(post, path = "/api/v1/time-slots/batch", request_body = BatchCreateTimeSlotsRequest,
    responses((status = 201, body = Vec<TimeSlotDto>)), security(("bearer" = [])), tag = "Scheduling")]
async fn batch_create_time_slots(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<BatchCreateTimeSlotsRequest>,
) -> Result<Json<ApiResponse<Vec<TimeSlotDto>>>, ApiError> {
    let items = p
        .slots
        .into_iter()
        .map(|s| {
            (
                s.day_of_week,
                s.slot_index,
                s.name,
                s.name_ar,
                s.start_time,
                s.end_time,
                s.is_break,
                s.break_type,
            )
        })
        .collect();
    let slots = state
        .scheduling_service
        .batch_create_time_slots(&ctx, p.period_id, items)
        .await?;
    let dtos = slots
        .into_iter()
        .map(|t| TimeSlotDto {
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
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/time-slots", responses((status = 200, body = Vec<TimeSlotDto>)),
    security(("bearer" = [])), tag = "Scheduling")]
async fn list_time_slots(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Query(q): Query<TimeSlotQuery>,
) -> Result<Json<ApiResponse<Vec<TimeSlotDto>>>, ApiError> {
    let slots = match q.period_id {
        Some(pid) => state.scheduling_service.list_time_slots(pid).await?,
        None => vec![],
    };
    let dtos = slots
        .into_iter()
        .map(|t| TimeSlotDto {
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
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/time-slots/{id}", params(("id" = Uuid, Path, description = "TimeSlot ID")),
    responses((status = 200, body = TimeSlotDto)), security(("bearer" = [])), tag = "Scheduling")]
async fn get_time_slot(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<TimeSlotDto>>, ApiError> {
    let t = state
        .scheduling_service
        .get_time_slot(id)
        .await?
        .ok_or_else(|| ApiError::not_found("Time slot not found"))?;
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

#[utoipa::path(patch, path = "/api/v1/time-slots/{id}", params(("id" = Uuid, Path, description = "TimeSlot ID")),
    request_body = UpdateTimeSlotRequest, responses((status = 200, body = TimeSlotDto)), security(("bearer" = [])), tag = "Scheduling")]
async fn update_time_slot(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateTimeSlotRequest>,
) -> Result<Json<ApiResponse<TimeSlotDto>>, ApiError> {
    let t = state
        .scheduling_service
        .update_time_slot(
            &ctx,
            id,
            p.day_of_week,
            p.slot_index,
            p.name.map(Some),
            p.name_ar.map(Some),
            p.start_time,
            p.end_time,
            p.is_break,
            p.break_type.map(Some),
        )
        .await?;
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

#[utoipa::path(delete, path = "/api/v1/time-slots/{id}", params(("id" = Uuid, Path, description = "TimeSlot ID")),
    responses((status = 200, body = String)), security(("bearer" = [])), tag = "Scheduling")]
async fn delete_time_slot(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    state.scheduling_service.delete_time_slot(&ctx, id).await?;
    Ok(ApiResponse::ok(
        "Time slot deleted successfully".to_string(),
    ))
}

#[utoipa::path(post, path = "/api/v1/scheduled-lessons", request_body = ScheduleLessonRequest,
    responses((status = 201, body = ScheduledLessonDto)), security(("bearer" = [])), tag = "Scheduling")]
async fn schedule_lesson(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<ScheduleLessonRequest>,
) -> Result<Json<ApiResponse<ScheduledLessonDto>>, ApiError> {
    let l = state
        .scheduling_service
        .schedule_lesson(
            &ctx,
            p.section_subject_id,
            p.time_slot_id,
            p.room_id,
            p.teacher_id,
        )
        .await?;
    Ok(ApiResponse::ok(ScheduledLessonDto {
        id: l.id,
        section_subject_id: l.section_subject_id,
        time_slot_id: l.time_slot_id,
        room_id: l.room_id,
        teacher_id: l.teacher_id,
        created_at: l.created_at,
        updated_at: l.updated_at,
    }))
}

#[utoipa::path(post, path = "/api/v1/scheduled-lessons/bulk", request_body = BulkScheduleLessonsRequest,
    responses((status = 201, body = Vec<ScheduledLessonDto>)), security(("bearer" = [])), tag = "Scheduling")]
async fn bulk_schedule_lessons(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<BulkScheduleLessonsRequest>,
) -> Result<Json<ApiResponse<Vec<ScheduledLessonDto>>>, ApiError> {
    let items = p
        .lessons
        .into_iter()
        .map(|l| (l.section_subject_id, l.time_slot_id, l.room_id, l.teacher_id))
        .collect();
    let lessons = state
        .scheduling_service
        .bulk_schedule_lessons(&ctx, items)
        .await?;
    let dtos = lessons
        .into_iter()
        .map(|l| ScheduledLessonDto {
            id: l.id,
            section_subject_id: l.section_subject_id,
            time_slot_id: l.time_slot_id,
            room_id: l.room_id,
            teacher_id: l.teacher_id,
            created_at: l.created_at,
            updated_at: l.updated_at,
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/scheduled-lessons", responses((status = 200, body = Vec<ScheduledLessonDto>)),
    security(("bearer" = [])), tag = "Scheduling")]
async fn list_lessons(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Query(q): Query<LessonQuery>,
) -> Result<Json<ApiResponse<Vec<ScheduledLessonDto>>>, ApiError> {
    let lessons = if let Some(csid) = q.section_subject_id {
        state
            .scheduling_service
            .list_lessons_by_section_subject(csid)
            .await?
    } else if let Some(cid) = q.section_id {
        state.scheduling_service.list_lessons_by_class(cid).await?
    } else if let Some(tid) = q.teacher_id {
        state
            .scheduling_service
            .list_lessons_by_teacher(tid)
            .await?
    } else if let Some(pid) = q.period_id {
        state.scheduling_service.list_lessons_by_period(pid).await?
    } else {
        vec![]
    };
    let dtos = lessons
        .into_iter()
        .map(|l| ScheduledLessonDto {
            id: l.id,
            section_subject_id: l.section_subject_id,
            time_slot_id: l.time_slot_id,
            room_id: l.room_id,
            teacher_id: l.teacher_id,
            created_at: l.created_at,
            updated_at: l.updated_at,
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(delete, path = "/api/v1/scheduled-lessons/{id}", params(("id" = Uuid, Path, description = "Scheduled Lesson ID")),
    responses((status = 200, body = String)), security(("bearer" = [])), tag = "Scheduling")]
async fn delete_lesson(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    state
        .scheduling_service
        .delete_scheduled_lesson(&ctx, id)
        .await?;
    Ok(ApiResponse::ok(
        "Scheduled lesson deleted successfully".to_string(),
    ))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/rooms", post(create_room).get(list_rooms))
        .route(
            "/api/v1/rooms/:id",
            get(get_room).patch(update_room).delete(delete_room),
        )
        .route(
            "/api/v1/time-slots",
            post(create_time_slot).get(list_time_slots),
        )
        .route("/api/v1/time-slots/batch", post(batch_create_time_slots))
        .route(
            "/api/v1/time-slots/:id",
            get(get_time_slot)
                .patch(update_time_slot)
                .delete(delete_time_slot),
        )
        .route(
            "/api/v1/scheduled-lessons",
            post(schedule_lesson).get(list_lessons),
        )
        .route(
            "/api/v1/scheduled-lessons/bulk",
            post(bulk_schedule_lessons),
        )
        .route("/api/v1/scheduled-lessons/:id", delete(delete_lesson))
}
