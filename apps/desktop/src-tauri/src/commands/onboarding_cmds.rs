//! Onboarding Tauri IPC commands — post-bootstrap institution configuration & DAG readiness.
//!
//! These commands provide 100% parity with the HTTP `/api/v1/onboarding/*` endpoints,
//! running directly against the local SQLite/PostgreSQL engine via Tauri IPC.

use super::make_ctx;
use crate::state::AppState;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

// ── DTOs ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupTaskDto {
    pub id: String,
    pub category: String,
    pub title: String,
    pub title_ar: String,
    pub description: String,
    pub description_ar: String,
    pub is_required: bool,
    pub status: String,
    pub dependencies: Vec<String>,
    pub progress: u8,
    pub route: String,
    pub action_type: String,
    pub institution_types: Vec<String>,
    pub required_permissions: Vec<String>,
    pub completion_condition: String,
    pub blocking_reason: Option<String>,
    pub blocking_reason_ar: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryProgressDto {
    pub category: String,
    pub label: String,
    pub label_ar: String,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub progress_percentage: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingStateDto {
    pub org_id: String,
    pub org_name: String,
    pub institution_type: String,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub required_tasks: usize,
    pub required_completed: usize,
    pub optional_tasks: usize,
    pub optional_completed: usize,
    pub blocked_tasks: usize,
    pub progress_percentage: u8,
    pub required_progress_percentage: u8,
    pub is_institution_ready: bool,
    pub tasks: Vec<SetupTaskDto>,
    pub categories: Vec<CategoryProgressDto>,
    pub recommended_next_task: Option<SetupTaskDto>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProvisionDefaultsReqDto {
    pub academic_year_name: Option<String>,
    pub academic_year_name_ar: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub provision_stages_and_grades: Option<bool>,
    pub provision_subjects: Option<bool>,
    pub provision_assessment_scheme: Option<bool>,
    pub provision_tuition_fee_template: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProvisionDefaultsResultDto {
    pub success: bool,
    pub created_items: Vec<String>,
    pub state: OnboardingStateDto,
}

fn map_task_to_dto(t: orbis_app::services::SetupTask) -> SetupTaskDto {
    SetupTaskDto {
        id: t.id,
        category: match t.category {
            orbis_app::services::OnboardingTaskCategory::Organization => "organization".to_string(),
            orbis_app::services::OnboardingTaskCategory::Calendar => "calendar".to_string(),
            orbis_app::services::OnboardingTaskCategory::Academics => "academics".to_string(),
            orbis_app::services::OnboardingTaskCategory::Assessment => "assessment".to_string(),
            orbis_app::services::OnboardingTaskCategory::Finance => "finance".to_string(),
            orbis_app::services::OnboardingTaskCategory::Students => "students".to_string(),
            orbis_app::services::OnboardingTaskCategory::Templates => "templates".to_string(),
            orbis_app::services::OnboardingTaskCategory::Integrations => "integrations".to_string(),
        },
        title: t.title,
        title_ar: t.title_ar,
        description: t.description,
        description_ar: t.description_ar,
        is_required: t.is_required,
        status: match t.status {
            orbis_app::services::OnboardingTaskStatus::NotStarted => "not_started".to_string(),
            orbis_app::services::OnboardingTaskStatus::InProgress => "in_progress".to_string(),
            orbis_app::services::OnboardingTaskStatus::Completed => "completed".to_string(),
            orbis_app::services::OnboardingTaskStatus::Blocked => "blocked".to_string(),
            orbis_app::services::OnboardingTaskStatus::Optional => "optional".to_string(),
            orbis_app::services::OnboardingTaskStatus::NotApplicable => {
                "not_applicable".to_string()
            }
        },
        dependencies: t.dependencies,
        progress: t.progress,
        route: t.route,
        action_type: t.action_type,
        institution_types: t.institution_types,
        required_permissions: t.required_permissions,
        completion_condition: t.completion_condition,
        blocking_reason: t.blocking_reason,
        blocking_reason_ar: t.blocking_reason_ar,
        metadata: t.metadata,
    }
}

fn map_state_to_dto(s: orbis_app::services::OnboardingStateResponse) -> OnboardingStateDto {
    OnboardingStateDto {
        org_id: s.org_id.to_string(),
        org_name: s.org_name,
        institution_type: s.institution_type,
        total_tasks: s.total_tasks,
        completed_tasks: s.completed_tasks,
        required_tasks: s.required_tasks,
        required_completed: s.required_completed,
        optional_tasks: s.optional_tasks,
        optional_completed: s.optional_completed,
        blocked_tasks: s.blocked_tasks,
        progress_percentage: s.progress_percentage,
        required_progress_percentage: s.required_progress_percentage,
        is_institution_ready: s.is_institution_ready,
        tasks: s.tasks.into_iter().map(map_task_to_dto).collect(),
        categories: s
            .categories
            .into_iter()
            .map(|c| CategoryProgressDto {
                category: match c.category {
                    orbis_app::services::OnboardingTaskCategory::Organization => {
                        "organization".to_string()
                    }
                    orbis_app::services::OnboardingTaskCategory::Calendar => "calendar".to_string(),
                    orbis_app::services::OnboardingTaskCategory::Academics => {
                        "academics".to_string()
                    }
                    orbis_app::services::OnboardingTaskCategory::Assessment => {
                        "assessment".to_string()
                    }
                    orbis_app::services::OnboardingTaskCategory::Finance => "finance".to_string(),
                    orbis_app::services::OnboardingTaskCategory::Students => "students".to_string(),
                    orbis_app::services::OnboardingTaskCategory::Templates => {
                        "templates".to_string()
                    }
                    orbis_app::services::OnboardingTaskCategory::Integrations => {
                        "integrations".to_string()
                    }
                },
                label: c.label,
                label_ar: c.label_ar,
                total_tasks: c.total_tasks,
                completed_tasks: c.completed_tasks,
                progress_percentage: c.progress_percentage,
            })
            .collect(),
        recommended_next_task: s.recommended_next_task.map(map_task_to_dto),
        warnings: s.warnings,
    }
}

// ── IPC Handlers ────────────────────────────────────────────────

/// Get active onboarding state and DAG task progress for the current organization.
#[tauri::command]
pub async fn onboarding_get_state(
    state: State<'_, AppState>,
    org_id: Option<String>,
) -> Result<OnboardingStateDto, String> {
    let ctx = make_ctx(&state);
    let target_org_id = if let Some(id_str) = org_id {
        Uuid::parse_str(&id_str).map_err(|e| format!("Invalid org_id UUID: {e}"))?
    } else if ctx.tenant_id != Uuid::nil() {
        ctx.tenant_id
    } else {
        // Fallback: fetch first org in the local database
        let orgs = state.org_repo.list_all().await.map_err(|e| e.to_string())?;
        orgs.first()
            .map(|o| o.id)
            .ok_or_else(|| "No organization exists to onboard".to_string())?
    };

    let res = state
        .onboarding_service
        .get_state(target_org_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(map_state_to_dto(res))
}

/// Provision standard blueprint defaults scaffolding.
#[tauri::command]
pub async fn onboarding_provision_defaults(
    state: State<'_, AppState>,
    org_id: Option<String>,
    request: ProvisionDefaultsReqDto,
) -> Result<ProvisionDefaultsResultDto, String> {
    let ctx = make_ctx(&state);
    let target_org_id = if let Some(id_str) = org_id {
        Uuid::parse_str(&id_str).map_err(|e| format!("Invalid org_id UUID: {e}"))?
    } else if ctx.tenant_id != Uuid::nil() {
        ctx.tenant_id
    } else {
        let orgs = state.org_repo.list_all().await.map_err(|e| e.to_string())?;
        orgs.first()
            .map(|o| o.id)
            .ok_or_else(|| "No organization exists to onboard".to_string())?
    };

    let service_req = orbis_app::services::ProvisionDefaultsRequest {
        academic_year_name: request.academic_year_name,
        academic_year_name_ar: request.academic_year_name_ar,
        start_date: request.start_date,
        end_date: request.end_date,
        provision_stages_and_grades: request.provision_stages_and_grades,
        provision_subjects: request.provision_subjects,
        provision_assessment_scheme: request.provision_assessment_scheme,
        provision_tuition_fee_template: request.provision_tuition_fee_template,
    };

    let result = state
        .onboarding_service
        .provision_blueprint_defaults(&ctx, target_org_id, service_req)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ProvisionDefaultsResultDto {
        success: result.success,
        created_items: result.created_items,
        state: map_state_to_dto(result.state),
    })
}
