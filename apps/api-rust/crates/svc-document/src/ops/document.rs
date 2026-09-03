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
    pub school_year: Option<String>,
    pub term: Option<String>,
    pub tags: Option<String>,
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
    let extension = req.file_name
        .rsplit('.')
        .next()
        .filter(|value| !value.is_empty() && value.len() <= 10 && value.chars().all(|c| c.is_ascii_alphanumeric()))
        .unwrap_or("bin")
        .to_ascii_lowercase();
    let key = format!("{}.{}", file_uuid, extension);

    // 2. Keep uploaded files isolated by organization and owning entity.
    let entity_type = clean_path_segment(&req.entity_type);
    let entity_id = clean_path_segment(&req.entity_id);
    let bucket = match entity_type.as_str() {
        "template" => format!("{}/templates/{}", ctx.org_id, entity_id),
        "student" => format!("{}/students/{}/avatars", ctx.org_id, entity_id),
        "user" | "teacher" | "employee" | "staff" => {
            format!("{}/users/{}/avatars", ctx.org_id, entity_id)
        }
        _ => format!("{}/{}/{}", ctx.org_id, entity_type, entity_id),
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
        entity_type: Some(entity_type),
        entity_id: Some(entity_id),
        name: req.file_name.clone(),
        description: None,
        category: req.category.clone(),
        asset_type: if req.mime_type.starts_with("image/") {
            "image".to_string()
        } else if req.mime_type.starts_with("video/") {
            "video".to_string()
        } else {
            "document".to_string()
        },
        school_year: req.school_year.clone(),
        term: req.term.clone(),
        tags: req.tags.clone(),
        visibility: req.visibility.clone(),
        size: Some(req.data.len() as i32),
        mime_type: Some(req.mime_type.clone()),
        extension: Some(extension.to_string()),
        storage_provider: storage.provider_name().to_string(),
        path: saved_path,
        url,
        thumbnail_path: None,
        preview_path: None,
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

fn clean_path_segment(value: &str) -> String {
    let cleaned: String = value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.'))
        .collect();
    if cleaned.is_empty() { "unknown".to_string() } else { cleaned }
}

pub async fn list(
    ctx: &ServiceContext,
    owner_id: Option<&str>,
    entity_type: Option<&str>,
    category: Option<&str>,
    entity_id: Option<&str>,
    request: shared_kernel::pagination::CursorRequest,
) -> Result<shared_kernel::pagination::CursorPage<documents::Model>, AppError> {
    use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
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

    if let Some(eid) = entity_id {
        query = query.filter(documents::Column::EntityId.eq(eid));
    }

    if let Some(filter) = request.filter.as_ref() {
        query = query.filter(filter.build_condition(|field| match field {
            "asset_type" => Some(documents::Column::AssetType.into_expr().into()),
            "school_year" => Some(documents::Column::SchoolYear.into_expr().into()),
            "term" => Some(documents::Column::Term.into_expr().into()),
            "tags" => Some(documents::Column::Tags.into_expr().into()),
            "entity_id" => Some(documents::Column::EntityId.into_expr().into()),
            _ => None,
        }));
        if let Some(search) = filter.search.as_deref().filter(|value| !value.is_empty()) {
            query = query.filter(documents::Column::Name.like(format!("%{}%", search)));
        }
    }

    let limit = request.effective_limit();
    
    if let Some(ref cursor) = request.cursor {
        if let Some((created_at, id)) = cursor.split_once('|') {
            query = query.filter(
                Condition::any()
                    .add(documents::Column::CreatedAt.lt(created_at))
                    .add(
                        Condition::all()
                            .add(documents::Column::CreatedAt.eq(created_at))
                            .add(documents::Column::Id.lt(id)),
                    ),
            );
        }
    }

    let items = query.limit(limit + 1).all(&ctx.db).await?;
    Ok(CursorPage::from_items_with(items, limit, |document| {
        format!("{}|{}", document.created_at, document.id)
    }))
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
        .ok_or_else(|| AppError::not_found("document", id))?
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
