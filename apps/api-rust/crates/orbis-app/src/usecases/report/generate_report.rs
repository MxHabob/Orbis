//! GenerateReport use case — the main report generation pipeline.
//!
//! 1. Look up ReportDefinition by code
//! 2. Validate parameters against schema
//! 3. Create ReportJob (Pending)
//! 4. Resolve data provider by key
//! 5. Fetch data → ReportDataset
//! 6. Resolve template binding (definition → template for locale/format)
//! 7. Resolve renderer for output format
//! 8. Render → ExportResult
//! 9. Update job status (Completed/Failed)
//! 10. Record audit entry

use crate::context::RequestContext;
use crate::ports::reporting::*;
use orbis_domain::entities::report::*;
use orbis_domain::errors::DomainError;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct GenerateReportUseCase {
    pub definition_repo: Arc<dyn ReportDefinitionRepository>,
    pub template_repo: Arc<dyn ReportTemplateRepository>,
    pub job_repo: Arc<dyn ReportJobRepository>,
    pub audit_repo: Arc<dyn ReportAuditRepository>,
    pub provider_registry: Arc<dyn DataProviderRegistry>,
    pub renderer_registry: Arc<dyn RendererRegistry>,
    pub org_context_resolver: Arc<dyn OrgContextResolver>,
}

/// Port for resolving organization branding context.
#[async_trait::async_trait]
pub trait OrgContextResolver: Send + Sync {
    async fn resolve(&self, org_id: Uuid) -> Result<OrgReportContext, DomainError>;
}

/// Input for the generate report use case.
#[derive(Debug, Clone)]
pub struct GenerateReportInput {
    pub report_code: String,
    pub parameters: HashMap<String, ParamValue>,
    pub output_format: OutputFormat,
    pub locale: ReportLocale,
}

impl GenerateReportUseCase {
    pub async fn execute(
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
        self.job_repo
            .update_status(job.id, &JobStatus::Processing, None, None, None)
            .await?;

        // 6. Resolve org context for branding
        let org_context = self.org_context_resolver.resolve(ctx.tenant_id).await?;

        // 7. Resolve data provider and fetch data
        let provider = self
            .provider_registry
            .get_provider(&definition.data_provider_key)
            .ok_or_else(|| {
                DomainError::Internal(format!(
                    "No data provider registered for key: {}",
                    definition.data_provider_key
                ))
            })?;

        let dataset = match provider
            .fetch_data(ctx.tenant_id, &org_context, &input.parameters, &input.locale)
            .await
        {
            Ok(ds) => ds,
            Err(e) => {
                self.job_repo
                    .update_status(job.id, &JobStatus::Failed, None, None, Some(e.to_string()))
                    .await
                    .ok();
                return Err(e);
            }
        };

        // 8. Resolve template (optional, depends on output format)
        let template = if input.output_format != OutputFormat::DataOnly
            && input.output_format != OutputFormat::Json
            && input.output_format != OutputFormat::Csv
        {
            if let Ok(Some(binding)) = self
                .template_repo
                .find_binding(definition.id, input.locale.primary_lang(), &input.output_format)
                .await
            {
                job.template_id = Some(binding.template_id);
                self.template_repo.find_by_id(binding.template_id).await?
            } else {
                None
            }
        } else {
            None
        };

        // 8b. Resolve master template if template references one
        let master_template = if let Some(ref tmpl) = template {
            if let Some(master_id) = tmpl.master_template_id {
                self.template_repo.find_by_id(master_id).await?
            } else {
                None
            }
        } else {
            None
        };

        // 9. Resolve renderer and render
        let renderer = self
            .renderer_registry
            .get_renderer(&input.output_format)
            .ok_or_else(|| {
                DomainError::Internal(format!(
                    "No renderer registered for format: {}",
                    input.output_format
                ))
            })?;

        let export_result = match renderer
            .render(&dataset, template.as_ref(), master_template.as_ref())
            .await
        {
            Ok(result) => result,
            Err(e) => {
                self.job_repo
                    .update_status(job.id, &JobStatus::Failed, None, None, Some(e.to_string()))
                    .await
                    .ok();
                return Err(e);
            }
        };

        // 10. Update job as completed
        let size = export_result.data.len() as i64;
        job.status = JobStatus::Completed;
        job.output_size_bytes = Some(size);
        job.completed_at = Some(Utc::now());
        self.job_repo
            .update_status(job.id, &JobStatus::Completed, None, Some(size), None)
            .await?;

        // 11. Audit
        let audit = ReportAuditEntry {
            id: Uuid::new_v4(),
            org_id: ctx.tenant_id,
            actor_id: ctx.actor.id().unwrap_or_default(),
            action: "report.generated".into(),
            entity_type: "report_job".into(),
            entity_id: job.id,
            details: Some(serde_json::json!({
                "report_code": input.report_code,
                "output_format": input.output_format.to_string(),
                "row_count": dataset.total_count,
            })),
            created_at: Utc::now(),
        };
        self.audit_repo.save(&audit).await.ok(); // best-effort

        Ok((job, export_result))
    }
}

/// Validate submitted parameters against the definition schema.
fn validate_parameters(
    params: &HashMap<String, ParamValue>,
    schema: &[ParameterDef],
) -> Result<(), DomainError> {
    // Check required parameters are present
    for def in schema {
        if def.required && !params.contains_key(&def.key) && def.default_value.is_none() {
            return Err(DomainError::Validation(format!(
                "Missing required parameter: '{}'",
                def.key
            )));
        }
    }

    // Check submitted parameters exist in schema and match types
    for (key, value) in params {
        let def = schema.iter().find(|p| p.key == *key).ok_or_else(|| {
            DomainError::Validation(format!("Unknown parameter: '{}'", key))
        })?;
        validate_param_type(key, value, def)?;
    }

    Ok(())
}

fn validate_param_type(key: &str, value: &ParamValue, def: &ParameterDef) -> Result<(), DomainError> {
    let type_matches = match (&def.param_type, value) {
        (ParamType::Text, ParamValue::Text(_)) => true,
        (ParamType::Integer, ParamValue::Integer(_)) => true,
        (ParamType::Decimal, ParamValue::Decimal(_)) => true,
        (ParamType::Decimal, ParamValue::Integer(_)) => true,
        (ParamType::Boolean, ParamValue::Boolean(_)) => true,
        (ParamType::Date, ParamValue::Date(_)) => true,
        (ParamType::DateRange, ParamValue::DateRange { .. }) => true,
        (ParamType::Id, ParamValue::Id(_)) => true,
        (ParamType::IdList, ParamValue::IdList(_)) => true,
        (ParamType::Enum, ParamValue::Enum(_)) => true,
        (ParamType::Period, ParamValue::Period(_)) => true,
        _ => false,
    };

    if !type_matches {
        return Err(DomainError::Validation(format!(
            "Parameter '{}' has wrong type: expected {:?}",
            key, def.param_type
        )));
    }

    // Enum value must be in options
    if def.param_type == ParamType::Enum {
        if let ParamValue::Enum(val) = value {
            if let Some(ref options) = def.options {
                if !options.iter().any(|o| o.value == *val) {
                    return Err(DomainError::Validation(format!(
                        "Parameter '{}' value '{}' is not in allowed options",
                        key, val
                    )));
                }
            }
        }
    }

    Ok(())
}
