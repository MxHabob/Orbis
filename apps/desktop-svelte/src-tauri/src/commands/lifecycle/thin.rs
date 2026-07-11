/// Thin Tauri command wrappers for Platform/Lifecycle operations.
use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};
use svc_organization::ops;

// ── Organization ────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_organization(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::organization::get(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_main_organization(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    let db = state.active_db().await;
    let r = ops::organization::get_main(&db).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_organization(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::organization::update(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_settings(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::organization::get_settings(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_settings(state: State<'_, Arc<AppState>>, org_id: String, settings: serde_json::Value) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::organization::update_settings(&ctx, settings).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_dashboard_stats(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::dashboard::get_stats(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_setup_status(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<bool> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let res = ops::organization::check_setup(&ctx).await.map_err(CommandError::from)?;
    let completed = res.get("setup_completed").and_then(|v| v.as_bool()).unwrap_or(false);
    Ok(completed)
}

#[tauri::command]
pub async fn initialize_school_setup(_state: State<'_, Arc<AppState>>, _org_id: String, _payload: serde_json::Value) -> CmdResult<()> {
    Ok(())
}

// ── Academic Years ──────────────────────────────────────────────────

#[tauri::command]
pub async fn get_current_academic_year(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let reg_db = state.provider.registry();
    let year_id = state.provider.current_year_id().await;
    let r = infra_persistence::provider::Entity::find_by_id(year_id)
        .one(reg_db)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Current academic year not found"))?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_academic_years(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
    let reg_db = state.provider.registry();
    let r = infra_persistence::provider::Entity::find()
        .filter(infra_persistence::provider::Column::OrgId.eq(org_id))
        .all(reg_db)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn switch_academic_year(state: State<'_, Arc<AppState>>, year_id: String) -> CmdResult<()> {
    state.provider.switch_year(&year_id).await.map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn is_year_archived(state: State<'_, Arc<AppState>>) -> CmdResult<bool> {
    let archived = state.provider.is_current_archived().await;
    Ok(archived)
}

#[tauri::command]
pub async fn get_academic_context(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, QueryOrder};
    let db = state.active_db().await;
    
    // Get current year from organization
    let org = svc_organization::entities::organization::Entity::find_by_id(&org_id)
        .one(&db)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Organization not found"))?;

    let current_year_id = org.current_year_id.clone().unwrap_or_default();

    // Find current year
    let current_year = if !current_year_id.is_empty() {
        svc_organization::entities::academic_year::Entity::find_by_id(&current_year_id)
            .one(&db)
            .await
            .map_err(CommandError::from)?
    } else {
        None
    };

    // Find all years
    let all_years = svc_organization::entities::academic_year::Entity::find()
        .filter(svc_organization::entities::academic_year::Column::OrgId.eq(&org_id))
        .order_by_desc(svc_organization::entities::academic_year::Column::StartDate)
        .all(&db)
        .await
        .map_err(CommandError::from)?;

    // Find periods for current year
    let periods = if !current_year_id.is_empty() {
        svc_organization::entities::academic_period::Entity::find()
            .filter(svc_organization::entities::academic_period::Column::YearId.eq(&current_year_id))
            .order_by_asc(svc_organization::entities::academic_period::Column::SortOrder)
            .all(&db)
            .await
            .map_err(CommandError::from)?
    } else {
        vec![]
    };

    let (year_status, is_read_only) = match &current_year {
        Some(y) => {
            let is_arch = y.status == "archived";
            let is_locked = y.is_locked == 1;
            (y.status.clone(), is_arch || is_locked)
        }
        None => ("active".to_string(), false),
    };

    Ok(serde_json::json!({
        "current_year": current_year,
        "all_years": all_years,
        "periods": periods,
        "year_status": year_status,
        "is_read_only": is_read_only,
        "year_id": current_year_id,
    }))
}

// ── Promotion ───────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_promotion_jobs(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::promotion::list_jobs(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_promotion_job(state: State<'_, Arc<AppState>>, org_id: String, job_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::promotion::get_job(&ctx, &job_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_promotion_details(state: State<'_, Arc<AppState>>, _org_id: String, job_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
    use svc_organization::entities::promotion_detail;
    let db = state.active_db().await;
    let details = promotion_detail::Entity::find()
        .filter(promotion_detail::Column::JobId.eq(&job_id))
        .all(&db)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(details).unwrap())
}

#[tauri::command]
pub async fn promotion_dry_run(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    config: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    use svc_organization::services::promotion::PromotionConfig;
    let config_str = config.to_string();
    let parsed_config: PromotionConfig = serde_json::from_value(config)
        .map_err(|e| CommandError::new(format!("Invalid config: {}", e)))?;
    let source_year_id = parsed_config.source_year_id.clone();
    
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    
    // 1. Create a promotion job in registry
    let job = ops::promotion::create_job(&ctx, source_year_id, config_str)
        .await
        .map_err(CommandError::from)?;
        
    // 2. Run the dry-run on that job
    let _preview = ops::promotion::start_dry_run(&ctx, &job.id, parsed_config)
        .await
        .map_err(CommandError::from)?;
        
    // 3. Return the updated job (re-fetch it)
    let updated_job = ops::promotion::get_job(&ctx, &job.id)
        .await
        .map_err(CommandError::from)?;
        
    Ok(serde_json::to_value(updated_job).unwrap())
}

#[tauri::command]
pub async fn execute_promotion(state: State<'_, Arc<AppState>>, org_id: String, job_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::promotion::execute(&ctx, &job_id, state.provider.app_data_dir()).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn rollback_promotion(state: State<'_, Arc<AppState>>, org_id: String, job_id: String, _user_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::promotion::rollback(&ctx, &job_id, state.provider.app_data_dir()).await.map_err(CommandError::from)
}

#[derive(Debug, serde::Deserialize)]
pub struct OverrideRequest {
    pub detail_id: String,
    pub new_decision: String,
    pub reason: String,
    pub user_id: String,
}

#[tauri::command]
pub async fn override_promotion_decision(
    state: State<'_, Arc<AppState>>,
    _org_id: String,
    request: serde_json::Value,
) -> CmdResult<()> {
    use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
    use svc_organization::entities::promotion_detail;
    let req: OverrideRequest = serde_json::from_value(request)
        .map_err(|e| CommandError::new(format!("Invalid override request: {}", e)))?;
    let db = state.active_db().await;
    let detail = promotion_detail::Entity::find_by_id(req.detail_id)
        .one(&db)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Promotion detail not found"))?;

    let valid_decisions = ["promoted", "retained", "graduated", "excluded"];
    if !valid_decisions.contains(&req.new_decision.as_str()) {
        return Err(CommandError::new(format!(
            "Invalid decision: {}. Valid: {:?}",
            req.new_decision, valid_decisions
        )));
    }

    let mut active: promotion_detail::ActiveModel = detail.into();
    active.original_decision = Set(Some(active.decision.unwrap()));
    active.decision = Set(req.new_decision);
    active.manual_override = Set(1);
    active.override_by = Set(Some(req.user_id));
    active.override_reason = Set(Some(req.reason));
    
    active.update(&db).await.map_err(CommandError::from)?;
    Ok(())
}

#[tauri::command]
pub async fn get_promotion_policy(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
    use svc_organization::entities::academic_policy;
    let db = state.active_db().await;
    let policy = academic_policy::Entity::find()
        .filter(academic_policy::Column::OrgId.eq(&org_id))
        .one(&db)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(policy).unwrap())
}

// ── Calendar (periods-based — no more academic_calendar_events) ──

#[tauri::command]
pub async fn get_calendar_events(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    year_id: String,
) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, QueryOrder};
    use svc_organization::entities::academic_period;
    let db = state.active_db().await;
    let periods = academic_period::Entity::find()
        .filter(academic_period::Column::OrgId.eq(org_id))
        .filter(academic_period::Column::YearId.eq(year_id))
        .order_by_asc(academic_period::Column::SortOrder)
        .all(&db)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(periods).unwrap())
}

#[tauri::command]
pub async fn get_pending_calendar_events(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, QueryOrder};
    use svc_organization::entities::academic_period;
    let db = state.active_db().await;
    let today = chrono::Local::now().date_naive().to_string();
    // Return periods that are active and have pending actions
    let periods = academic_period::Entity::find()
        .filter(academic_period::Column::OrgId.eq(org_id))
        .filter(academic_period::Column::Status.is_not_in(["closed"]))
        .filter(academic_period::Column::StartDate.lte(&today))
        .order_by_asc(academic_period::Column::SortOrder)
        .all(&db)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(periods).unwrap())
}

#[tauri::command]
pub async fn create_calendar_event(
    _state: State<'_, Arc<AppState>>,
    _org_id: String,
    _data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    // Calendar events are now managed through academic_periods.
    // Create a new period instead.
    Ok(serde_json::json!({"message": "Use create_period instead. Calendar events are now managed through academic periods."}))
}

#[tauri::command]
pub async fn execute_calendar_event(
    _state: State<'_, Arc<AppState>>,
    _org_id: String,
    _event_id: String,
    _executed_by: String,
) -> CmdResult<serde_json::Value> {
    Ok(serde_json::json!({"message": "Calendar events deprecated. Use Autopilot tasks."}))
}

#[tauri::command]
pub async fn delete_calendar_event(
    _state: State<'_, Arc<AppState>>,
    _org_id: String,
    _event_id: String,
) -> CmdResult<()> {
    Ok(())
}

#[tauri::command]
pub async fn generate_default_calendar(
    _state: State<'_, Arc<AppState>>,
    _org_id: String,
    _year_id: String,
) -> CmdResult<()> {
    // Period templates now handle this. See period_templates entity.
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CreateOverrideRequest {
    pub override_type: String,
    pub risk_level: String,
    pub entity_type: String,
    pub entity_id: String,
    pub field_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub reason: String,
    pub requested_by: String,
    pub requested_by_name: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ApprovalAction {
    pub action: String, // "approve" | "reject"
    pub user_id: String,
    pub user_name: Option<String>,
    pub notes: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct OverrideListParams {
    pub status: Option<String>,
    pub override_type: Option<String>,
    pub requested_by: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[tauri::command]
pub async fn list_override_requests(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, QueryOrder, QuerySelect};
    use svc_organization::entities::academic_override_request;
    let list_params: OverrideListParams = serde_json::from_value(params)
        .unwrap_or(OverrideListParams { status: None, override_type: None, requested_by: None, limit: None, offset: None });
    let db = state.active_db().await;
    let mut query = academic_override_request::Entity::find()
        .filter(academic_override_request::Column::OrgId.eq(&org_id));

    if let Some(status) = list_params.status {
        query = query.filter(academic_override_request::Column::Status.eq(status));
    }
    if let Some(override_type) = list_params.override_type {
        query = query.filter(academic_override_request::Column::OverrideType.eq(override_type));
    }
    if let Some(requested_by) = list_params.requested_by {
        query = query.filter(academic_override_request::Column::RequestedBy.eq(requested_by));
    }
    
    query = query.order_by_desc(academic_override_request::Column::RequestedAt);

    if let Some(limit) = list_params.limit {
        query = query.limit(limit);
    }
    if let Some(offset) = list_params.offset {
        query = query.offset(offset);
    }

    let list = query.all(&db).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(list).unwrap())
}

#[tauri::command]
pub async fn get_override_request(
    state: State<'_, Arc<AppState>>,
    _org_id: String,
    request_id: String,
) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    use svc_organization::entities::academic_override_request;
    let db = state.active_db().await;
    let request = academic_override_request::Entity::find_by_id(request_id)
        .one(&db)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Request not found"))?;
    Ok(serde_json::to_value(request).unwrap())
}

#[tauri::command]
pub async fn create_override_request(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    use sea_orm::ActiveModelTrait;
    use svc_organization::entities::academic_override_request;
    let request: CreateOverrideRequest = serde_json::from_value(data)
        .map_err(|e| CommandError::new(format!("Invalid request data: {}", e)))?;
    let db = state.active_db().await;
    let now = chrono::Utc::now().to_rfc3339();
    
    let approval_chain = serde_json::json!([
        { "role": "admin", "status": "pending" }
    ]).to_string();

    let model = academic_override_request::Model {
        id: uuid::Uuid::new_v4().to_string(),
        org_id,
        override_type: request.override_type,
        risk_level: request.risk_level,
        entity_type: request.entity_type,
        entity_id: request.entity_id,
        field_name: request.field_name,
        old_value: request.old_value,
        new_value: request.new_value,
        reason: request.reason,
        requested_by: request.requested_by,
        requested_by_name: request.requested_by_name,
        requested_at: now.clone(),
        status: "pending".to_string(),
        current_approver_role: Some("admin".to_string()),
        approval_chain,
        resolved_by: None,
        resolved_by_name: None,
        resolved_at: None,
        resolution_notes: None,
        expires_at: request.expires_at,
        metadata: None,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: academic_override_request::ActiveModel = model.into();
    let created = active.insert(&db).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(created).unwrap())
}

#[tauri::command]
pub async fn process_approval(
    state: State<'_, Arc<AppState>>,
    _org_id: String,
    request_id: String,
    action: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
    use svc_organization::entities::academic_override_request;
    let act: ApprovalAction = serde_json::from_value(action)
        .map_err(|e| CommandError::new(format!("Invalid action data: {}", e)))?;
    let db = state.active_db().await;
    
    let existing = academic_override_request::Entity::find_by_id(&request_id)
        .one(&db)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Request not found"))?;

    let now = chrono::Utc::now().to_rfc3339();
    let mut active: academic_override_request::ActiveModel = existing.into();
    
    if act.action == "approve" {
        active.status = Set("approved".to_string());
        active.current_approver_role = Set(None);
    } else {
        active.status = Set("rejected".to_string());
        active.current_approver_role = Set(None);
    }
    
    active.resolved_by = Set(Some(act.user_id));
    active.resolved_by_name = Set(act.user_name);
    active.resolved_at = Set(Some(now.clone()));
    active.resolution_notes = Set(act.notes);
    active.updated_at = Set(now);

    let updated = active.update(&db).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(updated).unwrap())
}

#[tauri::command]
pub async fn cancel_override(
    state: State<'_, Arc<AppState>>,
    _org_id: String,
    request_id: String,
    user_id: String,
) -> CmdResult<serde_json::Value> {
    use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
    use svc_organization::entities::academic_override_request;
    let db = state.active_db().await;
    
    let existing = academic_override_request::Entity::find_by_id(&request_id)
        .one(&db)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Request not found"))?;

    let now = chrono::Utc::now().to_rfc3339();
    let mut active: academic_override_request::ActiveModel = existing.into();
    active.status = Set("cancelled".to_string());
    active.resolved_by = Set(Some(user_id));
    active.resolved_at = Set(Some(now.clone()));
    active.updated_at = Set(now);

    let updated = active.update(&db).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(updated).unwrap())
}

#[tauri::command]
pub async fn get_pending_approvals(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    approver_role: String,
) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
    use svc_organization::entities::academic_override_request;
    let db = state.active_db().await;
    let pending = academic_override_request::Entity::find()
        .filter(academic_override_request::Column::OrgId.eq(org_id))
        .filter(academic_override_request::Column::Status.eq("pending"))
        .filter(academic_override_request::Column::CurrentApproverRole.eq(approver_role))
        .all(&db)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(pending).unwrap())
}

// ── Policy (Unified: academic_policies) ────────────────────────────

#[tauri::command]
pub async fn get_org_policy(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
    use svc_organization::entities::academic_policy;
    let db = state.active_db().await;
    let policy = academic_policy::Entity::find()
        .filter(academic_policy::Column::OrgId.eq(&org_id))
        .one(&db)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(policy).unwrap())
}

#[tauri::command]
pub async fn update_org_policy(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<()> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, ActiveValue::Set};
    use svc_organization::entities::academic_policy;
    let db = state.active_db().await;
    let existing = academic_policy::Entity::find()
        .filter(academic_policy::Column::OrgId.eq(&org_id))
        .one(&db)
        .await
        .map_err(CommandError::from)?;

    let now = chrono::Utc::now().to_rfc3339();
    let mut active: academic_policy::ActiveModel = match existing {
        Some(m) => m.into(),
        None => {
            let mut a = <academic_policy::ActiveModel as std::default::Default>::default();
            a.id = Set(uuid::Uuid::new_v4().to_string());
            a.org_id = Set(org_id.clone());
            a.created_at = Set(now.clone());
            a
        }
    };

    // Grading defaults
    if let Some(val) = data.get("default_grading_system") {
        if let Some(s) = val.as_str() { active.default_grading_system = Set(s.to_string()); }
    }
    if let Some(val) = data.get("default_passing_grade") {
        if let Some(i) = val.as_i64() { active.default_passing_grade = Set(i as i32); }
    }
    if let Some(val) = data.get("default_max_grade") {
        if let Some(i) = val.as_i64() { active.default_max_grade = Set(i as i32); }
    }
    // Enrollment policies
    if let Some(val) = data.get("enrollment_auto_close") {
        if let Some(i) = val.as_i64() { active.enrollment_auto_close = Set(i as i32); }
    }
    if let Some(val) = data.get("allow_late_enrollment") {
        if let Some(i) = val.as_i64() { active.allow_late_enrollment = Set(i as i32); }
    }
    if let Some(val) = data.get("late_enrollment_approval") {
        if let Some(i) = val.as_i64() { active.late_enrollment_approval = Set(i as i32); }
    }
    // Grade entry policies
    if let Some(val) = data.get("grade_entry_auto_close") {
        if let Some(i) = val.as_i64() { active.grade_entry_auto_close = Set(i as i32); }
    }
    if let Some(val) = data.get("allow_grade_edit_after_close") {
        if let Some(i) = val.as_i64() { active.allow_grade_edit_after_close = Set(i as i32); }
    }
    if let Some(val) = data.get("grade_edit_approval") {
        if let Some(i) = val.as_i64() { active.grade_edit_approval = Set(i as i32); }
    }
    // Promotion policies
    if let Some(val) = data.get("passing_score") {
        if let Some(i) = val.as_i64() { active.passing_score = Set(i as i32); }
    }
    if let Some(val) = data.get("min_attendance_rate") {
        if let Some(f) = val.as_f64() { active.min_attendance_rate = Set(f); }
    }
    // Autopilot policies
    if let Some(val) = data.get("autopilot_mode") {
        if let Some(s) = val.as_str() { active.autopilot_mode = Set(s.to_string()); }
    }
    if let Some(val) = data.get("autopilot_enabled") {
        if let Some(i) = val.as_i64() { active.autopilot_enabled = Set(i as i32); }
    }
    active.updated_at = Set(now);

    active.save(&db).await.map_err(CommandError::from)?;
    Ok(())
}

// ── Document Templates ──────────────────────────────────────────────

#[tauri::command]
pub async fn get_document_templates(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: Option<serde_json::Value>,
) -> CmdResult<serde_json::Value> {
    use docforge_orbis::template_service::{TemplateService, ListTemplatesParams};
    let db = state.active_db().await;
    let p = params.unwrap_or(serde_json::json!({}));
    let list_params = ListTemplatesParams {
        org_id,
        status: p.get("status").and_then(|v| v.as_str().map(|s| s.to_string())),
        document_type: p.get("document_type").and_then(|v| v.as_str().map(|s| s.to_string())),
        entity_type: p.get("entity_type").and_then(|v| v.as_str().map(|s| s.to_string())),
        category_id: p.get("category_id").and_then(|v| v.as_str().map(|s| s.to_string())),
        search: p.get("search").and_then(|v| v.as_str().map(|s| s.to_string())),
    };
    let r = TemplateService::list(&db, list_params).await.map_err(|e| CommandError::new(e))?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    template_id: String,
) -> CmdResult<serde_json::Value> {
    use docforge_orbis::template_service::TemplateService;
    let db = state.active_db().await;
    let r = TemplateService::get_by_id(&db, &org_id, &template_id).await.map_err(|e| CommandError::new(e))?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    use docforge_orbis::template_service::{TemplateService, CreateTemplateRequest};
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let request: CreateTemplateRequest = serde_json::from_value(data)
        .map_err(|e| CommandError::new(format!("Invalid request data: {}", e)))?;
    let user_id = resolve_user_id(&db).await?;
    let r = TemplateService::create(&db, &org_id, &user_id, request).await.map_err(|e| CommandError::new(e))?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    template_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    use docforge_orbis::template_service::{TemplateService, UpdateTemplateRequest};
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let request: UpdateTemplateRequest = serde_json::from_value(data)
        .map_err(|e| CommandError::new(format!("Invalid request data: {}", e)))?;
    let user_id = resolve_user_id(&db).await?;
    let r = TemplateService::update(&db, &org_id, &template_id, &user_id, request).await.map_err(|e| CommandError::new(e))?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn delete_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    template_id: String,
) -> CmdResult<()> {
    use docforge_orbis::template_service::TemplateService;
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    TemplateService::delete(&db, &org_id, &template_id).await.map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn publish_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    template_id: String,
    _user_id: String,
) -> CmdResult<serde_json::Value> {
    use docforge_orbis::template_service::TemplateService;
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let user_id = resolve_user_id(&db).await?;
    let r = TemplateService::publish(&db, &org_id, &template_id, &user_id).await.map_err(|e| CommandError::new(e))?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn archive_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    template_id: String,
) -> CmdResult<serde_json::Value> {
    use docforge_orbis::template_service::TemplateService;
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let r = TemplateService::archive(&db, &org_id, &template_id).await.map_err(|e| CommandError::new(e))?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn restore_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    template_id: String,
) -> CmdResult<serde_json::Value> {
    use docforge_orbis::template_service::TemplateService;
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let r = TemplateService::restore(&db, &org_id, &template_id).await.map_err(|e| CommandError::new(e))?;
    Ok(serde_json::to_value(r).unwrap())
}

// ── Enrollment Status ───────────────────────────────────────────────

#[tauri::command]
pub async fn transition_enrollment_status(
    state: State<'_, Arc<AppState>>,
    _org_id: String,
    enrollment_id: String,
    status: String,
) -> CmdResult<serde_json::Value> {
    use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
    use svc_academic::entities::enrollment;
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let e = enrollment::Entity::find_by_id(&enrollment_id)
        .one(&db)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Enrollment not found"))?;

    let now = chrono::Utc::now().to_rfc3339();
    let mut active: enrollment::ActiveModel = e.into();
    active.status = Set(status.clone());
    if status == "withdrawn" {
        active.withdrawal_date = Set(Some(now.clone()));
    }
    active.updated_at = Set(now);

    let updated = active.update(&db).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(updated).unwrap())
}

// ── Import / Export ─────────────────────────────────────────────────

#[tauri::command]
pub async fn import_students(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::bulk_import::students(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn import_grades(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let grades_list = data.get("grades").cloned().unwrap_or(data);
    let r = svc_assessment::ops::grade::submit_bulk(&ctx, grades_list).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn export_students(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::bulk_export::students(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn export_grades(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
    use svc_assessment::entities::grade;
    let db = state.active_db().await;
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ctx.permissions.check(&ctx.actor, shared_kernel::traits::Permission::GradeRead).await.map_err(CommandError::from)?;

    let grades = grade::Entity::find()
        .filter(grade::Column::OrgId.eq(&org_id))
        .all(&db)
        .await
        .map_err(CommandError::from)?;

    Ok(serde_json::json!({
        "success": true,
        "message": "Bulk export of grades completed successfully",
        "data": grades
    }))
}

async fn resolve_user_id(db: &sea_orm::DatabaseConnection) -> Result<String, CommandError> {
    use sea_orm::{EntityTrait, QueryOrder};
    let session = svc_iam::entities::user_session::Entity::find()
        .order_by_desc(svc_iam::entities::user_session::Column::CreatedAt)
        .one(db)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::unauthorized("No active session"))?;
    Ok(session.user_id)
}
