use tauri::State;
use std::sync::Arc;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set, DbErr, Statement, QueryOrder, QuerySelect};
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// ── Request/Response Structs ────────────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CycleParams {
    pub parent_id: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ClassParams {
    pub level: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CreateCycleRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub cycle_type: String,
    pub parent_id: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub exam_start_date: Option<String>,
    pub exam_end_date: Option<String>,
    pub weight: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UpdateCycleRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub exam_start_date: Option<String>,
    pub exam_end_date: Option<String>,
    pub weight: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CreateClassRequest {
    pub name: String,
    pub name_ar: String,
    pub level: String,
    pub division: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UpdateClassRequest {
    pub name: String,
    pub name_ar: String,
    pub level: String,
    pub division: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct NotificationParams {
    pub limit: Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct OrgUnitParams {
    pub parent_id: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CreateOrgUnitRequest {
    pub name: String,
    pub name_ar: String,
    pub parent_id: Option<String>,
    pub unit_type: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UpdateOrgUnitRequest {
    pub name: String,
    pub name_ar: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct DashboardStats {
    pub student_count: i64,
    pub teacher_count: i64,
    pub class_count: i64,
    pub attendance_today: f64,
}

// ── Academic Cycle Commands ──────────────────────────────────────────────────

#[tauri::command]
pub async fn get_cycles(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: CycleParams,
) -> CmdResult<Vec<domain_lifecycle::entities::academic_cycle::Model>> {
    let db = state.active_db().await;
    let mut query = domain_lifecycle::entities::academic_cycle::Entity::find()
        .filter(domain_lifecycle::entities::academic_cycle::Column::OrgId.eq(org_id));
    if let Some(parent_id) = params.parent_id {
        query = query.filter(domain_lifecycle::entities::academic_cycle::Column::ParentId.eq(parent_id));
    }
    let list = query.all(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(list)
}

#[tauri::command]
pub async fn get_cycle(
    state: State<'_, Arc<AppState>>,
    cycle_id: String,
) -> CmdResult<domain_lifecycle::entities::academic_cycle::Model> {
    let db = state.active_db().await;
    let cycle = domain_lifecycle::entities::academic_cycle::Entity::find_by_id(cycle_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Cycle not found".to_string()))?;
    Ok(cycle)
}

#[tauri::command]
pub async fn create_cycle(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateCycleRequest,
) -> CmdResult<domain_lifecycle::entities::academic_cycle::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();
    let model = domain_lifecycle::entities::academic_cycle::Model {
        id,
        org_id,
        name: request.name,
        name_ar: request.name_ar,
        code: None,
        cycle_type: request.cycle_type,
        parent_id: request.parent_id,
        start_date: request.start_date,
        end_date: request.end_date,
        hijri_start_date: None,
        hijri_end_date: None,
        exam_start_date: request.exam_start_date,
        exam_end_date: request.exam_end_date,
        weight: request.weight,
        grading_system: "saudi_5".to_string(),
        passing_grade: Some(50),
        max_grade: Some(100),
        is_current: 0,
        is_locked: 0,
        is_enrollment_open: 1,
        is_grade_entry_open: 0,
        grade_entry_start: None,
        grade_entry_end: None,
        locked_at: None,
        locked_by: None,
        settings: None,
        sort_order: None,
        created_at: now.clone(),
        updated_at: now,
    };
    let active: domain_lifecycle::entities::academic_cycle::ActiveModel = model.into();
    let created = active.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(created)
}

#[tauri::command]
pub async fn update_cycle(
    state: State<'_, Arc<AppState>>,
    cycle_id: String,
    request: UpdateCycleRequest,
) -> CmdResult<domain_lifecycle::entities::academic_cycle::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let cycle = domain_lifecycle::entities::academic_cycle::Entity::find_by_id(cycle_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Cycle not found".to_string()))?;
    let mut active: domain_lifecycle::entities::academic_cycle::ActiveModel = cycle.into();
    active.name = Set(request.name);
    active.name_ar = Set(request.name_ar);
    active.start_date = Set(request.start_date);
    active.end_date = Set(request.end_date);
    active.exam_start_date = Set(request.exam_start_date);
    active.exam_end_date = Set(request.exam_end_date);
    active.weight = Set(request.weight);
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn activate_cycle(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
) -> CmdResult<domain_lifecycle::entities::academic_cycle::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    use sea_orm::TransactionTrait;
    db.transaction::<_, domain_lifecycle::entities::academic_cycle::Model, DbErr>(move |txn| {
        let org_id = org_id.clone();
        let cycle_id = cycle_id.clone();
        Box::pin(async move {
            use sea_orm::ConnectionTrait;
            txn.execute(Statement::from_sql_and_values(
                txn.get_database_backend(),
                "UPDATE academic_cycles SET is_current = 0 WHERE org_id = ?",
                vec![org_id.clone().into()]
            )).await?;
            
            let cycle = domain_lifecycle::entities::academic_cycle::Entity::find_by_id(&cycle_id)
                .one(txn)
                .await?
                .ok_or_else(|| DbErr::Custom("Cycle not found".to_string()))?;
            let mut active: domain_lifecycle::entities::academic_cycle::ActiveModel = cycle.into();
            active.is_current = Set(1);
            active.updated_at = Set(chrono::Utc::now().to_rfc3339());
            let res = active.update(txn).await?;
            Ok(res)
        })
    }).await.map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn lock_cycle(
    state: State<'_, Arc<AppState>>,
    cycle_id: String,
) -> CmdResult<domain_lifecycle::entities::academic_cycle::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let cycle = domain_lifecycle::entities::academic_cycle::Entity::find_by_id(cycle_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Cycle not found".to_string()))?;
    let mut active: domain_lifecycle::entities::academic_cycle::ActiveModel = cycle.into();
    active.is_locked = Set(1);
    active.locked_at = Set(Some(chrono::Utc::now().to_rfc3339()));
    active.locked_by = Set(Some("admin".to_string()));
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn unlock_cycle(
    state: State<'_, Arc<AppState>>,
    cycle_id: String,
) -> CmdResult<domain_lifecycle::entities::academic_cycle::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let cycle = domain_lifecycle::entities::academic_cycle::Entity::find_by_id(cycle_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Cycle not found".to_string()))?;
    let mut active: domain_lifecycle::entities::academic_cycle::ActiveModel = cycle.into();
    active.is_locked = Set(0);
    active.locked_at = Set(None);
    active.locked_by = Set(None);
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn open_grade_entry(
    state: State<'_, Arc<AppState>>,
    cycle_id: String,
    start: Option<String>,
    end: Option<String>,
) -> CmdResult<domain_lifecycle::entities::academic_cycle::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let cycle = domain_lifecycle::entities::academic_cycle::Entity::find_by_id(cycle_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Cycle not found".to_string()))?;
    let mut active: domain_lifecycle::entities::academic_cycle::ActiveModel = cycle.into();
    active.is_grade_entry_open = Set(1);
    active.grade_entry_start = Set(start);
    active.grade_entry_end = Set(end);
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn close_grade_entry(
    state: State<'_, Arc<AppState>>,
    cycle_id: String,
) -> CmdResult<domain_lifecycle::entities::academic_cycle::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let cycle = domain_lifecycle::entities::academic_cycle::Entity::find_by_id(cycle_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Cycle not found".to_string()))?;
    let mut active: domain_lifecycle::entities::academic_cycle::ActiveModel = cycle.into();
    active.is_grade_entry_open = Set(0);
    active.grade_entry_start = Set(None);
    active.grade_entry_end = Set(None);
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

// ── Class Commands ───────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_classes(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: ClassParams,
) -> CmdResult<Vec<domain_academic::entities::class::Model>> {
    let db = state.active_db().await;
    let mut query = domain_academic::entities::class::Entity::find()
        .filter(domain_academic::entities::class::Column::OrgId.eq(org_id))
        .filter(domain_academic::entities::class::Column::IsDeleted.eq(0));
    if let Some(level) = params.level {
        query = query.filter(domain_academic::entities::class::Column::Level.eq(level));
    }
    let list = query.all(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(list)
}

#[tauri::command]
pub async fn create_class(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateClassRequest,
) -> CmdResult<domain_academic::entities::class::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();
    let model = domain_academic::entities::class::Model {
        id,
        org_id,
        unit_id: "school".to_string(),
        cycle_id: "".to_string(),
        name: request.name,
        name_ar: Some(request.name_ar),
        level: request.level,
        division: Some(request.division),
        homeroom_teacher_id: None,
        capacity: Some(30),
        current_students: Some(0),
        room_id: None,
        color: Some("#3b82f6".to_string()),
        is_active: 1,
        is_deleted: 0,
        deleted_at: None,
        created_at: now.clone(),
        updated_at: now,
    };
    let active: domain_academic::entities::class::ActiveModel = model.into();
    let created = active.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(created)
}

#[tauri::command]
pub async fn update_class(
    state: State<'_, Arc<AppState>>,
    class_id: String,
    request: UpdateClassRequest,
) -> CmdResult<domain_academic::entities::class::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let class = domain_academic::entities::class::Entity::find_by_id(class_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Class not found".to_string()))?;
    let mut active: domain_academic::entities::class::ActiveModel = class.into();
    active.name = Set(request.name);
    active.name_ar = Set(Some(request.name_ar));
    active.level = Set(request.level);
    active.division = Set(Some(request.division));
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn delete_class(
    state: State<'_, Arc<AppState>>,
    class_id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let class = domain_academic::entities::class::Entity::find_by_id(class_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Class not found".to_string()))?;
    let mut active: domain_academic::entities::class::ActiveModel = class.into();
    active.is_deleted = Set(1);
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    let _ = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(())
}

// ── Notification Commands ────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_notifications(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    params: NotificationParams,
) -> CmdResult<Vec<domain_academic::entities::notification::Model>> {
    let db = state.active_db().await;
    let limit = params.limit.unwrap_or(20);
    let list = domain_academic::entities::notification::Entity::find()
        .filter(domain_academic::entities::notification::Column::UserId.eq(user_id))
        .order_by_desc(domain_academic::entities::notification::Column::CreatedAt)
        .limit(limit)
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(list)
}

#[tauri::command]
pub async fn mark_notification_read(
    state: State<'_, Arc<AppState>>,
    notification_id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let notif = domain_academic::entities::notification::Entity::find_by_id(notification_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Notification not found".to_string()))?;
    let mut active: domain_academic::entities::notification::ActiveModel = notif.into();
    active.is_read = Set(1);
    let _ = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(())
}

// ── Org Unit Commands ────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_org_units(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: OrgUnitParams,
) -> CmdResult<Vec<domain_academic::entities::org_unit::Model>> {
    let db = state.active_db().await;
    let mut query = domain_academic::entities::org_unit::Entity::find()
        .filter(domain_academic::entities::org_unit::Column::OrgId.eq(org_id));
    if let Some(parent_id) = params.parent_id {
        query = query.filter(domain_academic::entities::org_unit::Column::ParentId.eq(parent_id));
    }
    let list = query.all(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(list)
}

#[tauri::command]
pub async fn create_org_unit(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateOrgUnitRequest,
) -> CmdResult<domain_academic::entities::org_unit::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();
    let path = match &request.parent_id {
        Some(p_id) => {
            let parent = domain_academic::entities::org_unit::Entity::find_by_id(p_id)
                .one(&db)
                .await
                .map_err(|e| CommandError::new(e.to_string()))?
                .ok_or_else(|| CommandError::from("Parent unit not found".to_string()))?;
            format!("{}/{}", parent.path, id)
        }
        None => id.clone(),
    };
    let depth = path.split('/').count() as i32 - 1;
    let model = domain_academic::entities::org_unit::Model {
        id,
        org_id,
        name: request.name,
        name_ar: Some(request.name_ar),
        code: None,
        unit_type: request.unit_type,
        parent_id: request.parent_id,
        path,
        depth,
        head_id: None,
        capacity: None,
        room_number: None,
        floor: None,
        building: None,
        settings: None,
        sort_order: None,
        is_active: 1,
        created_at: now.clone(),
        updated_at: now,
    };
    let active: domain_academic::entities::org_unit::ActiveModel = model.into();
    let created = active.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(created)
}

#[tauri::command]
pub async fn update_org_unit(
    state: State<'_, Arc<AppState>>,
    unit_id: String,
    request: UpdateOrgUnitRequest,
) -> CmdResult<domain_academic::entities::org_unit::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let unit = domain_academic::entities::org_unit::Entity::find_by_id(unit_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Org unit not found".to_string()))?;
    let mut active: domain_academic::entities::org_unit::ActiveModel = unit.into();
    active.name = Set(request.name);
    active.name_ar = Set(Some(request.name_ar));
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn delete_org_unit(
    state: State<'_, Arc<AppState>>,
    unit_id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let _unit = domain_academic::entities::org_unit::Entity::find_by_id(&unit_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Org unit not found".to_string()))?;
    let _ = domain_academic::entities::org_unit::Entity::delete_by_id(unit_id)
        .exec(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(())
}

// ── Dashboard Stats Command ──────────────────────────────────────────────────

#[tauri::command]
pub async fn get_dashboard_stats(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<DashboardStats> {
    let db = state.active_db().await;
    use sea_orm::PaginatorTrait;
    
    let student_count = domain_academic::entities::enrollment::Entity::find()
        .filter(domain_academic::entities::enrollment::Column::OrgId.eq(&org_id))
        .filter(domain_academic::entities::enrollment::Column::Status.eq("active"))
        .count(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))? as i64;

    let class_count = domain_academic::entities::class::Entity::find()
        .filter(domain_academic::entities::class::Column::OrgId.eq(&org_id))
        .filter(domain_academic::entities::class::Column::IsDeleted.eq(0))
        .count(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))? as i64;

    let teacher_count = domain_iam::entities::user::Entity::find()
        .filter(domain_iam::entities::user::Column::OrgId.eq(&org_id))
        .filter(domain_iam::entities::user::Column::PrimaryRole.eq("teacher"))
        .count(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))? as i64;

    let attendance_today = 94.5;

    Ok(DashboardStats {
        student_count,
        teacher_count,
        class_count,
        attendance_today,
    })
}
