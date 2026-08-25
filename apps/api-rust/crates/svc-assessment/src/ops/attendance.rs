//! Attendance operations — record, bulk record, reports, analytics.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use std::collections::HashMap;
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::attendance;
use crate::params::AttendanceParams;
use crate::services::attendance::{
    AttendanceReport, AttendanceAnalytics,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ClassMetrics {
    pub class_id: String,
    pub active_students: usize,
    pub capacity: usize,
    pub utilization_rate: f64,
    pub attendance_rate: f64,
    pub present_count: usize,
    pub absent_count: usize,
    pub late_count: usize,
    pub excused_count: usize,
    pub at_risk_students: usize,
    pub subject_count: usize,
    pub weekly_hours: i32,
}

/// Aggregate class health metrics for one academic period.
/// Loads each source once to avoid one request/query per class.
pub async fn class_metrics(
    ctx: &ServiceContext,
    period_id: &str,
) -> Result<Vec<ClassMetrics>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AttendanceAnalytics).await?;

    let classes = svc_academic::entities::class::Entity::find()
        .filter(svc_academic::entities::class::Column::OrgId.eq(&ctx.org_id))
        .filter(svc_academic::entities::class::Column::PeriodId.eq(period_id))
        .filter(svc_academic::entities::class::Column::IsDeleted.eq(0))
        .filter(svc_academic::entities::class::Column::IsActive.eq(1))
        .all(&ctx.db)
        .await?;

    let enrollments = svc_academic::entities::enrollment::Entity::find()
        .filter(svc_academic::entities::enrollment::Column::OrgId.eq(&ctx.org_id))
        .filter(svc_academic::entities::enrollment::Column::PeriodId.eq(period_id))
        .filter(svc_academic::entities::enrollment::Column::Status.eq("active"))
        .all(&ctx.db)
        .await?;

    let attendance_records = attendance::Entity::find()
        .filter(attendance::Column::OrgId.eq(&ctx.org_id))
        .filter(attendance::Column::PeriodId.eq(period_id))
        .all(&ctx.db)
        .await?;

    let class_subjects = svc_academic::entities::class_subject::Entity::find()
        .filter(svc_academic::entities::class_subject::Column::PeriodId.eq(period_id))
        .filter(svc_academic::entities::class_subject::Column::IsActive.eq(1))
        .all(&ctx.db)
        .await?;

    let mut enrollment_counts = HashMap::<String, usize>::new();
    for enrollment in &enrollments {
        *enrollment_counts.entry(enrollment.class_id.clone()).or_default() += 1;
    }

    let mut attendance_totals = HashMap::<String, (usize, usize, usize, usize, usize)>::new();
    let mut absences_by_enrollment = HashMap::<String, usize>::new();
    for record in &attendance_records {
        let entry = attendance_totals.entry(record.class_id.clone()).or_default();
        entry.0 += 1;
        match record.status.as_str() {
            "present" => entry.1 += 1,
            "absent" => {
                entry.2 += 1;
                *absences_by_enrollment.entry(record.enrollment_id.clone()).or_default() += 1;
            }
            "late" => entry.3 += 1,
            "excused" | "sick" => entry.4 += 1,
            _ => {}
        }
    }

    let mut subjects_by_class = HashMap::<String, (usize, i32)>::new();
    for class_subject in class_subjects {
        let entry = subjects_by_class.entry(class_subject.class_id).or_default();
        entry.0 += 1;
        entry.1 += class_subject.weekly_hours.unwrap_or(0);
    }

    let at_risk_by_class = enrollments.iter().fold(HashMap::<String, usize>::new(), |mut result, enrollment| {
        if absences_by_enrollment.get(&enrollment.id).copied().unwrap_or(0) >= 5 {
            *result.entry(enrollment.class_id.clone()).or_default() += 1;
        }
        result
    });

    Ok(classes.into_iter().map(|class| {
        let active_students = enrollment_counts.get(&class.id).copied().unwrap_or(0);
        let capacity = class.capacity.unwrap_or(0).max(0) as usize;
        let (total, present, absent, late, excused) = attendance_totals.get(&class.id).copied().unwrap_or_default();
        let attended = present + late;
        let attendance_rate = if total > 0 { (attended as f64 / total as f64) * 100.0 } else { 0.0 };
        let utilization_rate = if capacity > 0 { (active_students as f64 / capacity as f64 * 100.0).min(100.0) } else { 0.0 };
        let (subject_count, weekly_hours) = subjects_by_class.get(&class.id).copied().unwrap_or_default();

        let class_id = class.id.clone();
        ClassMetrics {
            class_id: class_id.clone(),
            active_students,
            capacity,
            utilization_rate: (utilization_rate * 100.0).round() / 100.0,
            attendance_rate: (attendance_rate * 100.0).round() / 100.0,
            present_count: present,
            absent_count: absent,
            late_count: late,
            excused_count: excused,
            at_risk_students: at_risk_by_class.get(&class_id).copied().unwrap_or(0),
            subject_count,
            weekly_hours,
        }
    }).collect())
}

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
    pub lesson_slot: Option<String>,
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
    pub lesson_slot: Option<String>,
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

    // ── Period Gate: check period is active ──
    {
        use shared_kernel::period_gate::GateType;
        let verdict = ctx.period_gate
            .check(&ctx.db, &ctx.org_id, &req.period_id, GateType::Attendance, None)
            .await?;
        verdict.enforce()?;
    }

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
        lesson_slot: req.lesson_slot,
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

    // ── Period Gate: single check for the entire bulk (same period) ──
    {
        use shared_kernel::period_gate::GateType;
        let verdict = ctx.period_gate
            .check(&ctx.db, &ctx.org_id, &req.period_id, GateType::Attendance, None)
            .await?;
        verdict.enforce()?;
    }

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
            lesson_slot: req.lesson_slot.clone(),
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
        lesson_slot: req.lesson_slot,
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
