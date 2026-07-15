use chrono::Utc;
use sea_orm::{ActiveModelTrait, Set};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use infra_integration::traits::StorageProvider;
use crate::entities::documents;

pub struct UploadRequest<'a> {
    pub file_name: String,
    pub mime_type: String,
    pub data: &'a [u8],
    pub entity_type: String,
    pub entity_id: String,
    pub category: String,
    pub visibility: String,
}

pub async fn upload_file(
    ctx: &ServiceContext,
    storage: &dyn StorageProvider,
    req: UploadRequest<'_>,
) -> Result<documents::Model, AppError> {
    // Basic permission check
    ctx.permissions.check(&ctx.actor, Permission::DocumentWrite).await?;

    // 1. Generate unique key to avoid collisions
    let file_uuid = Uuid::new_v4().to_string();
    let extension = req.file_name.split('.').last().unwrap_or("bin");
    let key = format!("{}.{}", file_uuid, extension);

    // 2. Determine bucket (e.g., users, templates, students)
    let bucket = match req.entity_type.as_str() {
        "template" => format!("{}/shared/templates", ctx.org_id),
        "user" | "student" => format!("{}/users/avatars", ctx.org_id),
        _ => format!("{}/{}/{}", ctx.org_id, req.entity_type, req.entity_id),
    };

    // 3. Upload bytes to storage provider
    let saved_path = storage
        .upload(&bucket, &key, req.data)
        .await
        .map_err(|e| AppError::internal(format!("Storage error: {}", e)))?;

    // 4. Record to Database
    let now = Utc::now().to_rfc3339();
    let url = storage.get_url(&bucket, &key);
    
    let model = documents::Model {
        id: file_uuid.clone(),
        org_id: ctx.org_id.clone(),
        uploaded_by: Some(ctx.actor.as_str().to_string()),
        entity_type: Some(req.entity_type.clone()),
        entity_id: Some(req.entity_id.clone()),
        name: req.file_name.clone(),
        description: None,
        category: req.category.clone(),
        visibility: req.visibility.clone(),
        size: Some(req.data.len() as i32),
        mime_type: Some(req.mime_type.clone()),
        extension: Some(extension.to_string()),
        storage_provider: storage.provider_name().to_string(),
        path: saved_path,
        url,
        deleted_at: None,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: documents::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    // 5. Audit Log
    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "document.upload".into(),
        entity_type: "document".into(),
        entity_id: file_uuid,
        old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

pub async fn list(
    ctx: &ServiceContext,
    owner_id: Option<&str>,
) -> Result<Vec<documents::Model>, AppError> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

    ctx.permissions.check(&ctx.actor, Permission::DocumentRead).await?;

    let mut query = documents::Entity::find()
        .filter(documents::Column::OrgId.eq(ctx.org_id.clone()))
        .filter(documents::Column::DeletedAt.is_null());

    if let Some(oid) = owner_id {
        // Here we could filter by uploaded_by, or by entity_id
        query = query.filter(documents::Column::UploadedBy.eq(oid));
    }

    let docs = query.all(&ctx.db).await?;
    Ok(docs)
}

pub async fn delete(
    ctx: &ServiceContext,
    id: &str,
) -> Result<(), AppError> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

    ctx.permissions.check(&ctx.actor, Permission::DocumentWrite).await?;

    let mut doc: documents::ActiveModel = documents::Entity::find_by_id(id.to_string())
        .filter(documents::Column::OrgId.eq(ctx.org_id.clone()))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("Document not found"))?
        .into();

    doc.deleted_at = Set(Some(Utc::now().to_rfc3339()));
    let updated = doc.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "document.delete".into(),
        entity_type: "document".into(),
        entity_id: id.to_string(),
        old_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        new_data: None,
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(())
}
