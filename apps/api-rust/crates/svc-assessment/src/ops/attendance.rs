//! Attendance operations — record, bulk record, reports, analytics.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, PaginatorTrait};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::attendance;
use crate::services::attendance::{
    CreateAttendanceRequest, BulkAttendanceRequest,
    AttendanceReport, AttendanceAnalytics,
};

// ── Queries ─────────────────────────────────────────────────────

/// List attendance records.
pub async fn list(
    ctx: &ServiceContext,
    cycle_id: &str,
    class_id: Option<String>,
    date: Option<String>,
    enrollment_id: Option<String>,
) -> Result<Vec<attendance::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AttendanceRead).await?;

    let mut query = attendance::Entity::find()
        .filter(attendance::Column::OrgId.eq(&ctx.org_id))
        .filter(attendance::Column::CycleId.eq(cycle_id));

    if let Some(cid) = class_id {
        query = query.filter(attendance::Column::ClassId.eq(cid));
    }
    if let Some(d) = date {
        query = query.filter(attendance::Column::Date.eq(d));
    }
    if let Some(eid) = enrollment_id {
        query = query.filter(attendance::Column::EnrollmentId.eq(eid));
    }

    Ok(query
        .order_by_desc(attendance::Column::Date)
        .all(&ctx.db)
        .await?)
}

/// Get attendance report for a student.
pub async fn get_report(
    ctx: &ServiceContext,
    cycle_id: &str,
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
    cycle_id: &str,
    class_id: &str,
) -> Result<AttendanceAnalytics, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AttendanceAnalytics).await?;

    let query_repo = crate::repository::AssessmentQueryRepo::new(ctx.db.clone());
    let cmd_repo = crate::repository::AssessmentCommandRepo::new(ctx.db.clone());
    let svc = crate::services::attendance::AttendanceService::new(
        std::sync::Arc::new(cmd_repo),
        std::sync::Arc::new(query_repo),
    );
    svc.get_attendance_analytics(cycle_id, class_id).await
}

// ── Commands ────────────────────────────────────────────────────

/// Record attendance for a single student.
pub async fn record(
    ctx: &ServiceContext,
    cycle_id: &str,
    req: CreateAttendanceRequest,
) -> Result<attendance::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AttendanceWrite).await?;

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
        cycle_id: cycle_id.to_string(),
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
    cycle_id: &str,
    req: BulkAttendanceRequest,
) -> Result<Vec<attendance::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AttendanceWrite).await?;

    let mut results = Vec::with_capacity(req.records.len());
    for record in req.records {
        let attendance_req = CreateAttendanceRequest {
            enrollment_id: record.enrollment_id,
            class_id: req.class_id.clone(),
            date: req.date.clone(),
            status: record.status,
            check_in_time: None,
            check_out_time: None,
            period: req.period.clone(),
            subject_name: req.subject_name.clone(),
            notes: record.notes,
            excuse_type: None,
            is_excused: None,
        };
        let created = record_single_no_check(ctx, cycle_id, attendance_req).await?;
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
    cycle_id: &str,
    req: CreateAttendanceRequest,
) -> Result<attendance::Model, AppError> {
    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = attendance::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        cycle_id: cycle_id.to_string(),
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
