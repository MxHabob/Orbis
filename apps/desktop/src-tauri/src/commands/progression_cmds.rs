//! Tauri IPC commands — Student Cohort Progression & Promotion.

use super::make_ctx;
use super::org_cmds::ApiResponse;
use crate::state::AppState;
use orbis_app::services::progression_service::StudentProgressionItem;
use orbis_domain::entities::progression::{
    ProgressionDetail, ProgressionJob, ProgressionJobStatus,
};
use rust_decimal::Decimal;
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

#[derive(Serialize)]
pub struct ProgressionDetailDto {
    pub id: String,
    pub job_id: String,
    pub student_id: String,
    pub enrollment_id: String,
    pub source_section_id: Option<String>,
    pub target_section_id: Option<String>,
    pub decision: String,
    pub reason: String,
    pub manual_override: bool,
    pub override_by: Option<String>,
    pub override_reason: Option<String>,
    pub final_gpa: Option<String>,
    pub attendance_rate: Option<String>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct StudentProgressionItemInput {
    pub student_id: String,
    #[serde(default)]
    pub student_number: Option<String>,
    #[serde(default)]
    pub student_name: Option<String>,
    #[serde(default)]
    pub student_name_ar: Option<String>,
    pub enrollment_id: String,
    pub source_section_id: Option<String>,
    pub target_section_id: Option<String>,
    pub decision: String,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub final_gpa: Option<String>,
    #[serde(default)]
    pub overall_percentage: Option<String>,
    #[serde(default)]
    pub attendance_rate: Option<String>,
    #[serde(default)]
    pub failed_subjects_count: u32,
    #[serde(default)]
    pub financial_balance_due: Option<String>,
    #[serde(default)]
    pub has_financial_hold: bool,
    #[serde(default)]
    pub already_enrolled_in_target: bool,
    #[serde(default)]
    pub manual_override: bool,
    #[serde(default)]
    pub override_reason: Option<String>,
}

#[derive(Deserialize)]
pub struct PromoteCohortInput {
    pub source_period_id: String,
    pub target_period_id: String,
    pub job_name: String,
    pub items: Vec<StudentProgressionItemInput>,
    #[serde(default)]
    pub auto_assess_fees: Option<bool>,
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

fn detail_to_dto(detail: ProgressionDetail) -> ProgressionDetailDto {
    ProgressionDetailDto {
        id: detail.id.to_string(),
        job_id: detail.job_id.to_string(),
        student_id: detail.student_id.to_string(),
        enrollment_id: detail.enrollment_id.to_string(),
        source_section_id: detail.source_section_id.map(|id| id.to_string()),
        target_section_id: detail.target_section_id.map(|id| id.to_string()),
        decision: detail.decision.to_string(),
        reason: detail.reason,
        manual_override: detail.manual_override,
        override_by: detail.override_by.map(|id| id.to_string()),
        override_reason: detail.override_reason,
        final_gpa: detail.final_gpa.map(|v| v.to_string()),
        attendance_rate: detail.attendance_rate.map(|v| v.to_string()),
        created_at: detail.created_at.to_rfc3339(),
    }
}

fn parse_optional_decimal(value: Option<String>) -> Result<Option<Decimal>, String> {
    value
        .map(|s| s.parse::<Decimal>())
        .transpose()
        .map_err(|_| "Invalid decimal value".to_string())
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
        let source_section_id = item
            .source_section_id
            .map(|section_id| Uuid::parse_str(&section_id))
            .transpose()
            .map_err(|_| "Invalid section ID")?;
        let target_section_id = item
            .target_section_id
            .map(|section_id| Uuid::parse_str(&section_id))
            .transpose()
            .map_err(|_| "Invalid section ID")?;

        items.push(StudentProgressionItem {
            student_id,
            student_number: item.student_number,
            student_name: item.student_name,
            student_name_ar: item.student_name_ar,
            enrollment_id,
            source_section_id,
            target_section_id,
            decision: item
                .decision
                .parse()
                .map_err(|error: orbis_domain::errors::DomainError| error.to_string())?,
            reason: item.reason,
            final_gpa: parse_optional_decimal(item.final_gpa)?,
            overall_percentage: parse_optional_decimal(item.overall_percentage)?,
            attendance_rate: parse_optional_decimal(item.attendance_rate)?,
            failed_subjects_count: item.failed_subjects_count,
            financial_balance_due: parse_optional_decimal(item.financial_balance_due)?
                .unwrap_or(Decimal::ZERO),
            has_financial_hold: item.has_financial_hold,
            already_enrolled_in_target: item.already_enrolled_in_target,
            manual_override: item.manual_override,
            override_reason: item.override_reason,
        });
    }

    let job = state
        .progression_service
        .promote_cohort_with_options(
            &ctx,
            ctx.tenant_id,
            source_period_id,
            target_period_id,
            request.job_name,
            items,
            request.auto_assess_fees,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(job_to_dto(job)))
}

#[tauri::command]
pub async fn progression_evaluate_cohort(
    source_period_id: String,
    target_period_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<StudentProgressionItemInput>>, String> {
    let ctx = make_ctx(&state);
    let pid = Uuid::parse_str(&source_period_id).map_err(|_| "Invalid source_period_id")?;
    let target_pid = target_period_id
        .filter(|s| !s.trim().is_empty())
        .map(|s| Uuid::parse_str(&s))
        .transpose()
        .map_err(|_| "Invalid target_period_id")?;

    let items = state
        .progression_service
        .evaluate_cohort_with_target(&ctx, ctx.tenant_id, pid, target_pid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(items
        .into_iter()
        .map(|i| StudentProgressionItemInput {
            student_id: i.student_id.to_string(),
            student_number: i.student_number,
            student_name: i.student_name,
            student_name_ar: i.student_name_ar,
            enrollment_id: i.enrollment_id.to_string(),
            source_section_id: i.source_section_id.map(|section_id| section_id.to_string()),
            target_section_id: i.target_section_id.map(|section_id| section_id.to_string()),
            decision: i.decision.to_string(),
            reason: i.reason,
            final_gpa: i.final_gpa.map(|v| v.to_string()),
            overall_percentage: i.overall_percentage.map(|v| v.to_string()),
            attendance_rate: i.attendance_rate.map(|v| v.to_string()),
            failed_subjects_count: i.failed_subjects_count,
            financial_balance_due: Some(i.financial_balance_due.to_string()),
            has_financial_hold: i.has_financial_hold,
            already_enrolled_in_target: i.already_enrolled_in_target,
            manual_override: i.manual_override,
            override_reason: i.override_reason,
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
        .list_jobs(&ctx)
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
        .get_job(&make_ctx(&state), uid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(job_to_dto(item)))
}

#[tauri::command]
pub async fn progression_get_job_details(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<ProgressionDetailDto>>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let items = state
        .progression_service
        .get_job_details(&make_ctx(&state), uid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(items.into_iter().map(detail_to_dto).collect()))
}

#[tauri::command]
pub async fn progression_rollback_job(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<ProgressionJobDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let job = state
        .progression_service
        .rollback_job(&make_ctx(&state), uid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(job_to_dto(job)))
}
