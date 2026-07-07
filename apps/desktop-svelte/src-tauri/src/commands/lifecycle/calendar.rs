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
) -> Result<EventProcessingResult, shared_kernel::errors::AppError> {
    // 1. Update event status
    let mut active: academic_calendar_event::ActiveModel = event.clone().into();
    active.executed_at = Set(Some(chrono::Utc::now().to_rfc3339()));
    active.executed_by = Set(Some(user_id.to_string()));
    active.update(db).await?;

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

    Ok(EventProcessingResult {
        event_id: event.id.clone(),
        event_type: event.event_type.clone(),
        title: event.title.clone(),
        execution_mode: event.execution_mode.clone(),
        outcome: "executed".to_string(),
        message: Some("Executed successfully".to_string()),
    })
}

// ── Calendar Events Commands ──────────────────────────────────────────────

#[tauri::command]
pub async fn get_calendar_events(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
) -> CmdResult<Vec<academic_calendar_event::Model>> {
    let db = state.active_db().await;
    
    // Auto-execute any due automatic events for this cycle
    let is_writable = !state.provider.is_current_archived().await;
    let today = chrono::Local::now().date_naive().to_string();
    
    if is_writable {
        let pending = academic_calendar_event::Entity::find()
            .filter(academic_calendar_event::Column::OrgId.eq(&org_id))
            .filter(academic_calendar_event::Column::CycleId.eq(&cycle_id))
            .filter(academic_calendar_event::Column::ExecutedAt.is_null())
            .filter(academic_calendar_event::Column::StartDate.lte(&today))
            .filter(academic_calendar_event::Column::IsActive.eq(1))
            .all(&db)
            .await
            .map_err(|e| CommandError::new(e.to_string()))?;

        for event in pending {
            if event.execution_mode == "automatic" {
                let _ = execute_calendar_event_action(&db, &event, "system").await;
            }
        }
    }

    let events = academic_calendar_event::Entity::find()
        .filter(academic_calendar_event::Column::OrgId.eq(org_id))
        .filter(academic_calendar_event::Column::CycleId.eq(cycle_id))
        .filter(academic_calendar_event::Column::IsActive.eq(1))
        .order_by_asc(academic_calendar_event::Column::StartDate)
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    Ok(events)
}

#[tauri::command]
pub async fn get_pending_calendar_events(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<academic_calendar_event::Model>> {
    let db = state.active_db().await;
    let today = chrono::Local::now().date_naive().to_string();
    
    let pending = academic_calendar_event::Entity::find()
        .filter(academic_calendar_event::Column::OrgId.eq(org_id))
        .filter(academic_calendar_event::Column::ExecutedAt.is_null())
        .filter(academic_calendar_event::Column::StartDate.lte(&today))
        .filter(academic_calendar_event::Column::IsActive.eq(1))
        .order_by_asc(academic_calendar_event::Column::StartDate)
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    Ok(pending)
}

#[tauri::command]
pub async fn create_calendar_event(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateCalendarEventRequest,
) -> CmdResult<academic_calendar_event::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    
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
        execution_mode: request.execution_mode,
        auto_action: None,
        prerequisites: None,
        executed_at: None,
        executed_by: None,
        execution_result: None,
        execution_notes: None,
        is_recurring: 0,
        recurrence_rule: None,
        color: None,
        is_active: 1,
        sort_order: Some(0),
        created_by: None,
        created_at: now.clone(),
        updated_at: now,
    };
    
    let active: academic_calendar_event::ActiveModel = model.clone().into();
    let event = active.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;

    // If automatic and due now, execute it
    let today = chrono::Local::now().date_naive().to_string();
    if event.execution_mode == "automatic" && event.start_date <= today {
        let _ = execute_calendar_event_action(&db, &event, "system").await;
        if let Ok(Some(updated)) = academic_calendar_event::Entity::find_by_id(&event.id).one(&db).await {
            return Ok(updated);
        }
    }

    Ok(event)
}

#[tauri::command]
pub async fn update_calendar_event(
    state: State<'_, Arc<AppState>>,
    event_id: String,
    request: UpdateCalendarEventRequest,
) -> CmdResult<academic_calendar_event::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    
    let existing = academic_calendar_event::Entity::find_by_id(&event_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Event not found".to_string()))?;

    let mut active: academic_calendar_event::ActiveModel = existing.into();
    if let Some(v) = request.title { active.title = Set(v); }
    if let Some(v) = request.description { active.description = Set(Some(v)); }
    if let Some(v) = request.start_date { active.start_date = Set(v); }
    if let Some(v) = request.end_date { active.end_date = Set(v); }
    if let Some(v) = request.execution_mode { active.execution_mode = Set(v); }
    if let Some(v) = request.status {
        if v == "executed" {
            active.executed_at = Set(Some(chrono::Utc::now().to_rfc3339()));
        } else {
            active.executed_at = Set(None);
        }
    }
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    
    let event = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;

    // If automatic and due now, execute it
    let today = chrono::Local::now().date_naive().to_string();
    if event.execution_mode == "automatic" && event.start_date <= today && event.executed_at.is_none() {
        let _ = execute_calendar_event_action(&db, &event, "system").await;
        if let Ok(Some(updated)) = academic_calendar_event::Entity::find_by_id(&event.id).one(&db).await {
            return Ok(updated);
        }
    }

    Ok(event)
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

    let event = academic_calendar_event::Entity::find_by_id(&event_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Calendar event not found".to_string()))?;

    let result = execute_calendar_event_action(&db, &event, &user_id)
        .await
        .map_err(CommandError::from)?;
        
    Ok(result)
}

#[tauri::command]
pub async fn generate_default_events(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
) -> CmdResult<Vec<academic_calendar_event::Model>> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;

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
    academic_calendar_event::Entity::delete_many()
        .filter(academic_calendar_event::Column::CycleId.eq(&cycle.id))
        .filter(academic_calendar_event::Column::ExecutedAt.is_null())
        .exec(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

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
        execution_mode: policy.default_event_execution_mode.clone(),
        auto_action: None,
        prerequisites: None,
        executed_at: None,
        executed_by: None,
        execution_result: None,
        execution_notes: None,
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
    created.push(start_event);

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
        execution_mode: policy.default_event_execution_mode,
        auto_action: None,
        prerequisites: None,
        executed_at: None,
        executed_by: None,
        execution_result: None,
        execution_notes: None,
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
    created.push(end_event);

    Ok(created)
}

#[tauri::command]
pub async fn process_pending_events(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    user_id: String,
) -> CmdResult<CalendarProcessingReport> {
    let db = state.active_db().await;
    let today = chrono::Local::now().date_naive().to_string();

    let pending = academic_calendar_event::Entity::find()
        .filter(academic_calendar_event::Column::OrgId.eq(org_id))
        .filter(academic_calendar_event::Column::ExecutedAt.is_null())
        .filter(academic_calendar_event::Column::StartDate.lte(&today))
        .filter(academic_calendar_event::Column::IsActive.eq(1))
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    let mut report = CalendarProcessingReport::new();
    let is_writable = !state.provider.is_current_archived().await;

    for event in &pending {
        match event.execution_mode.as_str() {
            "automatic" => {
                if !is_writable {
                    report.blocked.push(EventProcessingResult {
                        event_id: event.id.clone(),
                        event_type: event.event_type.clone(),
                        title: event.title.clone(),
                        execution_mode: event.execution_mode.clone(),
                        outcome: "blocked".to_string(),
                        message: Some("Year is archived (read-only)".to_string()),
                    });
                    continue;
                }
                match execute_calendar_event_action(&db, event, &user_id).await {
                    Ok(result) => report.executed.push(result),
                    Err(e) => report.blocked.push(EventProcessingResult {
                        event_id: event.id.clone(),
                        event_type: event.event_type.clone(),
                        title: event.title.clone(),
                        execution_mode: event.execution_mode.clone(),
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
                    execution_mode: event.execution_mode.clone(),
                    outcome: "suggested".to_string(),
                    message: Some(format!("Event '{}' is due. Review and execute.", event.title)),
                });
            }
            _ => {
                report.skipped.push(EventProcessingResult {
                    event_id: event.id.clone(),
                    event_type: event.event_type.clone(),
                    title: event.title.clone(),
                    execution_mode: event.execution_mode.clone(),
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
