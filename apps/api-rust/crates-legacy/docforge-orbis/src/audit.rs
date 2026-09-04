// ═══════════════════════════════════════════════════════════════
// docforge-orbis — Template Audit Logger
// Writes operational logs for template changes to template_audit_log
// ═══════════════════════════════════════════════════════════════

use chrono::Utc;
use sea_orm::ConnectionTrait;
use uuid::Uuid;

/// Log an action performed on a template or component.
pub async fn log_template_action(
    db: &impl ConnectionTrait,
    org_id: &str,
    template_id: Option<&str>,
    user_id: &str,
    action: &str,
    entity_type: &str,
    entity_id: &str,
    old_data: Option<&str>,
    new_data: Option<&str>,
    metadata: Option<&str>,
) -> Result<(), String> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let sql = format!(
        "INSERT INTO template_audit_log (id, org_id, template_id, user_id, action, entity_type, entity_id, old_data, new_data, metadata, created_at) VALUES ('{}', '{}', {}, '{}', '{}', '{}', '{}', {}, {}, {}, '{}')",
        id,
        sql_escape(org_id),
        opt_sql(template_id),
        sql_escape(user_id),
        sql_escape(action),
        sql_escape(entity_type),
        sql_escape(entity_id),
        opt_sql(old_data),
        opt_sql(new_data),
        opt_sql(metadata),
        now
    );

    db.execute_unprepared(&sql)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn sql_escape(s: &str) -> String {
    s.replace('\'', "''")
}

fn opt_sql(opt: Option<&str>) -> String {
    match opt {
        Some(s) => format!("'{}'", sql_escape(s)),
        None => "NULL".to_string(),
    }
}
