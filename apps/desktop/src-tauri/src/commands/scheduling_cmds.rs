use tauri::State;
use crate::state::AppState;
use orbis_app::usecases::scheduling::{
    room::CreateRoomUseCase,
    time_slot::CreateTimeSlotUseCase,
    scheduled_lesson::ScheduleLessonUseCase,
};
use orbis_app::context::RequestContext;
use uuid::Uuid;
use chrono::NaiveTime;

#[tauri::command]
pub async fn create_room(
    org_id: String,
    name: String,
    capacity: i32,
    room_type: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_org_id = Uuid::parse_str(&org_id).map_err(|_| "Invalid org ID")?;

    let ctx = RequestContext {
        tenant_id: parsed_org_id,
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = CreateRoomUseCase {
        room_repo: state.room_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, parsed_org_id, name, capacity, room_type).await {
        Ok(room) => Ok(room.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn create_time_slot(
    period_id: String,
    day_of_week: i16,
    start_time: String,
    end_time: String,
    slot_index: i32,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_period_id = Uuid::parse_str(&period_id).map_err(|_| "Invalid period ID")?;
    
    let start = NaiveTime::parse_from_str(&start_time, "%H:%M:%S").map_err(|_| "Invalid start time format")?;
    let end = NaiveTime::parse_from_str(&end_time, "%H:%M:%S").map_err(|_| "Invalid end time format")?;

    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = CreateTimeSlotUseCase {
        time_slot_repo: state.time_slot_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, parsed_period_id, day_of_week, start, end, slot_index).await {
        Ok(slot) => Ok(slot.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn schedule_lesson(
    class_subject_id: String,
    room_id: Option<String>,
    time_slot_id: String,
    teacher_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_class_subject_id = Uuid::parse_str(&class_subject_id).map_err(|_| "Invalid class subject ID")?;
    let parsed_time_slot_id = Uuid::parse_str(&time_slot_id).map_err(|_| "Invalid time slot ID")?;
    
    let parsed_room_id = match room_id {
        Some(id) => Some(Uuid::parse_str(&id).map_err(|_| "Invalid room ID")?),
        None => None,
    };
    
    let parsed_teacher_id = match teacher_id {
        Some(id) => Some(Uuid::parse_str(&id).map_err(|_| "Invalid teacher ID")?),
        None => None,
    };

    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = ScheduleLessonUseCase {
        lesson_repo: state.lesson_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, parsed_class_subject_id, parsed_room_id, parsed_time_slot_id, parsed_teacher_id).await {
        Ok(lesson) => Ok(lesson.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}
