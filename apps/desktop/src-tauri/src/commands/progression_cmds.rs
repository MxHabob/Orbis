//! Tauri IPC commands — Student Cohort Progression & Promotion.

use super::make_ctx;
use super::org_cmds::ApiResponse;
use crate::state::AppState;
use orbis_app::services::progression_service::StudentProgressionItem;
use orbis_domain::entities::progression::{PromotionDecision, ProgressionJob, ProgressionJobStatus};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse { data }
}

#[derive(Serialize)]
pub struct ProgressionJobDto {
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

#[derive(Serialize, Deserialize)]
pub struct StudentProgressionItemInput {
    pub student_id: String,
    pub enrollment_id: String,
    pub source_class_id: Option<String>,
    pub target_class_id: Option<String>,
    pub decision: String,
    #[serde(default)]
    pub reason: String,
}

#[derive(Deserialize)]
pub struct PromoteCohortInput {
    pub source_period_id: String,
    pub target_period_id: String,
    pub job_name: String,
    pub items: Vec<StudentProgressionItemInput>,
}

fn job_to_dto(job: ProgressionJob) -> ProgressionJobDto {
    ProgressionJobDto {
        id: job.id.to_string(),
        org_id: job.org_id.to_string(),
        source_period_id: job.source_period_id.to_string(),
        target_period_id: job.target_period_id.map(|t| t.to_string()),
        name: job.name,
        status: match job.status {
            ProgressionJobStatus::Draft => "draft".into(),
            ProgressionJobStatus::PreviewReady => "preview_ready".into(),
            ProgressionJobStatus::Executing => "executing".into(),
            ProgressionJobStatus::Completed => "completed".into(),
            ProgressionJobStatus::Failed => "failed".into(),
            ProgressionJobStatus::RolledBack => "rolled_back".into(),
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
) -> Result<ApiResponse<ProgressionJobDto>, String> {
    let ctx = make_ctx(&state);
    let source_period_id =
        Uuid::parse_str(&request.source_period_id).map_err(|_| "Invalid sourcePeriodId")?;
    let target_period_id =
        Uuid::parse_str(&request.target_period_id).map_err(|_| "Invalid targetPeriodId")?;

    let mut items = Vec::new();
    for item in request.items {
        let student_id = Uuid::parse_str(&item.student_id).map_err(|_| "Invalid studentId")?;
        let enrollment_id =
            Uuid::parse_str(&item.enrollment_id).map_err(|_| "Invalid enrollmentId")?;
        let source_class_id = item.source_class_id.and_then(|c| Uuid::parse_str(&c).ok());
        let target_class_id = item.target_class_id.and_then(|c| Uuid::parse_str(&c).ok());

        items.push(StudentProgressionItem {
            student_id,
            enrollment_id,
            source_section_id: source_class_id,
            target_section_id: target_class_id,
            decision: parse_decision(&item.decision),
            reason: item.reason,
        });
    }

    let job = state
        .progression_service
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
pub async fn progression_evaluate_cohort(
    source_period_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<StudentProgressionItemInput>>, String> {
    let ctx = make_ctx(&state);
    let pid = Uuid::parse_str(&source_period_id).map_err(|_| "Invalid source_period_id")?;
    let items = state
        .progression_service
        .evaluate_cohort_progression(&ctx, ctx.tenant_id, pid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(items
        .into_iter()
        .map(|i| StudentProgressionItemInput {
            student_id: i.student_id.to_string(),
            enrollment_id: i.enrollment_id.to_string(),
            source_class_id: i.source_section_id.map(|c| c.to_string()),
            target_class_id: i.target_section_id.map(|c| c.to_string()),
            decision: i.decision.to_string(),
            reason: i.reason,
        })
        .collect()))
}

#[tauri::command]
pub async fn progression_list_jobs(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<ProgressionJobDto>>, String> {
    let ctx = make_ctx(&state);
    let jobs = state
        .progression_service
        .progression_repo
        .find_jobs_by_org(ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(jobs.into_iter().map(job_to_dto).collect()))
}

#[tauri::command]
pub async fn progression_get_job(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<ProgressionJobDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let item = state
        .progression_service
        .progression_repo
        .find_job_by_id(uid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Promotion job not found".to_string())?;

    Ok(ok(job_to_dto(item)))
}
