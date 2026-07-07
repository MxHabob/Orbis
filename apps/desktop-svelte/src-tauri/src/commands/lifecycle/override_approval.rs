use tauri::State;
use std::sync::Arc;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set, QueryOrder, QuerySelect};
use domain_lifecycle::entities::academic_override_request;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// ── Request/Response Types ──────────────────────────────────────────

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

// ── Commands ────────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_override_request(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateOverrideRequest,
) -> CmdResult<academic_override_request::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
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
    let created = active.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(created)
}

#[tauri::command]
pub async fn process_approval(
    state: State<'_, Arc<AppState>>,
    request_id: String,
    action: ApprovalAction,
) -> CmdResult<academic_override_request::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    
    let existing = academic_override_request::Entity::find_by_id(&request_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Request not found".to_string()))?;

    let now = chrono::Utc::now().to_rfc3339();
    let mut active: academic_override_request::ActiveModel = existing.into();
    
    if action.action == "approve" {
        active.status = Set("approved".to_string());
        active.current_approver_role = Set(None);
    } else {
        active.status = Set("rejected".to_string());
        active.current_approver_role = Set(None);
    }
    
    active.resolved_by = Set(Some(action.user_id));
    active.resolved_by_name = Set(action.user_name);
    active.resolved_at = Set(Some(now.clone()));
    active.resolution_notes = Set(action.notes);
    active.updated_at = Set(now);

    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn cancel_override_request(
    state: State<'_, Arc<AppState>>,
    request_id: String,
    user_id: String,
) -> CmdResult<academic_override_request::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    
    let existing = academic_override_request::Entity::find_by_id(&request_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Request not found".to_string()))?;

    let now = chrono::Utc::now().to_rfc3339();
    let mut active: academic_override_request::ActiveModel = existing.into();
    active.status = Set("cancelled".to_string());
    active.resolved_by = Set(Some(user_id));
    active.resolved_at = Set(Some(now.clone()));
    active.updated_at = Set(now);

    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn list_override_requests(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: OverrideListParams,
) -> CmdResult<Vec<academic_override_request::Model>> {
    let db = state.active_db().await;
    let mut query = academic_override_request::Entity::find()
        .filter(academic_override_request::Column::OrgId.eq(org_id));

    if let Some(status) = params.status {
        query = query.filter(academic_override_request::Column::Status.eq(status));
    }
    if let Some(override_type) = params.override_type {
        query = query.filter(academic_override_request::Column::OverrideType.eq(override_type));
    }
    if let Some(requested_by) = params.requested_by {
        query = query.filter(academic_override_request::Column::RequestedBy.eq(requested_by));
    }
    
    query = query.order_by_desc(academic_override_request::Column::RequestedAt);

    if let Some(limit) = params.limit {
        query = query.limit(limit);
    }
    if let Some(offset) = params.offset {
        query = query.offset(offset);
    }

    let list = query.all(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(list)
}

#[tauri::command]
pub async fn get_override_request(
    state: State<'_, Arc<AppState>>,
    request_id: String,
) -> CmdResult<academic_override_request::Model> {
    let db = state.active_db().await;
    let request = academic_override_request::Entity::find_by_id(request_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Request not found".to_string()))?;
    Ok(request)
}

#[tauri::command]
pub async fn get_pending_approvals(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    approver_role: String,
) -> CmdResult<Vec<academic_override_request::Model>> {
    let db = state.active_db().await;
    let pending = academic_override_request::Entity::find()
        .filter(academic_override_request::Column::OrgId.eq(org_id))
        .filter(academic_override_request::Column::Status.eq("pending"))
        .filter(academic_override_request::Column::CurrentApproverRole.eq(approver_role))
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(pending)
}
