//! Subject operations — curriculum subject CRUD.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::events::event_types;
use crate::entities::subject;

// ── Queries ─────────────────────────────────────────────────────

/// List all subjects for an org.
pub async fn list(ctx: &ServiceContext) -> Result<Vec<subject::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::SubjectRead).await?;

    Ok(subject::Entity::find()
        .filter(subject::Column::OrgId.eq(&ctx.org_id))
        .filter(subject::Column::IsActive.eq(1))
        .order_by_asc(subject::Column::SortOrder)
        .all(&ctx.db)
        .await?)
}

/// Get a single subject by ID.
pub async fn get(ctx: &ServiceContext, id: &str) -> Result<subject::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::SubjectRead).await?;

    subject::Entity::find_by_id(id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("subject", id))
}

// ── Commands ────────────────────────────────────────────────────

#[derive(serde::Deserialize)]
pub struct CreateSubjectPayload {
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
}

/// Create a new subject.
pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<subject::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::SubjectWrite).await?;

    let req: CreateSubjectPayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid create subject payload: {e}")))?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = subject::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        unit_id: None,
        code: req.code,
        name: req.name,
        name_ar: req.name_ar,
        name_en: None,
        description: None,
        short_description: None,
        is_core: 1,
        is_elective: 0,
        credit_hours: None,
        requires_lab: 0,
        requires_practical: 0,
        max_students_per_class: None,
        default_grade_groups: None,
        prerequisites: None,
        learning_outcomes: None,
        color: None,
        icon: None,
        cover_image: None,
        sort_order: None,
        is_active: 1,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: subject::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "subject.create".into(),
        entity_type: "subject".into(),
        entity_id: id.clone(),
        old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::SUBJECT_CREATED.into(),
        entity_type: "subject".into(),
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
pub struct UpdateSubjectPayload {
    pub name: String,
    pub name_ar: Option<String>,
    pub code: String,
    pub category: Option<String>,
}

/// Update a subject.
pub async fn update(
    ctx: &ServiceContext,
    subject_id: &str,
    data: serde_json::Value,
) -> Result<subject::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::SubjectWrite).await?;

    let req: UpdateSubjectPayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid update subject payload: {e}")))?;

    let existing = get(ctx, subject_id).await?;
    let old_data = serde_json::to_value(&existing).ok();

    let mut active: subject::ActiveModel = existing.into();
    active.name = Set(req.name);
    active.name_ar = Set(req.name_ar);
    active.code = Set(req.code);
    if let Some(cat) = req.category {
        active.description = Set(Some(cat));
    }
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "subject.update".into(),
        entity_type: "subject".into(),
        entity_id: subject_id.into(),
        old_data,
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(updated)
}

/// Soft-delete a subject.
pub async fn delete(ctx: &ServiceContext, subject_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::SubjectDelete).await?;

    let existing = get(ctx, subject_id).await?;

    let mut active: subject::ActiveModel = existing.into();
    active.is_active = Set(0);
    active.updated_at = Set(Utc::now().to_rfc3339());
    active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "subject.delete".into(),
        entity_type: "subject".into(),
        entity_id: subject_id.into(),
        old_data: None,
        new_data: None,
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(())
}
