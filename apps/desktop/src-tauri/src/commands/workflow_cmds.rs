//! Tauri IPC commands — Workflow & Academic Override Requests.

use super::make_ctx;
use super::org_cmds::ApiResponse;
use crate::state::AppState;
use orbis_app::services::SubmitOverrideInput;
use orbis_domain::entities::override_request::{
    AcademicOverrideRequest, ApprovalStep, OverrideStatus, OverrideType, RiskLevel,
};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse { data }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ApprovalStepDto {
    pub step_order: u32,
    pub approver_role: String,
    pub status: String,
    pub approver_id: Option<String>,
    pub approver_name: Option<String>,
    pub decision_at: Option<String>,
    pub notes: Option<String>,
}

#[derive(Serialize)]
pub struct OverrideRequestDto {
    pub id: String,
    pub org_id: String,
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
    pub status: String,
    pub current_approver_role: Option<String>,
    pub approval_chain: Vec<ApprovalStepDto>,
    pub resolved_by: Option<String>,
    pub resolved_by_name: Option<String>,
    pub resolved_at: Option<String>,
    pub resolution_notes: Option<String>,
    pub expires_at: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct SubmitOverrideRequestInput {
    pub override_type: String,
    #[serde(default = "default_risk")]
    pub risk_level: String,
    pub entity_type: String,
    pub entity_id: String,
    pub field_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub reason: String,
    #[serde(default)]
    pub approval_chain: Vec<ApprovalStepDto>,
    pub metadata: Option<serde_json::Value>,
}

fn default_risk() -> String {
    "medium".into()
}

#[derive(Deserialize)]
pub struct ApproveOverrideInput {
    pub request_id: String,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct RejectOverrideInput {
    pub request_id: String,
    pub reason: String,
}

fn override_to_dto(req: AcademicOverrideRequest) -> OverrideRequestDto {
    OverrideRequestDto {
        id: req.id.to_string(),
        org_id: req.org_id.to_string(),
        override_type: req.override_type.to_string(),
        risk_level: match req.risk_level {
            RiskLevel::Low => "low".into(),
            RiskLevel::Medium => "medium".into(),
            RiskLevel::High => "high".into(),
            RiskLevel::Critical => "critical".into(),
        },
        entity_type: req.entity_type,
        entity_id: req.entity_id.to_string(),
        field_name: req.field_name,
        old_value: req.old_value,
        new_value: req.new_value,
        reason: req.reason,
        requested_by: req.requested_by.to_string(),
        requested_by_name: req.requested_by_name,
        status: match req.status {
            OverrideStatus::Pending => "pending".into(),
            OverrideStatus::Approved => "approved".into(),
            OverrideStatus::Rejected => "rejected".into(),
            OverrideStatus::Executed => "executed".into(),
            OverrideStatus::Cancelled => "cancelled".into(),
            OverrideStatus::Expired => "expired".into(),
        },
        current_approver_role: req.current_approver_role,
        approval_chain: req
            .approval_chain
            .into_iter()
            .map(|s| ApprovalStepDto {
                step_order: s.step_order,
                approver_role: s.approver_role,
                status: match s.status {
                    OverrideStatus::Pending => "pending".into(),
                    OverrideStatus::Approved => "approved".into(),
                    OverrideStatus::Rejected => "rejected".into(),
                    OverrideStatus::Executed => "executed".into(),
                    OverrideStatus::Cancelled => "cancelled".into(),
                    OverrideStatus::Expired => "expired".into(),
                },
                approver_id: s.approver_id.map(|u| u.to_string()),
                approver_name: s.approver_name,
                decision_at: s.decision_at.map(|d| d.to_rfc3339()),
                notes: s.notes,
            })
            .collect(),
        resolved_by: req.resolved_by.map(|u| u.to_string()),
        resolved_by_name: req.resolved_by_name,
        resolved_at: req.resolved_at.map(|d| d.to_rfc3339()),
        resolution_notes: req.resolution_notes,
        expires_at: req.expires_at.map(|d| d.to_rfc3339()),
        metadata: req.metadata,
        created_at: req.created_at.to_rfc3339(),
        updated_at: req.updated_at.to_rfc3339(),
    }
}

fn parse_override_type(s: &str) -> OverrideType {
    match s.to_lowercase().as_str() {
        "grade_change" | "grade_modification" => OverrideType::GradeChange,
        "prerequisite_bypass" => OverrideType::PrerequisiteBypass,
        "late_enrollment" => OverrideType::LateEnrollment,
        "capacity_override" => OverrideType::CapacityOverride,
        "credit_overload" => OverrideType::CreditOverload,
        "discount_override" | "fee_waiver" => OverrideType::DiscountOverride,
        "standing_override" | "attendance_exemption" => OverrideType::AcademicStandingOverride,
        _ => OverrideType::Custom(1),
    }
}

fn parse_risk_level(s: &str) -> RiskLevel {
    match s.to_lowercase().as_str() {
        "low" => RiskLevel::Low,
        "high" => RiskLevel::High,
        "critical" => RiskLevel::Critical,
        _ => RiskLevel::Medium,
    }
}

#[tauri::command]
pub async fn workflow_submit_override(
    request: SubmitOverrideRequestInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<OverrideRequestDto>, String> {
    let ctx = make_ctx(&state);
    let entity_id = Uuid::parse_str(&request.entity_id).map_err(|_| "Invalid entityId")?;
    let requested_by = match ctx.actor {
        orbis_domain::iam::actor::Actor::User(uid) => uid,
        _ => ctx.tenant_id,
    };

    let chain = if request.approval_chain.is_empty() {
        vec![ApprovalStep {
            step_order: 1,
            approver_role: "admin".into(),
            status: OverrideStatus::Pending,
            approver_id: None,
            approver_name: None,
            decision_at: None,
            notes: None,
        }]
    } else {
        request
            .approval_chain
            .into_iter()
            .map(|s| ApprovalStep {
                step_order: s.step_order,
                approver_role: s.approver_role,
                status: OverrideStatus::Pending,
                approver_id: s.approver_id.and_then(|u| Uuid::parse_str(&u).ok()),
                approver_name: s.approver_name,
                decision_at: None,
                notes: s.notes,
            })
            .collect()
    };

    let input = SubmitOverrideInput {
        override_type: parse_override_type(&request.override_type),
        risk_level: parse_risk_level(&request.risk_level),
        entity_type: request.entity_type,
        entity_id,
        field_name: request.field_name,
        old_value: request.old_value,
        new_value: request.new_value,
        reason: request.reason,
        requested_by,
        requested_by_name: None,
        approval_chain: chain,
        metadata: request.metadata,
    };

    let result = state
        .workflow_service
        .submit_override(&ctx, ctx.tenant_id, input)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(override_to_dto(result)))
}

#[tauri::command]
pub async fn workflow_approve_override(
    request: ApproveOverrideInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<OverrideRequestDto>, String> {
    let ctx = make_ctx(&state);
    let req_id = Uuid::parse_str(&request.request_id).map_err(|_| "Invalid requestId")?;
    let approver_id = match ctx.actor {
        orbis_domain::iam::actor::Actor::User(uid) => uid,
        _ => ctx.tenant_id,
    };

    let result = state
        .workflow_service
        .approve_override(&ctx, req_id, approver_id, None, request.notes)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(override_to_dto(result)))
}

#[tauri::command]
pub async fn workflow_reject_override(
    request: RejectOverrideInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<OverrideRequestDto>, String> {
    let ctx = make_ctx(&state);
    let req_id = Uuid::parse_str(&request.request_id).map_err(|_| "Invalid requestId")?;
    let rejecter_id = match ctx.actor {
        orbis_domain::iam::actor::Actor::User(uid) => uid,
        _ => ctx.tenant_id,
    };

    let result = state
        .workflow_service
        .reject_override(&ctx, req_id, rejecter_id, None, request.reason)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(override_to_dto(result)))
}

#[tauri::command]
pub async fn workflow_list_overrides(
    status: Option<String>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<OverrideRequestDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state
        .workflow_service
        .list_overrides(ctx.tenant_id, status.as_deref())
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(items.into_iter().map(override_to_dto).collect()))
}

#[tauri::command]
pub async fn workflow_get_override(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<OverrideRequestDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let item = state
        .workflow_service
        .get_override(uid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Override request not found".to_string())?;

    Ok(ok(override_to_dto(item)))
}
