use orbis_domain::entities::progression::{PromotionJob, PromotionDetail, PromotionJobStatus, PromotionDecision};
use orbis_domain::entities::enrollment::{Enrollment, EnrollmentStatus};
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::{PromotionRepository, EnrollmentRepository, StudentRepository};
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use orbis_domain::iam::permission::Permission;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct PromoteCohortUseCase {
    pub progression_repo: Arc<dyn PromotionRepository>,
    pub enrollment_repo: Arc<dyn EnrollmentRepository>,
    pub student_repo: Arc<dyn StudentRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl PromoteCohortUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        source_period_id: Uuid,
        target_period_id: Uuid,
        job_name: String,
        decisions: Vec<(Uuid, Uuid, Option<Uuid>, Option<Uuid>, PromotionDecision, String)>, // (student_id, enrollment_id, source_class_id, target_class_id, decision, reason)
    ) -> Result<PromotionJob, DomainError> {
        self.permission_checker.check(ctx, Permission::StudentUpdate).await?;

        let now = Utc::now();
        let job_id = Uuid::new_v4();

        let mut promoted_count = 0;
        let mut retained_count = 0;
        let mut graduated_count = 0;
        let mut excluded_count = 0;

        let mut details = Vec::new();

        for (student_id, enrollment_id, source_class_id, target_class_id, decision, reason) in decisions {
            match decision {
                PromotionDecision::Promote => {
                    promoted_count += 1;
                    // Create new enrollment in target period if promoted
                    if let Some(mut old_enrollment) = self.enrollment_repo.find_by_id(enrollment_id).await? {
                        let new_enrollment = Enrollment::new(
                            org_id,
                            target_period_id,
                            student_id,
                            target_class_id,
                            old_enrollment.student_number.clone(),
                            old_enrollment.study_mode,
                        );
                        self.enrollment_repo.save(&new_enrollment).await?;

                        old_enrollment.status = EnrollmentStatus::Graduated; // Completed term
                        old_enrollment.updated_at = now;
                        self.enrollment_repo.save(&old_enrollment).await?;
                    }
                }
                PromotionDecision::Retain => {
                    retained_count += 1;
                    // Retain in same level/class in new period
                    if let Some(old_enrollment) = self.enrollment_repo.find_by_id(enrollment_id).await? {
                        let new_enrollment = Enrollment::new(
                            org_id,
                            target_period_id,
                            student_id,
                            source_class_id,
                            old_enrollment.student_number.clone(),
                            old_enrollment.study_mode,
                        );
                        self.enrollment_repo.save(&new_enrollment).await?;
                    }
                }
                PromotionDecision::Graduate => {
                    graduated_count += 1;
                    if let Some(mut student) = self.student_repo.find_by_id(student_id).await? {
                        student.status = orbis_domain::entities::student::StudentStatus::Graduated;
                        student.graduated_at = Some(now);
                        student.updated_at = now;
                        self.student_repo.save(&student).await?;
                    }
                }
                PromotionDecision::Exclude => {
                    excluded_count += 1;
                }
            }

            let detail = PromotionDetail {
                id: Uuid::new_v4(),
                job_id,
                student_id,
                enrollment_id,
                source_class_id,
                target_class_id,
                decision,
                reason,
                manual_override: false,
                override_by: None,
                override_reason: None,
                final_gpa: None,
                attendance_rate: None,
                created_at: now,
            };
            details.push(detail);
        }

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

        let event = DomainEvent::CohortPromoted {
            job_id,
            org_id,
            promoted_count,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(job)
    }
}
