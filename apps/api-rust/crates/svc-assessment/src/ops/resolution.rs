//! Resolve assessment configuration: scheme, scale, calculation policy, period tree.

use sea_orm::{ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryFilter, Statement};
use shared_kernel::errors::AppError;

use crate::entities::{
    assessment_scheme, assessment_scheme_component, calculation_policy, grade_scale, grade_scale_item,
};
use crate::ops::grade_mapper::{GradeMapper, GradeScaleItemData};
use svc_academic::entities::class_subject;
use svc_calendar::entities::academic_period;

pub fn is_exam_component(component_type: &str) -> bool {
    matches!(
        component_type,
        "exam" | "written_exam" | "final" | "term_exam" | "midterm"
    )
}

pub fn is_grading_child_type(period_type: &str) -> bool {
    matches!(
        period_type,
        "month" | "monthly" | "quarter" | "module" | "block" | "custom"
    )
}

pub fn is_exam_period(period_type: &str, name: &str) -> bool {
    period_type == "exam"
        || name.contains("اختبار")
        || name.to_lowercase().contains("exam")
}

pub async fn child_periods(
    db: &DatabaseConnection,
    org_id: &str,
    parent_id: &str,
) -> Result<Vec<academic_period::Model>, AppError> {
    Ok(academic_period::Entity::find()
        .filter(academic_period::Column::OrgId.eq(org_id))
        .filter(academic_period::Column::ParentId.eq(parent_id))
        .all(db)
        .await?)
}

pub async fn grading_child_ids(
    db: &DatabaseConnection,
    org_id: &str,
    parent_id: &str,
) -> Result<Vec<String>, AppError> {
    let children = child_periods(db, org_id, parent_id).await?;
    Ok(children
        .into_iter()
        .filter(|p| is_grading_child_type(&p.period_type) && !is_exam_period(&p.period_type, &p.name))
        .map(|p| p.id)
        .collect())
}

pub async fn exam_child_id(
    db: &DatabaseConnection,
    org_id: &str,
    parent_id: &str,
) -> Result<Option<String>, AppError> {
    let children = child_periods(db, org_id, parent_id).await?;
    Ok(children
        .into_iter()
        .find(|p| is_exam_period(&p.period_type, &p.name))
        .map(|p| p.id))
}

pub async fn term_child_ids(
    db: &DatabaseConnection,
    org_id: &str,
    year_period_id: &str,
) -> Result<Vec<String>, AppError> {
    let children = child_periods(db, org_id, year_period_id).await?;
    Ok(children
        .into_iter()
        .filter(|p| matches!(p.period_type.as_str(), "term" | "semester" | "trimester"))
        .map(|p| p.id)
        .collect())
}

pub async fn resolve_scheme_id(
    db: &DatabaseConnection,
    org_id: &str,
    class_subject_id: &str,
    period_id: &str,
) -> Result<Option<String>, AppError> {
    let cs = class_subject::Entity::find_by_id(class_subject_id.to_string())
        .one(db)
        .await?;
    if let Some(cs) = cs {
        if let Some(sid) = cs.scheme_id {
            return Ok(Some(sid));
        }
    }

    let period_scheme = assessment_scheme::Entity::find()
        .filter(assessment_scheme::Column::OrgId.eq(org_id))
        .filter(assessment_scheme::Column::PeriodId.eq(period_id))
        .filter(assessment_scheme::Column::IsArchived.eq(0))
        .one(db)
        .await?;
    if let Some(s) = period_scheme {
        return Ok(Some(s.id));
    }

    Ok(assessment_scheme::Entity::find()
        .filter(assessment_scheme::Column::OrgId.eq(org_id))
        .filter(assessment_scheme::Column::PeriodId.is_null())
        .filter(assessment_scheme::Column::IsArchived.eq(0))
        .one(db)
        .await?
        .map(|s| s.id))
}

pub async fn scheme_components(
    db: &DatabaseConnection,
    scheme_id: &str,
) -> Result<Vec<assessment_scheme_component::Model>, AppError> {
    Ok(assessment_scheme_component::Entity::find()
        .filter(assessment_scheme_component::Column::SchemeId.eq(scheme_id))
        .all(db)
        .await?)
}

pub fn scheme_maxes(components: &[assessment_scheme_component::Model]) -> (f64, f64) {
    let mut coursework = 0.0;
    let mut exam = 0.0;
    for c in components {
        if is_exam_component(&c.component_type) {
            exam += c.max_score;
        } else {
            coursework += c.max_score;
        }
    }
    (coursework, exam)
}

pub fn weighted_sum_ast_from_components(
    components: &[assessment_scheme_component::Model],
) -> serde_json::Value {
    let weight_total: f64 = components.iter().map(|c| c.weight).sum();
    let denom = if weight_total > 0.0 { weight_total } else { 1.0 };
    let items: Vec<serde_json::Value> = components
        .iter()
        .map(|c| {
            serde_json::json!({
                "node": { "type": "variable", "name": c.component_type },
                "weight": c.weight / denom
            })
        })
        .collect();
    serde_json::json!({ "type": "weighted_sum", "items": items })
}

pub async fn resolve_calc_policy(
    db: &DatabaseConnection,
    org_id: &str,
    period_id: &str,
    rule_type: &str,
) -> Result<Option<calculation_policy::Model>, AppError> {
    let period_policy = calculation_policy::Entity::find()
        .filter(calculation_policy::Column::OrgId.eq(org_id))
        .filter(calculation_policy::Column::TargetPeriodId.eq(period_id))
        .filter(calculation_policy::Column::RuleType.eq(rule_type))
        .filter(calculation_policy::Column::IsArchived.eq(0))
        .one(db)
        .await?;
    if period_policy.is_some() {
        return Ok(period_policy);
    }

    if rule_type == "period" || rule_type == "weighted_sum" {
        let any_period = calculation_policy::Entity::find()
            .filter(calculation_policy::Column::OrgId.eq(org_id))
            .filter(calculation_policy::Column::TargetPeriodId.eq(period_id))
            .filter(calculation_policy::Column::IsArchived.eq(0))
            .one(db)
            .await?;
        if any_period.is_some() {
            return Ok(any_period);
        }
        let org_weighted = calculation_policy::Entity::find()
            .filter(calculation_policy::Column::OrgId.eq(org_id))
            .filter(calculation_policy::Column::RuleType.eq("weighted_sum"))
            .filter(calculation_policy::Column::IsArchived.eq(0))
            .filter(calculation_policy::Column::TargetPeriodId.is_null())
            .one(db)
            .await?;
        if org_weighted.is_some() {
            return Ok(org_weighted);
        }
    }

    Ok(calculation_policy::Entity::find()
        .filter(calculation_policy::Column::OrgId.eq(org_id))
        .filter(calculation_policy::Column::RuleType.eq(rule_type))
        .filter(calculation_policy::Column::IsArchived.eq(0))
        .filter(calculation_policy::Column::TargetPeriodId.is_null())
        .one(db)
        .await?)
}

pub async fn resolve_scale_id(
    db: &DatabaseConnection,
    org_id: &str,
    period_id: &str,
) -> Result<Option<String>, AppError> {
    let period = academic_period::Entity::find_by_id(period_id.to_string())
        .one(db)
        .await?;
    if let Some(p) = period {
        if let Some(sid) = p.scale_id {
            return Ok(Some(sid));
        }
    }

    let row = db
        .query_one(Statement::from_sql_and_values(
            db.get_database_backend(),
            "SELECT default_scale_id FROM academic_policies WHERE org_id = $1 AND org_unit_id IS NULL LIMIT 1",
            [org_id.into()],
        ))
        .await?;
    if let Some(row) = row {
        if let Ok(Some(id)) = row.try_get::<Option<String>>("", "default_scale_id") {
            if !id.is_empty() {
                return Ok(Some(id));
            }
        }
    }

    Ok(grade_scale::Entity::find()
        .filter(grade_scale::Column::OrgId.eq(org_id))
        .one(db)
        .await?
        .map(|s| s.id))
}

pub async fn mapper_for_scale(db: &DatabaseConnection, scale_id: &str) -> Result<GradeMapper, AppError> {
    let items = grade_scale_item::Entity::find()
        .filter(grade_scale_item::Column::ScaleId.eq(scale_id))
        .all(db)
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
    if mapper_items.is_empty() {
        return Err(AppError::validation("Grade scale has no items"));
    }
    Ok(GradeMapper::new(mapper_items))
}

pub async fn resolve_mapper(
    db: &DatabaseConnection,
    org_id: &str,
    period_id: &str,
) -> Result<GradeMapper, AppError> {
    if let Some(scale_id) = resolve_scale_id(db, org_id, period_id).await? {
        if let Ok(mapper) = mapper_for_scale(db, &scale_id).await {
            return Ok(mapper);
        }
    }
    Ok(crate::ops::term_engine::fallback_grade_mapper())
}
