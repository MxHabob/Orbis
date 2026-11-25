use async_trait::async_trait;
use orbis_app::ports::repositories::PromotionRepository;
use orbis_domain::entities::enrollment::Enrollment;
use orbis_domain::entities::progression::{
    ProgressionDetail, ProgressionJob, ProgressionJobStatus, PromotionDecision,
};
use orbis_domain::errors::DomainError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use uuid::Uuid;

use super::student_repos::enrollment_to_active_model;
use crate::models::{
    academic_period, enrollment,
    progression_detail::{self, Entity as ProgressionDetailEntity},
    progression_job::{self, Entity as ProgressionJobEntity},
    section, student,
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

    async fn commit_promotion(
        &self,
        job: &ProgressionJob,
        details: &[ProgressionDetail],
        new_enrollments: &[Enrollment],
    ) -> Result<(), DomainError> {
        let txn = self.db.begin().await.map_err(db_error)?;
        let target_period = job
            .target_period_id
            .ok_or_else(|| DomainError::Validation("Promotion requires a target period".into()))?;
        let periods = academic_period::Entity::find()
            .filter(academic_period::Column::OrgId.eq(job.org_id))
            .filter(academic_period::Column::Id.is_in([job.source_period_id, target_period]))
            .all(&txn)
            .await
            .map_err(db_error)?;
        if periods.len() != 2 {
            return Err(DomainError::NotFound("Promotion period not found".into()));
        }
        for detail in details
            .iter()
            .filter(|detail| detail.decision != PromotionDecision::Exclude)
        {
            let updated = enrollment::Entity::update_many()
                .set(enrollment::ActiveModel {
                    status: Set("graduated".into()),
                    updated_at: Set(job.updated_at.into()),
                    ..Default::default()
                })
                .filter(enrollment::Column::Id.eq(detail.enrollment_id))
                .filter(enrollment::Column::OrgId.eq(job.org_id))
                .filter(enrollment::Column::StudentId.eq(detail.student_id))
                .filter(enrollment::Column::PeriodId.eq(job.source_period_id))
                .filter(enrollment::Column::Status.ne("graduated"))
                .exec(&txn)
                .await
                .map_err(db_error)?;
            if updated.rows_affected != 1 {
                return Err(DomainError::Validation(
                    "Source enrollment is missing or already completed".into(),
                ));
            }
            if detail.decision == PromotionDecision::Graduate {
                let updated = student::Entity::update_many()
                    .set(student::ActiveModel {
                        status: Set("graduated".into()),
                        graduated_at: Set(Some(job.updated_at.into())),
                        updated_at: Set(job.updated_at.into()),
                        ..Default::default()
                    })
                    .filter(student::Column::Id.eq(detail.student_id))
                    .filter(student::Column::OrgId.eq(job.org_id))
                    .filter(student::Column::IsDeleted.eq(false))
                    .exec(&txn)
                    .await
                    .map_err(db_error)?;
                if updated.rows_affected != 1 {
                    return Err(DomainError::NotFound("Student not found".into()));
                }
            }
        }
        let section_ids: std::collections::HashSet<_> = new_enrollments
            .iter()
            .filter_map(|enrollment| enrollment.section_id)
            .collect();
        if !section_ids.is_empty() {
            let sections = section::Entity::find()
                .filter(section::Column::OrgId.eq(job.org_id))
                .filter(section::Column::IsDeleted.eq(false))
                .filter(section::Column::Id.is_in(section_ids.iter().copied()))
                .all(&txn)
                .await
                .map_err(db_error)?;
            if sections.len() != section_ids.len() {
                return Err(DomainError::NotFound("Target section not found".into()));
            }
        }
        for batch in new_enrollments.chunks(100) {
            enrollment::Entity::insert_many(batch.iter().map(enrollment_to_active_model))
                .exec(&txn)
                .await
                .map_err(db_error)?;
        }
        job_model(job).insert(&txn).await.map_err(db_error)?;
        for batch in details.chunks(100) {
            ProgressionDetailEntity::insert_many(batch.iter().map(detail_model))
                .exec(&txn)
                .await
                .map_err(db_error)?;
        }
        txn.commit().await.map_err(db_error)
    }

    async fn save_job(&self, job: &ProgressionJob) -> Result<(), DomainError> {
        let active = job_model(job);

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
        let active = detail_model(detail);

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

fn job_model(job: &ProgressionJob) -> progression_job::ActiveModel {
    progression_job::ActiveModel {
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
    }
}

fn detail_model(detail: &ProgressionDetail) -> progression_detail::ActiveModel {
    progression_detail::ActiveModel {
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
    }
}

fn db_error(error: sea_orm::DbErr) -> DomainError {
    DomainError::Internal(error.to_string())
}
