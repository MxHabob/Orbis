use chrono::Utc;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore, oneshot};
use uuid::Uuid;

use crate::context::RequestContext;
use crate::ports::reporting::{
    DataProviderRegistry, RendererRegistry, ReportAuditRepository, ReportDefinitionRepository,
    ReportJobRepository, ReportTemplateRepository,
};
use orbis_domain::entities::report::{
    ExportResult, JobStatus, OrgReportContext, OutputFormat, ParamType, ParamValue, ParameterDef,
    ReportAuditEntry, ReportDefinition, ReportJob, ReportLocale, ReportTemplate, TemplateStatus,
};
use orbis_domain::errors::DomainError;

enum ReportExecutionOutcome<T> {
    Cancelled,
    Finished(Result<T, DomainError>),
}

async fn await_report_execution<T>(
    cancel_rx: oneshot::Receiver<()>,
    execution: impl std::future::Future<Output = Result<T, DomainError>>,
) -> ReportExecutionOutcome<T> {
    tokio::select! {
        biased;
        _ = cancel_rx => ReportExecutionOutcome::Cancelled,
        result = execution => ReportExecutionOutcome::Finished(result),
    }
}

async fn store_report_file(
    directory: &Path,
    file_path: &Path,
    data: &[u8],
) -> Result<(), DomainError> {
    if data.len() > 50 * 1024 * 1024 {
        return Err(DomainError::Internal(
            "Report exceeds the 50 MiB output limit".into(),
        ));
    }

    let staging_path = file_path.with_extension(format!(
        "{}.part",
        file_path
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default()
    ));
    let storage_result = async {
        tokio::fs::create_dir_all(directory)
            .await
            .map_err(|error| DomainError::Internal(format!("Report storage failed: {error}")))?;
        #[cfg(unix)]
        tokio::fs::set_permissions(
            directory,
            std::os::unix::fs::PermissionsExt::from_mode(0o700),
        )
        .await
        .map_err(|error| DomainError::Internal(format!("Report storage failed: {error}")))?;
        tokio::fs::write(&staging_path, data)
            .await
            .map_err(|error| DomainError::Internal(format!("Report storage failed: {error}")))?;
        tokio::fs::rename(&staging_path, file_path)
            .await
            .map_err(|error| DomainError::Internal(format!("Report storage failed: {error}")))?;
        Ok::<_, DomainError>(())
    }
    .await;

    if let Err(error) = storage_result {
        if let Err(cleanup_error) = tokio::fs::remove_file(&staging_path).await {
            if cleanup_error.kind() != std::io::ErrorKind::NotFound {
                tracing::error!(path = %staging_path.display(), error = %cleanup_error, "Unable to remove incomplete report output");
            }
        }
        return Err(error);
    }
    Ok(())
}

async fn remove_report_file(file_path: &Path) -> Result<(), DomainError> {
    let extension = file_path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default();
    let staging_path = file_path.with_extension(format!("{extension}.part"));
    for path in [file_path, staging_path.as_path()] {
        match tokio::fs::remove_file(path).await {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(DomainError::Internal(format!(
                    "Report file cleanup failed: {error}"
                )));
            }
        }
    }
    Ok(())
}

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

#[derive(Clone)]
pub struct ReportService {
    pub definition_repo: Arc<dyn ReportDefinitionRepository>,
    pub template_repo: Arc<dyn ReportTemplateRepository>,
    pub job_repo: Arc<dyn ReportJobRepository>,
    pub audit_repo: Arc<dyn ReportAuditRepository>,
    pub provider_registry: Arc<dyn DataProviderRegistry>,
    pub renderer_registry: Arc<dyn RendererRegistry>,
    pub org_context_resolver: Arc<dyn OrgContextResolver>,
    active_report_jobs: Arc<Mutex<HashMap<Uuid, oneshot::Sender<()>>>>,
    report_worker_slots: Arc<Semaphore>,
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
            active_report_jobs: Arc::new(Mutex::new(HashMap::new())),
            report_worker_slots: Arc::new(Semaphore::new(4)),
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

        let active: Vec<ReportDefinition> =
            definitions.into_iter().filter(|d| d.is_active).collect();

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
        self.generate_report_with_job_id(ctx, input, Uuid::new_v4(), false)
            .await
    }

    pub async fn create_pending_job(
        &self,
        ctx: &RequestContext,
        input: &GenerateReportInput,
        job_id: Uuid,
    ) -> Result<ReportJob, DomainError> {
        let definition = self
            .definition_repo
            .find_by_code(ctx.tenant_id, &input.report_code)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!(
                    "Report definition not found: {}",
                    input.report_code
                ))
            })?;
        if !definition.is_active {
            return Err(DomainError::Validation(
                "Report definition is inactive".into(),
            ));
        }
        if !definition.supported_formats.contains(&input.output_format) {
            return Err(DomainError::Validation(format!(
                "Output format '{}' is not supported for report '{}'",
                input.output_format, input.report_code
            )));
        }
        validate_parameters(&input.parameters, &definition.parameters)?;

        let job = ReportJob {
            id: job_id,
            org_id: ctx.tenant_id,
            report_definition_id: definition.id,
            template_id: None,
            requested_by: ctx.actor.id().unwrap_or_default(),
            report_code: input.report_code.clone(),
            parameters: serde_json::to_value(&input.parameters)
                .map_err(|error| DomainError::Internal(error.to_string()))?,
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
        Ok(job)
    }

    pub async fn enqueue_report(
        &self,
        ctx: &RequestContext,
        input: GenerateReportInput,
    ) -> Result<ReportJob, DomainError> {
        let worker_slot = self
            .report_worker_slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| {
                DomainError::Validation("Report worker is busy; try again shortly".into())
            })?;
        let job = self.create_pending_job(ctx, &input, Uuid::new_v4()).await?;
        let service = self.clone();
        let worker_ctx = ctx.clone();
        let job_id = job.id;
        let output_extension = job.output_format.extension().to_string();
        let (cancel_tx, cancel_rx) = oneshot::channel();
        self.active_report_jobs
            .lock()
            .await
            .insert(job_id, cancel_tx);

        tokio::spawn(async move {
            let _worker_slot = worker_slot;
            let execution = async {
                let export = service
                    .generate_report_for_job(&worker_ctx, input, job_id)
                    .await?;
                let temp_dir = std::env::temp_dir()
                    .join("orbis_reports")
                    .join(worker_ctx.tenant_id.to_string());
                let file_path = temp_dir.join(format!("{job_id}.{output_extension}"));
                let storage_result = async {
                    store_report_file(&temp_dir, &file_path, &export.data).await?;
                    service
                        .job_repo
                        .update_status(
                            job_id,
                            &JobStatus::Completed,
                            Some(file_path.to_string_lossy().into_owned()),
                            Some(export.data.len() as i64),
                            None,
                        )
                        .await
                }
                .await;
                if let Err(error) = storage_result {
                    if let Err(cleanup_error) = remove_report_file(&file_path).await {
                        tracing::error!(report_id = %job_id, error = %cleanup_error, "Unable to remove incomplete report output");
                    }
                    return Err(error);
                }
                Ok::<_, DomainError>(file_path)
            };

            match await_report_execution(cancel_rx, execution).await {
                ReportExecutionOutcome::Cancelled => {
                    let file_path = std::env::temp_dir()
                        .join("orbis_reports")
                        .join(worker_ctx.tenant_id.to_string())
                        .join(format!("{job_id}.{output_extension}"));
                    if let Err(error) = remove_report_file(&file_path).await {
                        tracing::error!(report_id = %job_id, error = %error, "Unable to remove cancelled report output");
                    }
                    if let Err(error) = service
                        .job_repo
                        .update_status(job_id, &JobStatus::Cancelled, None, None, None)
                        .await
                    {
                        tracing::error!(report_id = %job_id, error = %error, "Unable to persist report cancellation");
                    }
                }
                ReportExecutionOutcome::Finished(result) => match result {
                    Ok(_) => {}
                    Err(error) => {
                        if let Err(update_error) = service
                            .job_repo
                            .update_status(
                                job_id,
                                &JobStatus::Failed,
                                None,
                                None,
                                Some(error.to_string()),
                            )
                            .await
                        {
                            tracing::error!(report_id = %job_id, error = %update_error, "Unable to persist report failure");
                        }
                    }
                },
            }
            service.active_report_jobs.lock().await.remove(&job_id);
        });

        Ok(job)
    }

    pub async fn cancel_report(&self, job_id: Uuid) -> Result<bool, DomainError> {
        let cancel = self.active_report_jobs.lock().await.remove(&job_id);
        match cancel {
            Some(cancel) => cancel.send(()).map(|()| true).map_err(|_| {
                DomainError::Internal("Report worker stopped before cancellation".into())
            }),
            None => Ok(false),
        }
    }

    pub async fn cleanup_expired_reports(&self) -> Result<usize, DomainError> {
        let cutoff = Utc::now() - chrono::Duration::hours(24);
        let mut removed = 0;
        loop {
            let expired = self
                .job_repo
                .find_expired_completed_before(cutoff, 100)
                .await?;
            if expired.is_empty() {
                break;
            }
            let batch_size = expired.len();
            for (job_id, file_path) in expired {
                match tokio::fs::remove_file(&file_path).await {
                    Ok(()) => {}
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                    Err(error) => {
                        return Err(DomainError::Internal(format!(
                            "Expired report cleanup failed for {job_id}: {error}"
                        )));
                    }
                }
                self.job_repo
                    .update_status(
                        job_id,
                        &JobStatus::Failed,
                        None,
                        None,
                        Some("Report download has expired".into()),
                    )
                    .await?;
                removed += 1;
            }
            if batch_size < 100 {
                break;
            }
        }
        Ok(removed)
    }

    pub async fn generate_report_for_job(
        &self,
        ctx: &RequestContext,
        input: GenerateReportInput,
        job_id: Uuid,
    ) -> Result<ExportResult, DomainError> {
        let (_, export) = self
            .generate_report_with_job_id(ctx, input, job_id, true)
            .await?;
        Ok(export)
    }

    async fn generate_report_with_job_id(
        &self,
        ctx: &RequestContext,
        input: GenerateReportInput,
        job_id: Uuid,
        defer_completion: bool,
    ) -> Result<(ReportJob, ExportResult), DomainError> {
        // 1. Look up definition
        let definition = self
            .definition_repo
            .find_by_code(ctx.tenant_id, &input.report_code)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!(
                    "Report definition not found: {}",
                    input.report_code
                ))
            })?;

        if !definition.is_active {
            return Err(DomainError::Validation(
                "Report definition is inactive".into(),
            ));
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
            id: job_id,
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

        let start_time = Utc::now();

        // 6. Resolve org branding context
        let org_context = self.org_context_resolver.resolve(ctx.tenant_id).await?;

        // 7. Fetch data from data provider
        let dataset = match self
            .provider_registry
            .get_provider(&definition.data_provider_key)
        {
            Some(provider) => {
                match provider
                    .fetch_data(
                        ctx.tenant_id,
                        &org_context,
                        &input.parameters,
                        &input.locale,
                    )
                    .await
                {
                    Ok(ds) => ds,
                    Err(e) => {
                        let err_msg = format!("Data provider failed: {}", e);
                        self.job_repo
                            .update_status(
                                job.id,
                                &JobStatus::Failed,
                                None,
                                None,
                                Some(err_msg.clone()),
                            )
                            .await?;
                        job.status = JobStatus::Failed;
                        job.error_message = Some(err_msg);
                        return Err(e);
                    }
                }
            }
            None => {
                let err_msg = format!(
                    "Data provider not registered: {}",
                    definition.data_provider_key
                );
                self.job_repo
                    .update_status(
                        job.id,
                        &JobStatus::Failed,
                        None,
                        None,
                        Some(err_msg.clone()),
                    )
                    .await?;
                job.status = JobStatus::Failed;
                job.error_message = Some(err_msg.clone());
                return Err(DomainError::Internal(err_msg));
            }
        };

        // 8. Resolve template (if binding exists)
        let binding = self
            .template_repo
            .find_binding(
                definition.id,
                input.locale.primary_lang(),
                &input.output_format,
            )
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
                DomainError::Internal(format!(
                    "No renderer registered for format: {}",
                    input.output_format
                ))
            })?;

        let export_result = match renderer.render(&dataset, template.as_ref(), None).await {
            Ok(res) => res,
            Err(e) => {
                let err_msg = format!("Renderer failed: {}", e);
                self.job_repo
                    .update_status(
                        job.id,
                        &JobStatus::Failed,
                        None,
                        None,
                        Some(err_msg.clone()),
                    )
                    .await?;
                job.status = JobStatus::Failed;
                job.error_message = Some(err_msg);
                return Err(e);
            }
        };

        // 10. Mark job completed
        let duration_ms = (Utc::now() - start_time).num_milliseconds();
        let size_bytes = export_result.data.len() as i64;

        if !defer_completion {
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
        }

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
        self.audit_repo.save(&audit).await?;

        Ok((job, export_result))
    }

    /// List templates for organization
    pub async fn list_templates(&self, org_id: Uuid) -> Result<Vec<ReportTemplate>, DomainError> {
        self.template_repo
            .list_by_org(org_id, None, None, None)
            .await
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
        let template = self
            .template_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Template not found".into()))?;

        if template.org_id != ctx.tenant_id {
            return Err(DomainError::Forbidden(
                "Template belongs to another organization".into(),
            ));
        }

        self.template_repo.soft_delete(id).await
    }

    /// Publish a template
    pub async fn publish_template(
        &self,
        ctx: &RequestContext,
        id: Uuid,
    ) -> Result<ReportTemplate, DomainError> {
        let mut template = self
            .template_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Template not found".into()))?;

        if template.org_id != ctx.tenant_id {
            return Err(DomainError::Forbidden(
                "Template belongs to another organization".into(),
            ));
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
    pub async fn archive_template(
        &self,
        ctx: &RequestContext,
        id: Uuid,
    ) -> Result<ReportTemplate, DomainError> {
        let mut template = self
            .template_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Template not found".into()))?;

        if template.org_id != ctx.tenant_id {
            return Err(DomainError::Forbidden(
                "Template belongs to another organization".into(),
            ));
        }

        template.status = TemplateStatus::Archived;
        template.updated_at = Utc::now();

        self.template_repo.save(&template).await?;
        Ok(template)
    }

    /// Revert template to a previous version
    pub async fn revert_template_version(
        &self,
        ctx: &RequestContext,
        id: Uuid,
        version: i32,
    ) -> Result<ReportTemplate, DomainError> {
        let mut template = self
            .template_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Template not found".into()))?;

        if template.org_id != ctx.tenant_id {
            return Err(DomainError::Forbidden(
                "Template belongs to another organization".into(),
            ));
        }

        let ver = self
            .template_repo
            .find_version(id, version)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Version {} not found", version)))?;

        template.document_json = ver.document_json;
        if let Some(ps) = ver.page_size {
            template.page_size = ps;
        }
        if let Some(ori) = ver.orientation {
            template.orientation = ori;
        }
        if let Some(mar) = ver.margins_json {
            template.margins_json = mar;
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn report_worker_cancellation_interrupts_pending_generation() {
        let (cancel_tx, cancel_rx) = oneshot::channel();
        let worker = tokio::spawn(await_report_execution(
            cancel_rx,
            std::future::pending::<Result<(), DomainError>>(),
        ));
        cancel_tx
            .send(())
            .expect("worker cancellation receiver exists");

        let outcome = tokio::time::timeout(Duration::from_millis(100), worker)
            .await
            .expect("cancellation must not wait for report generation")
            .expect("worker task should not panic");
        assert!(matches!(outcome, ReportExecutionOutcome::Cancelled));
    }

    #[tokio::test]
    async fn report_worker_returns_generation_failures_to_status_persistence() {
        let (_cancel_tx, cancel_rx) = oneshot::channel();
        let outcome = await_report_execution(cancel_rx, async {
            Err::<(), _>(DomainError::Internal("renderer failed".into()))
        })
        .await;

        assert!(matches!(
            outcome,
            ReportExecutionOutcome::Finished(Err(DomainError::Internal(message)))
                if message == "renderer failed"
        ));
    }

    #[tokio::test]
    async fn report_storage_publishes_complete_file_and_removes_staging_output() {
        let directory = std::env::temp_dir().join(format!("orbis-report-test-{}", Uuid::new_v4()));
        let file_path = directory.join("test.xlsx");
        let staging_path = directory.join("test.xlsx.part");
        let data = b"complete workbook bytes";

        store_report_file(&directory, &file_path, data)
            .await
            .expect("report should be atomically stored");
        assert_eq!(
            tokio::fs::read(&file_path)
                .await
                .expect("completed report is readable"),
            data
        );
        assert!(
            tokio::fs::metadata(&staging_path).await.is_err(),
            "staging output must not remain after publication"
        );
        remove_report_file(&file_path)
            .await
            .expect("report output should be removable");
        tokio::fs::remove_dir(&directory)
            .await
            .expect("test directory should be empty after cleanup");
    }

    #[tokio::test]
    async fn report_storage_failure_does_not_leave_a_partial_download() {
        let directory = std::env::temp_dir().join(format!("orbis-report-test-{}", Uuid::new_v4()));
        let file_path = directory.join("test.xlsx");
        tokio::fs::create_dir_all(&file_path)
            .await
            .expect("create a conflicting destination");

        let result = store_report_file(&directory, &file_path, b"workbook").await;
        assert!(matches!(result, Err(DomainError::Internal(_))));
        assert!(
            tokio::fs::metadata(directory.join("test.xlsx.part"))
                .await
                .is_err(),
            "failed publication must remove its staging file"
        );
        tokio::fs::remove_dir(&file_path)
            .await
            .expect("remove conflicting destination");
        tokio::fs::remove_dir(&directory)
            .await
            .expect("remove test directory");
    }
}
