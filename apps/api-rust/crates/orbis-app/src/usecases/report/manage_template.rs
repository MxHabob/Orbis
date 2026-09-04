//! Template management use cases.

use crate::context::RequestContext;
use crate::ports::reporting::*;
use orbis_domain::entities::report::*;
use orbis_domain::errors::DomainError;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct ManageTemplateUseCase {
    pub template_repo: Arc<dyn ReportTemplateRepository>,
    pub audit_repo: Arc<dyn ReportAuditRepository>,
}

impl ManageTemplateUseCase {
    pub async fn create(
        &self,
        ctx: &RequestContext,
        mut template: ReportTemplate,
    ) -> Result<ReportTemplate, DomainError> {
        template.id = Uuid::new_v4();
        template.org_id = ctx.tenant_id;
        template.status = TemplateStatus::Draft;
        template.version = 1;
        template.created_by = ctx.actor.id().unwrap_or_default();
        template.created_at = Utc::now();
        template.updated_at = Utc::now();

        self.template_repo.save(&template).await?;

        self.audit(ctx, "template.created", "report_template", template.id).await;
        Ok(template)
    }

    pub async fn update(
        &self,
        ctx: &RequestContext,
        template_id: Uuid,
        name: Option<String>,
        name_ar: Option<String>,
        description: Option<String>,
        description_ar: Option<String>,
        document_json: Option<String>,
        page_size: Option<String>,
        orientation: Option<String>,
        margins_json: Option<String>,
    ) -> Result<ReportTemplate, DomainError> {
        let mut template = self.template_repo
            .find_by_id(template_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Template not found".into()))?;

        // Tenant isolation
        if template.org_id != ctx.tenant_id {
            return Err(DomainError::Unauthorized("Template belongs to another organization".into()));
        }

        if let Some(n) = name { template.name = n; }
        if let Some(n) = name_ar { template.name_ar = Some(n); }
        if let Some(d) = description { template.description = Some(d); }
        if let Some(d) = description_ar { template.description_ar = Some(d); }
        if let Some(ps) = page_size { template.page_size = ps; }
        if let Some(o) = orientation { template.orientation = o; }
        if let Some(m) = margins_json { template.margins_json = m; }

        // If document changed, bump version and create snapshot
        if let Some(doc) = document_json {
            // Save version snapshot before updating
            let version_snapshot = ReportTemplateVersion {
                id: Uuid::new_v4(),
                template_id: template.id,
                version: template.version,
                document_json: template.document_json.clone(),
                page_size: Some(template.page_size.clone()),
                page_width_mm: template.page_width_mm,
                page_height_mm: template.page_height_mm,
                orientation: Some(template.orientation.clone()),
                margins_json: Some(template.margins_json.clone()),
                change_summary: None,
                created_by: ctx.actor.id().unwrap_or_default(),
                created_at: Utc::now(),
            };
            self.template_repo.save_version(&version_snapshot).await?;

            template.document_json = doc;
            template.version += 1;
        }

        template.updated_at = Utc::now();
        self.template_repo.save(&template).await?;

        self.audit(ctx, "template.updated", "report_template", template.id).await;
        Ok(template)
    }

    pub async fn publish(
        &self,
        ctx: &RequestContext,
        template_id: Uuid,
    ) -> Result<ReportTemplate, DomainError> {
        let mut template = self.template_repo
            .find_by_id(template_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Template not found".into()))?;

        if template.org_id != ctx.tenant_id {
            return Err(DomainError::Unauthorized("Template belongs to another organization".into()));
        }

        template.status = TemplateStatus::Published;
        template.published_version = Some(template.version);
        template.published_at = Some(Utc::now());
        template.published_by = Some(ctx.actor.id().unwrap_or_default());
        template.updated_at = Utc::now();

        self.template_repo.save(&template).await?;
        self.template_repo
            .update_status(template.id, &TemplateStatus::Published, Some(ctx.actor.id().unwrap_or_default()))
            .await?;

        self.audit(ctx, "template.published", "report_template", template.id).await;
        Ok(template)
    }

    pub async fn archive(
        &self,
        ctx: &RequestContext,
        template_id: Uuid,
    ) -> Result<(), DomainError> {
        let template = self.template_repo
            .find_by_id(template_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Template not found".into()))?;

        if template.org_id != ctx.tenant_id {
            return Err(DomainError::Unauthorized("Template belongs to another organization".into()));
        }

        self.template_repo
            .update_status(template.id, &TemplateStatus::Archived, None)
            .await?;

        self.audit(ctx, "template.archived", "report_template", template.id).await;
        Ok(())
    }

    pub async fn revert_to_version(
        &self,
        ctx: &RequestContext,
        template_id: Uuid,
        target_version: i32,
    ) -> Result<ReportTemplate, DomainError> {
        let mut template = self.template_repo
            .find_by_id(template_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Template not found".into()))?;

        if template.org_id != ctx.tenant_id {
            return Err(DomainError::Unauthorized("Template belongs to another organization".into()));
        }

        let version = self.template_repo
            .find_version(template_id, target_version)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Version {} not found", target_version)))?;

        // Save current as a version snapshot first
        let snapshot = ReportTemplateVersion {
            id: Uuid::new_v4(),
            template_id: template.id,
            version: template.version,
            document_json: template.document_json.clone(),
            page_size: Some(template.page_size.clone()),
            page_width_mm: template.page_width_mm,
            page_height_mm: template.page_height_mm,
            orientation: Some(template.orientation.clone()),
            margins_json: Some(template.margins_json.clone()),
            change_summary: Some(format!("Before revert to v{}", target_version)),
            created_by: ctx.actor.id().unwrap_or_default(),
            created_at: Utc::now(),
        };
        self.template_repo.save_version(&snapshot).await?;

        // Apply the old version content
        template.document_json = version.document_json;
        if let Some(ps) = version.page_size { template.page_size = ps; }
        if let Some(o) = version.orientation { template.orientation = o; }
        if let Some(m) = version.margins_json { template.margins_json = m; }
        template.page_width_mm = version.page_width_mm;
        template.page_height_mm = version.page_height_mm;
        template.version += 1;
        template.updated_at = Utc::now();

        self.template_repo.save(&template).await?;

        self.audit(ctx, "template.reverted", "report_template", template.id).await;
        Ok(template)
    }

    pub async fn get_versions(
        &self,
        ctx: &RequestContext,
        template_id: Uuid,
    ) -> Result<Vec<ReportTemplateVersion>, DomainError> {
        let template = self.template_repo
            .find_by_id(template_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Template not found".into()))?;

        if template.org_id != ctx.tenant_id {
            return Err(DomainError::Unauthorized("Template belongs to another organization".into()));
        }

        self.template_repo.find_versions(template_id).await
    }

    async fn audit(&self, ctx: &RequestContext, action: &str, entity_type: &str, entity_id: Uuid) {
        let entry = ReportAuditEntry {
            id: Uuid::new_v4(),
            org_id: ctx.tenant_id,
            actor_id: ctx.actor.id().unwrap_or_default(),
            action: action.into(),
            entity_type: entity_type.into(),
            entity_id,
            details: None,
            created_at: Utc::now(),
        };
        self.audit_repo.save(&entry).await.ok();
    }
}
