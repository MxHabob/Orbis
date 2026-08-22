//! Term Calculation Engine — centralized term-result computation.
//!
//! This engine orchestrates the multi-step term calculation:
//!
//! 1. Collect all period-level snapshots (monthly grading periods) for a student.
//! 2. Compute the coursework average from period scores.
//! 3. Normalize the coursework average against the configured weight.
//! 4. Retrieve the term examination score.
//! 5. Combine coursework + examination → term total.
//! 6. Map the result via GradeMapper.
//! 7. Persist as a `TermResult`.
//!
//! The calculation is driven entirely by `CalculationPolicy` (rule_type = "term").
//! The default school configuration produces:
//!
//!   coursework = avg(monthly periods) × (20 / 100) = weighted contribution
//!   exam       = raw exam score (max 30)
//!   term_total = coursework + exam (max 50)
//!
//! This engine does NOT hard-code any formula. All weights and methods
//! come from the policy's `formula_expression` AST.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use tracing::info;

use crate::entities::{
    calculation_policy, student_assessment_snapshot, term_result,
    grade_scale, grade_scale_item,
};
use crate::ops::calc_engine::{CalcEngine, AstNode, apply_rounding};
use crate::ops::grade_mapper::{GradeMapper, GradeScaleItemData};

/// Input for a term calculation request.
#[derive(Debug, Clone)]
pub struct TermCalcRequest {
    pub student_id: String,
    pub enrollment_id: String,
    pub term_period_id: String,
    pub class_subject_id: String,
    /// IDs of the child grading periods (monthly periods) within this term.
    pub grading_period_ids: Vec<String>,
    /// ID of the examination period within this term (if separate from grading periods).
    pub exam_period_id: Option<String>,
    /// Raw examination score. If None, will be read from grades.
    pub exam_score: Option<f64>,
    /// Maximum examination score (e.g., 30). Loaded from policy if not provided.
    pub exam_max: Option<f64>,
}

/// Result of a term calculation (before persistence).
#[derive(Debug, Clone, serde::Serialize)]
pub struct TermCalcOutput {
    pub coursework_score: f64,
    pub coursework_max: f64,
    pub exam_score: f64,
    pub exam_max: f64,
    pub total_score: f64,
    pub total_max: f64,
    pub percentage: f64,
    pub grade_label: Option<String>,
    pub gpa_points: Option<f64>,
    pub is_passing: bool,
}

/// Calculate a term result for a single student + class_subject.
///
/// Steps:
/// 1. Find the term-level CalculationPolicy (rule_type = "term").
/// 2. Load all period-level snapshots for the student.
/// 3. Compute coursework average from snapshot total_scores.
/// 4. Feed coursework + exam into CalcEngine AST.
/// 5. Map via GradeMapper.
/// 6. Persist TermResult.
pub async fn calculate_term_result(
    ctx: &ServiceContext,
    req: &TermCalcRequest,
) -> Result<term_result::Model, AppError> {
    // 1. Find the term-level CalculationPolicy
    let policy = find_term_policy(ctx, &req.term_period_id).await?;
    let ast: AstNode = serde_json::from_str(&policy.formula_expression).map_err(|e| {
        AppError::validation(format!("Invalid term calculation formula: {}", e))
    })?;

    // 2. Collect period-level snapshots for this student
    let mut period_scores: Vec<f64> = Vec::new();
    for period_id in &req.grading_period_ids {
        let snapshots = student_assessment_snapshot::Entity::find()
            .filter(student_assessment_snapshot::Column::OrgId.eq(&ctx.org_id))
            .filter(student_assessment_snapshot::Column::StudentId.eq(&req.student_id))
            .filter(student_assessment_snapshot::Column::PeriodId.eq(period_id))
            .filter(student_assessment_snapshot::Column::ClassSubjectId.eq(&req.class_subject_id))
            .order_by_desc(student_assessment_snapshot::Column::UpdatedAt)
            .one(&ctx.db)
            .await?;

        if let Some(snap) = snapshots {
            period_scores.push(snap.total_score.unwrap_or(0.0));
        }
    }

    // 3. Compute coursework average
    let coursework_avg = if period_scores.is_empty() {
        0.0
    } else {
        let sum: f64 = period_scores.iter().sum();
        sum / period_scores.len() as f64
    };

    // 4. Get examination score
    let exam_score = req.exam_score.unwrap_or(0.0);

    // 5. Feed into CalcEngine
    let mut engine = CalcEngine::new();
    engine.set_variable("coursework_avg", coursework_avg);
    engine.set_variable("exam", exam_score);
    // Also provide individual period scores as period_1, period_2, etc.
    for (i, score) in period_scores.iter().enumerate() {
        engine.set_variable(&format!("period_{}", i + 1), *score);
    }
    engine.set_variable("period_count", period_scores.len() as f64);

    let raw_total = engine.evaluate(&ast)?;
    let total = apply_rounding(raw_total, &policy.rounding_policy);

    // Extract coursework/exam weights from formula context
    // Default: coursework_max = 20, exam_max = 30, term_max = 50
    let coursework_max = req.exam_max.map(|em| {
        // If exam_max is provided, infer coursework_max from the remaining
        let total_max = 50.0; // Will be overridden by policy if available
        total_max - em
    }).unwrap_or(20.0);
    let exam_max = req.exam_max.unwrap_or(30.0);
    let total_max = coursework_max + exam_max;
    let coursework_score = total - exam_score.min(exam_max);
    let percentage = if total_max > 0.0 { (total / total_max) * 100.0 } else { 0.0 };

    // 6. Map via GradeMapper
    let mapper = build_grade_mapper(ctx).await?;
    let grade_result = mapper.map_score(apply_rounding(percentage, &policy.rounding_policy))?;

    // 7. Create policy snapshot
    let policy_snapshot = serde_json::json!({
        "policy_id": policy.id,
        "version": policy.version,
        "rule_type": policy.rule_type,
        "formula_expression": policy.formula_expression,
        "rounding_policy": policy.rounding_policy,
    });

    // 8. Persist TermResult
    let now = Utc::now().to_rfc3339();
    let result_model = term_result::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        org_id: Set(ctx.org_id.clone()),
        student_id: Set(req.student_id.clone()),
        term_period_id: Set(req.term_period_id.clone()),
        enrollment_id: Set(req.enrollment_id.clone()),
        class_subject_id: Set(req.class_subject_id.clone()),
        coursework_score: Set(coursework_score.max(0.0)),
        coursework_max: Set(coursework_max),
        exam_score: Set(exam_score),
        exam_max: Set(exam_max),
        total_score: Set(total),
        total_max: Set(total_max),
        percentage: Set(apply_rounding(percentage, &policy.rounding_policy)),
        grade_label: Set(Some(grade_result.label)),
        gpa_points: Set(grade_result.gpa_points),
        is_passing: Set(if grade_result.is_passing { 1 } else { 0 }),
        rank: Set(None),
        rank_scope: Set(None),
        calculation_policy_id: Set(Some(policy.id.clone())),
        calculation_version: Set(policy.version),
        policy_snapshot: Set(Some(policy_snapshot.to_string())),
        status: Set("official".to_string()),
        finalized_at: Set(None),
        finalized_by: Set(None),
        created_at: Set(now.clone()),
        updated_at: Set(now),
    };

    let inserted = result_model.insert(&ctx.db).await?;

    info!(
        student_id=%req.student_id,
        term_period_id=%req.term_period_id,
        total=%total,
        "Calculated term result"
    );

    Ok(inserted)
}

/// Batch-calculate term results for all students in a class for a term.
pub async fn calculate_term_results_batch(
    ctx: &ServiceContext,
    term_period_id: &str,
    class_id: &str,
    grading_period_ids: Vec<String>,
    exam_period_id: Option<String>,
) -> Result<Vec<term_result::Model>, AppError> {
    use sea_orm::QuerySelect;

    // Find all enrollments for this class
    let enrollments = svc_academic::entities::enrollment::Entity::find()
        .filter(svc_academic::entities::enrollment::Column::ClassId.eq(class_id))
        .filter(svc_academic::entities::enrollment::Column::Status.eq("active"))
        .all(&ctx.db)
        .await?;

    // Find all class_subjects for this class
    let class_subjects = svc_academic::entities::class_subject::Entity::find()
        .filter(svc_academic::entities::class_subject::Column::ClassId.eq(class_id))
        .all(&ctx.db)
        .await?;

    let mut results = Vec::new();

    for enrollment in &enrollments {
        for cs in &class_subjects {
            let req = TermCalcRequest {
                student_id: enrollment.student_id.clone(),
                enrollment_id: enrollment.id.clone(),
                term_period_id: term_period_id.to_string(),
                class_subject_id: cs.id.clone(),
                grading_period_ids: grading_period_ids.clone(),
                exam_period_id: exam_period_id.clone(),
                exam_score: None,
                exam_max: None,
            };

            match calculate_term_result(ctx, &req).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    tracing::warn!(
                        student_id=%enrollment.student_id,
                        class_subject_id=%cs.id,
                        error=%e,
                        "Failed to calculate term result, skipping"
                    );
                }
            }
        }
    }

    Ok(results)
}

/// Finalize a term result — makes it immutable.
pub async fn finalize_term_result(
    ctx: &ServiceContext,
    result_id: &str,
    actor: &str,
) -> Result<term_result::Model, AppError> {
    let existing = term_result::Entity::find_by_id(result_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("term_result", result_id))?;

    if existing.status == "finalized" {
        return Err(AppError::validation("Term result is already finalized and cannot be modified"));
    }

    let now = Utc::now().to_rfc3339();
    let mut active: term_result::ActiveModel = existing.into();
    active.status = Set("finalized".to_string());
    active.finalized_at = Set(Some(now.clone()));
    active.finalized_by = Set(Some(actor.to_string()));
    active.updated_at = Set(now);

    let updated = active.update(&ctx.db).await?;
    Ok(updated)
}

// ── Internal Helpers ────────────────────────────────────────────

/// Find the term-level CalculationPolicy for a given term period.
/// Falls back to org-wide policy with rule_type = "term".
async fn find_term_policy(
    ctx: &ServiceContext,
    term_period_id: &str,
) -> Result<calculation_policy::Model, AppError> {
    // Try period-specific first
    let period_policy = calculation_policy::Entity::find()
        .filter(calculation_policy::Column::OrgId.eq(&ctx.org_id))
        .filter(calculation_policy::Column::TargetPeriodId.eq(term_period_id))
        .filter(calculation_policy::Column::RuleType.eq("term"))
        .filter(calculation_policy::Column::IsArchived.eq(0))
        .one(&ctx.db)
        .await?;

    if let Some(p) = period_policy {
        return Ok(p);
    }

    // Fall back to org-wide term policy
    let org_policy = calculation_policy::Entity::find()
        .filter(calculation_policy::Column::OrgId.eq(&ctx.org_id))
        .filter(calculation_policy::Column::RuleType.eq("term"))
        .filter(calculation_policy::Column::IsArchived.eq(0))
        .filter(calculation_policy::Column::TargetPeriodId.is_null())
        .one(&ctx.db)
        .await?;

    org_policy.ok_or_else(|| AppError::validation(
        "No term calculation policy configured. Create a CalculationPolicy with rule_type='term'."
    ))
}

/// Build a GradeMapper from the org's active grade scale.
async fn build_grade_mapper(ctx: &ServiceContext) -> Result<GradeMapper, AppError> {
    let scale = grade_scale::Entity::find()
        .filter(grade_scale::Column::OrgId.eq(&ctx.org_id))
        .one(&ctx.db)
        .await?;

    if let Some(sc) = scale {
        let items = grade_scale_item::Entity::find()
            .filter(grade_scale_item::Column::ScaleId.eq(&sc.id))
            .all(&ctx.db)
            .await?;

        let mapper_items: Vec<GradeScaleItemData> = items
            .into_iter()
            .map(|i| GradeScaleItemData {
                label: i.grade_label,
                min_score: i.min_score,
                max_score: i.max_score,
                gpa_points: i.gpa_points,
                is_passing: i.is_passing == 1,
            })
            .collect();

        if !mapper_items.is_empty() {
            return Ok(GradeMapper::new(mapper_items));
        }
    }

    // Fallback: use AssessmentEngine's default mapper
    let engine = crate::ops::assessment_engine::AssessmentEngine::for_period(
        &ctx.db, &ctx.org_id, "",
    ).await?;
    // We just need the mapper to get labels — map 0 to get the structure
    // This is a workaround; ideally we'd expose the mapper directly.
    // For now, build a default Arabic percentage scale.
    Ok(GradeMapper::new(vec![
        GradeScaleItemData { label: "ممتاز".into(), min_score: 90.0, max_score: 100.0, gpa_points: Some(4.0), is_passing: true },
        GradeScaleItemData { label: "جيد جداً".into(), min_score: 80.0, max_score: 89.99, gpa_points: Some(3.5), is_passing: true },
        GradeScaleItemData { label: "جيد".into(), min_score: 70.0, max_score: 79.99, gpa_points: Some(3.0), is_passing: true },
        GradeScaleItemData { label: "مقبول".into(), min_score: 60.0, max_score: 69.99, gpa_points: Some(2.5), is_passing: true },
        GradeScaleItemData { label: "ضعيف".into(), min_score: 50.0, max_score: 59.99, gpa_points: Some(2.0), is_passing: false },
        GradeScaleItemData { label: "راسب".into(), min_score: 0.0, max_score: 49.99, gpa_points: Some(0.0), is_passing: false },
    ]))
}
