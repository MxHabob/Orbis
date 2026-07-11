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

/// List assessments for a cycle, optionally filtered by class_subject.
pub async fn list(
    ctx: &ServiceContext,
    cycle_id: &str,
    class_subject_id: Option<String>,
) -> Result<Vec<assessment::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentRead).await?;

    let mut query = assessment::Entity::find()
        .filter(assessment::Column::OrgId.eq(&ctx.org_id))
        .filter(assessment::Column::CycleId.eq(cycle_id));

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
pub async fn create(
    ctx: &ServiceContext,
    cycle_id: String,
    class_subject_id: String,
    title: String,
    title_ar: Option<String>,
    item_type: String,
    max_score: f64,
    weight: f64,
    grade_group_name: String,
    assessment_date: Option<String>,
    due_date: Option<String>,
) -> Result<assessment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = assessment::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        cycle_id,
        class_subject_id,
        title,
        title_ar,
        description: None,
        item_type,
        max_score,
        passing_score: None,
        weight,
        grade_group_name,
        assessment_date,
        due_date,
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

/// Update an assessment definition.
pub async fn update(
    ctx: &ServiceContext,
    assessment_id: &str,
    title: Option<String>,
    max_score: Option<f64>,
    weight: Option<f64>,
    assessment_date: Option<String>,
) -> Result<assessment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    let existing = assessment::Entity::find_by_id(assessment_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("assessment", assessment_id))?;

    let mut active: assessment::ActiveModel = existing.into();
    if let Some(t) = title {
        active.title = Set(t);
    }
    if let Some(ms) = max_score {
        active.max_score = Set(ms);
    }
    if let Some(w) = weight {
        active.weight = Set(w);
    }
    if let Some(d) = assessment_date {
        active.assessment_date = Set(Some(d));
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
