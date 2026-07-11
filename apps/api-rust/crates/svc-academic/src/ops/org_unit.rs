//! Org unit operations — departments, branches, divisions.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::events::event_types;
use crate::entities::org_unit;

// ── Queries ─────────────────────────────────────────────────────

/// List all org units for the organization.
pub async fn list(ctx: &ServiceContext) -> Result<Vec<org_unit::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrgUnitRead).await?;

    Ok(org_unit::Entity::find()
        .filter(org_unit::Column::OrgId.eq(&ctx.org_id))
        .filter(org_unit::Column::IsActive.eq(1))
        .order_by_asc(org_unit::Column::SortOrder)
        .all(&ctx.db)
        .await?)
}

// ── Commands ────────────────────────────────────────────────────

#[derive(serde::Deserialize)]
pub struct CreateOrgUnitPayload {
    pub name: String,
    pub name_ar: Option<String>,
    pub parent_id: Option<String>,
}

/// Create a new org unit.
pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<org_unit::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrgUnitWrite).await?;

    let req: CreateOrgUnitPayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid create org unit payload: {e}")))?;

    // Calculate depth and path based on parent
    let (depth, path) = if let Some(ref parent_id) = req.parent_id {
        let parent = org_unit::Entity::find_by_id(parent_id.clone())
            .one(&ctx.db)
            .await?
            .ok_or_else(|| AppError::not_found("org_unit", parent_id))?;
        (parent.depth + 1, format!("{}/{}", parent.path, parent_id))
    } else {
        (0, "/".into())
    };

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = org_unit::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        name: req.name,
        name_ar: req.name_ar,
        code: None,
        unit_type: "department".into(),
        parent_id: req.parent_id,
        path,
        depth,
        head_id: None,
        capacity: None,
        room_number: None,
        floor: None,
        building: None,
        settings: None,
        sort_order: None,
        is_active: 1,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: org_unit::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "org_unit.create".into(),
        entity_type: "org_unit".into(),
        entity_id: id.clone(),
        old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::ORG_UNIT_CREATED.into(),
        entity_type: "org_unit".into(),
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
pub struct UpdateOrgUnitPayload {
    pub name: String,
    pub name_ar: Option<String>,
}

/// Update an org unit.
pub async fn update(
    ctx: &ServiceContext,
    unit_id: &str,
    data: serde_json::Value,
) -> Result<org_unit::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrgUnitWrite).await?;

    let req: UpdateOrgUnitPayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid update org unit payload: {e}")))?;

    let existing = org_unit::Entity::find_by_id(unit_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("org_unit", unit_id))?;

    let old_data = serde_json::to_value(&existing).ok();
    let mut active: org_unit::ActiveModel = existing.into();
    active.name = Set(req.name);
    active.name_ar = Set(req.name_ar);
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "org_unit.update".into(),
        entity_type: "org_unit".into(),
        entity_id: unit_id.into(),
        old_data,
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(updated)
}

/// Soft-delete an org unit.
pub async fn delete(ctx: &ServiceContext, unit_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrgUnitDelete).await?;

    let existing = org_unit::Entity::find_by_id(unit_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("org_unit", unit_id))?;

    let mut active: org_unit::ActiveModel = existing.into();
    active.is_active = Set(0);
    active.updated_at = Set(Utc::now().to_rfc3339());
    active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "org_unit.delete".into(),
        entity_type: "org_unit".into(),
        entity_id: unit_id.into(),
        old_data: None,
        new_data: None,
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(())
}
