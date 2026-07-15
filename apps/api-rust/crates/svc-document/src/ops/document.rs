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
    entity_type: Option<&str>,
    category: Option<&str>,
    request: shared_kernel::pagination::CursorRequest,
) -> Result<shared_kernel::pagination::CursorPage<documents::Model>, AppError> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, QuerySelect, QueryOrder};
    use shared_kernel::pagination::CursorPage;

    ctx.permissions.check(&ctx.actor, Permission::DocumentRead).await?;

    let mut query = documents::Entity::find()
        .filter(documents::Column::OrgId.eq(ctx.org_id.clone()))
        .filter(documents::Column::DeletedAt.is_null())
        .order_by_desc(documents::Column::CreatedAt)
        .order_by_desc(documents::Column::Id); // tie-breaker

    if let Some(oid) = owner_id {
        query = query.filter(documents::Column::UploadedBy.eq(oid));
    }
    
    if let Some(et) = entity_type {
        query = query.filter(documents::Column::EntityType.eq(et));
    }

    if let Some(cat) = category {
        query = query.filter(documents::Column::Category.eq(cat));
    }

    let limit = request.effective_limit();
    
    // We treat cursor as a literal OFFSET index for simplicity
    if let Some(ref cursor) = request.cursor {
        if let Ok(offset) = cursor.parse::<u64>() {
            query = query.offset(offset);
        }
    }

    let items = query.limit(limit + 1).all(&ctx.db).await?;
    
    let effective_limit = limit.min(shared_kernel::pagination::CursorRequest::MAX_LIMIT).max(1) as usize;
    let has_more = items.len() > effective_limit;
    
    let mut items = items;
    if has_more {
        items.truncate(effective_limit);
    }
    
    let count = items.len();
    
    // The next cursor is the current offset + the number of items fetched this time
    let next_cursor = if has_more {
        let current_offset = request.cursor.as_deref().and_then(|c| c.parse::<u64>().ok()).unwrap_or(0);
        Some((current_offset + count as u64).to_string())
    } else {
        None
    };
    
    Ok(CursorPage {
        items,
        count,
        next_cursor,
        has_more,
    })
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
