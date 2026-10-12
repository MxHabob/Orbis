use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use orbis_domain::{
    entities::{document::Document, folder::Folder},
    errors::DomainError,
    iam::permission::Permission,
};

use crate::context::RequestContext;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{DocumentRepository, FolderRepository};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderInput {
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub folder_type: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub org_unit_id: Option<Uuid>,
    pub calendar_id: Option<Uuid>,
    pub term_id: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFolderInput {
    pub name: Option<String>,
    pub parent_id: Option<Uuid>,
    pub calendar_id: Option<Uuid>,
    pub term_id: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderContentsDto {
    pub folder: Folder,
    pub subfolders: Vec<Folder>,
    pub documents: Vec<Document>,
}

#[derive(Clone)]
pub struct FolderService {
    pub folder_repo: Arc<dyn FolderRepository>,
    pub document_repo: Arc<dyn DocumentRepository>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl FolderService {
    pub fn new(
        folder_repo: Arc<dyn FolderRepository>,
        document_repo: Arc<dyn DocumentRepository>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            folder_repo,
            document_repo,
            permission_checker,
        }
    }

    /// Create a new folder with auto-computed materialized path.
    pub async fn create_folder(
        &self,
        ctx: &RequestContext,
        input: CreateFolderInput,
    ) -> Result<Folder, DomainError> {
        self.permission_checker
            .check(ctx, Permission::DocumentWrite)
            .await?;
        let org_id = ctx.tenant_id;

        let parent_path = if let Some(pid) = input.parent_id {
            let parent = self
                .folder_repo
                .find_by_id(org_id, pid)
                .await?
                .ok_or_else(|| DomainError::NotFound(format!("Parent folder {pid} not found")))?;
            parent.path
        } else {
            "".to_string()
        };

        let sanitized_name = input.name.trim().replace('/', "-");
        let path = format!("{}/{}", parent_path.trim_end_matches('/'), sanitized_name);

        let user_id = match ctx.actor {
            orbis_domain::iam::actor::Actor::User(uid) => Some(uid),
            _ => None,
        };

        let folder = Folder {
            id: Uuid::new_v4(),
            org_id,
            parent_id: input.parent_id,
            name: input.name.trim().to_string(),
            path,
            folder_type: input.folder_type.unwrap_or_else(|| "general".to_string()),
            entity_type: input.entity_type,
            entity_id: input.entity_id,
            org_unit_id: input.org_unit_id,
            calendar_id: input.calendar_id,
            term_id: input.term_id,
            is_system: false,
            metadata: input.metadata,
            created_by: user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        self.folder_repo.save(&folder).await?;
        Ok(folder)
    }

    /// Retrieve or auto-provision a student's personal folder for all their records & media.
    pub async fn get_or_create_student_folder(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
        student_name: &str,
    ) -> Result<Folder, DomainError> {
        self.permission_checker
            .check(ctx, Permission::DocumentRead)
            .await?;
        let org_id = ctx.tenant_id;

        if let Some(folder) = self
            .folder_repo
            .find_by_entity(org_id, "student", student_id)
            .await?
        {
            return Ok(folder);
        }

        let user_id = match ctx.actor {
            orbis_domain::iam::actor::Actor::User(uid) => Some(uid),
            _ => None,
        };

        // 1. Ensure root "/Students" folder exists
        let root_student_folder = match self.folder_repo.find_by_path(org_id, "/Students").await? {
            Some(f) => f,
            None => {
                let root = Folder {
                    id: Uuid::new_v4(),
                    org_id,
                    parent_id: None,
                    name: "Students".to_string(),
                    path: "/Students".to_string(),
                    folder_type: "system".to_string(),
                    entity_type: None,
                    entity_id: None,
                    org_unit_id: None,
                    calendar_id: None,
                    term_id: None,
                    is_system: true,
                    metadata: None,
                    created_by: user_id,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                };
                self.folder_repo.save(&root).await?;
                root
            }
        };

        // 2. Create student folder
        let folder_name = format!("{}_{}", student_name.replace('/', "-").trim(), student_id);
        let path = format!("/Students/{}", folder_name);

        let student_folder = Folder {
            id: Uuid::new_v4(),
            org_id,
            parent_id: Some(root_student_folder.id),
            name: format!("{} ({})", student_name.trim(), student_id),
            path,
            folder_type: "student".to_string(),
            entity_type: Some("student".to_string()),
            entity_id: Some(student_id),
            org_unit_id: None,
            calendar_id: None,
            term_id: None,
            is_system: false,
            metadata: Some(serde_json::json!({
                "student_id": student_id,
                "student_name": student_name,
                "auto_provisioned": true,
            })),
            created_by: user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        self.folder_repo.save(&student_folder).await?;
        Ok(student_folder)
    }

    /// Retrieve or auto-provision a Curriculum folder for AI ingestion & knowledge retrieval.
    pub async fn get_or_create_curriculum_folder(
        &self,
        ctx: &RequestContext,
        folder_name: &str,
        calendar_id: Option<Uuid>,
        term_id: Option<Uuid>,
        metadata: Option<serde_json::Value>,
    ) -> Result<Folder, DomainError> {
        self.permission_checker
            .check(ctx, Permission::DocumentWrite)
            .await?;
        let org_id = ctx.tenant_id;

        let user_id = match ctx.actor {
            orbis_domain::iam::actor::Actor::User(uid) => Some(uid),
            _ => None,
        };

        // 1. Ensure root "/Curriculum" folder exists
        let root_curriculum_folder = match self
            .folder_repo
            .find_by_path(org_id, "/Curriculum")
            .await?
        {
            Some(f) => f,
            None => {
                let root = Folder {
                    id: Uuid::new_v4(),
                    org_id,
                    parent_id: None,
                    name: "Curriculum".to_string(),
                    path: "/Curriculum".to_string(),
                    folder_type: "curriculum_root".to_string(),
                    entity_type: None,
                    entity_id: None,
                    org_unit_id: None,
                    calendar_id: None,
                    term_id: None,
                    is_system: true,
                    metadata: Some(serde_json::json!({
                        "ai_agent_accessible": true,
                        "description": "Root storage for textbooks, lesson plans, syllabi and AI knowledge",
                    })),
                    created_by: user_id,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    deleted_at: None,
                };
                self.folder_repo.save(&root).await?;
                root
            }
        };

        let sanitized = folder_name.trim().replace('/', "-");
        let path = format!("/Curriculum/{}", sanitized);

        if let Some(folder) = self.folder_repo.find_by_path(org_id, &path).await? {
            return Ok(folder);
        }

        let mut meta = metadata.unwrap_or_else(|| serde_json::json!({}));
        if let Some(obj) = meta.as_object_mut() {
            obj.insert(
                "ai_agent_accessible".to_string(),
                serde_json::Value::Bool(true),
            );
            obj.insert("knowledge_base".to_string(), serde_json::Value::Bool(true));
        }

        let folder = Folder {
            id: Uuid::new_v4(),
            org_id,
            parent_id: Some(root_curriculum_folder.id),
            name: folder_name.trim().to_string(),
            path,
            folder_type: "curriculum".to_string(),
            entity_type: Some("curriculum".to_string()),
            entity_id: None,
            org_unit_id: None,
            calendar_id,
            term_id,
            is_system: false,
            metadata: Some(meta),
            created_by: user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        self.folder_repo.save(&folder).await?;
        Ok(folder)
    }

    /// List all curriculum folders (used by AI agent and teacher dashboard).
    pub async fn list_curriculum_folders(
        &self,
        ctx: &RequestContext,
    ) -> Result<Vec<Folder>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::DocumentRead)
            .await?;
        let org_id = ctx.tenant_id;
        self.folder_repo.list_by_type(org_id, "curriculum").await
    }

    /// Get folder contents: details, direct subfolders, and documents.
    pub async fn get_folder_contents(
        &self,
        ctx: &RequestContext,
        folder_id: Uuid,
    ) -> Result<FolderContentsDto, DomainError> {
        self.permission_checker
            .check(ctx, Permission::DocumentRead)
            .await?;
        let org_id = ctx.tenant_id;

        let folder = self
            .folder_repo
            .find_by_id(org_id, folder_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Folder {folder_id} not found")))?;

        let subfolders = self
            .folder_repo
            .list_children(org_id, Some(folder_id))
            .await?;
        let documents = self.document_repo.find_by_folder(org_id, folder_id).await?;

        Ok(FolderContentsDto {
            folder,
            subfolders,
            documents,
        })
    }

    /// List root folders or subfolders of a parent.
    pub async fn list_folders(
        &self,
        ctx: &RequestContext,
        parent_id: Option<Uuid>,
    ) -> Result<Vec<Folder>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::DocumentRead)
            .await?;
        let org_id = ctx.tenant_id;
        self.folder_repo.list_children(org_id, parent_id).await
    }

    /// Get folder by ID.
    pub async fn get_folder(&self, ctx: &RequestContext, id: Uuid) -> Result<Folder, DomainError> {
        self.permission_checker
            .check(ctx, Permission::DocumentRead)
            .await?;
        let org_id = ctx.tenant_id;
        self.folder_repo
            .find_by_id(org_id, id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Folder {id} not found")))
    }

    /// Update folder metadata or rename.
    pub async fn update_folder(
        &self,
        ctx: &RequestContext,
        id: Uuid,
        input: UpdateFolderInput,
    ) -> Result<Folder, DomainError> {
        self.permission_checker
            .check(ctx, Permission::DocumentWrite)
            .await?;
        let org_id = ctx.tenant_id;

        let mut folder = self
            .folder_repo
            .find_by_id(org_id, id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Folder {id} not found")))?;

        if let Some(name) = input.name {
            folder.name = name.trim().to_string();
        }
        if let Some(pid) = input.parent_id {
            folder.parent_id = Some(pid);
        }
        if let Some(ay_id) = input.calendar_id {
            folder.calendar_id = Some(ay_id);
        }
        if let Some(t_id) = input.term_id {
            folder.term_id = Some(t_id);
        }
        if let Some(meta) = input.metadata {
            folder.metadata = Some(meta);
        }
        folder.updated_at = Utc::now();

        self.folder_repo.save(&folder).await?;
        Ok(folder)
    }

    /// Delete folder.
    pub async fn delete_folder(&self, ctx: &RequestContext, id: Uuid) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::DocumentDelete)
            .await?;
        let org_id = ctx.tenant_id;

        let folder = self
            .folder_repo
            .find_by_id(org_id, id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Folder {id} not found")))?;

        if folder.is_system {
            return Err(DomainError::Validation(
                "System folders cannot be deleted".into(),
            ));
        }

        self.folder_repo.delete(org_id, id).await
    }
}
