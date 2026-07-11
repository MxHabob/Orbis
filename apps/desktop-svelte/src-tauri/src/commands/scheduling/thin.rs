/// Thin Tauri command wrappers for the Scheduling domain.
use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};
use svc_scheduling::ops;

// ── Time Slots ──────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_time_slots(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::time_slot::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_time_slot(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::time_slot::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_time_slot(state: State<'_, Arc<AppState>>, org_id: String, slot_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::time_slot::update(&ctx, &slot_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn delete_time_slot(state: State<'_, Arc<AppState>>, org_id: String, slot_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::time_slot::delete(&ctx, &slot_id).await.map_err(CommandError::from)
}

// ── Lessons ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_class_schedule(
    state: State<'_, Arc<AppState>>,
    cycle_id: String,
    class_id: String,
) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.active_db().await;
    let class = svc_academic::entities::class::Entity::find_by_id(class_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Class not found"))?;
    let ctx = state.build_context(&class.org_id).await.map_err(CommandError::from)?;
    let r = ops::lesson::list_by_class(&ctx, &cycle_id, &class_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_teacher_schedule(
    state: State<'_, Arc<AppState>>,
    cycle_id: String,
    teacher_id: String,
) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.active_db().await;
    let cycle = svc_organization::entities::academic_cycle::Entity::find_by_id(cycle_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Cycle not found"))?;
    let ctx = state.build_context(&cycle.org_id).await.map_err(CommandError::from)?;
    let r = ops::lesson::list_by_teacher(&ctx, &cycle_id, &teacher_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_lesson_by_id(
    state: State<'_, Arc<AppState>>,
    lesson_id: String,
) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.active_db().await;
    let lesson = svc_scheduling::entities::scheduled_lesson::Entity::find_by_id(lesson_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Lesson not found"))?;
    let ctx = state.build_context(&lesson.org_id).await.map_err(CommandError::from)?;
    let r = ops::lesson::get(&ctx, &lesson_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_lesson(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::lesson::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_lesson(state: State<'_, Arc<AppState>>, org_id: String, lesson_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::lesson::update(&ctx, &lesson_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn delete_lesson(state: State<'_, Arc<AppState>>, org_id: String, lesson_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::lesson::delete(&ctx, &lesson_id).await.map_err(CommandError::from)
}

// ── Rooms ───────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_rooms(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::room::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_room(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::room::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_room(state: State<'_, Arc<AppState>>, org_id: String, room_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::room::update(&ctx, &room_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn delete_room(state: State<'_, Arc<AppState>>, org_id: String, room_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::room::delete(&ctx, &room_id).await.map_err(CommandError::from)
}

// ── Solver ──────────────────────────────────────────────────────────

#[tauri::command]
pub async fn start_solver(state: State<'_, Arc<AppState>>, cycle_id: String) -> CmdResult<serde_json::Value> {
    let db = state.active_db().await;
    let org_id = resolve_org_id(&db).await?;
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::solver::start(&ctx, &cycle_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_solver_jobs(state: State<'_, Arc<AppState>>, cycle_id: String) -> CmdResult<serde_json::Value> {
    let db = state.active_db().await;
    let org_id = resolve_org_id(&db).await?;
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::solver::list_jobs(&ctx, &cycle_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_solver_job(state: State<'_, Arc<AppState>>, job_id: String) -> CmdResult<serde_json::Value> {
    let db = state.active_db().await;
    let org_id = resolve_org_id(&db).await?;
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::solver::get_job(&ctx, &job_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn cancel_solver(state: State<'_, Arc<AppState>>, job_id: String) -> CmdResult<serde_json::Value> {
    let db = state.active_db().await;
    let org_id = resolve_org_id(&db).await?;
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::solver::cancel(&ctx, &job_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn apply_solver(state: State<'_, Arc<AppState>>, job_id: String) -> CmdResult<()> {
    let db = state.active_db().await;
    let org_id = resolve_org_id(&db).await?;
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::solver::apply(&ctx, &job_id).await.map(|_| ()).map_err(CommandError::from)
}

#[tauri::command]
pub async fn delete_solver_job(state: State<'_, Arc<AppState>>, job_id: String) -> CmdResult<()> {
    let db = state.active_db().await;
    let org_id = resolve_org_id(&db).await?;
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::solver::delete_job(&ctx, &job_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn check_schedule_conflicts(
    state: State<'_, Arc<AppState>>,
    cycle_id: String,
    teacher_id: String,
    time_slot_id: String,
    day_of_week: i32,
    room_id: Option<String>,
    exclude_id: Option<String>,
) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.active_db().await;
    let cycle = svc_organization::entities::academic_cycle::Entity::find_by_id(cycle_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Cycle not found"))?;
    let ctx = state.build_context(&cycle.org_id).await.map_err(CommandError::from)?;

    let query_repo = std::sync::Arc::new(svc_scheduling::repository::SchedulingQueryRepo::new(ctx.db.clone()));
    let cmd_repo = std::sync::Arc::new(svc_scheduling::repository::SchedulingCommandRepo::new(ctx.db.clone()));
    let service = svc_scheduling::services::schedule::ScheduleService::new(cmd_repo, query_repo);

    let r = service.check_schedule_conflicts(
        &cycle.org_id,
        &cycle_id,
        &time_slot_id,
        day_of_week,
        exclude_id.as_deref(),
        &teacher_id,
        "",
        room_id.as_deref(),
    ).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

/// Helper function to resolve org_id from the latest active session.
async fn resolve_org_id(db: &sea_orm::DatabaseConnection) -> Result<String, CommandError> {
    use sea_orm::{EntityTrait, QueryOrder};
    let session = svc_iam::entities::user_session::Entity::find()
        .order_by_desc(svc_iam::entities::user_session::Column::CreatedAt)
        .one(db)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::unauthorized("No active session"))?;
    let user = svc_iam::entities::user::Entity::find_by_id(session.user_id)
        .one(db)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::unauthorized("Active user not found"))?;
    Ok(user.org_id)
}
