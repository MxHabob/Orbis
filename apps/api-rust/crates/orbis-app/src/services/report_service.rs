use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use orbis_domain::entities::report::{
    ReportDefinition, ReportTemplate, ReportJob, OutputFormat, ReportLocale,
    ParamValue, ParameterDef, ParamType, JobStatus, OrgReportContext,
    ReportAuditEntry, ExportResult, TemplateStatus,
};
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::reporting::{
    ReportDefinitionRepository, ReportTemplateRepository, ReportJobRepository,
    ReportAuditRepository, DataProviderRegistry, RendererRegistry,
};

/// Port for resolving organization branding context.
#[async_trait::async_trait]
pub trait OrgContextResolver: Send + Sync {
    async fn resolve(&self, org_id: Uuid) -> Result<OrgReportContext, DomainError>;
}

#[derive(Debug, Clone)]
pub struct GenerateReportInput {
    pub report_code: String,
    pub parameters: HashMap<String, ParamValue>,
    pub output_format: OutputFormat,
    pub locale: ReportLocale,
}

pub struct ReportService {
    pub definition_repo: Arc<dyn ReportDefinitionRepository>,
    pub template_repo: Arc<dyn ReportTemplateRepository>,
    pub job_repo: Arc<dyn ReportJobRepository>,
    pub audit_repo: Arc<dyn ReportAuditRepository>,
    pub provider_registry: Arc<dyn DataProviderRegistry>,
    pub renderer_registry: Arc<dyn RendererRegistry>,
    pub org_context_resolver: Arc<dyn OrgContextResolver>,
}

impl ReportService {
    pub fn new(
        definition_repo: Arc<dyn ReportDefinitionRepository>,
        template_repo: Arc<dyn ReportTemplateRepository>,
        job_repo: Arc<dyn ReportJobRepository>,
        audit_repo: Arc<dyn ReportAuditRepository>,
        provider_registry: Arc<dyn DataProviderRegistry>,
        renderer_registry: Arc<dyn RendererRegistry>,
        org_context_resolver: Arc<dyn OrgContextResolver>,
    ) -> Self {
        Self {
            definition_repo,
            template_repo,
            job_repo,
            audit_repo,
            provider_registry,
            renderer_registry,
            org_context_resolver,
        }
    }

    /// List active report definitions for the tenant organization
    pub async fn list_reports(
        &self,
        ctx: &RequestContext,
        domain: Option<&str>,
        category: Option<&str>,
    ) -> Result<Vec<ReportDefinition>, DomainError> {
        let definitions = self
            .definition_repo
            .list_by_org(ctx.tenant_id, domain, category)
            .await?;

        let active: Vec<ReportDefinition> = definitions
            .into_iter()
            .filter(|d| d.is_active)
            .collect();

        Ok(active)
    }

    /// Get report definition by code
    pub async fn get_report_by_code(
        &self,
        org_id: Uuid,
        code: &str,
    ) -> Result<Option<ReportDefinition>, DomainError> {
        self.definition_repo.find_by_code(org_id, code).await
    }

    /// Generate report pipeline
    pub async fn generate_report(
        &self,
        ctx: &RequestContext,
        input: GenerateReportInput,
    ) -> Result<(ReportJob, ExportResult), DomainError> {
        // 1. Look up definition
        let definition = self
            .definition_repo
            .find_by_code(ctx.tenant_id, &input.report_code)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Report definition not found: {}", input.report_code)))?;

        if !definition.is_active {
            return Err(DomainError::Validation("Report definition is inactive".into()));
        }

        // 2. Check output format is supported
        if !definition.supported_formats.contains(&input.output_format) {
            return Err(DomainError::Validation(format!(
                "Output format '{}' is not supported for report '{}'",
                input.output_format, input.report_code
            )));
        }

        // 3. Validate parameters against schema
        validate_parameters(&input.parameters, &definition.parameters)?;

        // 4. Create job record
        let mut job = ReportJob {
            id: Uuid::new_v4(),
            org_id: ctx.tenant_id,
            report_definition_id: definition.id,
            template_id: None,
            requested_by: ctx.actor.id().unwrap_or_default(),
            report_code: input.report_code.clone(),
            parameters: serde_json::to_value(&input.parameters).unwrap_or_default(),
            output_format: input.output_format.clone(),
            locale: input.locale.clone(),
            status: JobStatus::Pending,
            output_path: None,
            output_size_bytes: None,
            error_message: None,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
        };
        self.job_repo.save(&job).await?;

        // 5. Update to processing
        job.status = JobStatus::Processing;
        job.started_at = Some(Utc::now());
        self.job_repo.update_status(job.id, &JobStatus::Processing, None, None, None).await?;

        let start_time = Utc::now();

        // 6. Resolve org branding context
        let org_context = self.org_context_resolver.resolve(ctx.tenant_id).await?;

        // 7. Fetch data from data provider
        let dataset = match self.provider_registry.get_provider(&definition.data_provider_key) {
            Some(provider) => {
                match provider.fetch_data(ctx.tenant_id, &org_context, &input.parameters, &input.locale).await {
                    Ok(ds) => ds,
                    Err(e) => {
                        let err_msg = format!("Data provider failed: {}", e);
                        self.job_repo.update_status(job.id, &JobStatus::Failed, None, None, Some(err_msg.clone())).await?;
                        job.status = JobStatus::Failed;
                        job.error_message = Some(err_msg);
                        return Err(e);
                    }
                }
            }
            None => {
                let err_msg = format!("Data provider not registered: {}", definition.data_provider_key);
                self.job_repo.update_status(job.id, &JobStatus::Failed, None, None, Some(err_msg.clone())).await?;
                job.status = JobStatus::Failed;
                job.error_message = Some(err_msg.clone());
                return Err(DomainError::Internal(err_msg));
            }
        };

        // 8. Resolve template (if binding exists)
        let binding = self
            .template_repo
            .find_binding(definition.id, input.locale.primary_lang(), &input.output_format)
            .await?;

        let template = if let Some(b) = binding {
            self.template_repo.find_by_id(b.template_id).await?
        } else {
            None
        };

        if let Some(ref t) = template {
            job.template_id = Some(t.id);
        }

        // 9. Render output
        let renderer = self
            .renderer_registry
            .get_renderer(&input.output_format)
            .ok_or_else(|| {
                DomainError::Internal(format!("No renderer registered for format: {}", input.output_format))
            })?;

        let export_result = match renderer.render(
            &dataset,
            template.as_ref(),
            None,
        ).await {
            Ok(res) => res,
            Err(e) => {
                let err_msg = format!("Renderer failed: {}", e);
                self.job_repo.update_status(job.id, &JobStatus::Failed, None, None, Some(err_msg.clone())).await?;
                job.status = JobStatus::Failed;
                job.error_message = Some(err_msg);
                return Err(e);
            }
        };

        // 10. Mark job completed
        let duration_ms = (Utc::now() - start_time).num_milliseconds();
        let size_bytes = export_result.data.len() as i64;

        job.status = JobStatus::Completed;
        job.output_path = Some(export_result.filename.clone());
        job.output_size_bytes = Some(size_bytes);
        job.completed_at = Some(Utc::now());

        self.job_repo
            .update_status(
                job.id,
                &JobStatus::Completed,
                Some(export_result.filename.clone()),
                Some(size_bytes),
                None,
            )
            .await?;

        // 11. Audit log
        let audit = ReportAuditEntry {
            id: Uuid::new_v4(),
            org_id: ctx.tenant_id,
            actor_id: ctx.actor.id().unwrap_or_default(),
            action: "report.generated".into(),
            entity_type: "report_job".into(),
            entity_id: job.id,
            details: Some(serde_json::json!({
                "report_code": input.report_code,
                "format": input.output_format.to_string(),
                "record_count": dataset.rows.len(),
                "duration_ms": duration_ms,
            })),
            created_at: Utc::now(),
        };
        let _ = self.audit_repo.save(&audit).await;

        Ok((job, export_result))
    }

    /// List templates for organization
    pub async fn list_templates(&self, org_id: Uuid) -> Result<Vec<ReportTemplate>, DomainError> {
        self.template_repo.list_by_org(org_id, None, None, None).await
    }

    /// Save or update report template
    pub async fn save_template(&self, template: &ReportTemplate) -> Result<(), DomainError> {
        self.template_repo.save(template).await
    }

    /// Get template by ID
    pub async fn get_template(&self, id: Uuid) -> Result<Option<ReportTemplate>, DomainError> {
        self.template_repo.find_by_id(id).await
    }

    /// Delete (soft delete) a template
    pub async fn delete_template(&self, ctx: &RequestContext, id: Uuid) -> Result<(), DomainError> {
        let template = self.template_repo.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound("Template not found".into()))?;

        if template.org_id != ctx.tenant_id {
            return Err(DomainError::Forbidden("Template belongs to another organization".into()));
        }

        self.template_repo.soft_delete(id).await
    }

    /// Publish a template
    pub async fn publish_template(&self, ctx: &RequestContext, id: Uuid) -> Result<ReportTemplate, DomainError> {
        let mut template = self.template_repo.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound("Template not found".into()))?;

        if template.org_id != ctx.tenant_id {
            return Err(DomainError::Forbidden("Template belongs to another organization".into()));
        }

        template.status = TemplateStatus::Published;
        template.published_version = Some(template.version);
        template.published_at = Some(Utc::now());
        template.published_by = ctx.actor.id();
        template.updated_at = Utc::now();

        self.template_repo.save(&template).await?;
        Ok(template)
    }

    /// Archive a template
    pub async fn archive_template(&self, ctx: &RequestContext, id: Uuid) -> Result<ReportTemplate, DomainError> {
        let mut template = self.template_repo.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound("Template not found".into()))?;

        if template.org_id != ctx.tenant_id {
            return Err(DomainError::Forbidden("Template belongs to another organization".into()));
        }

        template.status = TemplateStatus::Archived;
        template.updated_at = Utc::now();

        self.template_repo.save(&template).await?;
        Ok(template)
    }

    /// Revert template to a previous version
    pub async fn revert_template_version(&self, ctx: &RequestContext, id: Uuid, version: i32) -> Result<ReportTemplate, DomainError> {
        let mut template = self.template_repo.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound("Template not found".into()))?;

        if template.org_id != ctx.tenant_id {
            return Err(DomainError::Forbidden("Template belongs to another organization".into()));
        }

        let ver = self.template_repo.find_version(id, version).await?
            .ok_or_else(|| DomainError::NotFound(format!("Version {} not found", version)))?;

        template.document_json = ver.document_json;
        if let Some(ps) = ver.page_size { template.page_size = ps; }
        if let Some(ori) = ver.orientation { template.orientation = ori; }
        if let Some(mar) = ver.margins_json { template.margins_json = mar; }
        template.version += 1;
        template.updated_at = Utc::now();

        self.template_repo.save(&template).await?;
        Ok(template)
    }
}

fn validate_parameters(
    provided: &HashMap<String, ParamValue>,
    defs: &[ParameterDef],
) -> Result<(), DomainError> {
    for def in defs {
        match provided.get(&def.key) {
            Some(val) => {
                validate_param_type(val, &def.param_type, &def.key)?;
            }
            None => {
                if def.required && def.default_value.is_none() {
                    return Err(DomainError::Validation(format!(
                        "Missing required parameter: {}",
                        def.key
                    )));
                }
            }
        }
    }
    Ok(())
}

fn validate_param_type(
    val: &ParamValue,
    expected_type: &ParamType,
    param_name: &str,
) -> Result<(), DomainError> {
    let matches = match (val, expected_type) {
        (ParamValue::Text(_), ParamType::Text) => true,
        (ParamValue::Integer(_), ParamType::Integer) => true,
        (ParamValue::Decimal(_), ParamType::Decimal) => true,
        (ParamValue::Boolean(_), ParamType::Boolean) => true,
        (ParamValue::Date(_), ParamType::Date) => true,
        (ParamValue::DateRange { .. }, ParamType::DateRange) => true,
        (ParamValue::Id(_), ParamType::Id) => true,
        (ParamValue::IdList(_), ParamType::IdList) => true,
        (ParamValue::Enum(_), ParamType::Enum) => true,
        (ParamValue::Period(_), ParamType::Period) => true,
        // Also allow Integer as Decimal
        (ParamValue::Integer(_), ParamType::Decimal) => true,
        // Also allow Text for Id / Period / Enum
        (ParamValue::Text(_), ParamType::Id | ParamType::Period | ParamType::Enum) => true,
        _ => false,
    };

    if !matches {
        return Err(DomainError::Validation(format!(
            "Parameter '{}' has invalid type, expected {:?}",
            param_name, expected_type
        )));
    }
    Ok(())
}
