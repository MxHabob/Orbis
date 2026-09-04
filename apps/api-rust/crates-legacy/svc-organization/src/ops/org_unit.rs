//! Org unit operations — departments, branches, divisions.
//!
//! Moved from `svc-academic` to `svc-organization` because org units
//! are organizational entities, not academic ones.
//!
//! Adds hierarchy traversal functions using the Materialized Path pattern.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, PaginatorTrait};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{Actor, AuditEntry, Permission};

use crate::entities::org_unit;

// ── Queries ─────────────────────────────────────────────────────

/// List all org units for the organization.
pub async fn list(ctx: &ServiceContext) -> Result<Vec<org_unit::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrgUnitRead).await?;

    visible_units(ctx).await
}

async fn visible_units(ctx: &ServiceContext) -> Result<Vec<org_unit::Model>, AppError> {
    let units = org_unit::Entity::find()
        .filter(org_unit::Column::OrgId.eq(&ctx.org_id))
        .order_by_asc(org_unit::Column::SortOrder)
        .all(&ctx.db)
        .await?;

    let Actor::User(user_id) = &ctx.actor else {
        return Ok(units);
    };

    let assignments = svc_iam::entities::user_role::Entity::find()
        .filter(svc_iam::entities::user_role::Column::UserId.eq(user_id))
        .filter(svc_iam::entities::user_role::Column::OrgId.eq(&ctx.org_id))
        .all(&ctx.db)
        .await?;

    if assignments.iter().any(|assignment| {
        matches!(
            assignment.scope_type.to_ascii_lowercase().as_str(),
            "global" | "organization" | "org"
        )
    }) {
        return Ok(units);
    }

    let allowed_scopes: Vec<(String, String)> = assignments
        .into_iter()
        .filter_map(|assignment| {
            assignment
                .scope_id
                .map(|scope_id| (assignment.scope_type.to_ascii_lowercase(), scope_id))
        })
        .collect();

    Ok(units
        .into_iter()
        .filter(|unit| {
            allowed_scopes.iter().any(|(scope_type, scope_id)| match scope_type.as_str() {
                "branch" | "org_unit" | "unit" => unit.id == *scope_id,
                "subtree" => {
                    unit.path == *scope_id
                        || unit.path.starts_with(&format!("{}/", scope_id.trim_end_matches('/')))
                }
                _ => false,
            })
        })
        .collect())
}

/// Get all descendants of a node (all levels) — single query via LIKE on path.
pub async fn get_descendants(
    ctx: &ServiceContext,
    node_path: &str,
) -> Result<Vec<org_unit::Model>, AppError> {
    let units = visible_units(ctx).await?;
    Ok(units
        .into_iter()
        .filter(|unit| unit.path.starts_with(&format!("{}/", node_path)))
        .collect())
}

/// Get all ancestors of a node by walking up the path.
pub async fn get_ancestors(
    ctx: &ServiceContext,
    node_path: &str,
) -> Result<Vec<org_unit::Model>, AppError> {
    // Build list of ancestor paths from the node's path
    let parts: Vec<&str> = node_path.split('/').filter(|p| !p.is_empty()).collect();
    let mut ancestor_paths = Vec::new();
    let mut current = String::from("/");
    ancestor_paths.push(current.clone());
    for part in &parts {
        current = format!("{}{}/", current, part);
        ancestor_paths.push(current.trim_end_matches('/').to_string());
    }

    if ancestor_paths.is_empty() {
        return Ok(Vec::new());
    }

    let units = visible_units(ctx).await?;
    Ok(units
        .into_iter()
        .filter(|unit| unit.is_active == 1 && ancestor_paths.contains(&unit.path))
        .collect())
}

/// Get direct children of a node.
pub async fn get_children(
    ctx: &ServiceContext,
    parent_id: &str,
) -> Result<Vec<org_unit::Model>, AppError> {
    let units = visible_units(ctx).await?;
    Ok(units
        .into_iter()
        .filter(|unit| unit.is_active == 1 && unit.parent_id.as_deref() == Some(parent_id))
        .collect())
}

// ── Commands ────────────────────────────────────────────────────

#[derive(serde::Deserialize)]
pub struct CreateOrgUnitPayload {
    pub name: String,
    pub name_ar: Option<String>,
    pub parent_id: Option<String>,
    pub unit_type: Option<String>,
    pub code: Option<String>,
    pub head_id: Option<String>,
    pub capacity: Option<i32>,
    pub room_number: Option<String>,
    pub floor: Option<i32>,
    pub building: Option<String>,
    pub data_isolation: Option<String>,
    pub sort_order: Option<i32>,
}

/// Create a new org unit.
pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<org_unit::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::OrgUnitWrite).await?;

    let req: CreateOrgUnitPayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid create org unit payload: {e}")))?;

    let settings = ctx.settings.get_org_settings(&ctx.org_id).await?;
    let unit_type = req.unit_type.clone().unwrap_or_else(|| "department".into());
    if unit_type == "branch" {
        if !settings.branch_policy.enabled {
            return Err(AppError::policy_blocked("Branch management is disabled by organization policy"));
        }
        if settings.branch_policy.creation_requires_approval {
            return Err(AppError::policy_blocked("Branch creation requires an approval workflow"));
        }
        if let Some(max_branches) = settings.branch_policy.max_branches {
            let branch_count = org_unit::Entity::find()
                .filter(org_unit::Column::OrgId.eq(&ctx.org_id))
                .filter(org_unit::Column::UnitType.eq("branch"))
                .filter(org_unit::Column::IsActive.eq(1))
                .count(&ctx.db)
                .await? as u32;
            if branch_count >= max_branches {
                return Err(AppError::policy_blocked("The organization branch limit has been reached"));
            }
        }
    }

    // Calculate depth and path based on parent
    let (depth, path) = if let Some(ref parent_id) = req.parent_id {
        if parent_id.is_empty() {
            (0, "/".into())
        } else {
            let parent = org_unit::Entity::find()
                .filter(org_unit::Column::Id.eq(parent_id.clone()))
                .filter(org_unit::Column::OrgId.eq(&ctx.org_id))
                .one(&ctx.db)
                .await?
                .ok_or_else(|| AppError::not_found("org_unit", parent_id))?;
            (parent.depth + 1, format!("{}/{}", parent.path, parent_id))
        }
    } else {
        (0, "/".into())
    };

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let parent_id = if req.parent_id.as_deref() == Some("") {
        None
    } else {
        req.parent_id
    };

    let model = org_unit::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        parent_id,
        path,
        depth,
        name: req.name,
        name_ar: req.name_ar,
        code: req.code,
        unit_type,
        head_id: req.head_id,
        capacity: req.capacity,
        room_number: req.room_number,
        floor: req.floor,
        building: req.building,
        settings: None,
        local_policies: None,
        data_isolation: req.data_isolation.unwrap_or(settings.branch_policy.data_isolation),
        children_count: 0,
        descendant_count: 0,
        sort_order: req.sort_order,
        is_active: 1,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: org_unit::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    // Update parent's children_count
    if let Some(ref pid) = created.parent_id {
        let _ = update_children_count(ctx, pid).await;
    }

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
        event_type: "org_unit.created".into(),
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
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub parent_id: Option<String>,
    pub unit_type: Option<String>,
    pub code: Option<String>,
    pub head_id: Option<String>,
    pub capacity: Option<i32>,
    pub room_number: Option<String>,
    pub floor: Option<i32>,
    pub building: Option<String>,
    pub data_isolation: Option<String>,
    pub sort_order: Option<i32>,
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

    let existing = org_unit::Entity::find()
        .filter(org_unit::Column::Id.eq(unit_id.to_string()))
        .filter(org_unit::Column::OrgId.eq(&ctx.org_id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("org_unit", unit_id))?;

    let old_data = serde_json::to_value(&existing).ok();
    let mut active: org_unit::ActiveModel = existing.into();

    if let Some(name) = req.name {
        active.name = Set(name);
    }
    if req.name_ar.is_some() {
        active.name_ar = Set(req.name_ar);
    }
    if req.code.is_some() {
        active.code = Set(req.code);
    }
    if let Some(ut) = req.unit_type {
        active.unit_type = Set(ut);
    }
    if let Some(ref pid) = req.parent_id {
        if pid.is_empty() {
            active.parent_id = Set(None);
            active.depth = Set(0);
            active.path = Set("/".into());
        } else {
            let parent = org_unit::Entity::find()
                .filter(org_unit::Column::Id.eq(pid.clone()) )
                .filter(org_unit::Column::OrgId.eq(&ctx.org_id))
                .one(&ctx.db)
                .await?
                .ok_or_else(|| AppError::not_found("org_unit", pid))?;
            active.parent_id = Set(Some(pid.clone()));
            active.depth = Set(parent.depth + 1);
            active.path = Set(format!("{}/{}", parent.path, pid));
        }
    }
    if req.head_id.is_some() {
        active.head_id = Set(req.head_id);
    }
    if req.capacity.is_some() {
        active.capacity = Set(req.capacity);
    }
    if req.room_number.is_some() {
        active.room_number = Set(req.room_number);
    }
    if req.floor.is_some() {
        active.floor = Set(req.floor);
    }
    if req.building.is_some() {
        active.building = Set(req.building);
    }
    if let Some(di) = req.data_isolation {
        active.data_isolation = Set(di);
    }
    if req.sort_order.is_some() {
        active.sort_order = Set(req.sort_order);
    }

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

    let existing = org_unit::Entity::find()
        .filter(org_unit::Column::Id.eq(unit_id.to_string()))
        .filter(org_unit::Column::OrgId.eq(&ctx.org_id))
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

// ── Internal Helpers ────────────────────────────────────────────

/// Update the children_count for a parent node.
async fn update_children_count(ctx: &ServiceContext, parent_id: &str) -> Result<(), AppError> {
    

    let count = org_unit::Entity::find()
        .filter(org_unit::Column::ParentId.eq(parent_id))
        .filter(org_unit::Column::IsActive.eq(1))
        .count(&ctx.db)
        .await? as i32;

    let parent = org_unit::Entity::find()
        .filter(org_unit::Column::Id.eq(parent_id.to_string()))
        .filter(org_unit::Column::OrgId.eq(&ctx.org_id))
        .one(&ctx.db)
        .await?;

    if let Some(parent) = parent {
        let mut active: org_unit::ActiveModel = parent.into();
        active.children_count = Set(count);
        active.update(&ctx.db).await?;
    }

    Ok(())
}
