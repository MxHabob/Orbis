use async_trait::async_trait;
use orbis_app::ports::repositories::ReportRepository;
use orbis_domain::entities::report::{ReportTemplate as DomainReportTemplate, ReportJob as DomainReportJob};
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::models::report::{report_template, report_job};

pub struct ReportRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ReportRepository for ReportRepoImpl {
    async fn find_template_by_id(&self, id: Uuid) -> Result<Option<DomainReportTemplate>, DomainError> {
        let model = report_template::Entity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainReportTemplate {
            id: m.id, org_id: m.org_id, name: m.name, description: m.description,
            template_type: m.template_type, template_body: m.template_body, is_active: m.is_active,
            created_at: m.created_at.into(), updated_at: m.updated_at.into(),
        }))
    }

    async fn find_job_by_id(&self, id: Uuid) -> Result<Option<DomainReportJob>, DomainError> {
        let model = report_job::Entity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainReportJob {
            id: m.id, org_id: m.org_id, template_id: m.template_id, requested_by: m.requested_by,
            parameters: m.parameters, status: m.status, file_path: m.file_path,
            created_at: m.created_at.into(), completed_at: m.completed_at.map(|d| d.into()),
        }))
    }

    async fn save_job(&self, job: &DomainReportJob) -> Result<(), DomainError> {
        let active = report_job::ActiveModel {
            id: Set(job.id), org_id: Set(job.org_id), template_id: Set(job.template_id),
            requested_by: Set(job.requested_by), parameters: Set(job.parameters.clone()),
            status: Set(job.status.clone()), file_path: Set(job.file_path.clone()),
            created_at: Set(job.created_at.into()), completed_at: Set(job.completed_at.map(|d| d.into())),
        };

        if report_job::Entity::find_by_id(job.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn update_job_status(&self, job_id: Uuid, status: &str, file_path: Option<String>) -> Result<(), DomainError> {
        if let Some(model) = report_job::Entity::find_by_id(job_id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))? {
            let mut active: report_job::ActiveModel = model.into();
            active.status = Set(status.to_string());
            if file_path.is_some() {
                active.file_path = Set(file_path);
            }
            if status == "completed" || status == "failed" {
                active.completed_at = Set(Some(chrono::Utc::now().into()));
            }
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}
