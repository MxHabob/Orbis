//! Academic cycle service — lifecycle operations for cycles.
//!
//! Manages year/semester/quarter hierarchy, locking, grade entry windows,
//! and enrollment state transitions.

use std::sync::Arc;
use chrono::Utc;

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
            return Err(AppError::Conflict {
                entity: "academic_cycle".into(),
                details: "Cycle is already locked".into(),
            });
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
            return Err(AppError::Conflict {
                entity: "academic_cycle".into(),
                details: "Cycle is not locked".into(),
            });
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
}
