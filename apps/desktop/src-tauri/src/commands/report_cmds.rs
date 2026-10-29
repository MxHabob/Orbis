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

fn coerce_param_value(value: serde_json::Value) -> Result<ParamValue, String> {
    match value {
        serde_json::Value::Null => Err("Report parameter value cannot be null".to_string()),
        serde_json::Value::String(s) => Ok(ParamValue::Text(s)),
        serde_json::Value::Bool(b) => Ok(ParamValue::Boolean(b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(ParamValue::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(ParamValue::Decimal(f))
            } else {
                Err("Unsupported numeric report parameter value".to_string())
            }
        }
        serde_json::Value::Array(items) => {
            let mut values = Vec::with_capacity(items.len());
            for item in items {
                let entry = match item {
                    serde_json::Value::String(s) => s,
                    _ => {
                        return Err(
                            "Report parameter id list must contain only strings".to_string()
                        );
                    }
                };
                values.push(entry);
            }
            Ok(ParamValue::IdList(values))
        }
        serde_json::Value::Object(mut obj) => {
            if let Some(kind) = obj.get("type").and_then(serde_json::Value::as_str) {
                match kind {
                    "date_range" | "dateRange" => {
                        let from = obj
                            .remove("from")
                            .and_then(|value| value.as_str().map(str::to_owned))
                            .ok_or_else(|| {
                                "date_range parameter requires a 'from' string".to_string()
                            })?;
                        let to = obj
                            .remove("to")
                            .and_then(|value| value.as_str().map(str::to_owned))
                            .ok_or_else(|| {
                                "date_range parameter requires a 'to' string".to_string()
                            })?;
                        Ok(ParamValue::DateRange { from, to })
                    }
                    _ => Ok(ParamValue::Text(serde_json::to_string(&obj).map_err(
                        |err| format!("Failed to coerce report parameter object: {err}"),
                    )?)),
                }
            } else if obj.get("from").is_some() || obj.get("to").is_some() {
                let from = obj
                    .remove("from")
                    .and_then(|value| value.as_str().map(str::to_owned))
                    .unwrap_or_default();
                let to = obj
                    .remove("to")
                    .and_then(|value| value.as_str().map(str::to_owned))
                    .unwrap_or_default();
                Ok(ParamValue::DateRange { from, to })
            } else {
                Ok(ParamValue::Text(serde_json::to_string(&obj).map_err(
                    |err| format!("Failed to coerce report parameter object: {err}"),
                )?))
            }
        }
    }
}

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

    let params = match parameters {
        serde_json::Value::Object(obj) => {
            let mut mapped = HashMap::with_capacity(obj.len());
            for (key, value) in obj {
                mapped.insert(key, coerce_param_value(value)?);
            }
            mapped
        }
        _ => {
            return Err(
                "Invalid report parameters: report parameters must be an object".to_string(),
            );
        }
    };

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
        title: None,
        selected_columns: None,
        template_id: None,
    };

    let job = state
        .report_service
        .enqueue_report(&ctx, input)
        .await
        .map_err(|e| e.to_string())?;

    Ok(job.id.to_string())
}

#[derive(serde::Serialize)]
pub struct ReportJobStatus {
    pub job_id: String,
    pub status: String,
    pub error_message: Option<String>,
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
        error_message: job.error_message,
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
            ip_address: None,
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

#[tauri::command]
pub async fn get_entity_audit(
    entity_type: String,
    entity_id: String,
    limit: Option<u64>,
    offset: Option<u64>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<AuditLogDto>>, String> {
    let entity_id = Uuid::parse_str(&entity_id).map_err(|_| "Invalid entity ID".to_string())?;
    let ctx = make_ctx(&state);
    let entries = state
        .report_audit_repo
        .list_by_entity(ctx.tenant_id, &entity_type, entity_id, limit, offset)
        .await
        .map_err(|error| error.to_string())?;

    let dtos = entries
        .into_iter()
        .map(|entry| {
            let metadata = entry
                .details
                .as_ref()
                .map(|details| serde_json::to_string_pretty(details))
                .transpose()
                .map_err(|error| format!("Failed to serialize audit details: {error}"))?;

            Ok::<AuditLogDto, String>(AuditLogDto {
                id: entry.id.to_string(),
                org_id: entry.org_id.to_string(),
                user_id: Some(entry.actor_id.to_string()),
                action: entry.action,
                entity_type: entry.entity_type,
                entity_id: Some(entry.entity_id.to_string()),
                ip_address: None,
                metadata,
                details: entry.details,
                created_at: entry.created_at.to_rfc3339(),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;

    Ok(ok(dtos))
}
