use tauri::State;
use std::sync::Arc;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set};
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// ── Request/Response Structs ────────────────────────────────────────────────

#[derive(serde::Deserialize, Debug, Clone)]
pub struct CreateRoomRequest {
    pub id: Option<String>,
    pub name: String,
    pub name_ar: String,
    pub capacity: i32,
    pub room_type: String,
    pub building: String,
    pub floor: i32,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct UpdateRoomRequest {
    pub name: String,
    pub name_ar: String,
    pub capacity: i32,
    pub room_type: String,
    pub building: String,
    pub floor: i32,
}

// ── Scheduling Services Helper ───────────────────────────────────────────────

fn get_schedule_services(
    db: sea_orm::DatabaseConnection,
) -> (
    Arc<domain_scheduling::services::schedule::ScheduleService>,
    domain_scheduling::services::solver::TimetableSolverService,
) {
    let cmd = Arc::new(domain_scheduling::repository::SchedulingCommandRepo::new(db.clone()));
    let query = Arc::new(domain_scheduling::repository::SchedulingQueryRepo::new(db.clone()));
    let schedule_service = Arc::new(domain_scheduling::services::schedule::ScheduleService::new(cmd.clone(), query.clone()));
    let solver = Arc::new(infra_scheduler::NoOpScheduleSolver);
    let solver_service = domain_scheduling::services::solver::TimetableSolverService::new(
        cmd,
        query,
        schedule_service.clone(),
        solver,
    );
    (schedule_service, solver_service)
}

// ── Time Slot Commands ───────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_time_slots(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
) -> CmdResult<Vec<domain_scheduling::entities::time_slot::Model>> {
    let db = state.active_db().await;
    let (service, _) = get_schedule_services(db);
    service.get_time_slots(&org_id, &cycle_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn create_time_slot(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: domain_scheduling::services::schedule::CreateTimeSlotRequest,
) -> CmdResult<domain_scheduling::entities::time_slot::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let (service, _) = get_schedule_services(db);
    service.create_time_slot(&org_id, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn update_time_slot(
    state: State<'_, Arc<AppState>>,
    id: String,
    request: domain_scheduling::services::schedule::UpdateTimeSlotRequest,
) -> CmdResult<domain_scheduling::entities::time_slot::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let (service, _) = get_schedule_services(db);
    service.update_time_slot(&id, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn delete_time_slot(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let (service, _) = get_schedule_services(db);
    service.delete_time_slot(&id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct CreateClassSubjectRequest {
    pub class_id: String,
    pub subject_id: String,
    pub cycle_id: String,
    pub primary_teacher_id: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct UpdateClassSubjectRequest {
    pub primary_teacher_id: Option<String>,
    pub assistant_teacher_id: Option<String>,
    pub is_active: Option<i32>,
}

// ── Class Subject Commands ───────────────────────────────────────────────────

#[tauri::command]
pub async fn get_class_subjects(
    state: State<'_, Arc<AppState>>,
    class_id: Option<String>,
    cycle_id: Option<String>,
) -> CmdResult<Vec<domain_academic::entities::class_subject::Model>> {
    let db = state.active_db().await;
    let mut query = domain_academic::entities::class_subject::Entity::find();
    if let Some(c_id) = class_id {
        query = query.filter(domain_academic::entities::class_subject::Column::ClassId.eq(c_id));
    }
    if let Some(cy_id) = cycle_id {
        query = query.filter(domain_academic::entities::class_subject::Column::CycleId.eq(cy_id));
    }
    let list = query.all(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(list)
}

#[tauri::command]
pub async fn create_class_subject(
    state: State<'_, Arc<AppState>>,
    request: CreateClassSubjectRequest,
) -> CmdResult<domain_academic::entities::class_subject::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let now = chrono::Utc::now().to_rfc3339();
    let model = domain_academic::entities::class_subject::Model {
        id: uuid::Uuid::new_v4().to_string(),
        class_id: request.class_id,
        subject_id: request.subject_id,
        cycle_id: request.cycle_id,
        primary_teacher_id: request.primary_teacher_id,
        assistant_teacher_id: None,
        substitute_teacher_id: None,
        grade_groups: None,
        weekly_hours: Some(4),
        theory_hours: Some(4),
        practical_hours: Some(0),
        assessment_config: None,
        is_active: 1,
        created_at: now.clone(),
        updated_at: now,
    };
    let active: domain_academic::entities::class_subject::ActiveModel = model.into();
    let created = active.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(created)
}

#[tauri::command]
pub async fn update_class_subject(
    state: State<'_, Arc<AppState>>,
    id: String,
    request: UpdateClassSubjectRequest,
) -> CmdResult<domain_academic::entities::class_subject::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    
    let existing = domain_academic::entities::class_subject::Entity::find_by_id(&id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("ClassSubject not found".to_string()))?;

    let mut active: domain_academic::entities::class_subject::ActiveModel = existing.into();
    if let Some(t_id) = request.primary_teacher_id {
        active.primary_teacher_id = Set(t_id);
    }
    if let Some(ast_id) = request.assistant_teacher_id {
        active.assistant_teacher_id = Set(Some(ast_id));
    }
    if let Some(active_val) = request.is_active {
        active.is_active = Set(active_val);
    }
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn delete_class_subject(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    
    let existing = domain_academic::entities::class_subject::Entity::find_by_id(&id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("ClassSubject not found".to_string()))?;

    let mut active: domain_academic::entities::class_subject::ActiveModel = existing.into();
    active.is_active = Set(0);
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    
    Ok(())
}

// ── Lesson Commands ──────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_class_schedule(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
    class_id: String,
) -> CmdResult<Vec<domain_scheduling::services::schedule::ScheduledLessonView>> {
    let db = state.active_db().await;
    let (service, _) = get_schedule_services(db);
    service.get_class_schedule(&org_id, &cycle_id, &class_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_teacher_schedule(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
    teacher_id: String,
) -> CmdResult<Vec<domain_scheduling::services::schedule::ScheduledLessonView>> {
    let db = state.active_db().await;
    let (service, _) = get_schedule_services(db);
    service.get_teacher_schedule(&org_id, &cycle_id, &teacher_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_lesson_by_id(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<domain_scheduling::services::schedule::ScheduledLessonView> {
    let db = state.active_db().await;
    let (service, _) = get_schedule_services(db);
    service.get_lesson_by_id(&id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn create_lesson(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: domain_scheduling::services::schedule::CreateLessonRequest,
) -> CmdResult<domain_scheduling::services::schedule::ScheduledLessonView> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let (service, _) = get_schedule_services(db);
    service.create_lesson(&org_id, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn update_lesson(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
    request: domain_scheduling::services::schedule::UpdateLessonRequest,
) -> CmdResult<domain_scheduling::services::schedule::ScheduledLessonView> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let (service, _) = get_schedule_services(db);
    service.update_lesson(&org_id, &id, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn delete_lesson(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let (service, _) = get_schedule_services(db);
    service.delete_lesson(&id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
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
) -> CmdResult<Vec<domain_scheduling::services::schedule::ScheduleConflict>> {
    let db = state.active_db().await;
    let (service, _) = get_schedule_services(db);
    service.check_schedule_conflicts(
        &org_id, &cycle_id, &time_slot_id, day_of_week,
        exclude_id.as_deref(), &teacher_id, &class_id, room_id.as_deref(),
    ).await.map_err(|e| CommandError::new(e.to_string()))
}

// ── Solver Commands ──────────────────────────────────────────────────────────

#[tauri::command]
pub async fn start_schedule_solve(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    user_id: String,
    req: domain_scheduling::services::solver::StartSolveRequest,
) -> CmdResult<domain_scheduling::services::solver::SolveJobResponse> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let (_, solver_service) = get_schedule_services(db);
    solver_service.start_schedule_solve(&org_id, &user_id, req)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_schedule_solve_job(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    job_id: String,
) -> CmdResult<domain_scheduling::services::solver::SolveJobResponse> {
    let db = state.active_db().await;
    let (_, solver_service) = get_schedule_services(db);
    solver_service.get_schedule_solve_job(&org_id, &job_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn list_schedule_solve_jobs(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: Option<String>,
) -> CmdResult<Vec<domain_scheduling::services::solver::SolveJobResponse>> {
    let db = state.active_db().await;
    let (_, solver_service) = get_schedule_services(db);
    solver_service.list_schedule_solve_jobs(&org_id, cycle_id.as_deref())
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn cancel_schedule_solve(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    job_id: String,
) -> CmdResult<domain_scheduling::services::solver::SolveJobResponse> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let (_, solver_service) = get_schedule_services(db);
    solver_service.cancel_schedule_solve(&org_id, &job_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn apply_schedule_solve(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    job_id: String,
) -> CmdResult<domain_scheduling::services::solver::SolveJobResponse> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let (_, solver_service) = get_schedule_services(db);
    solver_service.apply_schedule_solve(&org_id, &job_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

// ── Room Commands ────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_rooms(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<domain_scheduling::entities::room::Model>> {
    let db = state.active_db().await;
    domain_scheduling::entities::room::Entity::find()
        .filter(domain_scheduling::entities::room::Column::OrgId.eq(org_id))
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn create_room(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateRoomRequest,
) -> CmdResult<domain_scheduling::entities::room::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let now = chrono::Utc::now().to_rfc3339();
    let id = request.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    
    let model = domain_scheduling::entities::room::Model {
        id,
        org_id,
        name: request.name,
        name_ar: Some(request.name_ar),
        capacity: Some(request.capacity),
        room_type: request.room_type,
        building: Some(request.building),
        floor: Some(request.floor),
        is_active: 1,
        created_at: now.clone(),
        updated_at: now,
    };
    
    let active: domain_scheduling::entities::room::ActiveModel = model.into();
    let created = active.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(created)
}

#[tauri::command]
pub async fn update_room(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
    request: UpdateRoomRequest,
) -> CmdResult<domain_scheduling::entities::room::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let room = domain_scheduling::entities::room::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Room not found".to_string()))?;
        
    let mut active: domain_scheduling::entities::room::ActiveModel = room.into();
    active.name = Set(request.name);
    active.name_ar = Set(Some(request.name_ar));
    active.capacity = Set(Some(request.capacity));
    active.room_type = Set(request.room_type);
    active.building = Set(Some(request.building));
    active.floor = Set(Some(request.floor));
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn delete_room(
    state: State<'_, Arc<AppState>>,
    _org_id: String,
    id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let room = domain_scheduling::entities::room::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Room not found".to_string()))?;
        
    let _ = domain_scheduling::entities::room::Entity::delete_by_id(room.id)
        .exec(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(())
}
