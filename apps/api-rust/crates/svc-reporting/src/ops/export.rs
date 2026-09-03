use std::collections::HashMap;
use sea_orm::{ConnectionTrait, Statement};
use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::Permission;
use crate::contracts::report::{DataType, ReportDataset};

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
    ctx: &ServiceContext,
    definition_id: String,
    filters: HashMap<String, serde_json::Value>,
) -> Result<ReportDataset, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExportData).await?;
    let definition = crate::registry::get_definition(&definition_id)?;
    let backend = ctx.db.get_database_backend();
    let org_id = ctx.org_id.clone();
    let rows = match definition_id.as_str() {
        "student_performance" => {
            let mut sql = String::from(
                "SELECT u.full_name AS student_name, COALESCE(AVG(g.grade_points), AVG(g.score / NULLIF(g.max_score, 0) * 4.0)) AS gpa, CASE WHEN COALESCE(AVG(g.score / NULLIF(g.max_score, 0) * 100.0), 0) >= 90 THEN 'excellent' WHEN COALESCE(AVG(g.score / NULLIF(g.max_score, 0) * 100.0), 0) >= 75 THEN 'good' ELSE 'needs_improvement' END AS status FROM grades g JOIN enrollments e ON e.id = g.enrollment_id JOIN users u ON u.id = e.student_id WHERE g.org_id = $1 AND e.org_id = $1"
            );
            let mut values = vec![org_id.into()];
            if let Some(class_id) = filters.get("class_id").and_then(serde_json::Value::as_str) {
                sql.push_str(" AND e.class_id = $2");
                values.push(class_id.to_string().into());
            }
            sql.push_str(" GROUP BY u.id, u.full_name ORDER BY u.full_name");
            ctx.db.query_all(Statement::from_sql_and_values(backend, sql, values)).await?
        }
        "attendance_summary" => {
            let (from, to) = date_range(&filters)?;
            let sql = "SELECT u.full_name AS student_name, SUM(CASE WHEN a.status IN ('absent', 'excused') THEN 1 ELSE 0 END) AS absent_days, SUM(CASE WHEN a.status = 'present' THEN 1 ELSE 0 END) AS present_days, COALESCE(100.0 * SUM(CASE WHEN a.status = 'present' THEN 1 ELSE 0 END) / NULLIF(COUNT(*), 0), 0) AS rate FROM attendance a JOIN enrollments e ON e.id = a.enrollment_id JOIN users u ON u.id = e.student_id WHERE a.org_id = $1 AND a.date >= $2 AND a.date <= $3 GROUP BY u.id, u.full_name ORDER BY u.full_name";
            ctx.db.query_all(Statement::from_sql_and_values(
                backend,
                sql,
                vec![org_id.into(), from.into(), to.into()],
            )).await?
        }
        "trial_balance" => {
            let as_of = filters.get("as_of_date")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| AppError::validation("as_of_date is required"))?;
            let sql = "SELECT a.name AS account_name, COALESCE(SUM(CASE WHEN j.debit_account_id = a.id THEN j.amount ELSE 0 END), 0) AS debit, COALESCE(SUM(CASE WHEN j.credit_account_id = a.id THEN j.amount ELSE 0 END), 0) AS credit FROM accounts a LEFT JOIN journal_entries j ON j.org_id = a.org_id AND (j.debit_account_id = a.id OR j.credit_account_id = a.id) AND j.accounting_date <= $2 WHERE a.org_id = $1 GROUP BY a.id, a.name ORDER BY a.code";
            ctx.db.query_all(Statement::from_sql_and_values(
                backend,
                sql,
                vec![org_id.into(), as_of.to_string().into()],
            )).await?
        }
        _ => return Err(AppError::internal(format!(
            "Report data provider is not implemented for '{}'",
            definition_id
        ))),
    };

    let rows = rows.into_iter().map(|row| {
        let mut value = serde_json::Map::new();
        for column in &definition.columns {
            let json_value = match column.data_type {
                DataType::Number | DataType::Currency => row.try_get::<f64>("", &column.id)
                    .map(serde_json::Value::from)
                    .or_else(|_| row.try_get::<i64>("", &column.id).map(serde_json::Value::from))
                    .unwrap_or(serde_json::Value::Null),
                DataType::Boolean => row.try_get::<i32>("", &column.id)
                    .map(|value| serde_json::Value::Bool(value != 0))
                    .unwrap_or(serde_json::Value::Null),
                _ => row.try_get::<String>("", &column.id)
                    .map(serde_json::Value::String)
                    .unwrap_or(serde_json::Value::Null),
            };
            value.insert(column.id.clone(), json_value);
        }
        serde_json::Value::Object(value)
    }).collect();

    Ok(ReportDataset {
        definition_id: definition.id,
        metadata: crate::contracts::report::ReportMetadata {
            generated_at: chrono::Utc::now().to_rfc3339(),
            generated_by: Some(ctx.actor.as_str().to_string()),
            applied_filters: filters,
        },
        columns: definition.columns,
        rows,
        summary: None,
    })
}

fn date_range(filters: &HashMap<String, serde_json::Value>) -> Result<(String, String), AppError> {
    let range = filters.get("date_range")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| AppError::validation("date_range is required"))?;
    let from = range.get("from").and_then(serde_json::Value::as_str)
        .ok_or_else(|| AppError::validation("date_range.from is required"))?;
    let to = range.get("to").and_then(serde_json::Value::as_str)
        .ok_or_else(|| AppError::validation("date_range.to is required"))?;
    if from > to {
        return Err(AppError::validation("date_range.from must not be after date_range.to"));
    }
    Ok((from.to_string(), to.to_string()))
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
