use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, ActiveValue::Set};
use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::academic_policy;

/// Get the autopilot configuration (academic policy) for the organization.
pub async fn get_autopilot_config(
    ctx: &ServiceContext,
    org_id: &str,
) -> Result<serde_json::Value, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrganizationRead).await?;

    let policy = academic_policy::Entity::find()
        .filter(academic_policy::Column::OrgId.eq(org_id))
        .one(&ctx.db)
        .await?;
        
    let config_json = match policy {
        Some(p) => {
            serde_json::json!({
                "enabled": p.autopilot_enabled == 1,
                "execution_mode": p.autopilot_mode,
                "auto_open_enrollment": p.auto_open_enrollment == 1,
                "auto_close_enrollment": p.auto_close_enrollment == 1,
                "auto_record_attendance": true,
                "auto_fee_assignment": true,
                "auto_grade_entry_reminder": true,
                "auto_period_lock": p.auto_lock_periods == 1,
                "auto_promotion": p.auto_archive_years == 1,
                "auto_exam_scheduling": false,
                "auto_report_generation": true,
                "grace_days": p.grace_days_after_deadline,
                "notify_before_deadline_hours": p.notify_before_deadline_days * 24,
            })
        }
        None => {
            serde_json::json!({
                "enabled": true,
                "execution_mode": "suggested",
                "auto_open_enrollment": true,
                "auto_close_enrollment": true,
                "auto_record_attendance": true,
                "auto_fee_assignment": true,
                "auto_grade_entry_reminder": true,
                "auto_period_lock": true,
                "auto_promotion": false,
                "auto_exam_scheduling": false,
                "auto_report_generation": true,
                "grace_days": 3,
                "notify_before_deadline_hours": 72,
            })
        }
    };
    
    Ok(config_json)
}

/// Update the autopilot configuration (academic policy).
pub async fn update_autopilot_config(
    ctx: &ServiceContext,
    org_id: &str,
    config: serde_json::Value,
) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrganizationWrite).await?;

    let existing = academic_policy::Entity::find()
        .filter(academic_policy::Column::OrgId.eq(org_id))
        .one(&ctx.db)
        .await?;
        
    let now = chrono::Utc::now().to_rfc3339();
    let mut active: academic_policy::ActiveModel = match existing {
        Some(m) => m.into(),
        None => {
            let mut a = <academic_policy::ActiveModel as std::default::Default>::default();
            a.id = Set(uuid::Uuid::new_v4().to_string());
            a.org_id = Set(org_id.to_string());
            a.created_at = Set(now.clone());
            a
        }
    };
    
    if let Some(enabled) = config.get("enabled").and_then(|v| v.as_bool()) {
        active.autopilot_enabled = Set(if enabled { 1 } else { 0 });
    }
    if let Some(mode) = config.get("execution_mode").and_then(|v| v.as_str()) {
        active.autopilot_mode = Set(mode.to_string());
    }
    if let Some(val) = config.get("auto_open_enrollment").and_then(|v| v.as_bool()) {
        active.auto_open_enrollment = Set(if val { 1 } else { 0 });
    }
    if let Some(val) = config.get("auto_close_enrollment").and_then(|v| v.as_bool()) {
        active.auto_close_enrollment = Set(if val { 1 } else { 0 });
    }
    if let Some(val) = config.get("auto_period_lock").and_then(|v| v.as_bool()) {
        active.auto_lock_periods = Set(if val { 1 } else { 0 });
    }
    if let Some(val) = config.get("grace_days").and_then(|v| v.as_i64()) {
        active.grace_days_after_deadline = Set(val as i32);
    }
    if let Some(val) = config.get("notify_before_deadline_hours").and_then(|v| v.as_i64()) {
        active.notify_before_deadline_days = Set((val / 24) as i32);
    }
    
    active.updated_at = Set(now);
    active.save(&ctx.db).await?;
    
    // Audit log
    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "autopilot.config.update".into(),
        entity_type: "academic_policy".into(),
        entity_id: org_id.to_string(),
        old_data: None,
        new_data: Some(config),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;
    
    Ok(())
}
