use async_trait::async_trait;
use orbis_app::ports::repositories::PromotionRepository;
use orbis_domain::entities::progression::{
    ProgressionDetail, ProgressionJob, ProgressionJobStatus, PromotionDecision,
};
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::models::{
    progression_detail::{self, Entity as ProgressionDetailEntity},
    progression_job::{self, Entity as ProgressionJobEntity},
};

pub struct ProgressionRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl PromotionRepository for ProgressionRepoImpl {
    async fn find_job_by_id(&self, job_id: Uuid) -> Result<Option<ProgressionJob>, DomainError> {
        let model = ProgressionJobEntity::find_by_id(job_id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| ProgressionJob {
            id: m.id,
            org_id: m.org_id,
            source_period_id: m.source_period_id,
            target_period_id: m.target_period_id,
            name: m.name,
            status: match m.status.as_str() {
                "preview_ready" => ProgressionJobStatus::PreviewReady,
                "executing" => ProgressionJobStatus::Executing,
                "completed" => ProgressionJobStatus::Completed,
                "failed" => ProgressionJobStatus::Failed,
                "rolled_back" => ProgressionJobStatus::RolledBack,
                _ => ProgressionJobStatus::Draft,
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

    async fn find_jobs_by_org(&self, org_id: Uuid) -> Result<Vec<ProgressionJob>, DomainError> {
        let models = ProgressionJobEntity::find()
            .filter(progression_job::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(|m| ProgressionJob {
                id: m.id,
                org_id: m.org_id,
                source_period_id: m.source_period_id,
                target_period_id: m.target_period_id,
                name: m.name,
                status: match m.status.as_str() {
                    "preview_ready" => ProgressionJobStatus::PreviewReady,
                    "executing" => ProgressionJobStatus::Executing,
                    "completed" => ProgressionJobStatus::Completed,
                    "failed" => ProgressionJobStatus::Failed,
                    "rolled_back" => ProgressionJobStatus::RolledBack,
                    _ => ProgressionJobStatus::Draft,
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
            })
            .collect())
    }

    async fn find_details_by_job(
        &self,
        job_id: Uuid,
    ) -> Result<Vec<ProgressionDetail>, DomainError> {
        let models = ProgressionDetailEntity::find()
            .filter(progression_detail::Column::JobId.eq(job_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(|m| ProgressionDetail {
                id: m.id,
                job_id: m.job_id,
                student_id: m.student_id,
                enrollment_id: m.enrollment_id,
                source_section_id: m.source_section_id,
                target_section_id: m.target_section_id,
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
            })
            .collect())
    }

    async fn save_job(&self, job: &ProgressionJob) -> Result<(), DomainError> {
        let active = progression_job::ActiveModel {
            id: Set(job.id),
            org_id: Set(job.org_id),
            source_period_id: Set(job.source_period_id),
            target_period_id: Set(job.target_period_id),
            name: Set(job.name.clone()),
            status: Set(match job.status {
                ProgressionJobStatus::PreviewReady => "preview_ready".into(),
                ProgressionJobStatus::Executing => "executing".into(),
                ProgressionJobStatus::Completed => "completed".into(),
                ProgressionJobStatus::Failed => "failed".into(),
                ProgressionJobStatus::RolledBack => "rolled_back".into(),
                ProgressionJobStatus::Draft => "draft".into(),
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

        if ProgressionJobEntity::find_by_id(job.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn save_detail(&self, detail: &ProgressionDetail) -> Result<(), DomainError> {
        let active = progression_detail::ActiveModel {
            id: Set(detail.id),
            job_id: Set(detail.job_id),
            student_id: Set(detail.student_id),
            enrollment_id: Set(detail.enrollment_id),
            source_section_id: Set(detail.source_section_id),
            target_section_id: Set(detail.target_section_id),
            decision: Set(detail.decision.to_string()),
            reason: Set(detail.reason.clone()),
            manual_override: Set(detail.manual_override),
            override_by: Set(detail.override_by),
            override_reason: Set(detail.override_reason.clone()),
            final_gpa: Set(detail.final_gpa),
            attendance_rate: Set(detail.attendance_rate),
            created_at: Set(detail.created_at.into()),
        };

        if ProgressionDetailEntity::find_by_id(detail.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn save_details(&self, details: &[ProgressionDetail]) -> Result<(), DomainError> {
        for detail in details {
            self.save_detail(detail).await?;
        }
        Ok(())
    }
}
