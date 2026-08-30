use std::collections::HashMap;
use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use crate::contracts::report::{ReportDataset, ReportMetadata};

pub async fn export_report(
    ctx: &ServiceContext,
    definition_id: String,
    filters: HashMap<String, serde_json::Value>,
    format: String,
    _template_id: Option<String>,
) -> Result<Vec<u8>, AppError> {
    // 1. Query the data first (reuse the same logic)
    let dataset = query_report(ctx, definition_id, filters).await?;

    // 2. Export based on format
    match format.as_str() {
        "csv" => {
            crate::export::export_to_csv(&dataset)
                .map_err(|e| AppError::validation(&e.to_string()))
        }
        "pdf" => Err(AppError::validation("PDF export is not implemented yet")),
        "xlsx" => Err(AppError::validation("XLSX export is not implemented yet")),
        _ => Err(AppError::validation(&format!("Unsupported format: {}", format))),
    }
}

pub async fn query_report(
    _ctx: &ServiceContext,
    definition_id: String,
    filters: HashMap<String, serde_json::Value>,
) -> Result<ReportDataset, AppError> {
    let def = crate::registry::get_definition(&definition_id)?;

    // MOCK DATA for testing
    let rows: Vec<serde_json::Value> = if definition_id == "student_performance" {
        vec![
            serde_json::json!({
                "student_name": "أحمد علي",
                "gpa": 3.8,
                "status": "ممتاز"
            }),
            serde_json::json!({
                "student_name": "سارة خان",
                "gpa": 3.2,
                "status": "جيد جداً"
            }),
            serde_json::json!({
                "student_name": "محمد حسن",
                "gpa": 2.9,
                "status": "جيد"
            }),
        ]
    } else if definition_id == "attendance_summary" {
        vec![
            serde_json::json!({
                "student_name": "أحمد علي",
                "absent_days": 3,
                "present_days": 42,
                "rate": 93.3
            }),
            serde_json::json!({
                "student_name": "سارة خان",
                "absent_days": 1,
                "present_days": 44,
                "rate": 97.8
            }),
        ]
    } else if definition_id == "trial_balance" {
        vec![
            serde_json::json!({
                "account_name": "النقدية",
                "debit": 150000.0,
                "credit": 0.0
            }),
            serde_json::json!({
                "account_name": "الذمم المدينة",
                "debit": 85000.0,
                "credit": 0.0
            }),
            serde_json::json!({
                "account_name": "رأس المال",
                "debit": 0.0,
                "credit": 200000.0
            }),
            serde_json::json!({
                "account_name": "الإيرادات",
                "debit": 0.0,
                "credit": 35000.0
            }),
        ]
    } else {
        vec![]
    };

    Ok(ReportDataset {
        definition_id: def.id.clone(),
        metadata: ReportMetadata {
            generated_at: chrono::Utc::now().to_rfc3339(),
            generated_by: None,
            applied_filters: filters,
        },
        columns: def.columns.clone(),
        rows,
        summary: None,
    })
}

pub async fn get_report_definition(
    _ctx: &ServiceContext,
    definition_id: String,
) -> Result<crate::contracts::report::ReportDefinition, AppError> {
    crate::registry::get_definition(&definition_id)
}

pub async fn list_reports(
    _ctx: &ServiceContext,
    module: String,
) -> Result<Vec<crate::contracts::report::ReportDefinition>, AppError> {
    Ok(crate::registry::list_by_module(&module))
}
