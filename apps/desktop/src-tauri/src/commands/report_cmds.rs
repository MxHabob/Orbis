use tauri::State;
use crate::state::AppState;
use uuid::Uuid;
use std::collections::HashMap;
use orbis_domain::entities::report::{ParamValue, OutputFormat, ReportLocale, BatchPrintConfig, ColumnDef};

use super::make_ctx;


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
    let params: HashMap<String, ParamValue> = serde_json::from_value(parameters)
        .unwrap_or_default();

    let output_format = if format.is_empty() {
        OutputFormat::DataOnly
    } else {
        OutputFormat::from_str(&format)
    };

    // Default locale for now
    let report_locale = ReportLocale::Ar;

    let input = orbis_app::services::GenerateReportInput {
        report_code,
        parameters: params,
        output_format,
        locale: report_locale,
    };

    let (mut job, export) = state.report_service.generate_report(&ctx, input).await
        .map_err(|e| e.to_string())?;

    // Since the frontend expects to download a file later, we write it to a temp dir
    let temp_dir = std::env::temp_dir().join("orbis_reports");
    tokio::fs::create_dir_all(&temp_dir).await.map_err(|e| e.to_string())?;
    
    let file_path = temp_dir.join(format!("{}.{}", job.id, export.filename));
    tokio::fs::write(&file_path, &export.data).await.map_err(|e| e.to_string())?;

    // Update job with file path
    job.output_path = Some(file_path.to_string_lossy().to_string());
    job.output_size_bytes = Some(export.data.len() as i64);
    
    state.report_job_repo.save(&job).await.map_err(|e| e.to_string())?;

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
    
    let job = state.report_job_repo.find_by_id(parsed_job_id).await
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
    
    let job = state.report_job_repo.find_by_id(parsed_job_id).await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Job not found".to_string())?;

    if job.status.to_string() != "completed" {
        return Err("Report generation is not completed yet".to_string());
    }

    let file_path = job.output_path.ok_or_else(|| "File path missing".to_string())?;
    tokio::fs::read(&file_path).await.map_err(|e| format!("Failed to read file: {}", e))
}

#[tauri::command]
pub async fn batch_print(
    report_code: String,
    parameters: serde_json::Value,
    batch_config: serde_json::Value,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let ctx = make_ctx(&state);
    
    let config: BatchPrintConfig = serde_json::from_value(batch_config)
        .map_err(|e| format!("Invalid batch config: {}", e))?;
        
    let params: HashMap<String, ParamValue> = serde_json::from_value(parameters)
        .unwrap_or_default();

    // 1. Resolve definition
    let definition = state.report_definition_repo
        .find_by_code(ctx.tenant_id, &report_code)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Report definition not found: {}", report_code))?;

    // 2. Resolve org context
    let org_context = state.report_org_context_resolver
        .resolve(ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;

    // 3. Fetch data
    let locale = ReportLocale::Ar;

    let provider = state.report_provider_registry
        .get_provider(&definition.data_provider_key)
        .ok_or_else(|| format!("No data provider for key: {}", definition.data_provider_key))?;

    let dataset = provider
        .fetch_data(ctx.tenant_id, &org_context, &params, &locale)
        .await
        .map_err(|e| e.to_string())?;

    // 4. Compute layout
    let layout = config.compute_layout();
    let is_rtl = matches!(locale, ReportLocale::Ar | ReportLocale::ArEn);
    let dir = if is_rtl { "rtl" } else { "ltr" };

    // 5. Render HTML
    let primary_color = org_context.primary_color.as_deref().unwrap_or("#6366f1");
    let mut html = format!(r#"<!DOCTYPE html>
<html dir="{dir}">
<head>
<meta charset="UTF-8">
<style>
@page {{ size: {w}mm {h}mm; margin: 0; }}
body {{ margin: 0; padding: 0; font-family: 'Noto Sans Arabic', 'Segoe UI', sans-serif; }}
.sheet {{ width: {w}mm; height: {h}mm; position: relative; page-break-after: always; overflow: hidden; }}
.sheet:last-child {{ page-break-after: auto; }}
.item {{ position: absolute; overflow: hidden; border: 0.3px solid #e0e0e0; box-sizing: border-box;
    display: flex; flex-direction: column; align-items: center; justify-content: center;
    padding: 3mm; font-size: 10px; text-align: center; direction: {dir}; }}
.item .org-name {{ font-weight: 700; color: {color}; font-size: 11px; margin-bottom: 2mm; }}
.item .field-label {{ color: #888; font-size: 8px; }}
.item .field-value {{ font-weight: 600; font-size: 10px; margin-bottom: 1mm; }}
.cut-line-h {{ position: absolute; border-top: 0.5px dashed #ccc; width: 100%; left: 0; }}
.cut-line-v {{ position: absolute; border-left: 0.5px dashed #ccc; height: 100%; top: 0; }}
@media print {{ body {{ -webkit-print-color-adjust: exact; print-color-adjust: exact; }} }}
</style>
</head>
<body>"#,
        dir = dir,
        w = layout.sheet_width_mm,
        h = layout.sheet_height_mm,
        color = primary_color,
    );

    let pages = config.paginate(&dataset.rows);
    let visible_cols: Vec<&ColumnDef> = dataset.columns.iter().filter(|c| c.visible).collect();
    let org_name = if is_rtl { &org_context.name_ar } else { &org_context.name };

    for page_items in &pages {
        html.push_str(&format!(r#"<div class="sheet">"#));

        for (idx, row) in page_items.iter().enumerate() {
            if let Some(slot) = layout.slots.get(idx) {
                html.push_str(&format!(
                    r#"<div class="item" style="left:{x}mm; top:{y}mm; width:{w}mm; height:{h}mm;">"#,
                    x = slot.x_mm, y = slot.y_mm,
                    w = slot.width_mm, h = slot.height_mm,
                ));

                html.push_str(&format!(r#"<div class="org-name">{}</div>"#, org_name));

                for col in &visible_cols {
                    let label = if is_rtl { &col.header_ar } else { &col.header };
                    let val = row.get(&col.key)
                        .map(|v| match v {
                            serde_json::Value::String(s) => s.clone(),
                            serde_json::Value::Number(n) => n.to_string(),
                            _ => v.to_string(),
                        })
                        .unwrap_or_default();

                    if !val.is_empty() {
                        html.push_str(&format!(
                            r#"<div class="field-label">{}</div><div class="field-value">{}</div>"#,
                            label, val
                        ));
                    }
                }

                html.push_str("</div>");
            }
        }

        if config.show_cut_lines {
            for r in 1..layout.rows {
                let y = layout.slots.first().map(|s| s.y_mm).unwrap_or(0.0)
                    + r as f64 * (config.item_height_mm + config.gap_mm)
                    - config.gap_mm / 2.0;
                html.push_str(&format!(r#"<div class="cut-line-h" style="top:{y}mm;"></div>"#, y = y));
            }
            for c in 1..layout.columns {
                let x = layout.slots.first().map(|s| s.x_mm).unwrap_or(0.0)
                    + c as f64 * (config.item_width_mm + config.gap_mm)
                    - config.gap_mm / 2.0;
                html.push_str(&format!(r#"<div class="cut-line-v" style="left:{x}mm;"></div>"#, x = x));
            }
        }

        html.push_str("</div>");
    }

    html.push_str("</body></html>");
    Ok(html)
}
