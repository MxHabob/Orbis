use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder, Set, DbErr,
};

use crate::entities::{promotion_job, promotion_detail, student_registry_entry};

/// Repository for promotion operations against the registry database.
pub struct PromotionRepo;

impl PromotionRepo {
    // ── Promotion Jobs ──────────────────────────────────────────────

    /// List all promotion jobs for an organization.
    pub async fn get_jobs(
        db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<Vec<promotion_job::Model>, DbErr> {
        promotion_job::Entity::find()
            .filter(promotion_job::Column::OrgId.eq(org_id))
            .order_by_desc(promotion_job::Column::CreatedAt)
            .all(db)
            .await
    }

    /// Get a single promotion job by ID.
    pub async fn get_job_by_id(
        db: &DatabaseConnection,
        job_id: &str,
    ) -> Result<Option<promotion_job::Model>, DbErr> {
        promotion_job::Entity::find_by_id(job_id).one(db).await
    }

    /// Create a new promotion job.
    pub async fn create_job(
        db: &DatabaseConnection,
        job: promotion_job::Model,
    ) -> Result<promotion_job::Model, DbErr> {
        let active: promotion_job::ActiveModel = job.into();
        active.insert(db).await
    }

    /// Update promotion job status and optionally set associated fields.
    pub async fn update_job_status(
        db: &DatabaseConnection,
        job_id: &str,
        status: &str,
    ) -> Result<promotion_job::Model, DbErr> {
        let job = promotion_job::Entity::find_by_id(job_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("promotion job not found".into()))?;

        let now = chrono::Utc::now().to_rfc3339();
        let mut active: promotion_job::ActiveModel = job.into();
        active.status = Set(status.to_string());
        active.updated_at = Set(now.clone());

        match status {
            "executing" => {
                active.started_at = Set(Some(now));
            }
            "completed" => {
                active.completed_at = Set(Some(now));
            }
            "failed" => {
                active.completed_at = Set(Some(now));
            }
            "rolled_back" => {
                active.rolled_back_at = Set(Some(now));
                active.rollback_available = Set(0);
            }
            _ => {}
        }

        active.update(db).await
    }

    /// Update job with dry run results.
    pub async fn save_dry_run_result(
        db: &DatabaseConnection,
        job_id: &str,
        result_json: &str,
    ) -> Result<(), DbErr> {
        let job = promotion_job::Entity::find_by_id(job_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("promotion job not found".into()))?;

        let now = chrono::Utc::now().to_rfc3339();
        let mut active: promotion_job::ActiveModel = job.into();
        active.status = Set("dry_run".to_string());
        active.dry_run_result = Set(Some(result_json.to_string()));
        active.dry_run_at = Set(Some(now.clone()));
        active.updated_at = Set(now);
        active.update(db).await?;
        Ok(())
    }

    /// Update job with execution statistics.
    pub async fn save_execution_stats(
        db: &DatabaseConnection,
        job_id: &str,
        total: i32,
        promoted: i32,
        retained: i32,
        graduated: i32,
        withdrawn: i32,
        excluded: i32,
        target_year_id: &str,
        backup_path: &str,
    ) -> Result<(), DbErr> {
        let job = promotion_job::Entity::find_by_id(job_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("promotion job not found".into()))?;

        let now = chrono::Utc::now().to_rfc3339();
        let mut active: promotion_job::ActiveModel = job.into();
        active.total_students = Set(Some(total));
        active.promoted_count = Set(Some(promoted));
        active.retained_count = Set(Some(retained));
        active.graduated_count = Set(Some(graduated));
        active.withdrawn_count = Set(Some(withdrawn));
        active.excluded_count = Set(Some(excluded));
        active.target_year_id = Set(Some(target_year_id.to_string()));
        active.backup_path = Set(Some(backup_path.to_string()));
        active.updated_at = Set(now);
        active.update(db).await?;
        Ok(())
    }

    /// Set error log on a job.
    pub async fn set_error_log(
        db: &DatabaseConnection,
        job_id: &str,
        error: &str,
    ) -> Result<(), DbErr> {
        let job = promotion_job::Entity::find_by_id(job_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("promotion job not found".into()))?;

        let mut active: promotion_job::ActiveModel = job.into();
        active.error_log = Set(Some(error.to_string()));
        active.updated_at = Set(chrono::Utc::now().to_rfc3339());
        active.update(db).await?;
        Ok(())
    }

    /// Set rollback info on a completed job.
    pub async fn set_rolled_back(
        db: &DatabaseConnection,
        job_id: &str,
        user_id: &str,
    ) -> Result<(), DbErr> {
        let job = promotion_job::Entity::find_by_id(job_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("promotion job not found".into()))?;

        let now = chrono::Utc::now().to_rfc3339();
        let mut active: promotion_job::ActiveModel = job.into();
        active.status = Set("rolled_back".to_string());
        active.rolled_back_at = Set(Some(now.clone()));
        active.rolled_back_by = Set(Some(user_id.to_string()));
        active.rollback_available = Set(0);
        active.updated_at = Set(now);
        active.update(db).await?;
        Ok(())
    }

    // ── Promotion Details ───────────────────────────────────────────

    /// Get all promotion details for a job.
    pub async fn get_details(
        db: &DatabaseConnection,
        job_id: &str,
    ) -> Result<Vec<promotion_detail::Model>, DbErr> {
        promotion_detail::Entity::find()
            .filter(promotion_detail::Column::JobId.eq(job_id))
            .order_by_asc(promotion_detail::Column::SourceLevel)
            .order_by_asc(promotion_detail::Column::StudentName)
            .all(db)
            .await
    }

    /// Get details by decision type (promoted, retained, graduated, excluded).
    pub async fn get_details_by_decision(
        db: &DatabaseConnection,
        job_id: &str,
        decision: &str,
    ) -> Result<Vec<promotion_detail::Model>, DbErr> {
        promotion_detail::Entity::find()
            .filter(promotion_detail::Column::JobId.eq(job_id))
            .filter(promotion_detail::Column::Decision.eq(decision))
            .all(db)
            .await
    }

    /// Batch insert promotion details.
    pub async fn create_details_batch(
        db: &DatabaseConnection,
        details: Vec<promotion_detail::Model>,
    ) -> Result<(), DbErr> {
        if details.is_empty() {
            return Ok(());
        }

        let models: Vec<promotion_detail::ActiveModel> =
            details.into_iter().map(|d| d.into()).collect();

        promotion_detail::Entity::insert_many(models)
            .exec(db)
            .await?;
        Ok(())
    }

    /// Override a single promotion detail's decision.
    pub async fn override_detail(
        db: &DatabaseConnection,
        detail_id: &str,
        new_decision: &str,
        reason: &str,
        user_id: &str,
    ) -> Result<promotion_detail::Model, DbErr> {
        let detail = promotion_detail::Entity::find_by_id(detail_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("promotion detail not found".into()))?;

        let mut active: promotion_detail::ActiveModel = detail.clone().into();

        // Save original decision if this is the first override
        if detail.manual_override == 0 {
            active.original_decision = Set(Some(detail.decision.clone()));
        }

        active.decision = Set(new_decision.to_string());
        active.manual_override = Set(1);
        active.override_by = Set(Some(user_id.to_string()));
        active.override_reason = Set(Some(reason.to_string()));

        active.update(db).await
    }

    // ── Student Registry ────────────────────────────────────────────

    /// Get all students in the registry for an organization.
    pub async fn get_student_registry(
        db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<Vec<student_registry_entry::Model>, DbErr> {
        student_registry_entry::Entity::find()
            .filter(student_registry_entry::Column::OrgId.eq(org_id))
            .order_by_asc(student_registry_entry::Column::FullName)
            .all(db)
            .await
    }

    /// Get a student registry entry by user_id.
    pub async fn get_student_registry_entry(
        db: &DatabaseConnection,
        user_id: &str,
    ) -> Result<Option<student_registry_entry::Model>, DbErr> {
        student_registry_entry::Entity::find()
            .filter(student_registry_entry::Column::UserId.eq(user_id))
            .one(db)
            .await
    }

    /// Upsert a student registry entry. Updates if exists, inserts otherwise.
    pub async fn upsert_student_registry(
        db: &DatabaseConnection,
        entry: student_registry_entry::Model,
    ) -> Result<student_registry_entry::Model, DbErr> {
        let existing = student_registry_entry::Entity::find()
            .filter(student_registry_entry::Column::UserId.eq(&entry.user_id))
            .one(db)
            .await?;

        if let Some(existing) = existing {
            let mut active: student_registry_entry::ActiveModel = existing.into();
            active.status = Set(entry.status);
            active.current_year_id = Set(entry.current_year_id);
            active.current_level = Set(entry.current_level);
            active.current_class_name = Set(entry.current_class_name);
            active.years_enrolled = Set(entry.years_enrolled);
            active.full_name = Set(entry.full_name);
            active.name_ar = Set(entry.name_ar);
            active.updated_at = Set(chrono::Utc::now().to_rfc3339());
            active.update(db).await
        } else {
            let active: student_registry_entry::ActiveModel = entry.into();
            active.insert(db).await
        }
    }

    /// Update student registry status (graduated, withdrawn, etc.).
    pub async fn update_student_registry_status(
        db: &DatabaseConnection,
        user_id: &str,
        status: &str,
    ) -> Result<(), DbErr> {
        let entry = student_registry_entry::Entity::find()
            .filter(student_registry_entry::Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("student registry entry not found".into()))?;

        let mut active: student_registry_entry::ActiveModel = entry.into();
        active.status = Set(status.to_string());
        active.updated_at = Set(chrono::Utc::now().to_rfc3339());
        active.update(db).await?;
        Ok(())
    }
}
