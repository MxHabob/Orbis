//! Class operations — CRUD + teacher assignment.
//!
//! All business logic for classes lives here. No other layer should
//! build class models or validate class data directly.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::events::event_types;
use crate::entities::class;

// ── Queries ─────────────────────────────────────────────────────

/// List all classes for an org.
pub async fn list(
    ctx: &ServiceContext,
) -> Result<Vec<class::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ClassRead).await?;

    let mut query = class::Entity::find()
        .filter(class::Column::OrgId.eq(&ctx.org_id))
        .order_by_asc(class::Column::Level)
        .order_by_asc(class::Column::Name);

    if let Some(org_unit_id) = &ctx.org_unit_id {
        query = query.filter(class::Column::UnitId.eq(org_unit_id));
    }

    Ok(query.all(&ctx.db).await?)
}

/// Get a single class by ID.
pub async fn get(ctx: &ServiceContext, id: &str) -> Result<class::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ClassRead).await?;

    class::Entity::find()
        .filter(class::Column::Id.eq(id.to_string()))
        .filter(class::Column::OrgId.eq(&ctx.org_id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("class", id))
}

// ── Commands ────────────────────────────────────────────────────

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct CreateClassPayload {
    pub org_unit_id: String,
    pub period_id: String,
    pub name: String,
    #[serde(default)]
    pub name_ar: Option<String>,
    pub grade_level: String,
    pub capacity: i32,
    #[serde(default)]
    pub division: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
}

/// Create a new class.
pub async fn create(
    ctx: &ServiceContext,
    req: CreateClassPayload,
) -> Result<class::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ClassWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = class::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        unit_id: req.org_unit_id,
        period_id: req.period_id,
        name: req.name.clone(),
        name_ar: req.name_ar,
        level: req.grade_level,
        division: req.division,
        homeroom_teacher_id: None,
        capacity: Some(req.capacity),
        current_students: Some(0),
        room_id: None,
        color: req.color,
        is_active: 1,
        is_deleted: 0,
        deleted_at: None,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: class::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    // Audit
    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "class.create".into(),
        entity_type: "class".into(),
        entity_id: id.clone(),
        old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    // Domain event
    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::CLASS_CREATED.into(),
        entity_type: "class".into(),
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

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct UpdateClassPayload {
    pub name: String,
    pub name_ar: Option<String>,
    pub level: String,
    pub division: Option<String>,
    pub capacity: Option<i32>,
}

/// Update an existing class.
pub async fn update(
    ctx: &ServiceContext,
    class_id: &str,
    req: UpdateClassPayload,
) -> Result<class::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ClassWrite).await?;

    let existing = get(ctx, class_id).await?;
    let old_data = serde_json::to_value(&existing).ok();

    let mut active: class::ActiveModel = existing.into();
    active.name = Set(req.name);
    active.name_ar = Set(req.name_ar);
    active.level = Set(req.level);
    active.division = Set(req.division);
    if let Some(cap) = req.capacity {
        active.capacity = Set(Some(cap));
    }
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "class.update".into(),
        entity_type: "class".into(),
        entity_id: class_id.into(),
        old_data,
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::CLASS_UPDATED.into(),
        entity_type: "class".into(),
        entity_id: class_id.into(),
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&updated).unwrap_or_default(),
        chain_depth: 0,
    });

    Ok(updated)
}

/// Soft-delete a class.
pub async fn delete(ctx: &ServiceContext, class_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ClassDelete).await?;

    let existing = get(ctx, class_id).await?;
    let old_data = serde_json::to_value(&existing).ok();

    let mut active: class::ActiveModel = existing.into();
    active.is_deleted = Set(1);
    active.deleted_at = Set(Some(Utc::now().to_rfc3339()));
    active.updated_at = Set(Utc::now().to_rfc3339());
    active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "class.delete".into(),
        entity_type: "class".into(),
        entity_id: class_id.into(),
        old_data,
        new_data: None,
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::CLASS_DELETED.into(),
        entity_type: "class".into(),
        entity_id: class_id.into(),
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::json!({}),
        chain_depth: 0,
    });

    Ok(())
}

/// Assign a homeroom teacher to a class.
pub async fn assign_teacher(
    ctx: &ServiceContext,
    class_id: &str,
    teacher_id: &str,
) -> Result<class::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ClassWrite).await?;

    let existing = get(ctx, class_id).await?;
    let mut active: class::ActiveModel = existing.into();
    active.homeroom_teacher_id = Set(Some(teacher_id.to_string()));
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "class.assign_teacher".into(),
        entity_type: "class".into(),
        entity_id: class_id.into(),
        old_data: None,
        new_data: Some(serde_json::json!({ "teacher_id": teacher_id })),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(updated)
}

pub async fn list_students(
    ctx: &ServiceContext,
    class_id: &str,
) -> Result<Vec<crate::entities::enrollment::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ClassRead).await?;

    Ok(crate::entities::enrollment::Entity::find()
        .filter(crate::entities::enrollment::Column::OrgId.eq(&ctx.org_id))
        .filter(crate::entities::enrollment::Column::ClassId.eq(class_id))
        .filter(crate::entities::enrollment::Column::Status.ne("withdrawn"))
        .all(&ctx.db)
        .await?)
}

pub async fn list_enriched_students(
    ctx: &ServiceContext,
    class_id: &str,
) -> Result<Vec<crate::ops::enrollment::EnrichedEnrollmentResponse>, AppError> {
    let enrollments = list_students(ctx, class_id).await?;
    crate::ops::enrollment::enrich_enrollment_models(&ctx.db, &enrollments).await
}
