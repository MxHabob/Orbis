use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, ActiveValue::Set};
use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::academic_policy;

fn get_i32_or_bool(val: &serde_json::Value, keys: &[&str]) -> Option<i32> {
    for k in keys {
        if let Some(b) = val.get(*k).and_then(|v| v.as_bool()) {
            return Some(if b { 1 } else { 0 });
        }
        if let Some(n) = val.get(*k).and_then(|v| v.as_i64()) {
            return Some(n as i32);
        }
    }
    None
}

fn get_f64_or_num(val: &serde_json::Value, keys: &[&str]) -> Option<f64> {
    for k in keys {
        if let Some(f) = val.get(*k).and_then(|v| v.as_f64()) {
            return Some(f);
        }
        if let Some(n) = val.get(*k).and_then(|v| v.as_i64()) {
            return Some(n as f64);
        }
    }
    None
}

fn get_str_val(val: &serde_json::Value, keys: &[&str]) -> Option<String> {
    for k in keys {
        if let Some(s) = val.get(*k).and_then(|v| v.as_str()) {
            return Some(s.to_string());
        }
    }
    None
}

// ════════════════════════════════════════════════════════════════════════════
// 1. Academic Policy Operations (Grading, Enrollment, Grade Entry, Promotion)
// ════════════════════════════════════════════════════════════════════════════

/// Get the full academic policy for an organization.
pub async fn get_academic_policy(
    ctx: &ServiceContext,
    org_id: &str,
) -> Result<serde_json::Value, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrganizationRead).await?;

    let policy = academic_policy::Entity::find()
        .filter(academic_policy::Column::OrgId.eq(org_id))
        .one(&ctx.db)
        .await?;

    let result = match policy {
        Some(p) => serde_json::json!({
            "id": p.id,
            "org_id": p.org_id,
            "org_unit_id": p.org_unit_id,
            "default_grading_system": p.default_grading_system,
            "default_passing_score": p.default_passing_score,
            "default_passing_grade": p.default_passing_score,
            "default_max_score": p.default_max_score,
            "default_max_grade": p.default_max_score,
            "default_scale_id": p.default_scale_id,
            "default_pass_fail_policy_id": p.default_pass_fail_policy_id,
            "enrollment_auto_close": p.enrollment_auto_close,
            "allow_late_enrollment": p.allow_late_enrollment,
            "late_enrollment_approval": p.late_enrollment_approval,
            "grade_entry_auto_close": p.grade_entry_auto_close,
            "allow_grade_edit_after_close": p.allow_grade_edit_after_close,
            "grade_edit_approval": p.grade_edit_approval,
            "min_attendance_rate": p.min_attendance_rate,
            "auto_graduate_final_level": p.auto_graduate_final_level,
            "final_level": p.final_level,
            "exclude_withdrawn": p.exclude_withdrawn,
            "default_template_id": p.default_template_id,
            "created_at": p.created_at,
            "updated_at": p.updated_at,
        }),
        None => serde_json::json!({
            "id": uuid::Uuid::new_v4().to_string(),
            "org_id": org_id,
            "org_unit_id": null,
            "default_grading_system": "percentage",
            "default_passing_score": 50.0,
            "default_passing_grade": 50.0,
            "default_max_score": 100.0,
            "default_max_grade": 100.0,
            "default_scale_id": null,
            "default_pass_fail_policy_id": null,
            "enrollment_auto_close": 1,
            "allow_late_enrollment": 0,
            "late_enrollment_approval": 1,
            "grade_entry_auto_close": 1,
            "allow_grade_edit_after_close": 0,
            "grade_edit_approval": 1,
            "min_attendance_rate": 75.0,
            "auto_graduate_final_level": 0,
            "final_level": null,
            "exclude_withdrawn": 1,
            "default_template_id": null,
        }),
    };

    Ok(result)
}

/// Update the academic policy.
pub async fn update_academic_policy(
    ctx: &ServiceContext,
    org_id: &str,
    payload: serde_json::Value,
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

    // ── Grading references & defaults ──
    if let Some(gs) = get_str_val(&payload, &["default_grading_system", "grading_system"]) {
        active.default_grading_system = Set(gs);
    }
    if let Some(pass_score) = get_f64_or_num(&payload, &["default_passing_score", "default_passing_grade", "passing_grade"]) {
        active.default_passing_score = Set(pass_score);
    }
    if let Some(max_score) = get_f64_or_num(&payload, &["default_max_score", "default_max_grade", "max_grade"]) {
        active.default_max_score = Set(max_score);
    }
    if let Some(scale_id) = get_str_val(&payload, &["default_scale_id"]) {
        active.default_scale_id = Set(Some(scale_id));
    }
    if let Some(pfp_id) = get_str_val(&payload, &["default_pass_fail_policy_id"]) {
        active.default_pass_fail_policy_id = Set(Some(pfp_id));
    }

    // ── Enrollment policies ──
    if let Some(v) = get_i32_or_bool(&payload, &["enrollment_auto_close"]) {
        active.enrollment_auto_close = Set(v);
    }
    if let Some(v) = get_i32_or_bool(&payload, &["allow_late_enrollment"]) {
        active.allow_late_enrollment = Set(v);
    }
    if let Some(v) = get_i32_or_bool(&payload, &["late_enrollment_approval"]) {
        active.late_enrollment_approval = Set(v);
    }

    // ── Grade entry policies ──
    if let Some(v) = get_i32_or_bool(&payload, &["grade_entry_auto_close"]) {
        active.grade_entry_auto_close = Set(v);
    }
    if let Some(v) = get_i32_or_bool(&payload, &["allow_grade_edit_after_close"]) {
        active.allow_grade_edit_after_close = Set(v);
    }
    if let Some(v) = get_i32_or_bool(&payload, &["grade_edit_approval"]) {
        active.grade_edit_approval = Set(v);
    }

    // ── Promotion policies ──
    if let Some(v) = get_f64_or_num(&payload, &["min_attendance_rate"]) {
        active.min_attendance_rate = Set(v);
    }
    if let Some(v) = get_i32_or_bool(&payload, &["auto_graduate_final_level"]) {
        active.auto_graduate_final_level = Set(v);
    }
    if let Some(v) = get_str_val(&payload, &["final_level"]) {
        active.final_level = Set(Some(v));
    }
    if let Some(v) = get_i32_or_bool(&payload, &["exclude_withdrawn"]) {
        active.exclude_withdrawn = Set(v);
    }

    active.updated_at = Set(now);
    active.save(&ctx.db).await?;

    // Audit log
    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "academic_policy.update".into(),
        entity_type: "academic_policy".into(),
        entity_id: org_id.to_string(),
        old_data: None,
        new_data: Some(payload),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(())
}

// ════════════════════════════════════════════════════════════════════════════
// 2. AI Autopilot Operations (Automation mode, task triggers, deadlines)
// ════════════════════════════════════════════════════════════════════════════

/// Get the autopilot configuration for the AI Center.
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
        Some(p) => serde_json::json!({
            "enabled": p.autopilot_enabled == 1,
            "execution_mode": p.autopilot_mode,
            "auto_open_enrollment": p.auto_open_enrollment == 1,
            "auto_close_enrollment": p.auto_close_enrollment == 1,
            "auto_open_grade_entry": p.auto_open_grade_entry == 1,
            "auto_close_grade_entry": p.auto_close_grade_entry == 1,
            "auto_lock_periods": p.auto_lock_periods == 1,
            "auto_advance_periods": p.auto_advance_periods == 1,
            "auto_archive_years": p.auto_archive_years == 1,
            "grace_days": p.grace_days_after_deadline,
            "notify_before_deadline_hours": p.notify_before_deadline_days * 24,
        }),
        None => serde_json::json!({
            "enabled": true,
            "execution_mode": "suggested",
            "auto_open_enrollment": true,
            "auto_close_enrollment": true,
            "auto_open_grade_entry": true,
            "auto_close_grade_entry": true,
            "auto_lock_periods": true,
            "auto_advance_periods": false,
            "auto_archive_years": false,
            "grace_days": 3,
            "notify_before_deadline_hours": 72,
        }),
    };

    Ok(config_json)
}

/// Update the autopilot configuration from the AI Center.
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

    if let Some(v) = get_i32_or_bool(&config, &["enabled", "autopilot_enabled"]) {
        active.autopilot_enabled = Set(v);
    }
    if let Some(v) = get_str_val(&config, &["execution_mode", "autopilot_mode"]) {
        active.autopilot_mode = Set(v);
    }
    if let Some(v) = get_i32_or_bool(&config, &["auto_open_enrollment"]) {
        active.auto_open_enrollment = Set(v);
    }
    if let Some(v) = get_i32_or_bool(&config, &["auto_close_enrollment"]) {
        active.auto_close_enrollment = Set(v);
    }
    if let Some(v) = get_i32_or_bool(&config, &["auto_open_grade_entry"]) {
        active.auto_open_grade_entry = Set(v);
    }
    if let Some(v) = get_i32_or_bool(&config, &["auto_close_grade_entry"]) {
        active.auto_close_grade_entry = Set(v);
    }
    if let Some(v) = get_i32_or_bool(&config, &["auto_lock_periods", "auto_period_lock"]) {
        active.auto_lock_periods = Set(v);
    }
    if let Some(v) = get_i32_or_bool(&config, &["grace_days", "grace_days_after_deadline"]) {
        active.grace_days_after_deadline = Set(v);
    }
    if let Some(v) = get_i32_or_bool(&config, &["notify_before_deadline_days"]) {
        active.notify_before_deadline_days = Set(v);
    } else if let Some(hrs) = get_i32_or_bool(&config, &["notify_before_deadline_hours"]) {
        active.notify_before_deadline_days = Set(hrs / 24);
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
