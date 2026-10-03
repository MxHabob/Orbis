//! Report repository implementations — parameterized queries only.

use async_trait::async_trait;
use orbis_app::ports::reporting::*;
use orbis_domain::entities::report::*;
use orbis_domain::errors::DomainError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use uuid::Uuid;

use crate::models::report::{
    report_audit_log, report_definition, report_job, report_template, report_template_binding,
    report_template_version,
};

fn db_err(e: sea_orm::DbErr) -> DomainError {
    DomainError::Internal(e.to_string())
}

// ═══════════════════════════════════════════════════════════════
// ReportDefinitionRepository
// ═══════════════════════════════════════════════════════════════

pub struct ReportDefinitionRepoImpl {
    pub db: DatabaseConnection,
}

fn model_to_definition(m: report_definition::Model) -> ReportDefinition {
    ReportDefinition {
        id: m.id,
        org_id: m.org_id,
        code: m.code,
        name: m.name,
        name_ar: m.name_ar,
        domain: ReportDomain::from_str(&m.domain),
        category: m.category,
        description: m.description,
        description_ar: m.description_ar,
        parameters: serde_json::from_str(&m.parameters_json).unwrap_or_default(),
        required_permissions: serde_json::from_str(&m.required_permissions).unwrap_or_default(),
        supported_formats: serde_json::from_str(&m.supported_formats).unwrap_or_default(),
        data_provider_key: m.data_provider_key,
        is_system: m.is_system,
        is_active: m.is_active,
        sort_order: m.sort_order,
        version: m.version,
        created_at: m.created_at.into(),
        updated_at: m.updated_at.into(),
    }
}

#[async_trait]
impl ReportDefinitionRepository for ReportDefinitionRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ReportDefinition>, DomainError> {
        let model = report_definition::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(db_err)?;
        Ok(model.map(model_to_definition))
    }

    async fn find_by_code(
        &self,
        org_id: Uuid,
        code: &str,
    ) -> Result<Option<ReportDefinition>, DomainError> {
        let model = report_definition::Entity::find()
            .filter(report_definition::Column::OrgId.eq(org_id))
            .filter(report_definition::Column::Code.eq(code))
            .one(&self.db)
            .await
            .map_err(db_err)?;
        Ok(model.map(model_to_definition))
    }

    async fn list_by_org(
        &self,
        org_id: Uuid,
        domain: Option<&str>,
        category: Option<&str>,
    ) -> Result<Vec<ReportDefinition>, DomainError> {
        let mut query =
            report_definition::Entity::find().filter(report_definition::Column::OrgId.eq(org_id));

        if let Some(d) = domain {
            query = query.filter(report_definition::Column::Domain.eq(d));
        }
        if let Some(c) = category {
            query = query.filter(report_definition::Column::Category.eq(c));
        }

        let models = query
            .order_by_asc(report_definition::Column::SortOrder)
            .all(&self.db)
            .await
            .map_err(db_err)?;
        Ok(models.into_iter().map(model_to_definition).collect())
    }

    async fn save(&self, def: &ReportDefinition) -> Result<(), DomainError> {
        let active = report_definition::ActiveModel {
            id: Set(def.id),
            org_id: Set(def.org_id),
            code: Set(def.code.clone()),
            name: Set(def.name.clone()),
            name_ar: Set(def.name_ar.clone()),
            domain: Set(def.domain.to_string()),
            category: Set(def.category.clone()),
            description: Set(def.description.clone()),
            description_ar: Set(def.description_ar.clone()),
            parameters_json: Set(
                serde_json::to_string(&def.parameters).unwrap_or_else(|_| "[]".into())
            ),
            required_permissions: Set(
                serde_json::to_string(&def.required_permissions).unwrap_or_else(|_| "[]".into())
            ),
            supported_formats: Set(
                serde_json::to_string(&def.supported_formats).unwrap_or_else(|_| "[]".into())
            ),
            data_provider_key: Set(def.data_provider_key.clone()),
            is_system: Set(def.is_system),
            is_active: Set(def.is_active),
            sort_order: Set(def.sort_order),
            version: Set(def.version),
            created_at: Set(def.created_at.into()),
            updated_at: Set(def.updated_at.into()),
        };

        if report_definition::Entity::find_by_id(def.id)
            .one(&self.db)
            .await
            .map_err(db_err)?
            .is_some()
        {
            active.update(&self.db).await.map_err(db_err)?;
        } else {
            active.insert(&self.db).await.map_err(db_err)?;
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        report_definition::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(db_err)?;
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════
// ReportTemplateRepository
// ═══════════════════════════════════════════════════════════════

pub struct ReportTemplateRepoImpl {
    pub db: DatabaseConnection,
}

fn model_to_template(m: report_template::Model) -> ReportTemplate {
    ReportTemplate {
        id: m.id,
        org_id: m.org_id,
        code: m.code,
        name: m.name,
        name_ar: m.name_ar,
        description: m.description,
        description_ar: m.description_ar,
        document_type: m.document_type,
        entity_type: m.entity_type,
        master_template_id: m.master_template_id,
        status: TemplateStatus::from_str(&m.status),
        version: m.version,
        published_version: m.published_version,
        published_at: m.published_at.map(|d| d.into()),
        published_by: m.published_by,
        page_size: m.page_size,
        page_width_mm: m.page_width_mm,
        page_height_mm: m.page_height_mm,
        orientation: m.orientation,
        margins_json: m.margins_json,
        document_json: m.document_json,
        locale: m.locale,
        tags: m.tags,
        is_system: m.is_system,
        is_default: m.is_default,
        created_by: m.created_by,
        deleted_at: m.deleted_at.map(|d| d.into()),
        created_at: m.created_at.into(),
        updated_at: m.updated_at.into(),
    }
}

#[async_trait]
impl ReportTemplateRepository for ReportTemplateRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ReportTemplate>, DomainError> {
        let model = report_template::Entity::find_by_id(id)
            .filter(report_template::Column::DeletedAt.is_null())
            .one(&self.db)
            .await
            .map_err(db_err)?;
        Ok(model.map(model_to_template))
    }

    async fn list_by_org(
        &self,
        org_id: Uuid,
        status: Option<&str>,
        document_type: Option<&str>,
        entity_type: Option<&str>,
    ) -> Result<Vec<ReportTemplate>, DomainError> {
        let mut query = report_template::Entity::find()
            .filter(report_template::Column::OrgId.eq(org_id))
            .filter(report_template::Column::DeletedAt.is_null());

        if let Some(s) = status {
            query = query.filter(report_template::Column::Status.eq(s));
        }
        if let Some(dt) = document_type {
            query = query.filter(report_template::Column::DocumentType.eq(dt));
        }
        if let Some(et) = entity_type {
            query = query.filter(report_template::Column::EntityType.eq(et));
        }

        let models = query
            .order_by_desc(report_template::Column::UpdatedAt)
            .all(&self.db)
            .await
            .map_err(db_err)?;
        Ok(models.into_iter().map(model_to_template).collect())
    }

    async fn save(&self, tmpl: &ReportTemplate) -> Result<(), DomainError> {
        let active = report_template::ActiveModel {
            id: Set(tmpl.id),
            org_id: Set(tmpl.org_id),
            code: Set(tmpl.code.clone()),
            name: Set(tmpl.name.clone()),
            name_ar: Set(tmpl.name_ar.clone()),
            description: Set(tmpl.description.clone()),
            description_ar: Set(tmpl.description_ar.clone()),
            document_type: Set(tmpl.document_type.clone()),
            entity_type: Set(tmpl.entity_type.clone()),
            master_template_id: Set(tmpl.master_template_id),
            status: Set(tmpl.status.to_string()),
            version: Set(tmpl.version),
            published_version: Set(tmpl.published_version),
            published_at: Set(tmpl.published_at.map(|d| d.into())),
            published_by: Set(tmpl.published_by),
            page_size: Set(tmpl.page_size.clone()),
            page_width_mm: Set(tmpl.page_width_mm),
            page_height_mm: Set(tmpl.page_height_mm),
            orientation: Set(tmpl.orientation.clone()),
            margins_json: Set(tmpl.margins_json.clone()),
            document_json: Set(tmpl.document_json.clone()),
            locale: Set(tmpl.locale.clone()),
            tags: Set(tmpl.tags.clone()),
            is_system: Set(tmpl.is_system),
            is_default: Set(tmpl.is_default),
            created_by: Set(tmpl.created_by),
            deleted_at: Set(tmpl.deleted_at.map(|d| d.into())),
            created_at: Set(tmpl.created_at.into()),
            updated_at: Set(tmpl.updated_at.into()),
        };

        if report_template::Entity::find_by_id(tmpl.id)
            .one(&self.db)
            .await
            .map_err(db_err)?
            .is_some()
        {
            active.update(&self.db).await.map_err(db_err)?;
        } else {
            active.insert(&self.db).await.map_err(db_err)?;
        }
        Ok(())
    }

    async fn update_status(
        &self,
        id: Uuid,
        status: &TemplateStatus,
        published_by: Option<Uuid>,
    ) -> Result<(), DomainError> {
        if let Some(model) = report_template::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(db_err)?
        {
            let mut active: report_template::ActiveModel = model.into();
            active.status = Set(status.to_string());
            if *status == TemplateStatus::Published {
                active.published_at = Set(Some(chrono::Utc::now().into()));
                if let Some(by) = published_by {
                    active.published_by = Set(Some(by));
                }
            }
            active.updated_at = Set(chrono::Utc::now().into());
            active.update(&self.db).await.map_err(db_err)?;
        }
        Ok(())
    }

    async fn soft_delete(&self, id: Uuid) -> Result<(), DomainError> {
        if let Some(model) = report_template::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(db_err)?
        {
            let mut active: report_template::ActiveModel = model.into();
            active.deleted_at = Set(Some(chrono::Utc::now().into()));
            active.update(&self.db).await.map_err(db_err)?;
        }
        Ok(())
    }

    async fn save_version(&self, ver: &ReportTemplateVersion) -> Result<(), DomainError> {
        let active = report_template_version::ActiveModel {
            id: Set(ver.id),
            template_id: Set(ver.template_id),
            version: Set(ver.version),
            document_json: Set(ver.document_json.clone()),
            page_size: Set(ver.page_size.clone()),
            page_width_mm: Set(ver.page_width_mm),
            page_height_mm: Set(ver.page_height_mm),
            orientation: Set(ver.orientation.clone()),
            margins_json: Set(ver.margins_json.clone()),
            change_summary: Set(ver.change_summary.clone()),
            created_by: Set(ver.created_by),
            created_at: Set(ver.created_at.into()),
        };
        active.insert(&self.db).await.map_err(db_err)?;
        Ok(())
    }

    async fn find_versions(
        &self,
        template_id: Uuid,
    ) -> Result<Vec<ReportTemplateVersion>, DomainError> {
        let models = report_template_version::Entity::find()
            .filter(report_template_version::Column::TemplateId.eq(template_id))
            .order_by_desc(report_template_version::Column::Version)
            .all(&self.db)
            .await
            .map_err(db_err)?;

        Ok(models
            .into_iter()
            .map(|m| ReportTemplateVersion {
                id: m.id,
                template_id: m.template_id,
                version: m.version,
                document_json: m.document_json,
                page_size: m.page_size,
                page_width_mm: m.page_width_mm,
                page_height_mm: m.page_height_mm,
                orientation: m.orientation,
                margins_json: m.margins_json,
                change_summary: m.change_summary,
                created_by: m.created_by,
                created_at: m.created_at.into(),
            })
            .collect())
    }

    async fn find_version(
        &self,
        template_id: Uuid,
        version: i32,
    ) -> Result<Option<ReportTemplateVersion>, DomainError> {
        let model = report_template_version::Entity::find()
            .filter(report_template_version::Column::TemplateId.eq(template_id))
            .filter(report_template_version::Column::Version.eq(version))
            .one(&self.db)
            .await
            .map_err(db_err)?;

        Ok(model.map(|m| ReportTemplateVersion {
            id: m.id,
            template_id: m.template_id,
            version: m.version,
            document_json: m.document_json,
            page_size: m.page_size,
            page_width_mm: m.page_width_mm,
            page_height_mm: m.page_height_mm,
            orientation: m.orientation,
            margins_json: m.margins_json,
            change_summary: m.change_summary,
            created_by: m.created_by,
            created_at: m.created_at.into(),
        }))
    }

    async fn find_binding(
        &self,
        definition_id: Uuid,
        locale: &str,
        format: &OutputFormat,
    ) -> Result<Option<ReportTemplateBinding>, DomainError> {
        let model = report_template_binding::Entity::find()
            .filter(report_template_binding::Column::ReportDefinitionId.eq(definition_id))
            .filter(report_template_binding::Column::Locale.eq(locale))
            .filter(report_template_binding::Column::Format.eq(format.to_string()))
            .one(&self.db)
            .await
            .map_err(db_err)?;

        Ok(model.map(|m| ReportTemplateBinding {
            id: m.id,
            report_definition_id: m.report_definition_id,
            template_id: m.template_id,
            locale: m.locale,
            format: OutputFormat::from_str(&m.format),
            is_default: m.is_default,
            created_at: m.created_at.into(),
        }))
    }

    async fn save_binding(&self, binding: &ReportTemplateBinding) -> Result<(), DomainError> {
        let active = report_template_binding::ActiveModel {
            id: Set(binding.id),
            report_definition_id: Set(binding.report_definition_id),
            template_id: Set(binding.template_id),
            locale: Set(binding.locale.clone()),
            format: Set(binding.format.to_string()),
            is_default: Set(binding.is_default),
            created_at: Set(binding.created_at.into()),
        };
        active.insert(&self.db).await.map_err(db_err)?;
        Ok(())
    }

    async fn find_bindings_by_definition(
        &self,
        definition_id: Uuid,
    ) -> Result<Vec<ReportTemplateBinding>, DomainError> {
        let models = report_template_binding::Entity::find()
            .filter(report_template_binding::Column::ReportDefinitionId.eq(definition_id))
            .all(&self.db)
            .await
            .map_err(db_err)?;

        Ok(models
            .into_iter()
            .map(|m| ReportTemplateBinding {
                id: m.id,
                report_definition_id: m.report_definition_id,
                template_id: m.template_id,
                locale: m.locale,
                format: OutputFormat::from_str(&m.format),
                is_default: m.is_default,
                created_at: m.created_at.into(),
            })
            .collect())
    }
}

// ═══════════════════════════════════════════════════════════════
// ReportJobRepository
// ═══════════════════════════════════════════════════════════════

pub struct ReportJobRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ReportJobRepository for ReportJobRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ReportJob>, DomainError> {
        let model = report_job::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(db_err)?;
        Ok(model.map(|m| ReportJob {
            id: m.id,
            org_id: m.org_id,
            report_definition_id: m.report_definition_id,
            template_id: m.template_id,
            requested_by: m.requested_by,
            report_code: m.report_code,
            parameters: m.parameters,
            output_format: OutputFormat::from_str(&m.output_format),
            locale: ReportLocale::from_str(&m.locale),
            status: JobStatus::from_str(&m.status),
            output_path: m.output_path,
            output_size_bytes: m.output_size_bytes,
            error_message: m.error_message,
            created_at: m.created_at.into(),
            started_at: m.started_at.map(|d| d.into()),
            completed_at: m.completed_at.map(|d| d.into()),
        }))
    }

    async fn save(&self, job: &ReportJob) -> Result<(), DomainError> {
        let active = report_job::ActiveModel {
            id: Set(job.id),
            org_id: Set(job.org_id),
            report_definition_id: Set(job.report_definition_id),
            template_id: Set(job.template_id),
            requested_by: Set(job.requested_by),
            report_code: Set(job.report_code.clone()),
            parameters: Set(job.parameters.clone()),
            output_format: Set(job.output_format.to_string()),
            locale: Set(job.locale.primary_lang().to_string()),
            status: Set(job.status.to_string()),
            output_path: Set(job.output_path.clone()),
            output_size_bytes: Set(job.output_size_bytes),
            error_message: Set(job.error_message.clone()),
            created_at: Set(job.created_at.into()),
            started_at: Set(job.started_at.map(|d| d.into())),
            completed_at: Set(job.completed_at.map(|d| d.into())),
        };

        if report_job::Entity::find_by_id(job.id)
            .one(&self.db)
            .await
            .map_err(db_err)?
            .is_some()
        {
            active.update(&self.db).await.map_err(db_err)?;
        } else {
            active.insert(&self.db).await.map_err(db_err)?;
        }
        Ok(())
    }

    async fn update_status(
        &self,
        job_id: Uuid,
        status: &JobStatus,
        output_path: Option<String>,
        output_size: Option<i64>,
        error: Option<String>,
    ) -> Result<(), DomainError> {
        if let Some(model) = report_job::Entity::find_by_id(job_id)
            .one(&self.db)
            .await
            .map_err(db_err)?
        {
            let mut active: report_job::ActiveModel = model.into();
            active.status = Set(status.to_string());
            if let Some(p) = output_path {
                active.output_path = Set(Some(p));
            }
            if let Some(s) = output_size {
                active.output_size_bytes = Set(Some(s));
            }
            if let Some(e) = error {
                active.error_message = Set(Some(e));
            }
            if *status == JobStatus::Processing {
                active.started_at = Set(Some(chrono::Utc::now().into()));
            }
            if *status == JobStatus::Completed || *status == JobStatus::Failed {
                active.completed_at = Set(Some(chrono::Utc::now().into()));
            }
            active.update(&self.db).await.map_err(db_err)?;
        }
        Ok(())
    }

    async fn list_by_org(&self, org_id: Uuid, limit: u32) -> Result<Vec<ReportJob>, DomainError> {
        use sea_orm::QuerySelect;
        let models = report_job::Entity::find()
            .filter(report_job::Column::OrgId.eq(org_id))
            .order_by_desc(report_job::Column::CreatedAt)
            .limit(limit as u64)
            .all(&self.db)
            .await
            .map_err(db_err)?;

        Ok(models
            .into_iter()
            .map(|m| ReportJob {
                id: m.id,
                org_id: m.org_id,
                report_definition_id: m.report_definition_id,
                template_id: m.template_id,
                requested_by: m.requested_by,
                report_code: m.report_code,
                parameters: m.parameters,
                output_format: OutputFormat::from_str(&m.output_format),
                locale: ReportLocale::from_str(&m.locale),
                status: JobStatus::from_str(&m.status),
                output_path: m.output_path,
                output_size_bytes: m.output_size_bytes,
                error_message: m.error_message,
                created_at: m.created_at.into(),
                started_at: m.started_at.map(|d| d.into()),
                completed_at: m.completed_at.map(|d| d.into()),
            })
            .collect())
    }
}

// ═══════════════════════════════════════════════════════════════
// ReportAuditRepository
// ═══════════════════════════════════════════════════════════════

pub struct ReportAuditRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ReportAuditRepository for ReportAuditRepoImpl {
    async fn save(&self, entry: &ReportAuditEntry) -> Result<(), DomainError> {
        let active = report_audit_log::ActiveModel {
            id: Set(entry.id),
            org_id: Set(entry.org_id),
            actor_id: Set(entry.actor_id),
            action: Set(entry.action.clone()),
            entity_type: Set(entry.entity_type.clone()),
            entity_id: Set(entry.entity_id),
            details: Set(entry.details.clone()),
            created_at: Set(entry.created_at.into()),
        };
        active.insert(&self.db).await.map_err(db_err)?;
        Ok(())
    }

    async fn list_by_org(
        &self,
        org_id: Uuid,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<Vec<ReportAuditEntry>, DomainError> {
        let mut query = report_audit_log::Entity::find()
            .filter(report_audit_log::Column::OrgId.eq(org_id))
            .order_by_desc(report_audit_log::Column::CreatedAt);

        if let Some(l) = limit {
            query = query.limit(l);
        }
        if let Some(o) = offset {
            query = query.offset(o);
        }

        let models = query.all(&self.db).await.map_err(db_err)?;
        Ok(models
            .into_iter()
            .map(|m| ReportAuditEntry {
                id: m.id,
                org_id: m.org_id,
                actor_id: m.actor_id,
                action: m.action,
                entity_type: m.entity_type,
                entity_id: m.entity_id,
                details: m.details,
                created_at: m.created_at.into(),
            })
            .collect())
    }
}
