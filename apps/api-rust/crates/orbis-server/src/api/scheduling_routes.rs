use axum::{extract::State, Json, routing::post, Router};
use serde::{Deserialize, Serialize};
use orbis_app::context::RequestContext;
use orbis_app::usecases::scheduling::{
    room::CreateRoomUseCase,
    time_slot::CreateTimeSlotUseCase,
    scheduled_lesson::ScheduleLessonUseCase,
};
use uuid::Uuid;
use chrono::NaiveTime;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateRoomRequest {
    pub org_id: Uuid,
    pub name: String,
    pub capacity: i32,
    pub room_type: String,
}

#[derive(Serialize)]
pub struct RoomResponse {
    pub id: Uuid,
}

#[derive(Deserialize)]
pub struct CreateTimeSlotRequest {
    pub period_id: Uuid,
    pub day_of_week: i16,
    pub start_time: String,
    pub end_time: String,
    pub slot_index: i32,
}

#[derive(Serialize)]
pub struct TimeSlotResponse {
    pub id: Uuid,
}

#[derive(Deserialize)]
pub struct ScheduleLessonRequest {
    pub class_subject_id: Uuid,
    pub room_id: Option<Uuid>,
    pub time_slot_id: Uuid,
    pub teacher_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct ScheduledLessonResponse {
    pub id: Uuid,
}

async fn create_room(
    State(state): State<AppState>,
    Json(payload): Json<CreateRoomRequest>,
) -> Result<Json<RoomResponse>, String> {
    let ctx = RequestContext {
        tenant_id: payload.org_id,
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = CreateRoomUseCase {
        room_repo: state.room_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match use_case.execute(&ctx, payload.org_id, payload.name, payload.capacity, payload.room_type).await {
        Ok(room) => Ok(Json(RoomResponse {
            id: room.id,
        })),
        Err(e) => Err(e.to_string()),
    }
}

async fn create_time_slot(
    State(state): State<AppState>,
    Json(payload): Json<CreateTimeSlotRequest>,
) -> Result<Json<TimeSlotResponse>, String> {
    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let start = NaiveTime::parse_from_str(&payload.start_time, "%H:%M:%S").map_err(|_| "Invalid start time format")?;
    let end = NaiveTime::parse_from_str(&payload.end_time, "%H:%M:%S").map_err(|_| "Invalid end time format")?;

    let use_case = CreateTimeSlotUseCase {
        time_slot_repo: state.time_slot_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match use_case.execute(&ctx, payload.period_id, payload.day_of_week, start, end, payload.slot_index).await {
        Ok(slot) => Ok(Json(TimeSlotResponse {
            id: slot.id,
        })),
        Err(e) => Err(e.to_string()),
    }
}

async fn schedule_lesson(
    State(state): State<AppState>,
    Json(payload): Json<ScheduleLessonRequest>,
) -> Result<Json<ScheduledLessonResponse>, String> {
    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(), // Should be derived from session
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = ScheduleLessonUseCase {
        lesson_repo: state.lesson_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match use_case.execute(&ctx, payload.class_subject_id, payload.room_id, payload.time_slot_id, payload.teacher_id).await {
        Ok(lesson) => Ok(Json(ScheduledLessonResponse {
            id: lesson.id,
        })),
        Err(e) => Err(e.to_string()),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/scheduling/rooms", post(create_room))
        .route("/scheduling/time-slots", post(create_time_slot))
        .route("/scheduling/lessons", post(schedule_lesson))
}
