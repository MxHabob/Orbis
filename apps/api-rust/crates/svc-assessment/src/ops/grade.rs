//! Grade operations — submission, bulk entry, report cards, analytics.
//!
//! Business rules:
//! - Cycle must be unlocked and grade entry window open
//! - Grade history is saved on every update
//! - Grade letter/GPA calculated from org grading settings

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::{grade, grade_history};
use crate::services::grade::{
    CreateGradeRequest, UpdateGradeRequest, BulkSubjectGradeEntry, BulkSubjectGradeResult,
    UpsertCellRequest, UpsertCellResult, GradeReportCard, GradeAnalytics, GradeMatrixResponse,
    StudentScore,
};

use svc_organization::entities::academic_cycle;

// ── Helpers ─────────────────────────────────────────────────────

/// Check if cycle is open for grade entry.
async fn check_cycle_open(ctx: &ServiceContext, cycle_id: &str) -> Result<(), AppError> {
    let cycle = academic_cycle::Entity::find_by_id(cycle_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("academic_cycle", cycle_id))?;

    if cycle.is_locked == 1 {
        return Err(AppError::unauthorized("Academic cycle is locked"));
    }
    if cycle.is_grade_entry_open == 0 {
        return Err(AppError::unauthorized("Grade entry window is closed"));
    }
    if let (Some(ref start), Some(ref end)) = (&cycle.grade_entry_start, &cycle.grade_entry_end) {
        let now = Utc::now().to_rfc3339();
        if now < *start {
            return Err(AppError::unauthorized(format!("Grade entry has not started yet. Starts: {}", start)));
        }
        if now > *end {
            return Err(AppError::unauthorized(format!("Grade entry ended on: {}", end)));
        }
    }
    Ok(())
}

fn score_to_grade_letter(percentage: f64) -> String {
    match percentage as i32 {
        90..=100 => "ممتاز".to_string(),
        80..=89  => "جيد جداً".to_string(),
        70..=79  => "جيد".to_string(),
        60..=69  => "مقبول".to_string(),
        50..=59  => "ضعيف".to_string(),
        _        => "راسب".to_string(),
    }
}

fn score_to_grade_points(percentage: f64) -> f64 {
    match percentage as i32 {
        90..=100 => 4.0,
        80..=89  => 3.5,
        70..=79  => 3.0,
        60..=69  => 2.5,
        50..=59  => 2.0,
        _        => 0.0,
    }
}

async fn save_grade_history(
    ctx: &ServiceContext,
    existing: &grade::Model,
    changed_by: &str,
    change_reason: Option<String>,
    change_type: &str,
) -> Result<(), AppError> {
    let history = grade_history::Model {
        id: Uuid::new_v4().to_string(),
        grade_id: existing.id.clone(),
        score: Some(existing.score),
        grade_letter: existing.grade_letter.clone(),
        is_absent: None,
        is_excused: None,
        comment: existing.notes.clone(),
        change_type: change_type.to_string(),
        change_reason,
        changed_by: Some(changed_by.to_string()),
        changed_at: Utc::now().to_rfc3339(),
        snapshot: Some(serde_json::to_string(existing).unwrap_or_default()),
        created_at: Utc::now().to_rfc3339(),
    };
    let active: grade_history::ActiveModel = history.into();
    active.insert(&ctx.db).await?;
    Ok(())
}

// ── Queries ─────────────────────────────────────────────────────

/// List grades, filtered by cycle + optional class/enrollment/subject.
pub async fn list(
    ctx: &ServiceContext,
    cycle_id: &str,
    enrollment_id: Option<String>,
    class_subject_id: Option<String>,
) -> Result<Vec<grade::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeRead).await?;

    let mut query = grade::Entity::find()
        .filter(grade::Column::OrgId.eq(&ctx.org_id))
        .filter(grade::Column::CycleId.eq(cycle_id));

    if let Some(eid) = enrollment_id {
        query = query.filter(grade::Column::EnrollmentId.eq(eid));
    }
    if let Some(csid) = class_subject_id {
        query = query.filter(grade::Column::ClassSubjectId.eq(csid));
    }

    Ok(query
        .order_by_asc(grade::Column::SubjectName)
        .all(&ctx.db)
        .await?)
}

/// Get a single grade report card for a student enrollment.
pub async fn get_report_card(
    ctx: &ServiceContext,
    enrollment_id: &str,
    cycle_id: &str,
) -> Result<GradeReportCard, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeReport).await?;

    // Delegate to existing service logic
    let query_repo = crate::repository::AssessmentQueryRepo::new(ctx.db.clone());
    let cmd_repo = crate::repository::AssessmentCommandRepo::new(ctx.db.clone());
    let svc = crate::services::grade::GradeService::new(
        std::sync::Arc::new(cmd_repo),
        std::sync::Arc::new(query_repo),
    );
    svc.get_report_card(enrollment_id, cycle_id).await
}

/// Get grade analytics for a class-subject.
pub async fn get_analytics(
    ctx: &ServiceContext,
    cycle_id: &str,
    class_subject_id: &str,
) -> Result<GradeAnalytics, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeAnalytics).await?;

    let query_repo = crate::repository::AssessmentQueryRepo::new(ctx.db.clone());
    let cmd_repo = crate::repository::AssessmentCommandRepo::new(ctx.db.clone());
    let svc = crate::services::grade::GradeService::new(
        std::sync::Arc::new(cmd_repo),
        std::sync::Arc::new(query_repo),
    );
    svc.get_analytics(cycle_id, class_subject_id).await
}

/// Get the grade matrix for a class-subject (grid view).
pub async fn get_matrix(
    ctx: &ServiceContext,
    cycle_id: &str,
    class_subject_id: &str,
) -> Result<GradeMatrixResponse, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeRead).await?;

    let query_repo = crate::repository::AssessmentQueryRepo::new(ctx.db.clone());
    let cmd_repo = crate::repository::AssessmentCommandRepo::new(ctx.db.clone());
    let svc = crate::services::grade::GradeService::new(
        std::sync::Arc::new(cmd_repo),
        std::sync::Arc::new(query_repo),
    );
    svc.get_grade_matrix(cycle_id, class_subject_id).await
}

// ── Commands ────────────────────────────────────────────────────

/// Submit a single grade.
pub async fn submit(
    ctx: &ServiceContext,
    cycle_id: &str,
    req: CreateGradeRequest,
) -> Result<grade::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeWrite).await?;
    check_cycle_open(ctx, cycle_id).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let percentage = if req.max_score > 0.0 { (req.score / req.max_score) * 100.0 } else { 0.0 };

    let model = grade::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        cycle_id: cycle_id.to_string(),
        enrollment_id: req.enrollment_id,
        assessment_id: req.assessment_id,
        class_subject_id: req.class_subject_id,
        subject_name: req.subject_name,
        subject_name_ar: req.subject_name_ar,
        score: req.score,
        max_score: req.max_score,
        weight: req.weight.unwrap_or(1.0),
        grade_letter: Some(score_to_grade_letter(percentage)),
        grade_points: Some(score_to_grade_points(percentage)),
        exam_type: req.exam_type,
        semester: req.semester,
        notes: req.notes,
        graded_by: ctx.actor.as_str().to_string(),
        graded_at: now.clone(),
        is_final: req.is_final.unwrap_or(0),
        created_at: now.clone(),
        updated_at: now,
    };

    let active: grade::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "grade.submit".into(),
        entity_type: "grade".into(),
        entity_id: id,
        old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

/// Submit multiple grades at once.
pub async fn submit_bulk(
    ctx: &ServiceContext,
    cycle_id: &str,
    grades: Vec<CreateGradeRequest>,
) -> Result<Vec<grade::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeWrite).await?;
    check_cycle_open(ctx, cycle_id).await?;

    let mut results = Vec::with_capacity(grades.len());
    for req in grades {
        let grade = submit(ctx, cycle_id, req).await?;
        results.push(grade);
    }
    Ok(results)
}

/// Update an existing grade (saves history).
pub async fn update(
    ctx: &ServiceContext,
    grade_id: &str,
    req: UpdateGradeRequest,
) -> Result<grade::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeWrite).await?;

    let existing = grade::Entity::find_by_id(grade_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("grade", grade_id))?;

    check_cycle_open(ctx, &existing.cycle_id).await?;

    // Save history before update
    save_grade_history(ctx, &existing, ctx.actor.as_str(), req.change_reason.clone(), "update").await?;

    let old_data = serde_json::to_value(&existing).ok();
    let percentage = if req.max_score > 0.0 { (req.score / req.max_score) * 100.0 } else { 0.0 };

    let mut active: grade::ActiveModel = existing.into();
    active.score = Set(req.score);
    active.max_score = Set(req.max_score);
    active.grade_letter = Set(Some(score_to_grade_letter(percentage)));
    active.grade_points = Set(Some(score_to_grade_points(percentage)));
    if let Some(notes) = req.notes {
        active.notes = Set(Some(notes));
    }
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "grade.update".into(),
        entity_type: "grade".into(),
        entity_id: grade_id.into(),
        old_data,
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(updated)
}

/// Bulk entry for a single subject across an entire class.
pub async fn bulk_subject_entry(
    ctx: &ServiceContext,
    entry: BulkSubjectGradeEntry,
) -> Result<BulkSubjectGradeResult, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeWrite).await?;
    check_cycle_open(ctx, &entry.cycle_id).await?;

    // Delegate to existing service logic which handles upsert logic
    let query_repo = crate::repository::AssessmentQueryRepo::new(ctx.db.clone());
    let cmd_repo = crate::repository::AssessmentCommandRepo::new(ctx.db.clone());
    let svc = crate::services::grade::GradeService::new(
        std::sync::Arc::new(cmd_repo),
        std::sync::Arc::new(query_repo),
    );
    let result = svc.bulk_subject_grade_entry(&ctx.org_id, ctx.actor.as_str(), entry).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "grade.bulk_subject_entry".into(),
        entity_type: "grade".into(),
        entity_id: "bulk".into(),
        old_data: None,
        new_data: Some(serde_json::json!({
            "created": result.created,
            "updated": result.updated,
            "failed": result.failed,
        })),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(result)
}

/// Upsert a single grade cell (create or update).
pub async fn upsert_cell(
    ctx: &ServiceContext,
    req: UpsertCellRequest,
) -> Result<UpsertCellResult, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeWrite).await?;
    check_cycle_open(ctx, &req.cycle_id).await?;

    let query_repo = crate::repository::AssessmentQueryRepo::new(ctx.db.clone());
    let cmd_repo = crate::repository::AssessmentCommandRepo::new(ctx.db.clone());
    let svc = crate::services::grade::GradeService::new(
        std::sync::Arc::new(cmd_repo),
        std::sync::Arc::new(query_repo),
    );
    svc.upsert_grade_cell(&ctx.org_id, ctx.actor.as_str(), req).await
}

/// Compile semester grades (aggregate all grades into final scores).
pub async fn compile_semester(
    ctx: &ServiceContext,
    cycle_id: &str,
    class_id: &str,
    semester: &str,
) -> Result<Vec<grade::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeWrite).await?;

    let query_repo = crate::repository::AssessmentQueryRepo::new(ctx.db.clone());
    let cmd_repo = crate::repository::AssessmentCommandRepo::new(ctx.db.clone());
    let svc = crate::services::grade::GradeService::new(
        std::sync::Arc::new(cmd_repo),
        std::sync::Arc::new(query_repo),
    );
    svc.compile_semester_grades(&ctx.org_id, ctx.actor.as_str(), cycle_id, class_id, semester).await
}
