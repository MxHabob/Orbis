//! Guardian operations — student-parent/guardian relationships.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Condition};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::student_guardian;

// ── Queries ─────────────────────────────────────────────────────

/// List guardians linked to a student.
pub async fn list_by_student(
    ctx: &ServiceContext,
    student_id: &str,
) -> Result<Vec<student_guardian::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GuardianRead).await?;

    Ok(student_guardian::Entity::find()
        .filter(student_guardian::Column::StudentId.eq(student_id))
        .order_by_asc(student_guardian::Column::CreatedAt)
        .all(&ctx.db)
        .await?)
}

/// List students linked to a guardian.
pub async fn list_by_guardian(
    ctx: &ServiceContext,
    guardian_id: &str,
) -> Result<Vec<student_guardian::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GuardianRead).await?;

    Ok(student_guardian::Entity::find()
        .filter(student_guardian::Column::GuardianId.eq(guardian_id))
        .all(&ctx.db)
        .await?)
}

/// Search guardians by name or phone.
pub async fn search(
    ctx: &ServiceContext,
    query: &str,
) -> Result<Vec<student_guardian::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GuardianRead).await?;

    let pattern = format!("%{}%", query);
    Ok(student_guardian::Entity::find()
        .filter(student_guardian::Column::OrgId.eq(&ctx.org_id))
        .filter(
            Condition::any()
                .add(student_guardian::Column::GuardianName.contains(&pattern))
                .add(student_guardian::Column::Phone.contains(&pattern))
        )
        .all(&ctx.db)
        .await?)
}

// ── Commands ────────────────────────────────────────────────────

/// Create a guardian link.
pub async fn create(
    ctx: &ServiceContext,
    student_id: String,
    guardian_id: String,
    guardian_name: String,
    relationship: String,
    phone: Option<String>,
    is_primary: bool,
) -> Result<student_guardian::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GuardianWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = student_guardian::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        student_id,
        guardian_id,
        guardian_name,
        relationship,
        phone,
        is_primary: if is_primary { 1 } else { 0 },
        created_at: now.clone(),
        updated_at: now,
    };

    let active: student_guardian::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "guardian.create".into(),
        entity_type: "student_guardian".into(),
        entity_id: id,
        old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

/// Update a guardian link.
pub async fn update(
    ctx: &ServiceContext,
    link_id: &str,
    guardian_name: Option<String>,
    relationship: Option<String>,
    phone: Option<String>,
    is_primary: Option<bool>,
) -> Result<student_guardian::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GuardianWrite).await?;

    let existing = student_guardian::Entity::find_by_id(link_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("student_guardian", link_id))?;

    let mut active: student_guardian::ActiveModel = existing.into();
    if let Some(name) = guardian_name {
        active.guardian_name = Set(name);
    }
    if let Some(rel) = relationship {
        active.relationship = Set(rel);
    }
    if let Some(ph) = phone {
        active.phone = Set(Some(ph));
    }
    if let Some(primary) = is_primary {
        active.is_primary = Set(if primary { 1 } else { 0 });
    }
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "guardian.update".into(),
        entity_type: "student_guardian".into(),
        entity_id: link_id.into(),
        old_data: None,
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(updated)
}

/// Delete a guardian link.
pub async fn delete(ctx: &ServiceContext, link_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GuardianDelete).await?;

    let existing = student_guardian::Entity::find_by_id(link_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("student_guardian", link_id))?;

    let active: student_guardian::ActiveModel = existing.into();
    active.delete(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "guardian.delete".into(),
        entity_type: "student_guardian".into(),
        entity_id: link_id.into(),
        old_data: None,
        new_data: None,
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(())
}
