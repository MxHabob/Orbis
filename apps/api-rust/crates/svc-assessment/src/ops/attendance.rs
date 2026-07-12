//! Attendance operations — record, bulk record, reports, analytics.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::attendance;
use crate::params::AttendanceParams;
use crate::services::attendance::{
    AttendanceReport, AttendanceAnalytics,
};

// ── Queries ─────────────────────────────────────────────────────

/// List attendance records.
pub async fn list(
    ctx: &ServiceContext,
    class_id: &str,
    date: &str,
) -> Result<Vec<attendance::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AttendanceRead).await?;

    Ok(attendance::Entity::find()
        .filter(attendance::Column::OrgId.eq(&ctx.org_id))
        .filter(attendance::Column::ClassId.eq(class_id))
        .filter(attendance::Column::Date.eq(date))
        .order_by_asc(attendance::Column::EnrollmentId)
        .all(&ctx.db)
        .await?)
}

/// Get attendance report for a student.
pub async fn get_report(
    ctx: &ServiceContext,
    _period_id: &str,
    enrollment_id: &str,
) -> Result<AttendanceReport, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AttendanceReport).await?;

    let query_repo = crate::repository::AssessmentQueryRepo::new(ctx.db.clone());
    let cmd_repo = crate::repository::AssessmentCommandRepo::new(ctx.db.clone());
    let svc = crate::services::attendance::AttendanceService::new(
        std::sync::Arc::new(cmd_repo),
        std::sync::Arc::new(query_repo),
    );
    svc.get_attendance_report(enrollment_id).await
}

/// Get attendance analytics for a class.
pub async fn get_analytics(
    ctx: &ServiceContext,
    period_id: &str,
    class_id: &str,
) -> Result<AttendanceAnalytics, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AttendanceAnalytics).await?;

    let query_repo = crate::repository::AssessmentQueryRepo::new(ctx.db.clone());
    let cmd_repo = crate::repository::AssessmentCommandRepo::new(ctx.db.clone());
    let svc = crate::services::attendance::AttendanceService::new(
        std::sync::Arc::new(cmd_repo),
        std::sync::Arc::new(query_repo),
    );
    let params = AttendanceParams {
        period_id: Some(period_id.to_string()),
        class_id: Some(class_id.to_string()),
        ..Default::default()
    };
    svc.get_attendance_analytics(&ctx.org_id, params).await
}

// ── Commands ────────────────────────────────────────────────────

#[derive(serde::Deserialize)]
pub struct CreateAttendancePayload {
    pub period_id: String,
    pub enrollment_id: String,
    pub class_id: String,
    pub date: String,
    pub status: String,
    pub check_in_time: Option<String>,
    pub check_out_time: Option<String>,
    pub period: Option<String>,
    pub subject_name: Option<String>,
    pub notes: Option<String>,
    pub excuse_type: Option<String>,
    pub is_excused: Option<i32>,
}

#[derive(serde::Deserialize)]
pub struct BulkAttendancePayload {
    pub period_id: String,
    pub class_id: String,
    pub date: String,
    pub period: Option<String>,
    pub subject_name: Option<String>,
    pub records: Vec<BulkAttendanceItem>,
}

#[derive(serde::Deserialize)]
pub struct BulkAttendanceItem {
    pub enrollment_id: String,
    pub status: String,
    pub notes: Option<String>,
}

/// Record attendance for a single student.
pub async fn record(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<attendance::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AttendanceWrite).await?;

    let req: CreateAttendancePayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid record attendance payload: {e}")))?;

    // Validate status
    let valid_statuses = ["present", "absent", "late", "excused", "sick"];
    if !valid_statuses.contains(&req.status.as_str()) {
        return Err(AppError::validation(format!(
            "Invalid status: {}. Allowed: {:?}", req.status, valid_statuses
        )));
    }

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = attendance::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        period_id: req.period_id,
        enrollment_id: req.enrollment_id,
        class_id: req.class_id,
        date: req.date,
        status: req.status,
        check_in_time: req.check_in_time,
        check_out_time: req.check_out_time,
        period: req.period,
        subject_name: req.subject_name,
        recorded_by: ctx.actor.as_str().to_string(),
        notes: req.notes,
        excuse_type: req.excuse_type,
        excuse_document: None,
        is_excused: req.is_excused.unwrap_or(0),
        created_at: now,
    };

    let active: attendance::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "attendance.record".into(),
        entity_type: "attendance".into(),
        entity_id: id,
        old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

/// Record bulk attendance for an entire class.
pub async fn record_bulk(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<Vec<attendance::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AttendanceWrite).await?;

    let req: BulkAttendancePayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid bulk attendance payload: {e}")))?;

    let mut results = Vec::with_capacity(req.records.len());
    for item in req.records {
        let attendance_req = CreateAttendancePayload {
            period_id: req.period_id.clone(),
            enrollment_id: item.enrollment_id,
            class_id: req.class_id.clone(),
            date: req.date.clone(),
            status: item.status,
            check_in_time: None,
            check_out_time: None,
            period: req.period.clone(),
            subject_name: req.subject_name.clone(),
            notes: item.notes,
            excuse_type: None,
            is_excused: None,
        };
        let created = record_single_no_check(ctx, attendance_req).await?;
        results.push(created);
    }

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "attendance.record_bulk".into(),
        entity_type: "attendance".into(),
        entity_id: "bulk".into(),
        old_data: None,
        new_data: Some(serde_json::json!({ "count": results.len() })),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(results)
}

/// Internal: record without re-checking permissions (already checked in record_bulk).
async fn record_single_no_check(
    ctx: &ServiceContext,
    req: CreateAttendancePayload,
) -> Result<attendance::Model, AppError> {
    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = attendance::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        period_id: req.period_id,
        enrollment_id: req.enrollment_id,
        class_id: req.class_id,
        date: req.date,
        status: req.status,
        check_in_time: req.check_in_time,
        check_out_time: req.check_out_time,
        period: req.period,
        subject_name: req.subject_name,
        recorded_by: ctx.actor.as_str().to_string(),
        notes: req.notes,
        excuse_type: req.excuse_type,
        excuse_document: None,
        is_excused: req.is_excused.unwrap_or(0),
        created_at: now,
    };

    let active: attendance::ActiveModel = model.into();
    Ok(active.insert(&ctx.db).await?)
}

pub async fn report(
    ctx: &ServiceContext,
    class_id: &str,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<attendance::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AttendanceReport).await?;

    Ok(attendance::Entity::find()
        .filter(attendance::Column::OrgId.eq(&ctx.org_id))
        .filter(attendance::Column::ClassId.eq(class_id))
        .filter(attendance::Column::Date.gte(start_date))
        .filter(attendance::Column::Date.lte(end_date))
        .order_by_asc(attendance::Column::Date)
        .order_by_asc(attendance::Column::EnrollmentId)
        .all(&ctx.db)
        .await?)
}
