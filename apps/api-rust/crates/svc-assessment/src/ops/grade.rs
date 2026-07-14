//! Grade operations — submission, bulk entry, report cards, analytics.
//!
//! Business rules:
//! - Period must be unlocked and grade entry window open
//! - Grade history is saved on every update
//! - Grade letter/GPA calculated from org grading settings

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::{grade, grade_history};
use crate::services::grade::{
    UpdateGradeRequest, BulkSubjectGradeEntry, BulkSubjectGradeResult,
    UpsertCellRequest, UpsertCellResult, GradeReportCard, GradeAnalytics, GradeMatrixResponse,
};

use svc_organization::entities::academic_period;

// ── Helpers ─────────────────────────────────────────────────────

/// Check if period is open for grade entry via the centralized Period Gate.
/// Replaces the old manual check with cached, policy-aware enforcement.
async fn check_period_open(ctx: &ServiceContext, period_id: &str) -> Result<(), AppError> {
    use shared_kernel::period_gate::GateType;
    let verdict = ctx.period_gate
        .check(&ctx.db, &ctx.org_id, period_id, GateType::GradeEntry)
        .await?;
    verdict.enforce()
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

/// List grades, filtered by assessment_id.
pub async fn list(
    ctx: &ServiceContext,
    assessment_id: &str,
) -> Result<Vec<grade::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeRead).await?;

    Ok(grade::Entity::find()
        .filter(grade::Column::OrgId.eq(&ctx.org_id))
        .filter(grade::Column::AssessmentId.eq(assessment_id))
        .order_by_asc(grade::Column::SubjectName)
        .all(&ctx.db)
        .await?)
}

/// Paginated grades list — cursor-based for large grade sheets.
pub async fn list_paginated(
    ctx: &ServiceContext,
    assessment_id: &str,
    request: shared_kernel::pagination::CursorRequest,
) -> Result<shared_kernel::pagination::CursorPage<grade::Model>, AppError> {
    use shared_kernel::pagination::CursorPage;
    ctx.permissions.check(&ctx.actor, Permission::GradeRead).await?;

    let limit = request.effective_limit();
    let mut query = grade::Entity::find()
        .filter(grade::Column::OrgId.eq(&ctx.org_id))
        .filter(grade::Column::AssessmentId.eq(assessment_id))
        .order_by_asc(grade::Column::Id);

    if let Some(ref cursor) = request.cursor {
        query = query.filter(grade::Column::Id.gt(cursor));
    }

    let items = query.limit(limit + 1).all(&ctx.db).await?;
    Ok(CursorPage::from_items_with(items, limit, |g| g.id.clone()))
}

/// Get a single grade report card for a student enrollment.
pub async fn report_card(
    ctx: &ServiceContext,
    enrollment_id: &str,
    period_id: &str,
) -> Result<GradeReportCard, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeReport).await?;

    // Delegate to existing service logic
    let query_repo = crate::repository::AssessmentQueryRepo::new(ctx.db.clone());
    let cmd_repo = crate::repository::AssessmentCommandRepo::new(ctx.db.clone());
    let svc = crate::services::grade::GradeService::new(
        std::sync::Arc::new(cmd_repo),
        std::sync::Arc::new(query_repo),
    );
    svc.get_report_card(enrollment_id, period_id).await
}

/// Get grade analytics for a class-subject.
pub async fn analytics(
    ctx: &ServiceContext,
    period_id: &str,
    class_subject_id: &str,
) -> Result<GradeAnalytics, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeAnalytics).await?;

    let query_repo = crate::repository::AssessmentQueryRepo::new(ctx.db.clone());
    let cmd_repo = crate::repository::AssessmentCommandRepo::new(ctx.db.clone());
    let svc = crate::services::grade::GradeService::new(
        std::sync::Arc::new(cmd_repo),
        std::sync::Arc::new(query_repo),
    );
    svc.get_analytics(period_id, class_subject_id).await
}

/// Get the grade matrix for a class-subject (grid view).
pub async fn matrix(
    ctx: &ServiceContext,
    period_id: &str,
    class_subject_id: &str,
) -> Result<GradeMatrixResponse, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeRead).await?;

    let query_repo = crate::repository::AssessmentQueryRepo::new(ctx.db.clone());
    let cmd_repo = crate::repository::AssessmentCommandRepo::new(ctx.db.clone());
    let svc = crate::services::grade::GradeService::new(
        std::sync::Arc::new(cmd_repo),
        std::sync::Arc::new(query_repo),
    );
    svc.get_grade_matrix(period_id, class_subject_id).await
}

// ── Commands ────────────────────────────────────────────────────

#[derive(serde::Deserialize)]
pub struct CreateGradePayload {
    pub period_id: String,
    pub enrollment_id: String,
    pub subject_name: String,
    pub subject_name_ar: Option<String>,
    pub score: f64,
    pub max_score: f64,
    pub weight: Option<f64>,
    pub exam_type: String,
    pub semester: String,
    pub notes: Option<String>,
    pub is_final: Option<i32>,
    pub assessment_id: Option<String>,
    pub class_subject_id: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct SubmitBulkPayload {
    pub period_id: String,
    pub grades: Vec<CreateGradePayload>,
}

/// Submit a single grade.
pub async fn submit(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<grade::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeWrite).await?;

    let req: CreateGradePayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid grade payload: {e}")))?;

    check_period_open(ctx, &req.period_id).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let percentage = if req.max_score > 0.0 { (req.score / req.max_score) * 100.0 } else { 0.0 };

    let model = grade::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        period_id: req.period_id,
        enrollment_id: req.enrollment_id,
        assessment_id: req.assessment_id,
        class_subject_id: req.class_subject_id,
        subject_name: req.subject_name,
        subject_name_ar: req.subject_name_ar,
        score: req.score,
        max_score: req.max_score,
        weight: req.weight.unwrap_or(1.0),
        grade_letter: Some(crate::services::grade::GradeService::score_to_grade_letter(percentage)),
        grade_points: Some(crate::services::grade::GradeService::score_to_grade_points(percentage)),
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
    data: serde_json::Value,
) -> Result<Vec<grade::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeWrite).await?;

    let grades: Vec<CreateGradePayload> = if let Ok(payload) = serde_json::from_value::<SubmitBulkPayload>(data.clone()) {
        payload.grades
    } else {
        serde_json::from_value(data)
            .map_err(|e| AppError::validation(format!("Invalid bulk grade payload: {e}")))?
    };

    let mut results = Vec::with_capacity(grades.len());
    for req in grades {
        check_period_open(ctx, &req.period_id).await?;

        let now = Utc::now().to_rfc3339();
        let id = Uuid::new_v4().to_string();
        let percentage = if req.max_score > 0.0 { (req.score / req.max_score) * 100.0 } else { 0.0 };

        let model = grade::Model {
            id: id.clone(),
            org_id: ctx.org_id.clone(),
            period_id: req.period_id,
            enrollment_id: req.enrollment_id,
            assessment_id: req.assessment_id,
            class_subject_id: req.class_subject_id,
            subject_name: req.subject_name,
            subject_name_ar: req.subject_name_ar,
            score: req.score,
            max_score: req.max_score,
            weight: req.weight.unwrap_or(1.0),
            grade_letter: Some(crate::services::grade::GradeService::score_to_grade_letter(percentage)),
            grade_points: Some(crate::services::grade::GradeService::score_to_grade_points(percentage)),
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
        results.push(created);
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

    check_period_open(ctx, &existing.period_id).await?;

    // Save history before update
    save_grade_history(ctx, &existing, ctx.actor.as_str(), req.change_reason.clone(), "update").await?;

    let old_data = serde_json::to_value(&existing).ok();
    let percentage = if req.max_score > 0.0 { (req.score / req.max_score) * 100.0 } else { 0.0 };

    let mut active: grade::ActiveModel = existing.into();
    active.score = Set(req.score);
    active.max_score = Set(req.max_score);
    active.grade_letter = Set(Some(crate::services::grade::GradeService::score_to_grade_letter(percentage)));
    active.grade_points = Set(Some(crate::services::grade::GradeService::score_to_grade_points(percentage)));
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
    check_period_open(ctx, &entry.period_id).await?;

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
    data: serde_json::Value,
) -> Result<UpsertCellResult, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeWrite).await?;
    
    let req: UpsertCellRequest = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid upsert payload: {e}")))?;

    check_period_open(ctx, &req.period_id).await?;

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
    period_id: &str,
    class_id: &str,
) -> Result<Vec<grade::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeWrite).await?;

    let query_repo = crate::repository::AssessmentQueryRepo::new(ctx.db.clone());
    let cmd_repo = crate::repository::AssessmentCommandRepo::new(ctx.db.clone());
    let svc = crate::services::grade::GradeService::new(
        std::sync::Arc::new(cmd_repo),
        std::sync::Arc::new(query_repo),
    );
    svc.compile_semester_grades(&ctx.org_id, ctx.actor.as_str(), period_id, class_id, "semester_1").await
}
