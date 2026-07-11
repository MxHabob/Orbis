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

/// List all classes for an org, optionally filtered by cycle.
pub async fn list(
    ctx: &ServiceContext,
    cycle_id: Option<String>,
) -> Result<Vec<class::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ClassRead).await?;

    let mut query = class::Entity::find()
        .filter(class::Column::OrgId.eq(&ctx.org_id))
        .filter(class::Column::IsDeleted.eq(0));

    if let Some(cid) = cycle_id {
        query = query.filter(class::Column::CycleId.eq(cid));
    }

    Ok(query
        .order_by_asc(class::Column::Level)
        .order_by_asc(class::Column::Name)
        .all(&ctx.db)
        .await?)
}

/// Get a single class by ID.
pub async fn get(ctx: &ServiceContext, id: &str) -> Result<class::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ClassRead).await?;

    class::Entity::find_by_id(id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("class", id))
}

// ── Commands ────────────────────────────────────────────────────

/// Create a new class.
pub async fn create(
    ctx: &ServiceContext,
    cmd: crate::contracts::commands::CreateClassCmd,
) -> Result<class::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ClassWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = class::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        unit_id: cmd.org_unit_id,
        cycle_id: cmd.cycle_id,
        name: cmd.name.clone(),
        name_ar: cmd.name_ar,
        level: cmd.grade_level,
        division: None,
        homeroom_teacher_id: None,
        capacity: Some(cmd.capacity),
        current_students: Some(0),
        room_id: None,
        color: None,
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

/// Update an existing class.
pub async fn update(
    ctx: &ServiceContext,
    class_id: &str,
    name: String,
    name_ar: Option<String>,
    level: String,
    division: Option<String>,
    capacity: Option<i32>,
) -> Result<class::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ClassWrite).await?;

    let existing = get(ctx, class_id).await?;
    let old_data = serde_json::to_value(&existing).ok();

    let mut active: class::ActiveModel = existing.into();
    active.name = Set(name);
    active.name_ar = Set(name_ar);
    active.level = Set(level);
    active.division = Set(division);
    if let Some(cap) = capacity {
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
