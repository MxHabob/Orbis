use tauri::State;
use std::sync::Arc;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set, QueryOrder, DatabaseConnection};
use sea_orm::prelude::Expr;
use domain_lifecycle::entities::{academic_calendar_event, org_academic_policy};
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// ── Request & Response Types ───────────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CreateCalendarEventRequest {
    pub cycle_id: String,
    pub title: String,
    pub title_ar: Option<String>,
    pub description: Option<String>,
    pub event_type: String,
    pub start_date: String,
    pub end_date: String,
    pub execution_mode: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct UpdateCalendarEventRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub execution_mode: Option<String>,
    pub status: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CreateOrgPolicyRequest {
    pub default_grading_system: String,
    pub default_passing_grade: i32,
    pub default_max_grade: i32,
    pub default_event_execution_mode: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct EventProcessingResult {
    pub event_id: String,
    pub event_type: String,
    pub title: String,
    pub execution_mode: String,
    pub outcome: String,
    pub message: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CalendarProcessingReport {
    pub executed: Vec<EventProcessingResult>,
    pub suggested: Vec<EventProcessingResult>,
    pub skipped: Vec<EventProcessingResult>,
    pub blocked: Vec<EventProcessingResult>,
}

impl CalendarProcessingReport {
    pub fn new() -> Self {
        Self {
            executed: Vec::new(),
            suggested: Vec::new(),
            skipped: Vec::new(),
            blocked: Vec::new(),
        }
    }
}

// ── Helper functions for calendar logic ────────────────────────────────────

pub async fn execute_calendar_event_action(
    db: &DatabaseConnection,
    event: &academic_calendar_event::Model,
    user_id: &str,
    registry_db: &DatabaseConnection,
) -> Result<EventProcessingResult, shared_kernel::errors::AppError> {
    // 1. Resolve default mode
    let config = autopilot::entities::autopilot_config::Entity::find()
        .filter(autopilot::entities::autopilot_config::Column::OrgId.eq(&event.org_id))
        .one(registry_db)
        .await?
        .unwrap_or(autopilot::entities::autopilot_config::Model {
            id: format!("{}_config", event.org_id),
            org_id: event.org_id.clone(),
            mode: "suggested".to_string(),
            check_interval_minutes: 60,
            timezone: "Asia/Riyadh".to_string(),
            require_backup_before_critical: 1,
            max_auto_retries: 3,
            pause_on_error: 1,
            is_enabled: 1,
            last_tick_at: None,
            last_tick_result: None,
            paused_at: None,
            pause_reason: None,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        });
    let default_mode = config.mode.clone();

    // 2. Perform cycle side effects
    match event.event_type.as_str() {
        "cycle_start" => {
            domain_lifecycle::entities::academic_cycle::Entity::update_many()
                .col_expr(domain_lifecycle::entities::academic_cycle::Column::IsCurrent, Expr::value(0))
                .filter(domain_lifecycle::entities::academic_cycle::Column::OrgId.eq(&event.org_id))
                .exec(db)
                .await?;

            domain_lifecycle::entities::academic_cycle::Entity::update_many()
                .col_expr(domain_lifecycle::entities::academic_cycle::Column::IsCurrent, Expr::value(1))
                .filter(domain_lifecycle::entities::academic_cycle::Column::Id.eq(&event.cycle_id))
                .exec(db)
                .await?;
        }
        "grades_freeze" => {
            domain_lifecycle::entities::academic_cycle::Entity::update_many()
                .col_expr(domain_lifecycle::entities::academic_cycle::Column::IsLocked, Expr::value(1))
                .col_expr(domain_lifecycle::entities::academic_cycle::Column::IsGradeEntryOpen, Expr::value(0))
                .filter(domain_lifecycle::entities::academic_cycle::Column::Id.eq(&event.cycle_id))
                .exec(db)
                .await?;
        }
        _ => {}
    }

    // 3. Upsert a completed Autopilot Task in the registry DB
    let now = chrono::Utc::now().to_rfc3339();
    let idempotency_key = format!("event:{}:executed", event.id);

    let existing_task = autopilot::entities::autopilot_task::Entity::find()
        .filter(autopilot::entities::autopilot_task::Column::CalendarEventId.eq(Some(event.id.clone())))
        .one(registry_db)
        .await?;

    if let Some(task) = existing_task {
        let mut active: autopilot::entities::autopilot_task::ActiveModel = task.into();
        active.status = Set("completed".to_string());
        active.completed_at = Set(Some(now.clone()));
        active.approved_by = Set(Some(user_id.to_string()));
        active.approved_at = Set(Some(now.clone()));
        active.result = Set(Some("Executed manually via Tauri command".to_string()));
        active.update(registry_db).await?;
    } else {
        let new_task = autopilot::entities::autopilot_task::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: event.org_id.clone(),
            calendar_event_id: Some(event.id.clone()),
            task_type: event.event_type.clone(),
            scheduled_at: event.start_date.clone(),
            deadline_at: None,
            target_entity_type: Some("academic_calendar_event".to_string()),
            target_entity_id: Some(event.id.clone()),
            params: None,
            prerequisites: None,
            status: "completed".to_string(),
            execution_mode: default_mode.clone(),
            attempts: 0,
            last_attempt_at: None,
            last_error: None,
            suggested_at: None,
            approved_by: Some(user_id.to_string()),
            approved_at: Some(now.clone()),
            rejection_reason: None,
            completed_at: Some(now.clone()),
            result: Some("Executed manually via Tauri command".to_string()),
            idempotency_key,
            created_at: now.clone(),
            updated_at: now.clone(),
        };
        let active: autopilot::entities::autopilot_task::ActiveModel = new_task.into();
        active.insert(registry_db).await?;
    }

    Ok(EventProcessingResult {
        event_id: event.id.clone(),
        event_type: event.event_type.clone(),
        title: event.title.clone(),
        execution_mode: default_mode,
        outcome: "executed".to_string(),
        message: Some("Executed successfully".to_string()),
    })
}

async fn merge_event_with_task(
    registry_db: &DatabaseConnection,
    event: academic_calendar_event::Model,
    default_mode: &str,
) -> serde_json::Value {
    let task = autopilot::entities::autopilot_task::Entity::find()
        .filter(autopilot::entities::autopilot_task::Column::CalendarEventId.eq(Some(event.id.clone())))
        .one(registry_db)
        .await
        .unwrap_or(None);

    let (exec_mode, auto_action, prerequisites) = match event.event_type.as_str() {
        "enrollment_open" => (default_mode.to_string(), Some(r#"{"action":"enrollment_opened"}"#.to_string()), None),
        "enrollment_close" => (default_mode.to_string(), Some(r#"{"action":"enrollment_closed"}"#.to_string()), Some(r#"["enrollment_open"]"#.to_string())),
        "grade_entry_open" => (default_mode.to_string(), Some(r#"{"action":"grade_entry_opened"}"#.to_string()), None),
        "grade_entry_close" => (default_mode.to_string(), Some(r#"{"action":"grade_entry_closed"}"#.to_string()), Some(r#"["grade_entry_open"]"#.to_string())),
        "cycle_lock" => (default_mode.to_string(), Some(r#"{"action":"cycle_locked"}"#.to_string()), Some(r#"["grade_entry_close"]"#.to_string())),
        "report_generation" => (default_mode.to_string(), Some(r#"{"action":"reports_generated"}"#.to_string()), Some(r#"["cycle_lock"]"#.to_string())),
        "backup" => ("automatic".to_string(), Some(r#"{"action":"backup_created"}"#.to_string()), None),
        "promotion_dry_run" => (default_mode.to_string(), Some(r#"{"action":"promotion_dry_run_complete"}"#.to_string()), Some(r#"["cycle_lock"]"#.to_string())),
        _ => ("manual".to_string(), None, None),
    };

    let mut val = serde_json::to_value(&event).unwrap_or(serde_json::Value::Null);
    if let serde_json::Value::Object(ref mut map) = val {
        if let Some(t) = task {
            map.insert("execution_mode".to_string(), serde_json::Value::String(t.execution_mode));
            map.insert("executed_at".to_string(), t.completed_at.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null));
            map.insert("executed_by".to_string(), t.approved_by.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null));
            map.insert("execution_result".to_string(), t.result.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null));
            map.insert("execution_notes".to_string(), t.last_error.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null));
            map.insert("auto_action".to_string(), t.params.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null));
            map.insert("prerequisites".to_string(), t.prerequisites.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null));
        } else {
            map.insert("execution_mode".to_string(), serde_json::Value::String(exec_mode));
            map.insert("executed_at".to_string(), serde_json::Value::Null);
            map.insert("executed_by".to_string(), serde_json::Value::Null);
            map.insert("execution_result".to_string(), serde_json::Value::Null);
            map.insert("execution_notes".to_string(), serde_json::Value::Null);
            map.insert("auto_action".to_string(), auto_action.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null));
            map.insert("prerequisites".to_string(), prerequisites.map(serde_json::Value::String).unwrap_or(serde_json::Value::Null));
        }
    }
    val
}

// ── Calendar Events Commands ──────────────────────────────────────────────

#[tauri::command]
pub async fn get_calendar_events(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
) -> CmdResult<Vec<serde_json::Value>> {
    let db = state.active_db().await;
    let registry_db = state.provider.registry();

    let config = autopilot::entities::autopilot_config::Entity::find()
        .filter(autopilot::entities::autopilot_config::Column::OrgId.eq(&org_id))
        .one(registry_db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    let default_mode = config.map(|c| c.mode).unwrap_or("suggested".to_string());

    let events = academic_calendar_event::Entity::find()
        .filter(academic_calendar_event::Column::OrgId.eq(org_id))
        .filter(academic_calendar_event::Column::CycleId.eq(cycle_id))
        .filter(academic_calendar_event::Column::IsActive.eq(1))
        .order_by_asc(academic_calendar_event::Column::StartDate)
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    let mut merged = Vec::new();
    for event in events {
        merged.push(merge_event_with_task(registry_db, event, &default_mode).await);
    }

    Ok(merged)
}

#[tauri::command]
pub async fn get_pending_calendar_events(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<serde_json::Value>> {
    let db = state.active_db().await;
    let registry_db = state.provider.registry();
    let today = chrono::Local::now().date_naive().to_string();

    let config = autopilot::entities::autopilot_config::Entity::find()
        .filter(autopilot::entities::autopilot_config::Column::OrgId.eq(&org_id))
        .one(registry_db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    let default_mode = config.map(|c| c.mode).unwrap_or("suggested".to_string());

    // Fetch active events on or before today
    let active_events = academic_calendar_event::Entity::find()
        .filter(academic_calendar_event::Column::OrgId.eq(org_id))
        .filter(academic_calendar_event::Column::StartDate.lte(&today))
        .filter(academic_calendar_event::Column::IsActive.eq(1))
        .order_by_asc(academic_calendar_event::Column::StartDate)
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    // Fetch completed tasks to filter completed events
    let completed_event_ids: std::collections::HashSet<String> = autopilot::entities::autopilot_task::Entity::find()
        .filter(autopilot::entities::autopilot_task::Column::Status.eq("completed"))
        .all(registry_db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .into_iter()
        .flat_map(|t| t.calendar_event_id)
        .collect();

    let mut pending = Vec::new();
    for event in active_events {
        if !completed_event_ids.contains(&event.id) {
            pending.push(merge_event_with_task(registry_db, event, &default_mode).await);
        }
    }

    Ok(pending)
}

#[tauri::command]
pub async fn create_calendar_event(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateCalendarEventRequest,
) -> CmdResult<serde_json::Value> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let registry_db = state.provider.registry();
    
    let now = chrono::Utc::now().to_rfc3339();
    let model = academic_calendar_event::Model {
        id: uuid::Uuid::new_v4().to_string(),
        org_id: org_id.clone(),
        cycle_id: request.cycle_id,
        title: request.title,
        title_ar: request.title_ar,
        description: request.description,
        event_type: request.event_type,
        start_date: request.start_date,
        end_date: request.end_date,
        start_time: None,
        end_time: None,
        is_recurring: 0,
        recurrence_rule: None,
        color: None,
        is_active: 1,
        sort_order: Some(0),
        created_by: None,
        created_at: now.clone(),
        updated_at: now.clone(),
    };
    
    let active: academic_calendar_event::ActiveModel = model.clone().into();
    let event = active.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;

    // Create autopilot task if mode is not manual
    if request.execution_mode != "manual" {
        let task_id = uuid::Uuid::new_v4().to_string();
        let initial_status = match request.execution_mode.as_str() {
            "automatic" => "ready",
            "suggested" => "suggested",
            _ => "pending",
        };
        let task = autopilot::entities::autopilot_task::Model {
            id: task_id,
            org_id: org_id.clone(),
            calendar_event_id: Some(event.id.clone()),
            task_type: event.event_type.clone(),
            scheduled_at: event.start_date.clone(),
            deadline_at: Some(event.end_date.clone()),
            target_entity_type: Some("academic_calendar_event".to_string()),
            target_entity_id: Some(event.id.clone()),
            params: None,
            prerequisites: None,
            status: initial_status.to_string(),
            execution_mode: request.execution_mode.clone(),
            attempts: 0,
            last_attempt_at: None,
            last_error: None,
            suggested_at: if initial_status == "suggested" { Some(now.clone()) } else { None },
            approved_by: None,
            approved_at: None,
            rejection_reason: None,
            completed_at: None,
            result: None,
            idempotency_key: format!("event:{}:{}", event.id, event.start_date),
            created_at: now.clone(),
            updated_at: now.clone(),
        };
        let active_task: autopilot::entities::autopilot_task::ActiveModel = task.into();
        let _ = active_task.insert(registry_db).await.map_err(|e| CommandError::new(e.to_string()))?;
    }

    let merged = merge_event_with_task(registry_db, event, &request.execution_mode).await;
    Ok(merged)
}

#[tauri::command]
pub async fn update_calendar_event(
    state: State<'_, Arc<AppState>>,
    event_id: String,
    request: UpdateCalendarEventRequest,
) -> CmdResult<serde_json::Value> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let registry_db = state.provider.registry();
    
    let existing = academic_calendar_event::Entity::find_by_id(&event_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Event not found".to_string()))?;

    let org_id = existing.org_id.clone();
    let mut active: academic_calendar_event::ActiveModel = existing.clone().into();
    if let Some(v) = request.title { active.title = Set(v); }
    if let Some(v) = request.description { active.description = Set(Some(v)); }
    if let Some(v) = request.start_date { active.start_date = Set(v); }
    if let Some(v) = request.end_date { active.end_date = Set(v); }
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    
    let event = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;

    // Update corresponding autopilot task if exists
    let existing_task = autopilot::entities::autopilot_task::Entity::find()
        .filter(autopilot::entities::autopilot_task::Column::CalendarEventId.eq(Some(event.id.clone())))
        .one(registry_db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    let default_mode = if let Some(ref t) = existing_task {
        t.execution_mode.clone()
    } else {
        "suggested".to_string()
    };

    if let Some(task) = existing_task {
        let mut active_task: autopilot::entities::autopilot_task::ActiveModel = task.into();
        if let Some(ref v) = request.execution_mode {
            active_task.execution_mode = Set(v.clone());
            let status = match v.as_str() {
                "automatic" => "ready",
                "suggested" => "suggested",
                _ => "pending",
            };
            active_task.status = Set(status.to_string());
        }
        if let Some(ref v) = request.status {
            if v == "executed" {
                active_task.status = Set("completed".to_string());
                active_task.completed_at = Set(Some(chrono::Utc::now().to_rfc3339()));
            } else {
                active_task.status = Set("pending".to_string());
                active_task.completed_at = Set(None);
            }
        }
        let _ = active_task.update(registry_db).await.map_err(|e| CommandError::new(e.to_string()))?;
    } else if let Some(ref v) = request.execution_mode {
        // Create task if mode is not manual
        if v != "manual" {
            let task_id = uuid::Uuid::new_v4().to_string();
            let initial_status = match v.as_str() {
                "automatic" => "ready",
                "suggested" => "suggested",
                _ => "pending",
            };
            let task = autopilot::entities::autopilot_task::Model {
                id: task_id,
                org_id: org_id.clone(),
                calendar_event_id: Some(event.id.clone()),
                task_type: event.event_type.clone(),
                scheduled_at: event.start_date.clone(),
                deadline_at: Some(event.end_date.clone()),
                target_entity_type: Some("academic_calendar_event".to_string()),
                target_entity_id: Some(event.id.clone()),
                params: None,
                prerequisites: None,
                status: initial_status.to_string(),
                execution_mode: v.clone(),
                attempts: 0,
                last_attempt_at: None,
                last_error: None,
                suggested_at: if initial_status == "suggested" { Some(chrono::Utc::now().to_rfc3339()) } else { None },
                approved_by: None,
                approved_at: None,
                rejection_reason: None,
                completed_at: None,
                result: None,
                idempotency_key: format!("event:{}:{}", event.id, event.start_date),
                created_at: chrono::Utc::now().to_rfc3339(),
                updated_at: chrono::Utc::now().to_rfc3339(),
            };
            let active_task: autopilot::entities::autopilot_task::ActiveModel = task.into();
            let _ = active_task.insert(registry_db).await.map_err(|e| CommandError::new(e.to_string()))?;
        }
    }

    let resolved_mode = request.execution_mode.unwrap_or(default_mode);
    let merged = merge_event_with_task(registry_db, event, &resolved_mode).await;
    Ok(merged)
}

#[tauri::command]
pub async fn delete_calendar_event(
    state: State<'_, Arc<AppState>>,
    event_id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    
    let existing = academic_calendar_event::Entity::find_by_id(&event_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Event not found".to_string()))?;

    let mut active: academic_calendar_event::ActiveModel = existing.into();
    active.is_active = Set(0);
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    
    Ok(())
}

#[tauri::command]
pub async fn execute_calendar_event(
    state: State<'_, Arc<AppState>>,
    event_id: String,
    user_id: String,
) -> CmdResult<EventProcessingResult> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let registry_db = state.provider.registry();

    let event = academic_calendar_event::Entity::find_by_id(&event_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Calendar event not found".to_string()))?;

    let result = execute_calendar_event_action(&db, &event, &user_id, registry_db)
        .await
        .map_err(CommandError::from)?;
        
    Ok(result)
}

#[tauri::command]
pub async fn generate_default_events(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
) -> CmdResult<Vec<serde_json::Value>> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let registry_db = state.provider.registry();

    // Get the cycle
    let cycle = domain_lifecycle::entities::academic_cycle::Entity::find_by_id(&cycle_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Academic cycle not found".to_string()))?;

    // Get default execution mode from policy
    let policy = org_academic_policy::Entity::find()
        .filter(org_academic_policy::Column::OrgId.eq(&org_id))
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .unwrap_or(org_academic_policy::Model {
            id: format!("{}_policy", org_id),
            org_id: org_id.clone(),
            default_grading_system: "saudi_5".to_string(),
            default_passing_grade: 50,
            default_max_grade: 100,
            enrollment_auto_close_on_capacity: 0,
            allow_late_enrollment: 1,
            late_enrollment_requires_approval: 1,
            grade_entry_auto_close_on_deadline: 0,
            allow_grade_modification_after_close: 0,
            grade_modification_requires_approval: 1,
            default_event_execution_mode: "suggested".to_string(),
            settings: None,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        });

    let mut created = Vec::new();
    let now = chrono::Utc::now().to_rfc3339();

    // Delete existing pending default events
    let events = academic_calendar_event::Entity::find()
        .filter(academic_calendar_event::Column::CycleId.eq(&cycle.id))
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    // Find completed task IDs
    let completed_event_ids: std::collections::HashSet<String> = autopilot::entities::autopilot_task::Entity::find()
        .filter(autopilot::entities::autopilot_task::Column::Status.eq("completed"))
        .all(registry_db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .into_iter()
        .flat_map(|t| t.calendar_event_id)
        .collect();

    // Delete events that have not been completed
    for event in events {
        if !completed_event_ids.contains(&event.id) {
            academic_calendar_event::Entity::delete_by_id(event.id)
                .exec(&db)
                .await
                .map_err(|e| CommandError::new(e.to_string()))?;
        }
    }

    // Start event
    let start_event = academic_calendar_event::Model {
        id: uuid::Uuid::new_v4().to_string(),
        org_id: org_id.clone(),
        cycle_id: cycle.id.clone(),
        title: format!("Start of {}", cycle.name),
        title_ar: Some(format!("بداية {}", cycle.name)),
        description: Some(format!("Academic cycle {} officially begins.", cycle.name)),
        event_type: "cycle_start".to_string(),
        start_date: cycle.start_date.clone(),
        end_date: cycle.start_date.clone(),
        start_time: None,
        end_time: None,
        is_recurring: 0,
        recurrence_rule: None,
        color: None,
        is_active: 1,
        sort_order: Some(0),
        created_by: None,
        created_at: now.clone(),
        updated_at: now.clone(),
    };
    let active_start: academic_calendar_event::ActiveModel = start_event.clone().into();
    active_start.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    
    let merged_start = merge_event_with_task(registry_db, start_event, &policy.default_event_execution_mode).await;
    created.push(merged_start);

    // End event
    let end_event = academic_calendar_event::Model {
        id: uuid::Uuid::new_v4().to_string(),
        org_id: org_id.clone(),
        cycle_id: cycle.id.clone(),
        title: format!("End of {}", cycle.name),
        title_ar: Some(format!("نهاية {}", cycle.name)),
        description: Some(format!("Academic cycle {} officially ends.", cycle.name)),
        event_type: "term_end".to_string(),
        start_date: cycle.end_date.clone(),
        end_date: cycle.end_date.clone(),
        start_time: None,
        end_time: None,
        is_recurring: 0,
        recurrence_rule: None,
        color: None,
        is_active: 1,
        sort_order: Some(0),
        created_by: None,
        created_at: now.clone(),
        updated_at: now,
    };
    let active_end: academic_calendar_event::ActiveModel = end_event.clone().into();
    active_end.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    
    let merged_end = merge_event_with_task(registry_db, end_event, &policy.default_event_execution_mode).await;
    created.push(merged_end);

    Ok(created)
}

#[tauri::command]
pub async fn process_pending_events(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    user_id: String,
) -> CmdResult<CalendarProcessingReport> {
    let db = state.active_db().await;
    let registry_db = state.provider.registry();
    let today = chrono::Local::now().date_naive().to_string();

    let config = autopilot::entities::autopilot_config::Entity::find()
        .filter(autopilot::entities::autopilot_config::Column::OrgId.eq(&org_id))
        .one(registry_db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    let default_mode = config.map(|c| c.mode).unwrap_or("suggested".to_string());

    // Fetch active events on or before today
    let active_events = academic_calendar_event::Entity::find()
        .filter(academic_calendar_event::Column::OrgId.eq(&org_id))
        .filter(academic_calendar_event::Column::StartDate.lte(&today))
        .filter(academic_calendar_event::Column::IsActive.eq(1))
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    // Fetch completed tasks
    let completed_event_ids: std::collections::HashSet<String> = autopilot::entities::autopilot_task::Entity::find()
        .filter(autopilot::entities::autopilot_task::Column::Status.eq("completed"))
        .all(registry_db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .into_iter()
        .flat_map(|t| t.calendar_event_id)
        .collect();

    // Filter pending events
    let pending: Vec<academic_calendar_event::Model> = active_events
        .into_iter()
        .filter(|e| !completed_event_ids.contains(&e.id))
        .collect();

    let mut report = CalendarProcessingReport::new();
    let is_writable = !state.provider.is_current_archived().await;

    for event in &pending {
        let execution_mode = if event.event_type == "backup" {
            "automatic".to_string()
        } else {
            default_mode.clone()
        };

        match execution_mode.as_str() {
            "automatic" => {
                if !is_writable {
                    report.blocked.push(EventProcessingResult {
                        event_id: event.id.clone(),
                        event_type: event.event_type.clone(),
                        title: event.title.clone(),
                        execution_mode: execution_mode.clone(),
                        outcome: "blocked".to_string(),
                        message: Some("Year is archived (read-only)".to_string()),
                    });
                    continue;
                }
                match execute_calendar_event_action(&db, event, &user_id, registry_db).await {
                    Ok(result) => report.executed.push(result),
                    Err(e) => report.blocked.push(EventProcessingResult {
                        event_id: event.id.clone(),
                        event_type: event.event_type.clone(),
                        title: event.title.clone(),
                        execution_mode: execution_mode.clone(),
                        outcome: "blocked".to_string(),
                        message: Some(e.to_string()),
                    }),
                }
            }
            "suggested" => {
                report.suggested.push(EventProcessingResult {
                    event_id: event.id.clone(),
                    event_type: event.event_type.clone(),
                    title: event.title.clone(),
                    execution_mode: execution_mode.clone(),
                    outcome: "suggested".to_string(),
                    message: Some(format!("Event '{}' is due. Review and execute.", event.title)),
                });
            }
            _ => {
                report.skipped.push(EventProcessingResult {
                    event_id: event.id.clone(),
                    event_type: event.event_type.clone(),
                    title: event.title.clone(),
                    execution_mode: execution_mode.clone(),
                    outcome: "skipped".to_string(),
                    message: Some("Manual mode — requires explicit user action".to_string()),
                });
            }
        }
    }

    Ok(report)
}

// ── Org Academic Policies Commands ─────────────────────────────────────────

#[tauri::command]
pub async fn get_org_academic_policy(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<org_academic_policy::Model> {
    let db = state.active_db().await;
    let policy = org_academic_policy::Entity::find()
        .filter(org_academic_policy::Column::OrgId.eq(&org_id))
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .unwrap_or(org_academic_policy::Model {
            id: format!("{}_policy", org_id),
            org_id: org_id.clone(),
            default_grading_system: "saudi_5".to_string(),
            default_passing_grade: 50,
            default_max_grade: 100,
            enrollment_auto_close_on_capacity: 0,
            allow_late_enrollment: 1,
            late_enrollment_requires_approval: 1,
            grade_entry_auto_close_on_deadline: 0,
            allow_grade_modification_after_close: 0,
            grade_modification_requires_approval: 1,
            default_event_execution_mode: "suggested".to_string(),
            settings: None,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        });
        
    Ok(policy)
}

#[tauri::command]
pub async fn update_org_academic_policy(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateOrgPolicyRequest,
) -> CmdResult<org_academic_policy::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    
    let existing = org_academic_policy::Entity::find()
        .filter(org_academic_policy::Column::OrgId.eq(&org_id))
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    let now = chrono::Utc::now().to_rfc3339();
    let updated = match existing {
        Some(policy) => {
            let mut active: org_academic_policy::ActiveModel = policy.into();
            active.default_grading_system = Set(request.default_grading_system);
            active.default_passing_grade = Set(request.default_passing_grade);
            active.default_max_grade = Set(request.default_max_grade);
            active.default_event_execution_mode = Set(request.default_event_execution_mode);
            active.updated_at = Set(now);
            active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?
        }
        None => {
            let model = org_academic_policy::Model {
                id: format!("{}_policy", org_id),
                org_id: org_id.clone(),
                default_grading_system: request.default_grading_system,
                default_passing_grade: request.default_passing_grade,
                default_max_grade: request.default_max_grade,
                enrollment_auto_close_on_capacity: 0,
                allow_late_enrollment: 1,
                late_enrollment_requires_approval: 1,
                grade_entry_auto_close_on_deadline: 0,
                allow_grade_modification_after_close: 0,
                grade_modification_requires_approval: 1,
                default_event_execution_mode: request.default_event_execution_mode,
                settings: None,
                created_at: now.clone(),
                updated_at: now,
            };
            let active: org_academic_policy::ActiveModel = model.into();
            let _ = active.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
            org_academic_policy::Entity::find_by_id(format!("{}_policy", org_id))
                .one(&db)
                .await
                .map_err(|e| CommandError::new(e.to_string()))?
                .ok_or_else(|| CommandError::from("Policy insert failed".to_string()))?
        }
    };
    
    Ok(updated)
}
