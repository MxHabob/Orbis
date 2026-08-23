//! Annual Calculation Engine — combines term results into annual results.
//!
//! The annual result is the sum of all term results for a student in an
//! academic year. The default school configuration produces:
//!
//!   Term 1 = 50 points
//!   Term 2 = 50 points
//!   Annual Total = 100 points
//!
//! Like the term engine, this is fully driven by CalculationPolicy
//! (rule_type = "annual"). No formulas are hard-coded.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use tracing::info;

use crate::entities::{calculation_policy, term_result, annual_result};
use crate::ops::calc_engine::{CalcEngine, AstNode, apply_rounding};
use crate::ops::grade_mapper::{GradeMapper, GradeScaleItemData};

/// Input for an annual calculation request.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct AnnualCalcRequest {
    pub student_id: String,
    pub enrollment_id: String,
    pub year_period_id: String,
    pub class_subject_id: String,
    /// IDs of the term-level periods within this year.
    pub term_period_ids: Vec<String>,
}

/// Calculate the annual result by combining term results.
pub async fn calculate_annual_result(
    ctx: &ServiceContext,
    req: &AnnualCalcRequest,
) -> Result<annual_result::Model, AppError> {
    // 1. Find all term results for this student + class_subject
    let mut term_scores: Vec<serde_json::Value> = Vec::new();
    let mut annual_total = 0.0f64;
    let mut annual_max = 0.0f64;

    for term_period_id in &req.term_period_ids {
        let term = term_result::Entity::find()
            .filter(term_result::Column::OrgId.eq(&ctx.org_id))
            .filter(term_result::Column::StudentId.eq(&req.student_id))
            .filter(term_result::Column::TermPeriodId.eq(term_period_id))
            .filter(term_result::Column::ClassSubjectId.eq(&req.class_subject_id))
            .one(&ctx.db)
            .await?;

        if let Some(tr) = term {
            annual_total += tr.total_score;
            annual_max += tr.total_max;
            term_scores.push(serde_json::json!({
                "term_period_id": tr.term_period_id,
                "total_score": tr.total_score,
                "total_max": tr.total_max,
                "percentage": tr.percentage,
                "grade_label": tr.grade_label,
                "is_passing": tr.is_passing == 1,
            }));
        }
    }

    // 2. Try to use an annual CalculationPolicy, else use simple sum
    let policy = find_annual_policy(ctx, &req.year_period_id).await.ok();

    let (final_total, rounding) = if let Some(ref p) = policy {
        let ast: AstNode = serde_json::from_str(&p.formula_expression).map_err(|e| {
            AppError::validation(format!("Invalid annual calculation formula: {}", e))
        })?;

        let mut engine = CalcEngine::new();
        for (i, ts) in term_scores.iter().enumerate() {
            let score = ts.get("total_score").and_then(|v| v.as_f64()).unwrap_or(0.0);
            engine.set_variable(&format!("term_{}", i + 1), score);
        }
        engine.set_variable("term_count", term_scores.len() as f64);
        engine.set_variable("annual_total", annual_total);

        let raw = engine.evaluate(&ast)?;
        (apply_rounding(raw, &p.rounding_policy), p.rounding_policy.clone())
    } else {
        // Default: simple sum of term totals
        (annual_total, "round_half_up".to_string())
    };

    let percentage = if annual_max > 0.0 {
        apply_rounding((final_total / annual_max) * 100.0, &rounding)
    } else {
        0.0
    };

    // 3. Grade mapping
    let mapper = build_grade_mapper_annual(ctx).await?;
    let grade_result = mapper.map_score(percentage)?;

    // 4. Policy snapshot
    let policy_snapshot = policy.as_ref().map(|p| {
        serde_json::json!({
            "policy_id": p.id,
            "version": p.version,
            "rule_type": p.rule_type,
            "formula_expression": p.formula_expression,
            "rounding_policy": p.rounding_policy,
        }).to_string()
    });

    // 5. Persist
    let now = Utc::now().to_rfc3339();
    let result_model = annual_result::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        org_id: Set(ctx.org_id.clone()),
        student_id: Set(req.student_id.clone()),
        year_period_id: Set(req.year_period_id.clone()),
        enrollment_id: Set(req.enrollment_id.clone()),
        class_subject_id: Set(req.class_subject_id.clone()),
        term_scores_json: Set(serde_json::to_string(&term_scores).unwrap_or_else(|_| "[]".to_string())),
        annual_total: Set(final_total),
        annual_max: Set(annual_max),
        percentage: Set(percentage),
        grade_label: Set(Some(grade_result.label)),
        gpa_points: Set(grade_result.gpa_points),
        is_passing: Set(if grade_result.is_passing { 1 } else { 0 }),
        rank: Set(None),
        rank_scope: Set(None),
        calculation_policy_id: Set(policy.as_ref().map(|p| p.id.clone())),
        calculation_version: Set(policy.as_ref().map(|p| p.version).unwrap_or(1)),
        policy_snapshot: Set(policy_snapshot),
        status: Set("official".to_string()),
        finalized_at: Set(None),
        finalized_by: Set(None),
        created_at: Set(now.clone()),
        updated_at: Set(now),
    };

    let inserted = result_model.insert(&ctx.db).await?;

    info!(
        student_id=%req.student_id,
        year_period_id=%req.year_period_id,
        annual_total=%final_total,
        "Calculated annual result"
    );

    Ok(inserted)
}

/// Finalize an annual result — makes it immutable.
pub async fn finalize_annual_result(
    ctx: &ServiceContext,
    result_id: &str,
    actor: &str,
) -> Result<annual_result::Model, AppError> {
    let existing = annual_result::Entity::find_by_id(result_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("annual_result", result_id))?;

    if existing.status == "finalized" {
        return Err(AppError::validation("Annual result is already finalized"));
    }

    let now = Utc::now().to_rfc3339();
    let mut active: annual_result::ActiveModel = existing.into();
    active.status = Set("finalized".to_string());
    active.finalized_at = Set(Some(now.clone()));
    active.finalized_by = Set(Some(actor.to_string()));
    active.updated_at = Set(now);

    let updated = active.update(&ctx.db).await?;
    Ok(updated)
}

// ── Internal Helpers ────────────────────────────────────────────

async fn find_annual_policy(
    ctx: &ServiceContext,
    year_period_id: &str,
) -> Result<calculation_policy::Model, AppError> {
    // Try period-specific
    let period_policy = calculation_policy::Entity::find()
        .filter(calculation_policy::Column::OrgId.eq(&ctx.org_id))
        .filter(calculation_policy::Column::TargetPeriodId.eq(year_period_id))
        .filter(calculation_policy::Column::RuleType.eq("annual"))
        .filter(calculation_policy::Column::IsArchived.eq(0))
        .one(&ctx.db)
        .await?;

    if let Some(p) = period_policy {
        return Ok(p);
    }

    // Fallback to org-wide
    calculation_policy::Entity::find()
        .filter(calculation_policy::Column::OrgId.eq(&ctx.org_id))
        .filter(calculation_policy::Column::RuleType.eq("annual"))
        .filter(calculation_policy::Column::IsArchived.eq(0))
        .filter(calculation_policy::Column::TargetPeriodId.is_null())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::validation(
            "No annual calculation policy configured."
        ))
}

async fn build_grade_mapper_annual(ctx: &ServiceContext) -> Result<GradeMapper, AppError> {
    // Reuse the term engine's mapper builder
    super::term_engine::build_grade_mapper(ctx).await
}
