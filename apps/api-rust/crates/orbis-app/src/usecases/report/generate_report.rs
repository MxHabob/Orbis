use crate::ports::repositories::ReportRepository;
use crate::ports::gateways::ReportEnginePort;
use crate::context::RequestContext;
use orbis_domain::entities::report::ReportJob;
use orbis_domain::errors::DomainError;
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;

pub struct GenerateReportUseCase {
    pub report_repo: Arc<dyn ReportRepository>,
    pub report_engine: Arc<dyn ReportEnginePort>,
}

impl GenerateReportUseCase {
    pub async fn execute(
        &self,
        _ctx: &RequestContext,
        template_id: Uuid,
        parameters: serde_json::Value,
        format: &str, // "pdf" or "excel"
    ) -> Result<ReportJob, DomainError> {
        // 1. Fetch Template
        let template = self.report_repo.find_template_by_id(template_id).await?
            .ok_or_else(|| DomainError::NotFound("Report Template not found".into()))?;

        if !template.is_active {
            return Err(DomainError::Validation("Template is not active".into()));
        }

        // 1b. Fetch Master Layout Template (if it exists)
        let layout_id = Uuid::parse_str("44444444-4444-4444-4444-444444444444").unwrap();
        let layout_template = self.report_repo.find_template_by_id(layout_id).await?;
        let layout_body = layout_template.map(|t| t.template_body);

        // 2. Create Job Record
        let job = ReportJob {
            id: Uuid::new_v4(),
            org_id: _ctx.tenant_id,
            template_id,
            requested_by: _ctx.actor.id().unwrap_or_default(),
            parameters: parameters.clone(),
            status: "pending".to_string(),
            file_path: None,
            created_at: Utc::now(),
            completed_at: None,
        };

        self.report_repo.save_job(&job).await?;

        // 3. Update Status to processing
        self.report_repo.update_job_status(job.id, "processing", None).await?;

        // 4. Generate Document
        let result = if format == "pdf" {
            self.report_engine.generate_pdf(_ctx, layout_body.as_deref(), &template.template_body, &parameters).await
        } else {
            self.report_engine.generate_excel(_ctx, layout_body.as_deref(), &template.template_body, &parameters).await
        };

        match result {
            Ok(_bytes) => {
                let file_path = format!("reports/{}/{}.{}", _ctx.tenant_id, job.id, format);
                self.report_repo.update_job_status(job.id, "completed", Some(file_path)).await?;
            },
            Err(e) => {
                self.report_repo.update_job_status(job.id, "failed", None).await?;
                return Err(e);
            }
        }

        Ok(job)
    }
}
