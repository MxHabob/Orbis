//! Cross-domain fee assignment — lives in infra to avoid svc-academic ↔ svc-finance cycles.

use sea_orm::{ConnectionTrait, Statement};
use tracing::info;
use uuid::Uuid;

use shared_kernel::errors::AppError;

/// Assign mandatory fee templates to an enrollment within any DB connection (including transactions).
pub async fn assign_mandatory_fees<C: ConnectionTrait>(
    conn: &C,
    org_id: &str,
    enrollment_id: &str,
    class_id: &str,
    period_id: &str,
) -> Result<Vec<String>, AppError> {
    let backend = conn.get_database_backend();
    let templates = conn
        .query_all(Statement::from_sql_and_values(
            backend,
            "SELECT id, default_amount, applicable_levels FROM fee_templates \
             WHERE org_id = ?1 AND is_mandatory = 1 AND is_active = 1",
            vec![org_id.into()],
        ))
        .await
        .map_err(|e| AppError::internal(format!("Failed to load fee templates: {e}")))?;

    if templates.is_empty() {
        return Ok(vec![]);
    }

    let class_level: Option<String> = conn
        .query_one(Statement::from_sql_and_values(
            backend,
            "SELECT level FROM classes WHERE id = ?1",
            vec![class_id.into()],
        ))
        .await
        .ok()
        .flatten()
        .and_then(|row| row.try_get("", "level").ok());

    let mut assigned_ids = Vec::new();
    let now = chrono::Utc::now().to_rfc3339();

    for row in templates {
        let template_id: String = row
            .try_get("", "id")
            .map_err(|e| AppError::internal(e.to_string()))?;
        let default_amount: f64 = row
            .try_get("", "default_amount")
            .map_err(|e| AppError::internal(e.to_string()))?;
        let applicable_levels: Option<String> = row.try_get("", "applicable_levels").ok();

        if let Some(ref levels_raw) = applicable_levels {
            if !levels_raw.is_empty() && levels_raw != "[]" && levels_raw != "{}" {
                let mut target_levels: Vec<String> = Vec::new();
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(levels_raw) {
                    if let Some(arr) = val.as_array() {
                        for item in arr {
                            if let Some(s) = item.as_str() {
                                target_levels.push(s.to_string());
                            }
                        }
                    } else if let Some(obj) = val.as_object() {
                        if let Some(arr) = obj.get("levels").and_then(|v| v.as_array()) {
                            for item in arr {
                                if let Some(s) = item.as_str() {
                                    target_levels.push(s.to_string());
                                }
                            }
                        } else if let Some(s) = obj.get("level").and_then(|v| v.as_str()) {
                            target_levels.push(s.to_string());
                        }
                    }
                }

                if !target_levels.is_empty() {
                    let matches_class_id = target_levels.contains(&class_id.to_string());
                    let matches_level = class_level.as_ref().map_or(false, |lvl| target_levels.contains(lvl));

                    if !matches_class_id && !matches_level {
                        continue;
                    }
                }
            }
        }

        let fee_id = Uuid::new_v4().to_string();
        conn.execute(Statement::from_sql_and_values(
            backend,
            "INSERT INTO student_fees \
             (id, org_id, period_id, enrollment_id, fee_template_id, original_amount, \
              discount_amount, net_amount, status, due_date, created_at, updated_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0, ?6, 'pending', ?7, ?7, ?7)",
            vec![
                fee_id.clone().into(),
                org_id.into(),
                period_id.into(),
                enrollment_id.into(),
                template_id.into(),
                default_amount.into(),
                now.clone().into(),
            ],
        ))
        .await
        .map_err(|e| AppError::internal(format!("Failed to insert student fee: {e}")))?;

        assigned_ids.push(fee_id);
    }

    info!(
        org_id = org_id,
        enrollment_id = enrollment_id,
        count = assigned_ids.len(),
        "Assigned mandatory fees"
    );

    Ok(assigned_ids)
}

/// Resolve student_id from enrollment without depending on svc-academic.
pub async fn enrollment_student_id<C: ConnectionTrait>(
    conn: &C,
    enrollment_id: &str,
) -> Result<String, AppError> {
    let backend = conn.get_database_backend();
    let row = conn
        .query_one(Statement::from_sql_and_values(
            backend,
            "SELECT student_id FROM enrollments WHERE id = ?1",
            vec![enrollment_id.into()],
        ))
        .await
        .map_err(|e| AppError::internal(e.to_string()))?
        .ok_or_else(|| AppError::not_found("enrollment", enrollment_id))?;

    row.try_get("", "student_id")
        .map_err(|e| AppError::internal(e.to_string()))
}
