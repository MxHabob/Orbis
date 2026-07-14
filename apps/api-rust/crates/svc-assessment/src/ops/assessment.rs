//! Assessment operations — CRUD for assessment definitions (quizzes, exams, etc).

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::events::event_types;
use crate::entities::assessment;

// ── Queries ─────────────────────────────────────────────────────

/// List assessments for a period, optionally filtered by class_subject.
pub async fn list(
    ctx: &ServiceContext,
    period_id: &str,
    class_subject_id: Option<&str>,
) -> Result<Vec<assessment::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentRead).await?;

    let mut query = assessment::Entity::find()
        .filter(assessment::Column::OrgId.eq(&ctx.org_id))
        .filter(assessment::Column::PeriodId.eq(period_id));

    if let Some(csid) = class_subject_id {
        query = query.filter(assessment::Column::ClassSubjectId.eq(csid));
    }

    Ok(query
        .order_by_asc(assessment::Column::AssessmentDate)
        .all(&ctx.db)
        .await?)
}

// ── Commands ────────────────────────────────────────────────────

/// Create a new assessment definition.
#[derive(serde::Deserialize)]
pub struct CreateAssessmentRequest {
    pub period_id: String,
    pub class_subject_id: String,
    pub title: String,
    pub title_ar: Option<String>,
    pub item_type: String,
    pub max_score: f64,
    pub weight: f64,
    pub grade_group_name: String,
    pub assessment_date: Option<String>,
    pub due_date: Option<String>,
}

/// Create a new assessment definition.
pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<assessment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    let req: CreateAssessmentRequest = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid create assessment payload: {e}")))?;

    // ── Period Gate: check period is not locked ──
    {
        use shared_kernel::period_gate::GateType;
        let verdict = ctx.period_gate
            .check(&ctx.db, &ctx.org_id, &req.period_id, GateType::Assessment)
            .await?;
        verdict.enforce()?;
    }

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = assessment::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        period_id: req.period_id,
        class_subject_id: req.class_subject_id,
        title: req.title,
        title_ar: req.title_ar,
        description: None,
        item_type: req.item_type,
        max_score: req.max_score,
        passing_score: None,
        weight: req.weight,
        grade_group_name: req.grade_group_name,
        assessment_date: req.assessment_date,
        due_date: req.due_date,
        duration_minutes: None,
        is_published: 0,
        published_at: None,
        published_by: None,
        is_locked: 0,
        locked_at: None,
        locked_by: None,
        grading_method: None,
        rubric_id: None,
        instructions: None,
        instructions_ar: None,
        attachments: None,
        created_by: ctx.actor.as_str().to_string(),
        created_at: now.clone(),
        updated_at: now,
    };

    let active: assessment::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "assessment.create".into(),
        entity_type: "assessment".into(),
        entity_id: id.clone(),
        old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::ASSESSMENT_CREATED.into(),
        entity_type: "assessment".into(),
        entity_id: id,
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&created).unwrap_or_default(),
        chain_depth: 0,
    });

    Ok(created)
}

#[derive(serde::Deserialize)]
pub struct UpdateAssessmentRequest {
    pub title: Option<String>,
    pub title_ar: Option<String>,
    pub max_score: Option<f64>,
    pub weight: Option<f64>,
    pub assessment_date: Option<String>,
    pub due_date: Option<String>,
}

/// Update an assessment definition.
pub async fn update(
    ctx: &ServiceContext,
    assessment_id: &str,
    data: serde_json::Value,
) -> Result<assessment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    let req: UpdateAssessmentRequest = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid update assessment payload: {e}")))?;

    let existing = assessment::Entity::find_by_id(assessment_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("assessment", assessment_id))?;

    let mut active: assessment::ActiveModel = existing.into();
    if let Some(t) = req.title {
        active.title = Set(t);
    }
    if let Some(ta) = req.title_ar {
        active.title_ar = Set(Some(ta));
    }
    if let Some(ms) = req.max_score {
        active.max_score = Set(ms);
    }
    if let Some(w) = req.weight {
        active.weight = Set(w);
    }
    if let Some(d) = req.assessment_date {
        active.assessment_date = Set(Some(d));
    }
    if let Some(due) = req.due_date {
        active.due_date = Set(Some(due));
    }
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "assessment.update".into(),
        entity_type: "assessment".into(),
        entity_id: assessment_id.into(),
        old_data: None,
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(updated)
}

/// Delete an assessment definition.
pub async fn delete(ctx: &ServiceContext, assessment_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentDelete).await?;

    let existing = assessment::Entity::find_by_id(assessment_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("assessment", assessment_id))?;

    existing.delete(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "assessment.delete".into(),
        entity_type: "assessment".into(),
        entity_id: assessment_id.into(),
        old_data: None,
        new_data: None,
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(())
}
