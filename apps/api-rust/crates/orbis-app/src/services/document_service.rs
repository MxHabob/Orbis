use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use orbis_domain::entities::document::Document;
use orbis_domain::entities::document_requirement::{
    DocumentRequirement, RequirementTargetType, DocumentCondition,
};
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use orbis_domain::iam::permission::Permission;

use crate::context::RequestContext;
use crate::ports::repositories::{DocumentRepository, DocumentRequirementRepository};
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;

#[derive(Debug, Clone)]
pub struct CreateDocumentInput {
    pub uploaded_by: Option<Uuid>,
    pub org_unit_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub asset_type: String,
    pub academic_year_id: Option<Uuid>,
    pub term_id: Option<Uuid>,
    pub tags: Option<String>,
    pub visibility: String,
    pub file_size_bytes: Option<i64>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub checksum_sha256: Option<String>,
    pub storage_provider: String,
    pub file_path: String,
    pub storage_key: Option<String>,
    pub url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub preview_path: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateDocumentInput {
    pub name: Option<String>,
    pub folder_id: Option<Uuid>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub asset_type: Option<String>,
    pub academic_year_id: Option<Uuid>,
    pub term_id: Option<Uuid>,
    pub tags: Option<String>,
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateDocumentRequirementInput {
    pub target_type: RequirementTargetType,
    pub target_id: Option<Uuid>,
    pub code: String,
    pub name_ar: String,
    pub name_en: String,
    pub is_mandatory: bool,
    pub blocks_enrollment: bool,
    pub condition: DocumentCondition,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct UpdateDocumentRequirementInput {
    pub target_type: Option<RequirementTargetType>,
    pub target_id: Option<Option<Uuid>>,
    pub code: Option<String>,
    pub name_ar: Option<String>,
    pub name_en: Option<String>,
    pub is_mandatory: Option<bool>,
    pub blocks_enrollment: Option<bool>,
    pub condition: Option<DocumentCondition>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StudentDocumentFulfillment {
    pub requirement: DocumentRequirement,
    pub is_fulfilled: bool,
    pub uploaded_document: Option<Document>,
    pub is_applicable: bool,
}

pub struct DocumentService {
    pub document_repo: Arc<dyn DocumentRepository>,
    pub doc_requirement_repo: Arc<dyn DocumentRequirementRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl DocumentService {
    pub fn new(
        document_repo: Arc<dyn DocumentRepository>,
        doc_requirement_repo: Arc<dyn DocumentRequirementRepository>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            document_repo,
            doc_requirement_repo,
            event_bus,
            permission_checker,
        }
    }

    /// Upload/Record a document metadata record
    pub async fn create_document(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: CreateDocumentInput,
    ) -> Result<Document, DomainError> {
        self.permission_checker.check(ctx, Permission::DocumentWrite).await?;

        let now = Utc::now();
        let doc_id = Uuid::new_v4();

        let doc = Document {
            id: doc_id,
            org_id,
            org_unit_id: input.org_unit_id,
            folder_id: input.folder_id,
            uploaded_by: input.uploaded_by,
            entity_type: input.entity_type.clone(),
            entity_id: input.entity_id,
            name: input.name,
            description: input.description,
            category: input.category,
            asset_type: input.asset_type,
            academic_year_id: input.academic_year_id,
            term_id: input.term_id,
            tags: input.tags,
            visibility: input.visibility,
            file_size_bytes: input.file_size_bytes,
            mime_type: input.mime_type,
            extension: input.extension,
            checksum_sha256: input.checksum_sha256,
            storage_provider: input.storage_provider,
            file_path: input.file_path,
            storage_key: input.storage_key,
            url: input.url,
            thumbnail_path: input.thumbnail_path,
            preview_path: input.preview_path,
            is_archived: false,
            archived_at: None,
            archived_by: None,
            archive_reason: None,
            is_legal_hold: false,
            deleted_at: None,
            deleted_by: None,
            created_at: now,
            updated_at: now,
        };

        self.document_repo.save(&doc).await?;

        let event = DomainEvent::DocumentUploaded {
            document_id: doc_id,
            entity_type: input.entity_type,
            entity_id: input.entity_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(doc)
    }

    /// Update an existing document's metadata
    pub async fn update_document(
        &self,
        ctx: &RequestContext,
        id: Uuid,
        input: UpdateDocumentInput,
    ) -> Result<Document, DomainError> {
        self.permission_checker.check(ctx, Permission::DocumentWrite).await?;

        let mut doc = self.document_repo.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Document {id} not found")))?;

        if let Some(name) = input.name {
            doc.name = name;
        }
        if let Some(fid) = input.folder_id {
            doc.folder_id = Some(fid);
        }
        if let Some(desc) = input.description {
            doc.description = Some(desc);
        }
        if let Some(cat) = input.category {
            doc.category = cat;
        }
        if let Some(asset) = input.asset_type {
            doc.asset_type = asset;
        }
        if let Some(ay_id) = input.academic_year_id {
            doc.academic_year_id = Some(ay_id);
        }
        if let Some(t_id) = input.term_id {
            doc.term_id = Some(t_id);
        }
        if let Some(tags) = input.tags {
            doc.tags = Some(tags);
        }
        if let Some(vis) = input.visibility {
            doc.visibility = vis;
        }
        doc.updated_at = Utc::now();

        self.document_repo.save(&doc).await?;

        let event = DomainEvent::DocumentUpdated {
            document_id: id,
            org_id: doc.org_id,
            timestamp: doc.updated_at,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(doc)
    }

    /// Get document by ID
    pub async fn get_document(&self, id: Uuid) -> Result<Option<Document>, DomainError> {
        self.document_repo.find_by_id(id).await
    }

    /// List active documents for an organization
    pub async fn list_documents_by_org(&self, org_id: Uuid) -> Result<Vec<Document>, DomainError> {
        self.document_repo.find_by_org(org_id).await
    }

    /// List documents attached to an entity (e.g., Student, Employee)
    pub async fn list_documents_by_entity(&self, entity_type: &str, entity_id: Uuid) -> Result<Vec<Document>, DomainError> {
        self.document_repo.find_by_entity(entity_type, entity_id).await
    }

    /// List archived documents for an organization
    pub async fn list_archived_documents(&self, org_id: Uuid) -> Result<Vec<Document>, DomainError> {
        self.document_repo.find_archived(org_id).await
    }

    /// List soft-deleted documents (trash) for an organization
    pub async fn list_trash_documents(&self, org_id: Uuid) -> Result<Vec<Document>, DomainError> {
        self.document_repo.find_trash(org_id).await
    }

    /// Archive a document
    pub async fn archive_document(&self, ctx: &RequestContext, id: Uuid, reason: Option<String>) -> Result<Document, DomainError> {
        self.permission_checker.check(ctx, Permission::DocumentWrite).await?;

        let mut doc = self.document_repo.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Document {id} not found")))?;

        let actor_id = match ctx.actor {
            orbis_domain::iam::actor::Actor::User(uid) => Some(uid),
            _ => None,
        };

        doc.is_archived = true;
        doc.archived_at = Some(Utc::now());
        doc.archived_by = actor_id;
        doc.archive_reason = reason.clone();
        doc.updated_at = Utc::now();

        self.document_repo.save(&doc).await?;

        let event = DomainEvent::DocumentArchived {
            document_id: id,
            org_id: doc.org_id,
            reason,
            timestamp: doc.updated_at,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(doc)
    }

    /// Restore an archived or soft-deleted document
    pub async fn restore_document(&self, ctx: &RequestContext, id: Uuid) -> Result<Document, DomainError> {
        self.permission_checker.check(ctx, Permission::DocumentWrite).await?;

        self.document_repo.restore(id).await?;

        let doc = self.document_repo.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Document {id} not found")))?;

        let event = DomainEvent::DocumentRestored {
            document_id: id,
            org_id: doc.org_id,
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(doc)
    }

    /// Soft delete document (move to trash)
    pub async fn delete_document(&self, ctx: &RequestContext, id: Uuid) -> Result<(), DomainError> {
        self.permission_checker.check(ctx, Permission::DocumentDelete).await?;

        let actor_id = match ctx.actor {
            orbis_domain::iam::actor::Actor::User(uid) => Some(uid),
            _ => None,
        };

        self.document_repo.soft_delete(id, actor_id).await?;

        let event = DomainEvent::DocumentDeleted {
            document_id: id,
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(())
    }

    /// Permanently purge document from database
    pub async fn purge_document(&self, ctx: &RequestContext, id: Uuid) -> Result<(), DomainError> {
        self.permission_checker.check(ctx, Permission::DocumentDelete).await?;

        self.document_repo.purge(id).await?;

        let event = DomainEvent::DocumentPurged {
            document_id: id,
            org_id: ctx.tenant_id,
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(())
    }

    // ── Document Requirements ────────────────────────────────────────

    /// Create or save a document requirement
    pub async fn create_document_requirement(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: CreateDocumentRequirementInput,
    ) -> Result<DocumentRequirement, DomainError> {
        self.permission_checker.check(ctx, Permission::DocumentWrite).await?;

        let req = DocumentRequirement::new(
            org_id,
            input.target_type,
            input.target_id,
            input.code,
            input.name_ar,
            input.name_en,
            input.is_mandatory,
            input.blocks_enrollment,
            input.condition,
        );

        self.doc_requirement_repo.save(&req).await?;

        let event = DomainEvent::DocumentRequirementCreated {
            requirement_id: req.id,
            org_id,
            code: req.code.clone(),
            timestamp: req.created_at,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(req)
    }

    /// Get document requirement by ID
    pub async fn get_document_requirement(&self, id: Uuid) -> Result<Option<DocumentRequirement>, DomainError> {
        self.doc_requirement_repo.find_by_id(id).await
    }

    /// Update an existing document requirement
    pub async fn update_document_requirement(
        &self,
        ctx: &RequestContext,
        id: Uuid,
        input: UpdateDocumentRequirementInput,
    ) -> Result<DocumentRequirement, DomainError> {
        self.permission_checker.check(ctx, Permission::DocumentWrite).await?;

        let mut req = self.doc_requirement_repo.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Document requirement {id} not found")))?;

        if let Some(target_type) = input.target_type {
            req.target_type = target_type;
        }
        if let Some(target_id) = input.target_id {
            req.target_id = target_id;
        }
        if let Some(code) = input.code {
            req.code = code;
        }
        if let Some(name_ar) = input.name_ar {
            req.name_ar = name_ar;
        }
        if let Some(name_en) = input.name_en {
            req.name_en = name_en;
        }
        if let Some(is_mandatory) = input.is_mandatory {
            req.is_mandatory = is_mandatory;
        }
        if let Some(blocks) = input.blocks_enrollment {
            req.blocks_enrollment = blocks;
        }
        if let Some(cond) = input.condition {
            req.condition = cond;
        }
        req.updated_at = Utc::now();

        self.doc_requirement_repo.save(&req).await?;

        let event = DomainEvent::DocumentRequirementUpdated {
            requirement_id: id,
            org_id: req.org_id,
            timestamp: req.updated_at,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(req)
    }

    /// List document requirements for an organization
    pub async fn list_document_requirements_by_org(&self, org_id: Uuid) -> Result<Vec<DocumentRequirement>, DomainError> {
        self.doc_requirement_repo.find_by_org(org_id).await
    }

    /// List document requirements by target (Stage, GradeLevel, Program)
    pub async fn list_document_requirements_by_target(
        &self,
        org_id: Uuid,
        target_type: &str,
        target_id: Option<Uuid>,
    ) -> Result<Vec<DocumentRequirement>, DomainError> {
        self.doc_requirement_repo.find_by_target(org_id, target_type, target_id).await
    }

    /// Delete document requirement
    pub async fn delete_document_requirement(&self, ctx: &RequestContext, id: Uuid) -> Result<(), DomainError> {
        self.permission_checker.check(ctx, Permission::DocumentDelete).await?;
        let req = self.doc_requirement_repo.find_by_id(id).await?;
        self.doc_requirement_repo.delete(id).await?;

        if let Some(r) = req {
            let event = DomainEvent::DocumentRequirementDeleted {
                requirement_id: id,
                org_id: r.org_id,
                timestamp: Utc::now(),
            };
            self.event_bus.publish(ctx, &event).await?;
        }
        Ok(())
    }

    /// Evaluate document requirements fulfillment for a student
    pub async fn evaluate_student_requirements(
        &self,
        org_id: Uuid,
        student_id: Uuid,
        grade_level_id: Option<Uuid>,
        is_national: bool,
        is_transfer: bool,
    ) -> Result<Vec<StudentDocumentFulfillment>, DomainError> {
        let all_org_reqs = self.doc_requirement_repo.find_by_org(org_id).await?;
        let student_docs = self.document_repo.find_by_entity("student", student_id).await?;

        let mut results = Vec::new();

        for req in all_org_reqs {
            // Check target match
            let is_target_match = match req.target_type {
                RequirementTargetType::Global => true,
                RequirementTargetType::GradeLevel => req.target_id == grade_level_id,
                _ => true,
            };

            if !is_target_match {
                continue;
            }

            // Check condition match
            let is_applicable = match req.condition {
                DocumentCondition::AllStudents => true,
                DocumentCondition::NonNationalOnly => !is_national,
                DocumentCondition::TransferStudentsOnly => is_transfer,
                DocumentCondition::SpecialNeedsOnly => true,
            };

            // Check if student uploaded matching document
            let uploaded = student_docs.iter().find(|d| {
                let d_name = d.name.to_lowercase();
                let r_code = req.code.to_lowercase();
                let r_name_ar = req.name_ar.to_lowercase();
                let r_name_en = req.name_en.to_lowercase();
                d_name == r_code || d_name.contains(&r_name_ar) || d_name.contains(&r_name_en)
            }).cloned();

            let is_fulfilled = uploaded.is_some();

            results.push(StudentDocumentFulfillment {
                requirement: req,
                is_fulfilled,
                uploaded_document: uploaded,
                is_applicable,
            });
        }

        Ok(results)
    }
}
