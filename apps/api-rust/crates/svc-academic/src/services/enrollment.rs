//! Enrollment service — student enrollment lifecycle.

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use sea_orm::EntityTrait;

use shared_kernel::errors::AppError;
use crate::entities::enrollment;
use crate::repository::{AcademicCommandRepo, AcademicQueryRepo};

/// Enrollment service.
pub struct EnrollmentService {
    pub cmd: Arc<AcademicCommandRepo>,
    pub query: Arc<AcademicQueryRepo>,
}

impl EnrollmentService {
    pub fn new(cmd: Arc<AcademicCommandRepo>, query: Arc<AcademicQueryRepo>) -> Self {
        Self { cmd, query }
    }

    /// Enroll a student in a class.
    pub async fn enroll(
        &self,
        org_id: &str,
        period_id: &str,
        student_id: &str,
        class_id: &str,
        student_number: &str,
    ) -> Result<enrollment::Model, AppError> {
        // Check for duplicate enrollment
        if let Some(_existing) = self.query.get_enrollment_by_student(student_id, period_id).await? {
            return Err(AppError::conflict("Student is already enrolled in this period"));
        }

        // Check class capacity
        let class = self.query.get_class_by_id(class_id).await?
            .ok_or_else(|| AppError::not_found("class", class_id))?;

        if let Some(cap) = class.capacity {
            let current = self.query.count_enrollments_by_class(class_id, period_id).await?;
            if current as i32 >= cap {
                return Err(AppError::conflict(format!("Class '{}' is at capacity ({}/{})", class.name, current, cap)));
            }
        }

        let now = Utc::now().to_rfc3339();
        let enrollment = enrollment::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            period_id: period_id.to_string(),
            student_id: student_id.to_string(),
            class_id: class_id.to_string(),
            student_number: student_number.to_string(),
            status: "active".to_string(),
            enrollment_date: now.clone(),
            enrollment_type: "regular".to_string(),
            withdrawal_date: None,
            withdrawal_reason: None,
            final_grade: None,
            final_score: None,
            final_rank: None,
            is_promoted: None,
            promotion_decision: None,
            promotion_notes: None,
            meta: None,
            created_at: now.clone(),
            updated_at: now,
        };

        Ok(self.cmd.create_enrollment(enrollment).await?)
    }

    /// Transfer a student to a different class.
    pub async fn transfer(
        &self,
        enrollment_id: &str,
        new_class_id: &str,
    ) -> Result<enrollment::Model, AppError> {
        use sea_orm::ActiveValue::Set;

        let existing = self.query.get_class_by_id(enrollment_id).await?;
        if existing.is_none() {
            return Err(AppError::not_found("enrollment", enrollment_id));
        }

        // Verify new class exists
        self.query.get_class_by_id(new_class_id).await?
            .ok_or_else(|| AppError::not_found("class", new_class_id))?;

        // Load enrollment and update
        let enrollment = enrollment::Entity::find_by_id(enrollment_id.to_string())
            .one(&self.query.db)
            .await?
            .ok_or_else(|| AppError::not_found("enrollment", enrollment_id))?;

        let mut active: enrollment::ActiveModel = enrollment.into();
        active.class_id = Set(new_class_id.to_string());
        active.enrollment_type = Set("transfer".to_string());
        active.updated_at = Set(Utc::now().to_rfc3339());

        Ok(self.cmd.update_enrollment(active).await?)
    }

    /// Withdraw a student from enrollment.
    pub async fn withdraw(
        &self,
        enrollment_id: &str,
        reason: &str,
    ) -> Result<enrollment::Model, AppError> {
        use sea_orm::ActiveValue::Set;

        let enrollment: enrollment::Model = enrollment::Entity::find_by_id(enrollment_id.to_string())
            .one(&self.query.db)
            .await?
            .ok_or_else(|| AppError::not_found("enrollment", enrollment_id))?;

        let mut active: enrollment::ActiveModel = enrollment.into();
        active.status = Set("withdrawn".to_string());
        active.withdrawal_date = Set(Some(Utc::now().to_rfc3339()));
        active.withdrawal_reason = Set(Some(reason.to_string()));
        active.updated_at = Set(Utc::now().to_rfc3339());

        Ok(self.cmd.update_enrollment(active).await?)
    }
}
