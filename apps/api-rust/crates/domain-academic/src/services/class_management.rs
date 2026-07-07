//! Class management service — CRUD + teacher assignment.

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use shared_kernel::errors::AppError;
use crate::entities::class;
use crate::repository::{AcademicCommandRepo, AcademicQueryRepo};

/// Class management service.
pub struct ClassService {
    pub cmd: Arc<AcademicCommandRepo>,
    pub query: Arc<AcademicQueryRepo>,
}

impl ClassService {
    pub fn new(cmd: Arc<AcademicCommandRepo>, query: Arc<AcademicQueryRepo>) -> Self {
        Self { cmd, query }
    }

    /// List classes in a cycle.
    pub async fn list_by_cycle(
        &self,
        org_id: &str,
        cycle_id: &str,
    ) -> Result<Vec<class::Model>, AppError> {
        Ok(self.query.list_classes_by_cycle(org_id, cycle_id).await?)
    }

    /// Get a class by ID.
    pub async fn get(&self, id: &str) -> Result<class::Model, AppError> {
        self.query.get_class_by_id(id).await?
            .ok_or_else(|| AppError::not_found("class", id))
    }

    /// Create a new class.
    pub async fn create(
        &self,
        org_id: &str,
        unit_id: &str,
        cycle_id: &str,
        name: &str,
        level: &str,
        capacity: Option<i32>,
    ) -> Result<class::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        let new_class = class::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            unit_id: unit_id.to_string(),
            cycle_id: cycle_id.to_string(),
            name: name.to_string(),
            name_ar: None,
            level: level.to_string(),
            division: None,
            homeroom_teacher_id: None,
            capacity,
            current_students: Some(0),
            room_id: None,
            color: None,
            is_active: 1,
            is_deleted: 0,
            deleted_at: None,
            created_at: now.clone(),
            updated_at: now,
        };

        Ok(self.cmd.create_class(new_class).await?)
    }

    /// Assign a homeroom teacher to a class.
    pub async fn assign_teacher(
        &self,
        class_id: &str,
        teacher_id: &str,
    ) -> Result<class::Model, AppError> {
        use sea_orm::ActiveValue::Set;

        let existing = self.get(class_id).await?;
        let mut active: class::ActiveModel = existing.into();
        active.homeroom_teacher_id = Set(Some(teacher_id.to_string()));
        active.updated_at = Set(Utc::now().to_rfc3339());

        Ok(self.cmd.update_class(active).await?)
    }
}
