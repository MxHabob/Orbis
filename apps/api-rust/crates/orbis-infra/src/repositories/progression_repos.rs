use async_trait::async_trait;
use orbis_app::ports::repositories::PromotionRepository;
use orbis_domain::entities::enrollment::Enrollment;
use orbis_domain::entities::progression::{
    ProgressionDetail, ProgressionJob, ProgressionJobStatus, PromotionDecision,
};
use orbis_domain::errors::DomainError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
    TransactionTrait,
};
use std::collections::HashMap;
use uuid::Uuid;

use super::student_repos::enrollment_to_active_model;
use crate::models::{
    academic_period, enrollment, grade,
    progression_detail::{self, Entity as ProgressionDetailEntity},
    progression_job::{self, Entity as ProgressionJobEntity},
    section, student, student_fee,
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
            .order_by_desc(progression_job::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_job_model).collect())
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
        if let Some(target) = periods.iter().find(|p| p.id == target_period) {
            if target.is_locked || target.status == "archived" {
                return Err(DomainError::BusinessRule(
                    "Target period is locked or archived and cannot receive enrollments".into(),
                ));
            }
        }

        // Group source enrollments by target status so updates run in batches instead of N+1 queries.
        let mut promote_enrollment_ids = Vec::new();
        let mut retain_enrollment_ids = Vec::new();
        let mut graduate_enrollment_ids = Vec::new();
        let mut graduate_student_ids = Vec::new();

        for detail in details {
            match detail.decision {
                PromotionDecision::Promote => promote_enrollment_ids.push(detail.enrollment_id),
                PromotionDecision::Retain => retain_enrollment_ids.push(detail.enrollment_id),
                PromotionDecision::Graduate => {
                    graduate_enrollment_ids.push(detail.enrollment_id);
                    graduate_student_ids.push(detail.student_id);
                }
                PromotionDecision::Exclude => {}
            }
        }

        let terminal_statuses = [
            "completed",
            "failed",
            "graduated",
            "withdrawn",
            "transferred",
        ];
        for (ids, new_status) in [
            (&promote_enrollment_ids, "completed"),
            (&retain_enrollment_ids, "failed"),
            (&graduate_enrollment_ids, "graduated"),
        ] {
            if ids.is_empty() {
                continue;
            }
            for batch in ids.chunks(200) {
                let updated = enrollment::Entity::update_many()
                    .set(enrollment::ActiveModel {
                        status: Set(new_status.into()),
                        updated_at: Set(job.updated_at.into()),
                        ..Default::default()
                    })
                    .filter(enrollment::Column::Id.is_in(batch.iter().copied()))
                    .filter(enrollment::Column::OrgId.eq(job.org_id))
                    .filter(enrollment::Column::PeriodId.eq(job.source_period_id))
                    .filter(enrollment::Column::Status.is_not_in(terminal_statuses))
                    .exec(&txn)
                    .await
                    .map_err(db_error)?;
                if updated.rows_affected as usize != batch.len() {
                    return Err(DomainError::Validation(
                        "One or more source enrollments are missing or already finalized".into(),
                    ));
                }
            }
        }

        if !graduate_student_ids.is_empty() {
            for batch in graduate_student_ids.chunks(200) {
                let updated = student::Entity::update_many()
                    .set(student::ActiveModel {
                        status: Set("graduated".into()),
                        graduated_at: Set(Some(job.updated_at.into())),
                        updated_at: Set(job.updated_at.into()),
                        ..Default::default()
                    })
                    .filter(student::Column::Id.is_in(batch.iter().copied()))
                    .filter(student::Column::OrgId.eq(job.org_id))
                    .filter(student::Column::IsDeleted.eq(false))
                    .exec(&txn)
                    .await
                    .map_err(db_error)?;
                if updated.rows_affected as usize != batch.len() {
                    return Err(DomainError::NotFound(
                        "One or more graduating students were not found".into(),
                    ));
                }
            }
        }

        // Prevent duplicate active/pending enrollments in target_period_id.
        if !new_enrollments.is_empty() {
            let target_student_ids: Vec<Uuid> =
                new_enrollments.iter().map(|e| e.student_id).collect();
            for batch in target_student_ids.chunks(200) {
                let existing = enrollment::Entity::find()
                    .filter(enrollment::Column::OrgId.eq(job.org_id))
                    .filter(enrollment::Column::PeriodId.eq(target_period))
                    .filter(enrollment::Column::StudentId.is_in(batch.iter().copied()))
                    .filter(enrollment::Column::Status.is_in(["active", "pending_approval"]))
                    .all(&txn)
                    .await
                    .map_err(db_error)?;
                if !existing.is_empty() {
                    return Err(DomainError::Conflict(format!(
                        "{} student(s) already have an active enrollment in the target period",
                        existing.len()
                    )));
                }
            }
        }

        // Validate target sections exist, are active, and have capacity for incoming students.
        let mut incoming_per_section: HashMap<Uuid, usize> = HashMap::new();
        for enrollment in new_enrollments {
            if let Some(sec_id) = enrollment.section_id {
                *incoming_per_section.entry(sec_id).or_default() += 1;
            }
        }
        if !incoming_per_section.is_empty() {
            let section_ids: Vec<Uuid> = incoming_per_section.keys().copied().collect();
            let sections = section::Entity::find()
                .filter(section::Column::OrgId.eq(job.org_id))
                .filter(section::Column::IsDeleted.eq(false))
                .filter(section::Column::IsActive.eq(true))
                .filter(section::Column::Id.is_in(section_ids.iter().copied()))
                .all(&txn)
                .await
                .map_err(db_error)?;
            if sections.len() != section_ids.len() {
                return Err(DomainError::NotFound(
                    "One or more target sections were not found or are inactive".into(),
                ));
            }
            let existing_target_enrollments = enrollment::Entity::find()
                .filter(enrollment::Column::OrgId.eq(job.org_id))
                .filter(enrollment::Column::PeriodId.eq(target_period))
                .filter(enrollment::Column::SectionId.is_in(section_ids))
                .filter(enrollment::Column::Status.is_in(["active", "pending_approval"]))
                .all(&txn)
                .await
                .map_err(db_error)?;
            let mut current_per_section: HashMap<Uuid, usize> = HashMap::new();
            for existing in existing_target_enrollments {
                if let Some(sec_id) = existing.section_id {
                    *current_per_section.entry(sec_id).or_default() += 1;
                }
            }
            for sec in sections {
                if let Some(cap) = sec.capacity.filter(|c| *c > 0) {
                    let current = current_per_section.get(&sec.id).copied().unwrap_or(0);
                    let incoming = incoming_per_section.get(&sec.id).copied().unwrap_or(0);
                    if current + incoming > cap as usize {
                        return Err(DomainError::BusinessRule(format!(
                            "Target section '{}' capacity ({}) would be exceeded ({}/{})",
                            sec.name,
                            cap,
                            current + incoming,
                            cap
                        )));
                    }
                }
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

    async fn rollback_promotion(
        &self,
        job_id: Uuid,
        org_id: Uuid,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<ProgressionJob, DomainError> {
        let txn = self.db.begin().await.map_err(db_error)?;
        let Some(job_row) = ProgressionJobEntity::find_by_id(job_id)
            .filter(progression_job::Column::OrgId.eq(org_id))
            .one(&txn)
            .await
            .map_err(db_error)?
        else {
            return Err(DomainError::NotFound("Promotion job not found".into()));
        };
        if job_row.status != "completed" {
            return Err(DomainError::Conflict(format!(
                "Only completed promotion jobs can be rolled back (current status: {})",
                job_row.status
            )));
        }

        let updated = ProgressionJobEntity::update_many()
            .col_expr(progression_job::Column::Status, "rolled_back".into())
            .col_expr(progression_job::Column::UpdatedAt, updated_at.into())
            .filter(progression_job::Column::Id.eq(job_id))
            .filter(progression_job::Column::OrgId.eq(org_id))
            .filter(progression_job::Column::Status.eq("completed"))
            .exec(&txn)
            .await
            .map_err(db_error)?;
        if updated.rows_affected != 1 {
            return Err(DomainError::ConcurrencyConflict(
                "Promotion job was already rolled back".into(),
            ));
        }

        let details = ProgressionDetailEntity::find()
            .filter(progression_detail::Column::JobId.eq(job_id))
            .all(&txn)
            .await
            .map_err(db_error)?;

        let mut restore_enrollment_ids = Vec::new();
        let mut target_student_ids = Vec::new();
        let mut revert_graduated_student_ids = Vec::new();
        for detail in &details {
            match detail.decision.as_str() {
                "promote" | "retain" => {
                    restore_enrollment_ids.push(detail.enrollment_id);
                    target_student_ids.push(detail.student_id);
                }
                "graduate" => {
                    restore_enrollment_ids.push(detail.enrollment_id);
                    revert_graduated_student_ids.push(detail.student_id);
                }
                _ => {}
            }
        }

        if let Some(target_period_id) = job_row.target_period_id {
            if !target_student_ids.is_empty() {
                let target_enrollments = enrollment::Entity::find()
                    .filter(enrollment::Column::OrgId.eq(org_id))
                    .filter(enrollment::Column::PeriodId.eq(target_period_id))
                    .filter(enrollment::Column::StudentId.is_in(target_student_ids.iter().copied()))
                    .filter(enrollment::Column::EnrollmentType.is_in(["promotion", "repeat"]))
                    .all(&txn)
                    .await
                    .map_err(db_error)?;

                let target_enr_ids: Vec<Uuid> = target_enrollments.iter().map(|e| e.id).collect();
                if !target_enr_ids.is_empty() {
                    let has_grades = grade::Entity::find()
                        .filter(grade::Column::OrgId.eq(org_id))
                        .filter(grade::Column::EnrollmentId.is_in(target_enr_ids.iter().copied()))
                        .one(&txn)
                        .await
                        .map_err(db_error)?
                        .is_some();
                    if has_grades {
                        return Err(DomainError::BusinessRule(
                            "Cannot rollback promotion because grades have already been recorded in the target period".into(),
                        ));
                    }

                    let active_fees = student_fee::Entity::find()
                        .filter(student_fee::Column::OrgId.eq(org_id))
                        .filter(
                            student_fee::Column::EnrollmentId.is_in(target_enr_ids.iter().copied()),
                        )
                        .filter(student_fee::Column::Status.ne("cancelled"))
                        .all(&txn)
                        .await
                        .map_err(db_error)?;
                    if active_fees
                        .iter()
                        .any(|f| f.paid_amount.unwrap_or_default() > rust_decimal::Decimal::ZERO)
                    {
                        return Err(DomainError::BusinessRule(
                            "Cannot rollback promotion because payments have been applied to target period fees".into(),
                        ));
                    }
                    if !active_fees.is_empty() {
                        return Err(DomainError::BusinessRule(
                            "Cannot rollback promotion while target period fees are active; cancel target fees first".into(),
                        ));
                    }

                    enrollment::Entity::delete_many()
                        .filter(enrollment::Column::Id.is_in(target_enr_ids))
                        .filter(enrollment::Column::OrgId.eq(org_id))
                        .exec(&txn)
                        .await
                        .map_err(db_error)?;
                }
            }
        }

        if !restore_enrollment_ids.is_empty() {
            for batch in restore_enrollment_ids.chunks(200) {
                enrollment::Entity::update_many()
                    .set(enrollment::ActiveModel {
                        status: Set("active".into()),
                        updated_at: Set(updated_at.into()),
                        ..Default::default()
                    })
                    .filter(enrollment::Column::Id.is_in(batch.iter().copied()))
                    .filter(enrollment::Column::OrgId.eq(org_id))
                    .filter(enrollment::Column::PeriodId.eq(job_row.source_period_id))
                    .exec(&txn)
                    .await
                    .map_err(db_error)?;
            }
        }

        if !revert_graduated_student_ids.is_empty() {
            for batch in revert_graduated_student_ids.chunks(200) {
                student::Entity::update_many()
                    .set(student::ActiveModel {
                        status: Set("active".into()),
                        graduated_at: Set(None),
                        updated_at: Set(updated_at.into()),
                        ..Default::default()
                    })
                    .filter(student::Column::Id.is_in(batch.iter().copied()))
                    .filter(student::Column::OrgId.eq(org_id))
                    .filter(student::Column::Status.eq("graduated"))
                    .exec(&txn)
                    .await
                    .map_err(db_error)?;
            }
        }

        txn.commit().await.map_err(db_error)?;

        let mut job = map_job_model(job_row);
        job.status = ProgressionJobStatus::RolledBack;
        job.updated_at = updated_at;
        Ok(job)
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

fn map_job_model(m: progression_job::Model) -> ProgressionJob {
    ProgressionJob {
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
