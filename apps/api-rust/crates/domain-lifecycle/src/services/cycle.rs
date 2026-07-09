//! Academic cycle service — lifecycle operations for cycles.
//!
//! Manages year/semester/quarter hierarchy, locking, grade entry windows,
//! and enrollment state transitions.

use std::sync::Arc;
use chrono::Utc;
use sea_orm::{DbErr, EntityTrait, ActiveModelTrait};

use shared_kernel::errors::AppError;
use crate::entities::academic_cycle;
use crate::repository::{LifecycleCommandRepo, LifecycleQueryRepo};

/// Cycle service.
pub struct CycleService {
    pub cmd: Arc<LifecycleCommandRepo>,
    pub query: Arc<LifecycleQueryRepo>,
}

impl CycleService {
    pub fn new(cmd: Arc<LifecycleCommandRepo>, query: Arc<LifecycleQueryRepo>) -> Self {
        Self { cmd, query }
    }

    /// Get a cycle by ID.
    pub async fn get(&self, id: &str) -> Result<academic_cycle::Model, AppError> {
        self.query.get_cycle_by_id(id).await?
            .ok_or_else(|| AppError::not_found("academic_cycle", id))
    }

    /// List all cycles for an organization.
    pub async fn list_by_org(&self, org_id: &str) -> Result<Vec<academic_cycle::Model>, AppError> {
        Ok(self.query.list_cycles_by_org(org_id).await?)
    }

    /// Get the current active cycle.
    pub async fn get_current(&self, org_id: &str) -> Result<academic_cycle::Model, AppError> {
        self.query.get_current_cycle(org_id).await?
            .ok_or_else(|| AppError::not_found("academic_cycle", "current"))
    }

    /// Get child cycles (semesters of a year, quarters of a semester).
    pub async fn get_children(&self, parent_id: &str) -> Result<Vec<academic_cycle::Model>, AppError> {
        Ok(self.query.get_child_cycles(parent_id).await?)
    }

    /// Lock a cycle (prevent modifications to grades/enrollment).
    pub async fn lock(&self, cycle_id: &str, locked_by: &str) -> Result<academic_cycle::Model, AppError> {
        use sea_orm::ActiveValue::Set;

        let cycle = self.get(cycle_id).await?;

        if cycle.is_locked == 1 {
            return Err(AppError::conflict("Cycle is already locked"));
        }

        let mut active: academic_cycle::ActiveModel = cycle.into();
        active.is_locked = Set(1);
        active.locked_at = Set(Some(Utc::now().to_rfc3339()));
        active.locked_by = Set(Some(locked_by.to_string()));
        active.updated_at = Set(Utc::now().to_rfc3339());

        Ok(self.cmd.update_cycle(active).await?)
    }

    /// Unlock a cycle.
    pub async fn unlock(&self, cycle_id: &str) -> Result<academic_cycle::Model, AppError> {
        use sea_orm::ActiveValue::Set;

        let cycle = self.get(cycle_id).await?;

        if cycle.is_locked == 0 {
            return Err(AppError::conflict("Cycle is not locked"));
        }

        let mut active: academic_cycle::ActiveModel = cycle.into();
        active.is_locked = Set(0);
        active.locked_at = Set(None);
        active.locked_by = Set(None);
        active.updated_at = Set(Utc::now().to_rfc3339());

        Ok(self.cmd.update_cycle(active).await?)
    }

    /// Toggle enrollment open/close for a cycle.
    pub async fn set_enrollment_open(
        &self,
        cycle_id: &str,
        open: bool,
    ) -> Result<academic_cycle::Model, AppError> {
        use sea_orm::ActiveValue::Set;

        let cycle = self.get(cycle_id).await?;
        let mut active: academic_cycle::ActiveModel = cycle.into();
        active.is_enrollment_open = Set(if open { 1 } else { 0 });
        active.updated_at = Set(Utc::now().to_rfc3339());

        Ok(self.cmd.update_cycle(active).await?)
    }

    /// Toggle grade entry window open/close for a cycle.
    pub async fn set_grade_entry_open(
        &self,
        cycle_id: &str,
        open: bool,
    ) -> Result<academic_cycle::Model, AppError> {
        use sea_orm::ActiveValue::Set;

        let cycle = self.get(cycle_id).await?;
        let mut active: academic_cycle::ActiveModel = cycle.into();
        active.is_grade_entry_open = Set(if open { 1 } else { 0 });
        active.updated_at = Set(Utc::now().to_rfc3339());

        Ok(self.cmd.update_cycle(active).await?)
    }

    /// Create a new cycle.
    pub async fn create(
        &self,
        org_id: &str,
        name: &str,
        name_ar: Option<&str>,
        cycle_type: &str,
        parent_id: Option<&str>,
        start_date: &str,
        end_date: &str,
        exam_start_date: Option<&str>,
        exam_end_date: Option<&str>,
        weight: Option<i32>,
    ) -> Result<academic_cycle::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        let model = academic_cycle::Model {
            id: uuid::Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            name: name.to_string(),
            name_ar: name_ar.map(|s| s.to_string()),
            code: None,
            cycle_type: cycle_type.to_string(),
            parent_id: parent_id.map(|s| s.to_string()),
            start_date: start_date.to_string(),
            end_date: end_date.to_string(),
            hijri_start_date: None,
            hijri_end_date: None,
            exam_start_date: exam_start_date.map(|s| s.to_string()),
            exam_end_date: exam_end_date.map(|s| s.to_string()),
            weight,
            grading_system: "saudi_5".to_string(),
            passing_grade: Some(50),
            max_grade: Some(100),
            is_current: 0,
            is_locked: 0,
            is_enrollment_open: 1,
            is_grade_entry_open: 0,
            grade_entry_start: None,
            grade_entry_end: None,
            locked_at: None,
            locked_by: None,
            settings: None,
            sort_order: None,
            created_at: now.clone(),
            updated_at: now,
        };
        Ok(self.cmd.create_cycle(model).await?)
    }

    /// Update an existing cycle.
    pub async fn update(
        &self,
        cycle_id: &str,
        name: &str,
        name_ar: Option<&str>,
        start_date: &str,
        end_date: &str,
        exam_start_date: Option<&str>,
        exam_end_date: Option<&str>,
        weight: Option<i32>,
    ) -> Result<academic_cycle::Model, AppError> {
        use sea_orm::ActiveValue::Set;

        let cycle = self.get(cycle_id).await?;
        let mut active: academic_cycle::ActiveModel = cycle.into();
        active.name = Set(name.to_string());
        active.name_ar = Set(name_ar.map(|s| s.to_string()));
        active.start_date = Set(start_date.to_string());
        active.end_date = Set(end_date.to_string());
        active.exam_start_date = Set(exam_start_date.map(|s| s.to_string()));
        active.exam_end_date = Set(exam_end_date.map(|s| s.to_string()));
        active.weight = Set(weight);
        active.updated_at = Set(Utc::now().to_rfc3339());

        Ok(self.cmd.update_cycle(active).await?)
    }

    /// Activate a cycle.
    pub async fn activate(&self, org_id: &str, cycle_id: &str) -> Result<academic_cycle::Model, AppError> {
        use sea_orm::ActiveValue::Set;
        use sea_orm::TransactionTrait;
        use sea_orm::ConnectionTrait;
        use sea_orm::Statement;

        let db = &self.cmd.db;
        let cycle_id = cycle_id.to_string();
        let org_id = org_id.to_string();

        let updated_cycle = db.transaction::<_, academic_cycle::Model, DbErr>(move |txn| {
            let cycle_id = cycle_id.clone();
            let org_id = org_id.clone();
            Box::pin(async move {
                txn.execute(Statement::from_sql_and_values(
                    txn.get_database_backend(),
                    "UPDATE academic_cycles SET is_current = 0 WHERE org_id = ?",
                    vec![org_id.into()]
                )).await?;
                
                let cycle = academic_cycle::Entity::find_by_id(&cycle_id)
                    .one(txn)
                    .await?
                    .ok_or_else(|| DbErr::Custom("Cycle not found".to_string()))?;
                let mut active: academic_cycle::ActiveModel = cycle.into();
                active.is_current = Set(1);
                active.updated_at = Set(Utc::now().to_rfc3339());
                let res = active.update(txn).await?;
                Ok(res)
            })
        }).await.map_err(|e| AppError::Database(e.to_string()))?;

        Ok(updated_cycle)
    }
}

