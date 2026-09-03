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

use crate::entities::grade;
use crate::ops::assessment_engine::AssessmentEngine;
use crate::services::grade::{
    UpdateGradeRequest, BulkSubjectGradeEntry, BulkSubjectGradeResult,
    UpsertCellRequest, UpsertCellResult, GradeReportCard, GradeAnalytics, GradeMatrixResponse,
};


// ── Helpers ─────────────────────────────────────────────────────

/// Check if period is open for grade entry via the centralized Period Gate.
/// Replaces the old manual check with cached, policy-aware enforcement.
async fn check_period_open(ctx: &ServiceContext, period_id: &str) -> Result<(), AppError> {
    use shared_kernel::period_gate::GateType;
    let verdict = ctx.period_gate
        .check(&ctx.db, &ctx.org_id, period_id, GateType::GradeEntry, None)
        .await?;
    verdict.enforce()
}

async fn apply_branch_filter(
    ctx: &ServiceContext,
    query: sea_orm::Select<grade::Entity>,
) -> Result<sea_orm::Select<grade::Entity>, AppError> {
    if ctx.org_unit_id.is_none() {
        return Ok(query);
    }
    let enrollment_ids = svc_academic::ops::scope::scoped_enrollment_ids(ctx).await?;
    Ok(query.filter(grade::Column::EnrollmentId.is_in(enrollment_ids)))
}

/// Attempt to enqueue a snapshot generation job after a grade is saved.
/// If no CalculationPolicy or GradeScale exists yet, this is a no-op (best effort).
async fn try_enqueue_snapshot(
    ctx: &ServiceContext,
    grade: &grade::Model,
) {
    use sea_orm::{QueryFilter, ColumnTrait};

    // Find the student_id from the enrollment
    let enrollment = svc_academic::entities::enrollment::Entity::find_by_id(grade.enrollment_id.clone())
        .one(&ctx.db)
        .await;
    let student_id = match enrollment {
        Ok(Some(e)) => e.student_id,
        _ => return, // no enrollment found — skip
    };

    let policy = match crate::ops::resolution::resolve_calc_policy(
        &ctx.db,
        &ctx.org_id,
        &grade.period_id,
        "period",
    )
    .await
    {
        Ok(Some(p)) => p,
        _ => match crate::ops::resolution::resolve_calc_policy(
            &ctx.db,
            &ctx.org_id,
            &grade.period_id,
            "weighted_sum",
        )
        .await
        {
            Ok(Some(p)) => p,
            _ => return,
        },
    };

    let scale_id = match crate::ops::resolution::resolve_scale_id(&ctx.db, &ctx.org_id, &grade.period_id).await {
        Ok(Some(id)) => id,
        _ => return,
    };

    // Build raw_scores from all grades for this enrollment + class_subject + period
    let class_subject_id = match &grade.class_subject_id {
        Some(id) => id.clone(),
        None => return, // no class_subject_id — can't snapshot
    };

    let all_grades = grade::Entity::find()
        .filter(grade::Column::EnrollmentId.eq(&grade.enrollment_id))
        .filter(grade::Column::PeriodId.eq(&grade.period_id))
        .filter(grade::Column::ClassSubjectId.eq(&class_subject_id))
        .all(&ctx.db)
        .await;
    let all_grades = match all_grades {
        Ok(g) => g,
        _ => return,
    };

    let mut raw_scores = std::collections::HashMap::new();
    for g in &all_grades {
        if let Some(cid) = &g.component_id {
            if let Ok(Some(comp)) = crate::entities::assessment_scheme_component::Entity::find_by_id(cid.clone())
                .one(&ctx.db)
                .await
            {
                raw_scores.insert(comp.component_type.clone(), g.score);
                continue;
            }
        }
        raw_scores.insert(g.exam_type.clone(), g.score);
    }

    let job = crate::ops::snapshot_job::SnapshotJob {
        ctx: ctx.clone(),
        student_id,
        period_id: grade.period_id.clone(),
        class_subject_id,
        policy_id: policy.id,
        scale_id,
        raw_scores,
    };

    if let Err(e) = ctx.job_queue.try_enqueue(job) {
        tracing::warn!(error=%e, "Failed to enqueue snapshot job after grade save");
    }
}


// ── Queries ─────────────────────────────────────────────────────

use shared_kernel::query_filter::UniversalFilter;

/// List grades, filtered by assessment_id.
pub async fn list(
    ctx: &ServiceContext,
    assessment_id: &str,
    filter: Option<UniversalFilter>,
) -> Result<Vec<grade::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeRead).await?;

    let mut query = grade::Entity::find()
        .filter(grade::Column::OrgId.eq(&ctx.org_id))
        .filter(grade::Column::AssessmentId.eq(assessment_id));
    query = apply_branch_filter(ctx, query).await?;

    if let Some(f) = filter {
        let condition = f.build_condition(|field| match field {
            "enrollment_id" => Some(grade::Column::EnrollmentId.into_expr().into()),
            "score" => Some(grade::Column::Score.into_expr().into()),
            "subject_name" => Some(grade::Column::SubjectName.into_expr().into()),
            _ => None,
        });
        query = query.filter(condition);
    }

    Ok(query.order_by_asc(grade::Column::SubjectName).all(&ctx.db).await?)
}

/// List grades for a period + class_subject (gradebook cells).
pub async fn list_by_period_and_class_subject(
    ctx: &ServiceContext,
    period_id: &str,
    class_subject_id: &str,
) -> Result<Vec<grade::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeRead).await?;

    let mut query = grade::Entity::find()
        .filter(grade::Column::OrgId.eq(&ctx.org_id))
        .filter(grade::Column::PeriodId.eq(period_id))
        .filter(grade::Column::ClassSubjectId.eq(class_subject_id));
    query = apply_branch_filter(ctx, query).await?;
    Ok(query.order_by_asc(grade::Column::EnrollmentId).all(&ctx.db).await?)
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
    query = apply_branch_filter(ctx, query).await?;

    if let Some(f) = request.filter.clone() {
        let condition = f.build_condition(|field| match field {
            "enrollment_id" => Some(grade::Column::EnrollmentId.into_expr().into()),
            "score" => Some(grade::Column::Score.into_expr().into()),
            "subject_name" => Some(grade::Column::SubjectName.into_expr().into()),
            _ => None,
        });
        query = query.filter(condition);
    }

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
    svc_academic::ops::scope::ensure_enrollment_scope(ctx, enrollment_id).await?;

    // Delegate to existing service logic
    let query_repo = crate::repository::AssessmentQueryRepo::new(ctx.db.clone());
    let cmd_repo = crate::repository::AssessmentCommandRepo::new(ctx.db.clone());
    let svc = crate::services::grade::GradeService::new(
        std::sync::Arc::new(cmd_repo),
        std::sync::Arc::new(query_repo),
    );
    svc.get_report_card(&ctx.org_id, enrollment_id, period_id).await
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
    svc.get_analytics(&ctx.org_id, period_id, class_subject_id).await
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
    svc.get_grade_matrix(&ctx.org_id, period_id, class_subject_id).await
}

// ── Commands ────────────────────────────────────────────────────

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
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
    pub component_id: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct SubmitBulkPayload {
    pub period_id: String,
    pub grades: Vec<CreateGradePayload>,
}

async fn find_existing_component_grade(
    ctx: &ServiceContext,
    req: &CreateGradePayload,
) -> Result<Option<grade::Model>, AppError> {
    let (Some(cs), Some(comp)) = (req.class_subject_id.as_ref(), req.component_id.as_ref()) else {
        return Ok(None);
    };
    Ok(grade::Entity::find()
        .filter(grade::Column::OrgId.eq(&ctx.org_id))
        .filter(grade::Column::EnrollmentId.eq(&req.enrollment_id))
        .filter(grade::Column::ClassSubjectId.eq(cs))
        .filter(grade::Column::PeriodId.eq(&req.period_id))
        .filter(grade::Column::ComponentId.eq(comp))
        .one(&ctx.db)
        .await?)
}

async fn persist_grade(
    ctx: &ServiceContext,
    req: CreateGradePayload,
    engine: &AssessmentEngine,
) -> Result<grade::Model, AppError> {
    svc_academic::ops::scope::ensure_enrollment_scope(ctx, &req.enrollment_id).await?;
    check_period_open(ctx, &req.period_id).await?;

    let now = Utc::now().to_rfc3339();
    let percentage = if req.max_score > 0.0 { (req.score / req.max_score) * 100.0 } else { 0.0 };
    let mapped = engine.map_score(percentage)?;

    if let Some(existing) = find_existing_component_grade(ctx, &req).await? {
        let mut active: grade::ActiveModel = existing.into();
        active.score = Set(req.score);
        active.max_score = Set(req.max_score);
        active.weight = Set(req.weight.unwrap_or(1.0));
        active.grade_letter = Set(Some(mapped.grade_letter));
        active.grade_points = Set(mapped.grade_points);
        active.exam_type = Set(req.exam_type);
        active.notes = Set(req.notes);
        active.graded_by = Set(ctx.actor.as_str().to_string());
        active.graded_at = Set(now.clone());
        active.updated_at = Set(now);
        let updated = active.update(&ctx.db).await?;
        try_enqueue_snapshot(ctx, &updated).await;
        return Ok(updated);
    }

    let id = Uuid::new_v4().to_string();
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
        grade_letter: Some(mapped.grade_letter),
        grade_points: mapped.grade_points,
        exam_type: req.exam_type,
        component_id: req.component_id,
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
    try_enqueue_snapshot(ctx, &created).await;
    Ok(created)
}

/// Submit a single grade (upsert when component + offering is set).
pub async fn submit(
    ctx: &ServiceContext,
    req: CreateGradePayload,
) -> Result<grade::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeWrite).await?;
    svc_academic::ops::scope::ensure_enrollment_scope(ctx, &req.enrollment_id).await?;

    let engine = AssessmentEngine::for_period(&ctx.db, &ctx.org_id, &req.period_id).await?;
    let saved = persist_grade(ctx, req, &engine).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "grade.submit".into(),
        entity_type: "grade".into(),
        entity_id: saved.id.clone(),
        old_data: None,
        new_data: Some(serde_json::to_value(&saved).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(saved)
}

/// Submit multiple grades at once.
pub async fn submit_bulk(
    ctx: &ServiceContext,
    req: SubmitBulkPayload,
) -> Result<Vec<grade::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeWrite).await?;

    let grades = req.grades;

    // Cache AssessmentEngine instances by period_id to avoid N+1 queries.
    // In the common case (bulk entry for a single period), this reduces
    // the engine construction from N calls to 1.
    let mut engine_cache: std::collections::HashMap<String, AssessmentEngine> = std::collections::HashMap::new();

    let mut results = Vec::with_capacity(grades.len());
    for req in grades {
        if !engine_cache.contains_key(&req.period_id) {
            let new_engine = AssessmentEngine::for_period(&ctx.db, &ctx.org_id, &req.period_id).await?;
            engine_cache.insert(req.period_id.clone(), new_engine);
        }
        let engine = engine_cache.get(&req.period_id).unwrap();
        let saved = persist_grade(ctx, req, engine).await?;
        results.push(saved);
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
        .filter(grade::Column::OrgId.eq(&ctx.org_id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("grade", grade_id))?;
    svc_academic::ops::scope::ensure_enrollment_scope(ctx, &existing.enrollment_id).await?;

    check_period_open(ctx, &existing.period_id).await?;

    // Save history before update
    

    let old_data = serde_json::to_value(&existing).ok();
    let percentage = if req.max_score > 0.0 { (req.score / req.max_score) * 100.0 } else { 0.0 };

    // Use AssessmentEngine instead of hardcoded scale
    let engine = AssessmentEngine::for_period(&ctx.db, &ctx.org_id, &existing.period_id).await?;
    let mapped = engine.map_score(percentage)?;

    let mut active: grade::ActiveModel = existing.into();
    active.score = Set(req.score);
    active.max_score = Set(req.max_score);
    active.grade_letter = Set(Some(mapped.grade_letter));
    active.grade_points = Set(mapped.grade_points);
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

    // Auto-trigger snapshot regeneration after grade update
    try_enqueue_snapshot(ctx, &updated).await;

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
    let result = svc.bulk_subject_grade_entry(&ctx.org_id, entry).await?;

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

    check_period_open(ctx, &req.period_id).await?;

    let query_repo = crate::repository::AssessmentQueryRepo::new(ctx.db.clone());
    let cmd_repo = crate::repository::AssessmentCommandRepo::new(ctx.db.clone());
    let svc = crate::services::grade::GradeService::new(
        std::sync::Arc::new(cmd_repo),
        std::sync::Arc::new(query_repo),
    );
    svc.upsert_grade_cell(&ctx.org_id, req).await
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
    svc.compile_semester_grades(&ctx.org_id, period_id, class_id, "semester_1").await
}
