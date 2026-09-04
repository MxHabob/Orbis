//! Term Calculation Engine — centralized term-result computation.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::Permission;
use tracing::info;

use crate::entities::{
    calculation_policy, grade, grade_scale, grade_scale_item, period_score, term_result,
};
use crate::ops::calc_engine::{apply_rounding, AstNode, CalcEngine};
use crate::ops::grade_mapper::{GradeMapper, GradeScaleItemData};
use crate::ops::resolution;
use svc_iam::entities::{person, user};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TermCalcRequest {
    pub student_id: String,
    pub enrollment_id: String,
    pub term_period_id: String,
    pub class_subject_id: String,
    #[serde(default)]
    pub grading_period_ids: Vec<String>,
    pub exam_period_id: Option<String>,
    pub exam_score: Option<f64>,
    pub exam_max: Option<f64>,
}

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

pub fn fallback_grade_mapper() -> GradeMapper {
    GradeMapper::new(vec![
        GradeScaleItemData { label: "ممتاز".into(), min_score: 90.0, max_score: 100.0, gpa_points: Some(4.0), is_passing: true },
        GradeScaleItemData { label: "جيد جداً".into(), min_score: 80.0, max_score: 89.99, gpa_points: Some(3.5), is_passing: true },
        GradeScaleItemData { label: "جيد".into(), min_score: 70.0, max_score: 79.99, gpa_points: Some(3.0), is_passing: true },
        GradeScaleItemData { label: "مقبول".into(), min_score: 60.0, max_score: 69.99, gpa_points: Some(2.5), is_passing: true },
        GradeScaleItemData { label: "ضعيف".into(), min_score: 50.0, max_score: 59.99, gpa_points: Some(2.0), is_passing: false },
        GradeScaleItemData { label: "راسب".into(), min_score: 0.0, max_score: 49.99, gpa_points: Some(0.0), is_passing: false },
    ])
}

pub async fn list_term_results(
    ctx: &ServiceContext,
    term_period_id: &str,
    class_subject_id: Option<&str>,
) -> Result<Vec<term_result::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeRead).await?;
    let mut q = term_result::Entity::find()
        .filter(term_result::Column::OrgId.eq(&ctx.org_id))
        .filter(term_result::Column::TermPeriodId.eq(term_period_id));
    if let Some(cs) = class_subject_id {
        q = q.filter(term_result::Column::ClassSubjectId.eq(cs));
    }
    Ok(q.order_by_asc(term_result::Column::StudentId).all(&ctx.db).await?)
}

pub async fn list_term_results_enriched(
    ctx: &ServiceContext,
    term_period_id: &str,
    class_subject_id: Option<&str>,
) -> Result<Vec<serde_json::Value>, AppError> {
    let results = list_term_results(ctx, term_period_id, class_subject_id).await?;
    if results.is_empty() {
        return Ok(Vec::new());
    }

    let student_ids: Vec<String> = results.iter().map(|r| r.student_id.clone()).collect();
    
    let persons_by_id: HashMap<String, person::Model> = person::Entity::find()
        .filter(person::Column::Id.is_in(student_ids.clone()))
        .all(&ctx.db)
        .await?
        .into_iter()
        .map(|p| (p.id.clone(), p))
        .collect();

    let users_by_id: HashMap<String, user::Model> = user::Entity::find()
        .filter(user::Column::Id.is_in(student_ids.clone()))
        .all(&ctx.db)
        .await?
        .into_iter()
        .map(|u| (u.id.clone(), u))
        .collect();
        
    let mut missing_person_ids = Vec::new();
    for u in users_by_id.values() {
        if !persons_by_id.contains_key(&u.person_id) {
            missing_person_ids.push(u.person_id.clone());
        }
    }
    
    let persons_from_users: HashMap<String, person::Model> = if !missing_person_ids.is_empty() {
        person::Entity::find()
            .filter(person::Column::Id.is_in(missing_person_ids))
            .all(&ctx.db)
            .await?
            .into_iter()
            .map(|p| (p.id.clone(), p))
            .collect()
    } else {
        HashMap::new()
    };

    let mut enriched = Vec::with_capacity(results.len());
    for r in results {
        let mut val = serde_json::to_value(&r).unwrap_or_default();
        let person_opt = persons_by_id.get(&r.student_id).or_else(|| {
            users_by_id.get(&r.student_id).and_then(|u| persons_by_id.get(&u.person_id).or_else(|| persons_from_users.get(&u.person_id)))
        });
        let user_opt = users_by_id.get(&r.student_id);

        if let Some(obj) = val.as_object_mut() {
            let full_name = person_opt
                .and_then(|p| p.full_name.clone().or_else(|| Some(p.first_name.clone())))
                .or_else(|| user_opt.map(|u| u.full_name.clone()))
                .unwrap_or_default();
            obj.insert("student_name".into(), serde_json::Value::String(full_name.clone()));
            obj.insert("full_name".into(), serde_json::Value::String(full_name));
        }
        enriched.push(val);
    }
    Ok(enriched)
}

pub async fn get_result(ctx: &ServiceContext, result_id: &str) -> Result<term_result::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeRead).await?;
    term_result::Entity::find_by_id(result_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("term_result", result_id))
}

pub async fn calculate_term_result(
    ctx: &ServiceContext,
    req: &TermCalcRequest,
) -> Result<term_result::Model, AppError> {
    let policy = resolution::resolve_calc_policy(&ctx.db, &ctx.org_id, &req.term_period_id, "term")
        .await?
        .ok_or_else(|| {
            AppError::validation(
                "No term calculation policy configured. Create a CalculationPolicy with rule_type='term'.",
            )
        })?;

    let ast: AstNode = serde_json::from_str(&policy.formula_expression).map_err(|e| {
        AppError::validation(format!("Invalid term calculation formula: {}", e))
    })?;

    let grading_period_ids = if req.grading_period_ids.is_empty() {
        resolution::grading_child_ids(&ctx.db, &ctx.org_id, &req.term_period_id).await?
    } else {
        req.grading_period_ids.clone()
    };

    let exam_period_id = if req.exam_period_id.is_some() {
        req.exam_period_id.clone()
    } else {
        resolution::exam_child_id(&ctx.db, &ctx.org_id, &req.term_period_id).await?
    };

    let scheme_id = resolution::resolve_scheme_id(
        &ctx.db,
        &ctx.org_id,
        &req.class_subject_id,
        &req.term_period_id,
    )
    .await?;
    let components = if let Some(ref sid) = scheme_id {
        resolution::scheme_components(&ctx.db, sid).await?
    } else {
        Vec::new()
    };
    let (scheme_coursework_max, scheme_exam_max) = resolution::scheme_maxes(&components);
    let exam_types: Vec<String> = components
        .iter()
        .filter(|c| resolution::is_exam_component(&c.component_type))
        .map(|c| c.component_type.clone())
        .collect();

    let mut period_scores: Vec<f64> = Vec::new();
    for period_id in &grading_period_ids {
        let snap = period_score::Entity::find()
            .filter(period_score::Column::OrgId.eq(&ctx.org_id))
            .filter(period_score::Column::StudentId.eq(&req.student_id))
            .filter(period_score::Column::PeriodId.eq(period_id))
            .filter(period_score::Column::ClassSubjectId.eq(&req.class_subject_id))
            .order_by_desc(period_score::Column::UpdatedAt)
            .one(&ctx.db)
            .await?;
        if let Some(snap) = snap {
            period_scores.push(snap.total_score.unwrap_or(0.0));
        }
    }

    let coursework_avg = if period_scores.is_empty() {
        0.0
    } else {
        period_scores.iter().sum::<f64>() / period_scores.len() as f64
    };

    let exam_score = if let Some(s) = req.exam_score {
        s
    } else {
        load_exam_score(
            ctx,
            &req.enrollment_id,
            &req.class_subject_id,
            exam_period_id.as_deref(),
            &grading_period_ids,
            &exam_types,
        )
        .await?
    };

    let mut engine = CalcEngine::new();
    engine.set_variable("coursework_avg", coursework_avg);
    engine.set_variable("exam", exam_score);
    for (i, score) in period_scores.iter().enumerate() {
        engine.set_variable(&format!("period_{}", i + 1), *score);
    }
    engine.set_variable("period_count", period_scores.len() as f64);

    let raw_total = engine.evaluate(&ast)?;
    let total = apply_rounding(raw_total, &policy.rounding_policy);

    let coursework_max = if scheme_coursework_max > 0.0 {
        scheme_coursework_max
    } else {
        coursework_avg.max(0.0)
    };
    let exam_max = if scheme_exam_max > 0.0 {
        scheme_exam_max
    } else {
        req.exam_max.unwrap_or(exam_score.max(0.0))
    };
    let total_max = if coursework_max + exam_max > 0.0 {
        coursework_max + exam_max
    } else {
        total.max(1.0)
    };
    let coursework_score = (total - exam_score.min(exam_max)).max(0.0);
    let percentage = if total_max > 0.0 {
        apply_rounding((total / total_max) * 100.0, &policy.rounding_policy)
    } else {
        0.0
    };

    let mapper = resolution::resolve_mapper(&ctx.db, &ctx.org_id, &req.term_period_id).await?;
    let grade_result = mapper.map_score(percentage)?;

    let policy_snapshot = serde_json::json!({
        "policy_id": policy.id,
        "version": policy.version,
        "rule_type": policy.rule_type,
        "formula_expression": policy.formula_expression,
        "rounding_policy": policy.rounding_policy,
    });

    let now = Utc::now().to_rfc3339();
    let existing = term_result::Entity::find()
        .filter(term_result::Column::OrgId.eq(&ctx.org_id))
        .filter(term_result::Column::StudentId.eq(&req.student_id))
        .filter(term_result::Column::TermPeriodId.eq(&req.term_period_id))
        .filter(term_result::Column::ClassSubjectId.eq(&req.class_subject_id))
        .one(&ctx.db)
        .await?;

    if let Some(ex) = existing {
        if ex.status == "finalized" {
            return Err(AppError::validation("Term result is finalized and cannot be recalculated"));
        }
        let mut active: term_result::ActiveModel = ex.into();
        active.enrollment_id = Set(req.enrollment_id.clone());
        active.coursework_score = Set(coursework_score);
        active.coursework_max = Set(coursework_max);
        active.exam_score = Set(exam_score);
        active.exam_max = Set(exam_max);
        active.total_score = Set(total);
        active.total_max = Set(total_max);
        active.percentage = Set(percentage);
        active.grade_label = Set(Some(grade_result.label.clone()));
        active.gpa_points = Set(grade_result.gpa_points);
        active.is_passing = Set(if grade_result.is_passing { 1 } else { 0 });
        active.calculation_policy_id = Set(Some(policy.id.clone()));
        active.calculation_version = Set(policy.version);
        active.policy_snapshot = Set(Some(policy_snapshot.to_string()));
        active.status = Set("official".to_string());
        active.updated_at = Set(now);
        let updated = active.update(&ctx.db).await?;
        info!(student_id=%req.student_id, term_period_id=%req.term_period_id, total=%total, "Updated term result");
        return Ok(updated);
    }

    let inserted = term_result::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        org_id: Set(ctx.org_id.clone()),
        student_id: Set(req.student_id.clone()),
        term_period_id: Set(req.term_period_id.clone()),
        enrollment_id: Set(req.enrollment_id.clone()),
        class_subject_id: Set(req.class_subject_id.clone()),
        coursework_score: Set(coursework_score),
        coursework_max: Set(coursework_max),
        exam_score: Set(exam_score),
        exam_max: Set(exam_max),
        total_score: Set(total),
        total_max: Set(total_max),
        percentage: Set(percentage),
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
    }
    .insert(&ctx.db)
    .await?;

    info!(student_id=%req.student_id, term_period_id=%req.term_period_id, total=%total, "Calculated term result");
    Ok(inserted)
}

pub async fn calculate_term_results_batch(
    ctx: &ServiceContext,
    term_period_id: &str,
    class_id: &str,
    grading_period_ids: Vec<String>,
    exam_period_id: Option<String>,
) -> Result<Vec<term_result::Model>, AppError> {
    let enrollments = svc_academic::entities::enrollment::Entity::find()
        .filter(svc_academic::entities::enrollment::Column::ClassId.eq(class_id))
        .filter(svc_academic::entities::enrollment::Column::Status.eq("active"))
        .all(&ctx.db)
        .await?;

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
    Ok(active.update(&ctx.db).await?)
}

pub async fn build_grade_mapper(ctx: &ServiceContext) -> Result<GradeMapper, AppError> {
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
    let _ = calculation_policy::Entity::find()
        .filter(calculation_policy::Column::OrgId.eq(&ctx.org_id))
        .one(&ctx.db)
        .await;
    Ok(fallback_grade_mapper())
}

async fn load_exam_score(
    ctx: &ServiceContext,
    enrollment_id: &str,
    class_subject_id: &str,
    exam_period_id: Option<&str>,
    grading_period_ids: &[String],
    exam_types: &[String],
) -> Result<f64, AppError> {
    let mut period_ids: Vec<String> = Vec::new();
    if let Some(pid) = exam_period_id {
        period_ids.push(pid.to_string());
    } else {
        period_ids.extend(grading_period_ids.iter().cloned());
    }
    if period_ids.is_empty() {
        return Ok(0.0);
    }

    let grades = grade::Entity::find()
        .filter(grade::Column::OrgId.eq(&ctx.org_id))
        .filter(grade::Column::EnrollmentId.eq(enrollment_id))
        .filter(grade::Column::ClassSubjectId.eq(class_subject_id))
        .filter(grade::Column::PeriodId.is_in(period_ids))
        .all(&ctx.db)
        .await?;

    Ok(grades
        .iter()
        .filter(|g| {
            if !exam_types.is_empty() {
                exam_types.iter().any(|t| t == &g.exam_type)
            } else {
                resolution::is_exam_component(&g.exam_type)
            }
        })
        .map(|g| g.score)
        .sum())
}
