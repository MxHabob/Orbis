use orbis_domain::entities::progression::{PromotionJob, PromotionDetail, PromotionJobStatus, PromotionDecision};
use orbis_domain::entities::enrollment::{Enrollment, EnrollmentStatus};
use orbis_domain::entities::student::StudentStatus;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use orbis_domain::iam::permission::Permission;
use crate::context::RequestContext;
use crate::ports::repositories::{PromotionRepository, EnrollmentRepository, StudentRepository};
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct ProgressionService {
    pub progression_repo: Arc<dyn PromotionRepository>,
    pub enrollment_repo: Arc<dyn EnrollmentRepository>,
    pub student_repo: Arc<dyn StudentRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

#[derive(Debug, Clone)]
pub struct StudentProgressionItem {
    pub student_id: Uuid,
    pub enrollment_id: Uuid,
    pub source_class_id: Option<Uuid>,
    pub target_class_id: Option<Uuid>,
    pub decision: PromotionDecision,
    pub reason: String,
}

impl ProgressionService {
    pub fn new(
        progression_repo: Arc<dyn PromotionRepository>,
        enrollment_repo: Arc<dyn EnrollmentRepository>,
        student_repo: Arc<dyn StudentRepository>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            progression_repo,
            enrollment_repo,
            student_repo,
            event_bus,
            permission_checker,
        }
    }

    /// Execute atomic bulk cohort promotion from source period to target period.
    pub async fn promote_cohort(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        source_period_id: Uuid,
        target_period_id: Uuid,
        job_name: String,
        items: Vec<StudentProgressionItem>,
    ) -> Result<PromotionJob, DomainError> {
        self.permission_checker.check(ctx, Permission::StudentUpdate).await?;

        let now = Utc::now();
        let job_id = Uuid::new_v4();

        let mut promoted_count = 0;
        let mut retained_count = 0;
        let mut graduated_count = 0;
        let mut excluded_count = 0;

        let mut new_enrollments = Vec::new();
        let mut old_enrollment_ids_to_complete = Vec::new();
        let mut details = Vec::new();

        for item in items {
            match item.decision {
                PromotionDecision::Promote => {
                    promoted_count += 1;
                    if let Some(old_enrollment) = self.enrollment_repo.find_by_id(item.enrollment_id).await? {
                        let new_enrollment = Enrollment::new(
                            org_id,
                            target_period_id,
                            item.student_id,
                            item.target_class_id,
                            old_enrollment.student_number.clone(),
                            old_enrollment.study_mode,
                        );
                        new_enrollments.push(new_enrollment);
                        old_enrollment_ids_to_complete.push(item.enrollment_id);
                    }
                }
                PromotionDecision::Retain => {
                    retained_count += 1;
                    if let Some(old_enrollment) = self.enrollment_repo.find_by_id(item.enrollment_id).await? {
                        let new_enrollment = Enrollment::new(
                            org_id,
                            target_period_id,
                            item.student_id,
                            item.source_class_id,
                            old_enrollment.student_number.clone(),
                            old_enrollment.study_mode,
                        );
                        new_enrollments.push(new_enrollment);
                        old_enrollment_ids_to_complete.push(item.enrollment_id);
                    }
                }
                PromotionDecision::Graduate => {
                    graduated_count += 1;
                    if let Some(mut student) = self.student_repo.find_by_id(item.student_id).await? {
                        student.status = StudentStatus::Graduated;
                        student.graduated_at = Some(now);
                        student.updated_at = now;
                        self.student_repo.save(&student).await?;
                    }
                    old_enrollment_ids_to_complete.push(item.enrollment_id);
                }
                PromotionDecision::Exclude => {
                    excluded_count += 1;
                }
            }

            let detail = PromotionDetail {
                id: Uuid::new_v4(),
                job_id,
                student_id: item.student_id,
                enrollment_id: item.enrollment_id,
                source_class_id: item.source_class_id,
                target_class_id: item.target_class_id,
                decision: item.decision,
                reason: item.reason,
                manual_override: false,
                override_by: None,
                override_reason: None,
                final_gpa: None,
                attendance_rate: None,
                created_at: now,
            };
            details.push(detail);
        }

        // 1. Bulk insert new enrollments
        if !new_enrollments.is_empty() {
            self.enrollment_repo.save_batch(&new_enrollments).await?;
        }

        // 2. Bulk update old enrollments
        if !old_enrollment_ids_to_complete.is_empty() {
            self.enrollment_repo.update_status_batch(&old_enrollment_ids_to_complete, EnrollmentStatus::Graduated).await?;
        }

        // 3. Save promotion job and audit details
        let total_students = details.len() as u32;
        let job = PromotionJob {
            id: job_id,
            org_id,
            source_period_id,
            target_period_id: Some(target_period_id),
            name: job_name,
            status: PromotionJobStatus::Completed,
            total_students,
            promoted_count,
            retained_count,
            graduated_count,
            excluded_count,
            executed_by: ctx.user_id(),
            executed_at: Some(now),
            created_at: now,
            updated_at: now,
        };

        self.progression_repo.save_job(&job).await?;
        self.progression_repo.save_details(&details).await?;

        // 4. Publish Event
        let _ = self.event_bus.publish(ctx, &DomainEvent::CohortPromoted {
            job_id,
            org_id,
            promoted_count,
            timestamp: now,
        }).await;

        Ok(job)
    }
}
