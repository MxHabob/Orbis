//! Tauri IPC commands — Student Cohort Progression & Promotion.

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use orbis_domain::entities::progression::{
    PromotionJob, PromotionJobStatus, PromotionDecision,
};
use orbis_app::services::progression_service::StudentProgressionItem;
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct PromotionJobDto {
    pub id: String,
    pub org_id: String,
    pub source_period_id: String,
    pub target_period_id: Option<String>,
    pub name: String,
    pub status: String,
    pub total_students: u32,
    pub promoted_count: u32,
    pub retained_count: u32,
    pub graduated_count: u32,
    pub excluded_count: u32,
    pub executed_by: Option<String>,
    pub executed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct StudentProgressionItemInput {
    #[serde(rename = "studentId")]
    pub student_id: String,
    #[serde(rename = "enrollmentId")]
    pub enrollment_id: String,
    #[serde(rename = "sourceClassId")]
    pub source_class_id: Option<String>,
    #[serde(rename = "targetClassId")]
    pub target_class_id: Option<String>,
    pub decision: String,
    #[serde(default)]
    pub reason: String,
}

#[derive(Deserialize)]
pub struct PromoteCohortInput {
    #[serde(rename = "sourcePeriodId")]
    pub source_period_id: String,
    #[serde(rename = "targetPeriodId")]
    pub target_period_id: String,
    #[serde(rename = "jobName")]
    pub job_name: String,
    pub items: Vec<StudentProgressionItemInput>,
}

fn job_to_dto(job: PromotionJob) -> PromotionJobDto {
    PromotionJobDto {
        id: job.id.to_string(),
        org_id: job.org_id.to_string(),
        source_period_id: job.source_period_id.to_string(),
        target_period_id: job.target_period_id.map(|t| t.to_string()),
        name: job.name,
        status: match job.status {
            PromotionJobStatus::Draft => "draft".into(),
            PromotionJobStatus::PreviewReady => "preview_ready".into(),
            PromotionJobStatus::Executing => "executing".into(),
            PromotionJobStatus::Completed => "completed".into(),
            PromotionJobStatus::Failed => "failed".into(),
            PromotionJobStatus::RolledBack => "rolled_back".into(),
        },
        total_students: job.total_students,
        promoted_count: job.promoted_count,
        retained_count: job.retained_count,
        graduated_count: job.graduated_count,
        excluded_count: job.excluded_count,
        executed_by: job.executed_by.map(|u| u.to_string()),
        executed_at: job.executed_at.map(|d| d.to_rfc3339()),
        created_at: job.created_at.to_rfc3339(),
        updated_at: job.updated_at.to_rfc3339(),
    }
}

fn parse_decision(s: &str) -> PromotionDecision {
    match s.to_lowercase().as_str() {
        "retain" | "retained" => PromotionDecision::Retain,
        "graduate" | "graduated" => PromotionDecision::Graduate,
        "exclude" | "excluded" => PromotionDecision::Exclude,
        _ => PromotionDecision::Promote,
    }
}

#[tauri::command]
pub async fn progression_promote_cohort(
    request: PromoteCohortInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<PromotionJobDto>, String> {
    let ctx = make_ctx(&state);
    let source_period_id = Uuid::parse_str(&request.source_period_id).map_err(|_| "Invalid sourcePeriodId")?;
    let target_period_id = Uuid::parse_str(&request.target_period_id).map_err(|_| "Invalid targetPeriodId")?;

    let mut items = Vec::new();
    for item in request.items {
        let student_id = Uuid::parse_str(&item.student_id).map_err(|_| "Invalid studentId")?;
        let enrollment_id = Uuid::parse_str(&item.enrollment_id).map_err(|_| "Invalid enrollmentId")?;
        let source_class_id = item.source_class_id.and_then(|c| Uuid::parse_str(&c).ok());
        let target_class_id = item.target_class_id.and_then(|c| Uuid::parse_str(&c).ok());

        items.push(StudentProgressionItem {
            student_id,
            enrollment_id,
            source_class_id,
            target_class_id,
            decision: parse_decision(&item.decision),
            reason: item.reason,
        });
    }

    let job = state.progression_service
        .promote_cohort(
            &ctx,
            ctx.tenant_id,
            source_period_id,
            target_period_id,
            request.job_name,
            items,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(job_to_dto(job)))
}

#[tauri::command]
pub async fn progression_list_jobs(
    _state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<PromotionJobDto>>, String> {
    // List jobs
    Ok(ok(vec![]))
}

#[tauri::command]
pub async fn progression_get_job(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<PromotionJobDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let item = state.progression_service
        .progression_repo
        .find_job_by_id(uid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Promotion job not found".to_string())?;

    Ok(ok(job_to_dto(item)))
}
