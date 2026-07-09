use uuid::Uuid;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set};

use crate::entities::document;

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct CreateDocumentRequest {
    pub name: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub item_type: Option<String>,
    pub visibility: Option<String>,
    pub storage_provider: Option<String>,
    pub key: Option<String>,
    pub url: Option<String>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub size: Option<i32>,
    pub parent_id: Option<String>,
    pub is_template: Option<i32>,
    pub tags: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct UpdateDocumentRequest {
    pub name: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub is_pinned: Option<i32>,
    pub is_starred: Option<i32>,
    pub tags: Option<String>,
}

impl super::Service {
    pub async fn get_documents(
        &self,
        org_id: &str,
        owner_id: Option<&str>,
        is_template: Option<i32>,
    ) -> Result<Vec<document::Model>, String> {
        self.repo.get_documents(org_id, owner_id, is_template).await.map_err(|e| e.to_string())
    }

    pub async fn get_document(&self, id: &str) -> Result<document::Model, String> {
        self.repo.get_document_by_id(id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "document not found".to_string())
    }

    pub async fn create_document(
        &self,
        org_id: &str,
        owner_id: &str,
        uploaded_by: Option<&str>,
        req: CreateDocumentRequest,
    ) -> Result<document::Model, String> {
        if req.name.trim().is_empty() {
            return Err("document name is required".to_string());
        }

        let now = Utc::now().to_rfc3339();
        let doc = document::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            owner_id: owner_id.to_string(),
            uploaded_by: uploaded_by.map(String::from),
            name: req.name,
            title: req.title,
            description: req.description,
            item_type: req.item_type.unwrap_or_else(|| "file".to_string()),
            visibility: req.visibility.unwrap_or_else(|| "private".to_string()),
            size: req.size,
            mime_type: req.mime_type,
            extension: req.extension,
            storage_provider: req.storage_provider.unwrap_or_else(|| "local".to_string()),
            bucket: None,
            key: req.key.unwrap_or_default(),
            url: req.url,
            thumbnail_url: None,
            parent_id: req.parent_id,
            path: None,
            depth: None,
            shared_with: None,
            share_link: None,
            share_link_expires_at: None,
            share_password: None,
            version: Some(1),
            previous_version_id: None,
            is_latest_version: 1,
            processing_status: Some("ready".to_string()),
            ocr_text: None,
            extracted_data: None,
            is_pinned: 0,
            is_starred: 0,
            is_template: req.is_template.unwrap_or(0),
            tags: req.tags,
            meta: None,
            deleted_at: None,
            created_at: now.clone(),
            updated_at: now,
        };
        self.repo.insert_document(doc).await.map_err(|e| e.to_string())
    }

    pub async fn update_document(
        &self,
        id: &str,
        req: UpdateDocumentRequest,
    ) -> Result<document::Model, String> {
        let doc = self.repo.get_document_by_id(id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "document not found".to_string())?;

        let mut active: document::ActiveModel = doc.into();
        if let Some(v) = req.name { active.name = Set(v); }
        if let Some(v) = req.title { active.title = Set(Some(v)); }
        if let Some(v) = req.description { active.description = Set(Some(v)); }
        if let Some(v) = req.visibility { active.visibility = Set(v); }
        if let Some(v) = req.is_pinned { active.is_pinned = Set(v); }
        if let Some(v) = req.is_starred { active.is_starred = Set(v); }
        if let Some(v) = req.tags { active.tags = Set(Some(v)); }
        active.updated_at = Set(Utc::now().to_rfc3339());

        active.update(&self.repo.db).await.map_err(|e| e.to_string())
    }

    pub async fn delete_document(&self, id: &str) -> Result<(), String> {
        let doc = self.repo.get_document_by_id(id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "document not found".to_string())?;

        let mut active: document::ActiveModel = doc.into();
        active.deleted_at = Set(Some(Utc::now().to_rfc3339()));
        active.update(&self.repo.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
