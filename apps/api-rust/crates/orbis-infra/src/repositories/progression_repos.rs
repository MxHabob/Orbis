use async_trait::async_trait;
use orbis_app::ports::repositories::PromotionRepository;
use orbis_domain::entities::progression::{PromotionJob, PromotionDetail, PromotionJobStatus, PromotionDecision};
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::models::{
    promotion_job::{self, Entity as PromotionJobEntity},
    promotion_detail::{self, Entity as PromotionDetailEntity},
};

pub struct ProgressionRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl PromotionRepository for ProgressionRepoImpl {
    async fn find_job_by_id(&self, job_id: Uuid) -> Result<Option<PromotionJob>, DomainError> {
        let model = PromotionJobEntity::find_by_id(job_id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| PromotionJob {
            id: m.id,
            org_id: m.org_id,
            source_period_id: m.source_period_id,
            target_period_id: m.target_period_id,
            name: m.name,
            status: match m.status.as_str() {
                "preview_ready" => PromotionJobStatus::PreviewReady,
                "executing" => PromotionJobStatus::Executing,
                "completed" => PromotionJobStatus::Completed,
                "failed" => PromotionJobStatus::Failed,
                "rolled_back" => PromotionJobStatus::RolledBack,
                _ => PromotionJobStatus::Draft,
            },
            total_students: m.total_students as u32,
            promoted_count: m.promoted_count as u32,
            retained_count: m.retained_count as u32,
            graduated_count: m.graduated_count as u32,
            excluded_count: m.excluded_count as u32,
            executed_by: m.executed_by,
            executed_at: m.executed_at.map(Into::into),
            created_at: m.created_at.into(),
            updated_at: m.updated_at.into(),
        }))
    }

    async fn find_jobs_by_org(&self, org_id: Uuid) -> Result<Vec<PromotionJob>, DomainError> {
        let models = PromotionJobEntity::find()
            .filter(promotion_job::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| PromotionJob {
            id: m.id,
            org_id: m.org_id,
            source_period_id: m.source_period_id,
            target_period_id: m.target_period_id,
            name: m.name,
            status: match m.status.as_str() {
                "preview_ready" => PromotionJobStatus::PreviewReady,
                "executing" => PromotionJobStatus::Executing,
                "completed" => PromotionJobStatus::Completed,
                "failed" => PromotionJobStatus::Failed,
                "rolled_back" => PromotionJobStatus::RolledBack,
                _ => PromotionJobStatus::Draft,
            },
            total_students: m.total_students as u32,
            promoted_count: m.promoted_count as u32,
            retained_count: m.retained_count as u32,
            graduated_count: m.graduated_count as u32,
            excluded_count: m.excluded_count as u32,
            executed_by: m.executed_by,
            executed_at: m.executed_at.map(Into::into),
            created_at: m.created_at.into(),
            updated_at: m.updated_at.into(),
        }).collect())
    }

    async fn find_details_by_job(&self, job_id: Uuid) -> Result<Vec<PromotionDetail>, DomainError> {
        let models = PromotionDetailEntity::find()
            .filter(promotion_detail::Column::JobId.eq(job_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| PromotionDetail {
            id: m.id,
            job_id: m.job_id,
            student_id: m.student_id,
            enrollment_id: m.enrollment_id,
            source_class_id: m.source_class_id,
            target_class_id: m.target_class_id,
            decision: match m.decision.as_str() {
                "retain" => PromotionDecision::Retain,
                "graduate" => PromotionDecision::Graduate,
                "exclude" => PromotionDecision::Exclude,
                _ => PromotionDecision::Promote,
            },
            reason: m.reason,
            manual_override: m.manual_override,
            override_by: m.override_by,
            override_reason: m.override_reason,
            final_gpa: m.final_gpa,
            attendance_rate: m.attendance_rate,
            created_at: m.created_at.into(),
        }).collect())
    }

    async fn save_job(&self, job: &PromotionJob) -> Result<(), DomainError> {
        let active = promotion_job::ActiveModel {
            id: Set(job.id),
            org_id: Set(job.org_id),
            source_period_id: Set(job.source_period_id),
            target_period_id: Set(job.target_period_id),
            name: Set(job.name.clone()),
            status: Set(match job.status {
                PromotionJobStatus::PreviewReady => "preview_ready".into(),
                PromotionJobStatus::Executing => "executing".into(),
                PromotionJobStatus::Completed => "completed".into(),
                PromotionJobStatus::Failed => "failed".into(),
                PromotionJobStatus::RolledBack => "rolled_back".into(),
                PromotionJobStatus::Draft => "draft".into(),
            }),
            total_students: Set(job.total_students as i32),
            promoted_count: Set(job.promoted_count as i32),
            retained_count: Set(job.retained_count as i32),
            graduated_count: Set(job.graduated_count as i32),
            excluded_count: Set(job.excluded_count as i32),
            executed_by: Set(job.executed_by),
            executed_at: Set(job.executed_at.map(Into::into)),
            created_at: Set(job.created_at.into()),
            updated_at: Set(job.updated_at.into()),
        };

        if PromotionJobEntity::find_by_id(job.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn save_detail(&self, detail: &PromotionDetail) -> Result<(), DomainError> {
        let active = promotion_detail::ActiveModel {
            id: Set(detail.id),
            job_id: Set(detail.job_id),
            student_id: Set(detail.student_id),
            enrollment_id: Set(detail.enrollment_id),
            source_class_id: Set(detail.source_class_id),
            target_class_id: Set(detail.target_class_id),
            decision: Set(detail.decision.to_string()),
            reason: Set(detail.reason.clone()),
            manual_override: Set(detail.manual_override),
            override_by: Set(detail.override_by),
            override_reason: Set(detail.override_reason.clone()),
            final_gpa: Set(detail.final_gpa),
            attendance_rate: Set(detail.attendance_rate),
            created_at: Set(detail.created_at.into()),
        };

        if PromotionDetailEntity::find_by_id(detail.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn save_details(&self, details: &[PromotionDetail]) -> Result<(), DomainError> {
        for detail in details {
            self.save_detail(detail).await?;
        }
        Ok(())
    }
}

