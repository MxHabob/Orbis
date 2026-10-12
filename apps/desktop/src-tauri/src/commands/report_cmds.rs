use crate::state::AppState;
use docforge_engine::layout::multi_up::MultiUpConfig;
use docforge_engine::render::multi_up_renderer::render_multi_up;
use orbis_domain::entities::report::{OutputFormat, ParamValue, ReportLocale};
use std::collections::HashMap;
use tauri::State;
use uuid::Uuid;

use super::make_ctx;

#[derive(serde::Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

fn ok<T: serde::Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse { data }
}

// ── Commands ────────────────────────────────────────────────────

#[tauri::command]
pub async fn generate_report(
    template_id: String, // Actually report_code in the new system, we keep the param name for frontend compat
    format: String,
    parameters: serde_json::Value,
    _query: Option<serde_json::Value>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let ctx = make_ctx(&state);
    let report_code = template_id; // Mapping frontend's 'template_id' to 'report_code'

    // Parse parameters
    let params: HashMap<String, ParamValue> =
        serde_json::from_value(parameters).unwrap_or_default();

    let output_format = if format.is_empty() {
        OutputFormat::DataOnly
    } else {
        OutputFormat::from_str(&format)
    };

    // Resolve locale from organization
    let org = state
        .org_repo
        .find_by_id(ctx.tenant_id)
        .await
        .ok()
        .flatten();
    let report_locale = match org.as_ref().map(|o| o.locale.as_str()) {
        Some("en") | Some("en-US") | Some("en_US") => ReportLocale::En,
        Some("ar-en") | Some("ar_en") => ReportLocale::ArEn,
        Some("en-ar") | Some("en_ar") => ReportLocale::EnAr,
        _ => ReportLocale::Ar,
    };

    let input = orbis_app::services::GenerateReportInput {
        report_code,
        parameters: params,
        output_format,
        locale: report_locale,
    };

    let (mut job, export) = state
        .report_service
        .generate_report(&ctx, input)
        .await
        .map_err(|e| e.to_string())?;

    // Since the frontend expects to download a file later, we write it to a temp dir
    let temp_dir = std::env::temp_dir().join("orbis_reports");
    tokio::fs::create_dir_all(&temp_dir)
        .await
        .map_err(|e| e.to_string())?;

    let file_path = temp_dir.join(format!("{}.{}", job.id, export.filename));
    tokio::fs::write(&file_path, &export.data)
        .await
        .map_err(|e| e.to_string())?;

    // Update job with file path
    job.output_path = Some(file_path.to_string_lossy().to_string());
    job.output_size_bytes = Some(export.data.len() as i64);

    state
        .report_job_repo
        .save(&job)
        .await
        .map_err(|e| e.to_string())?;

    Ok(job.id.to_string())
}

#[derive(serde::Serialize)]
pub struct ReportJobStatus {
    pub job_id: String,
    pub status: String,
}

#[tauri::command]
pub async fn get_report_job_status(
    job_id: String,
    state: State<'_, AppState>,
) -> Result<ReportJobStatus, String> {
    let parsed_job_id = Uuid::parse_str(&job_id).map_err(|_| "Invalid job ID")?;

    let job = state
        .report_job_repo
        .find_by_id(parsed_job_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Job not found".to_string())?;

    Ok(ReportJobStatus {
        job_id: job.id.to_string(),
        status: job.status.to_string(),
    })
}

#[tauri::command]
pub async fn download_report_file(
    job_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<u8>, String> {
    let parsed_job_id = Uuid::parse_str(&job_id).map_err(|_| "Invalid job ID")?;

    let job = state
        .report_job_repo
        .find_by_id(parsed_job_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Job not found".to_string())?;

    if job.status.to_string() != "completed" {
        return Err("Report generation is not completed yet".to_string());
    }

    let file_path = job
        .output_path
        .ok_or_else(|| "File path missing".to_string())?;
    tokio::fs::read(&file_path)
        .await
        .map_err(|e| format!("Failed to read file: {}", e))
}

#[tauri::command]
pub async fn batch_print(
    document_json: String,
    contexts: Vec<serde_json::Value>,
    batch_config: serde_json::Value,
    title: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<u8>, String> {
    let config: MultiUpConfig =
        serde_json::from_value(batch_config).map_err(|e| format!("Invalid batch config: {e}"))?;
    let ctx = make_ctx(&state);
    let org_context = state
        .report_org_context_resolver
        .resolve(ctx.tenant_id)
        .await
        .map_err(|error| error.to_string())?;
    let org_context = serde_json::json!({
        "id": org_context.id,
        "name": org_context.name,
        "name_ar": org_context.name_ar,
        "currency": org_context.currency_code,
        "logo_url": org_context.logo_url,
        "primary_color": org_context.primary_color,
    });
    let mut contexts = contexts;
    for (index, context) in contexts.iter_mut().enumerate() {
        let context = context
            .as_object_mut()
            .ok_or_else(|| format!("Data context at index {index} must be a JSON object"))?;
        context.entry("$org").or_insert_with(|| org_context.clone());
    }
    let pdf = render_multi_up(&document_json, &contexts, &config, title)
        .map_err(|error| error.to_string())?;
    Ok(pdf.bytes)
}

#[derive(serde::Serialize)]
pub struct AuditLogDto {
    pub id: String,
    pub org_id: String,
    pub user_id: Option<String>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub ip_address: Option<String>,
    pub metadata: Option<String>,
    pub details: Option<serde_json::Value>,
    pub created_at: String,
}

#[tauri::command]
pub async fn audit_list_logs(
    limit: Option<u64>,
    offset: Option<u64>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<AuditLogDto>>, String> {
    let ctx = make_ctx(&state);
    let entries = state
        .report_audit_repo
        .list_by_org(ctx.tenant_id, limit, offset)
        .await
        .map_err(|e| e.to_string())?;

    let dtos = entries
        .into_iter()
        .map(|e| AuditLogDto {
            id: e.id.to_string(),
            org_id: e.org_id.to_string(),
            user_id: Some(e.actor_id.to_string()),
            action: e.action,
            entity_type: e.entity_type,
            entity_id: Some(e.entity_id.to_string()),
            ip_address: Some("127.0.0.1".to_string()),
            metadata: e
                .details
                .as_ref()
                .map(|d| serde_json::to_string_pretty(d).unwrap_or_default()),
            details: e.details,
            created_at: e.created_at.to_rfc3339(),
        })
        .collect();

    Ok(ok(dtos))
}
