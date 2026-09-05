//! Tauri IPC commands — Scheduling (Rooms, TimeSlots, Lessons).

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct RoomDto { pub id: String, pub org_id: String, pub name: String, pub room_type: String, pub capacity: i32 }
#[derive(Serialize)]
pub struct TimeSlotDto { pub id: String, pub period_id: String, pub day_of_week: i32, pub start_time: String, pub end_time: String, pub label: String }
#[derive(Serialize)]
pub struct ScheduledLessonDto { pub id: String, pub class_subject_id: String, pub time_slot_id: String, pub room_id: Option<String>, pub teacher_id: Option<String> }

#[derive(Deserialize, Default)]
pub struct LessonFilter {
    #[serde(rename = "classSubjectId")]
    pub class_subject_id: Option<String>,
    #[serde(rename = "periodId")]
    pub period_id: Option<String>,
}

#[tauri::command]
pub async fn scheduling_list_rooms(state: State<'_, AppState>) -> Result<ApiResponse<Vec<RoomDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.room_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|r| RoomDto { id: r.id.to_string(), org_id: r.org_id.to_string(), name: r.name, room_type: r.room_type, capacity: r.capacity }).collect()))
}

#[tauri::command]
pub async fn scheduling_get_room(id: String, state: State<'_, AppState>) -> Result<ApiResponse<RoomDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let r = state.room_repo.find_by_id(uid).await.map_err(|e| e.to_string())?.ok_or("Room not found")?;
    Ok(ok(RoomDto { id: r.id.to_string(), org_id: r.org_id.to_string(), name: r.name, room_type: r.room_type, capacity: r.capacity }))
}

#[tauri::command]
pub async fn scheduling_list_time_slots(period_id: String, state: State<'_, AppState>) -> Result<ApiResponse<Vec<TimeSlotDto>>, String> {
    let pid = Uuid::parse_str(&period_id).map_err(|_| "Invalid period ID")?;
    let items = state.time_slot_repo.find_by_period(pid).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|t| TimeSlotDto {
        id: t.id.to_string(), period_id: t.period_id.to_string(), day_of_week: t.day_of_week,
        start_time: t.start_time, end_time: t.end_time, label: t.label,
    }).collect()))
}

#[tauri::command]
pub async fn scheduling_list_lessons(filter: Option<LessonFilter>, state: State<'_, AppState>) -> Result<ApiResponse<Vec<ScheduledLessonDto>>, String> {
    let f = filter.unwrap_or_default();
    let items = if let Some(csid) = f.class_subject_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        state.lesson_repo.find_by_class_subject(csid).await
    } else if let Some(pid) = f.period_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        state.lesson_repo.find_by_period(pid).await
    } else { Ok(vec![]) }.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|l| ScheduledLessonDto {
        id: l.id.to_string(), class_subject_id: l.class_subject_id.to_string(),
        time_slot_id: l.time_slot_id.to_string(),
        room_id: l.room_id.map(|r| r.to_string()), teacher_id: l.teacher_id.map(|t| t.to_string()),
    }).collect()))
}
