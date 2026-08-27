//! Document operations — student/staff document CRUD.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::document;

// ── Queries ─────────────────────────────────────────────────────

/// List documents, filtered by owner.
pub async fn list(
    ctx: &ServiceContext,
    owner_id: &str,
) -> Result<Vec<document::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::DocumentRead).await?;

    Ok(document::Entity::find()
        .filter(document::Column::OrgId.eq(&ctx.org_id))
        .filter(document::Column::OwnerId.eq(owner_id))
        .order_by_desc(document::Column::CreatedAt)
        .all(&ctx.db)
        .await?)
}

/// Get a single document by ID.
pub async fn get(ctx: &ServiceContext, id: &str) -> Result<document::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::DocumentRead).await?;

    document::Entity::find_by_id(id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("document", id))
}

// ── Commands ────────────────────────────────────────────────────

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct CreateDocumentPayload {
    pub owner_id: String,
    pub name: String,
    pub item_type: String,
    pub key: String,
}

/// Create a new document record.
pub async fn create(
    ctx: &ServiceContext,
    req: CreateDocumentPayload,
) -> Result<document::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::DocumentWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = document::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        owner_id: req.owner_id,
        uploaded_by: Some(ctx.actor.as_str().to_string()),
        name: req.name,
        title: None,
        description: None,
        item_type: req.item_type,
        visibility: "private".into(),
        size: None,
        mime_type: None,
        extension: None,
        storage_provider: "local".into(),
        bucket: None,
        key: req.key,
        url: None,
        thumbnail_url: None,
        parent_id: None,
        path: None,
        depth: Some(0),
        shared_with: None,
        share_link: None,
        share_link_expires_at: None,
        share_password: None,
        version: Some(1),
        previous_version_id: None,
        is_latest_version: 1,
        processing_status: None,
        ocr_text: None,
        extracted_data: None,
        is_pinned: 0,
        is_starred: 0,
        is_template: 0,
        tags: None,
        meta: None,
        deleted_at: None,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: document::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "document.create".into(),
        entity_type: "document".into(),
        entity_id: id,
        old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

/// Delete a document.
pub async fn delete(ctx: &ServiceContext, doc_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::DocumentDelete).await?;

    let existing = get(ctx, doc_id).await?;
    existing.delete(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "document.delete".into(),
        entity_type: "document".into(),
        entity_id: doc_id.into(),
        old_data: None,
        new_data: None,
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(())
}
