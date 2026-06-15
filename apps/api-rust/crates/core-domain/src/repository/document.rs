use crate::entities::document;
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait, QueryOrder};

impl super::Repository {
    pub async fn get_documents(
        &self,
        org_id: &str,
        owner_id: Option<&str>,
        is_template: Option<i32>,
    ) -> Result<Vec<document::Model>, DbErr> {
        let mut query = document::Entity::find()
            .filter(document::Column::OrgId.eq(org_id))
            .filter(document::Column::DeletedAt.is_null());

        if let Some(oid) = owner_id {
            query = query.filter(document::Column::OwnerId.eq(oid));
        }
        if let Some(tpl) = is_template {
            query = query.filter(document::Column::IsTemplate.eq(tpl));
        }

        query
            .order_by_desc(document::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    pub async fn get_document_by_id(&self, id: &str) -> Result<Option<document::Model>, DbErr> {
        document::Entity::find_by_id(id.to_string())
            .filter(document::Column::DeletedAt.is_null())
            .one(&self.db)
            .await
    }

    pub async fn insert_document(&self, doc: document::Model) -> Result<document::Model, DbErr> {
        let active = document::ActiveModel {
            id: Set(doc.id),
            org_id: Set(doc.org_id),
            owner_id: Set(doc.owner_id),
            uploaded_by: Set(doc.uploaded_by),
            name: Set(doc.name),
            title: Set(doc.title),
            description: Set(doc.description),
            item_type: Set(doc.item_type),
            visibility: Set(doc.visibility),
            size: Set(doc.size),
            mime_type: Set(doc.mime_type),
            extension: Set(doc.extension),
            storage_provider: Set(doc.storage_provider),
            bucket: Set(doc.bucket),
            key: Set(doc.key),
            url: Set(doc.url),
            thumbnail_url: Set(doc.thumbnail_url),
            parent_id: Set(doc.parent_id),
            path: Set(doc.path),
            depth: Set(doc.depth),
            shared_with: Set(doc.shared_with),
            share_link: Set(doc.share_link),
            share_link_expires_at: Set(doc.share_link_expires_at),
            share_password: Set(doc.share_password),
            version: Set(doc.version),
            previous_version_id: Set(doc.previous_version_id),
            is_latest_version: Set(doc.is_latest_version),
            processing_status: Set(doc.processing_status),
            ocr_text: Set(doc.ocr_text),
            extracted_data: Set(doc.extracted_data),
            is_pinned: Set(doc.is_pinned),
            is_starred: Set(doc.is_starred),
            is_template: Set(doc.is_template),
            tags: Set(doc.tags),
            meta: Set(doc.meta),
            deleted_at: Set(doc.deleted_at),
            created_at: Set(doc.created_at),
            updated_at: Set(doc.updated_at),
        };
        active.insert(&self.db).await
    }
}
