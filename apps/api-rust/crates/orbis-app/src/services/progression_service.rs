use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{EnrollmentRepository, PromotionRepository, StudentRepository};
use chrono::Utc;
use orbis_domain::entities::enrollment::Enrollment;
use orbis_domain::entities::progression::{
    ProgressionDetail, ProgressionJob, ProgressionJobStatus, PromotionDecision,
};
use orbis_domain::entities::student::{AcademicStanding, StudentStatus};
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::iam::permission::Permission;
use std::sync::Arc;
use uuid::Uuid;

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
    pub student_number: Option<String>,
    pub enrollment_id: Uuid,
    pub source_section_id: Option<Uuid>,
    pub target_section_id: Option<Uuid>,
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
    ) -> Result<ProgressionJob, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentUpdate)
            .await?;

        if org_id != ctx.tenant_id || source_period_id == target_period_id {
            return Err(DomainError::Validation(
                "Invalid promotion scope or target period".into(),
            ));
        }

        let now = Utc::now();
        let job_id = Uuid::new_v4();

        let mut promoted_count = 0;
        let mut retained_count = 0;
        let mut graduated_count = 0;
        let mut excluded_count = 0;

        let mut new_enrollments = Vec::new();
        let mut details = Vec::new();

        for item in items {
            let enrollment = self
                .enrollment_repo
                .find_by_id(item.enrollment_id)
                .await?
                .filter(|enrollment| {
                    enrollment.org_id == org_id
                        && enrollment.period_id == source_period_id
                        && enrollment.student_id == item.student_id
                        && enrollment.section_id == item.source_section_id
                })
                .ok_or_else(|| DomainError::NotFound("Source enrollment not found".into()))?;
            match item.decision {
                PromotionDecision::Promote | PromotionDecision::Retain => {
                    let target_section = if item.decision == PromotionDecision::Promote {
                        promoted_count += 1;
                        item.target_section_id
                    } else {
                        retained_count += 1;
                        enrollment.section_id
                    };
                    new_enrollments.push(Enrollment::new(
                        org_id,
                        target_period_id,
                        enrollment.student_id,
                        target_section,
                        enrollment.student_number,
                        enrollment.study_mode,
                    ));
                }
                PromotionDecision::Graduate => graduated_count += 1,
                PromotionDecision::Exclude => excluded_count += 1,
            }

            let detail = ProgressionDetail {
                id: Uuid::new_v4(),
                job_id,
                student_id: item.student_id,
                enrollment_id: item.enrollment_id,
                source_section_id: item.source_section_id,
                target_section_id: item.target_section_id,
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

        let total_students = details.len() as u32;
        let job = ProgressionJob {
            id: job_id,
            org_id,
            source_period_id,
            target_period_id: Some(target_period_id),
            name: job_name,
            status: ProgressionJobStatus::Completed,
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

        self.progression_repo
            .commit_promotion(&job, &details, &new_enrollments)
            .await?;

        // 4. Publish Event
        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::CohortPromoted {
                    job_id,
                    org_id,
                    promoted_count,
                    timestamp: now,
                },
            )
            .await;

        Ok(job)
    }

    pub async fn list_jobs(
        &self,
        ctx: &RequestContext,
    ) -> Result<Vec<ProgressionJob>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentRead)
            .await?;
        self.progression_repo.find_jobs_by_org(ctx.tenant_id).await
    }

    pub async fn get_job(
        &self,
        ctx: &RequestContext,
        id: Uuid,
    ) -> Result<ProgressionJob, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentRead)
            .await?;
        self.progression_repo
            .find_job_by_id(id)
            .await?
            .filter(|job| job.org_id == ctx.tenant_id)
            .ok_or_else(|| DomainError::NotFound("Promotion job not found".into()))
    }

    /// Evaluate cohort progression eligibility (Dry-run preview)
    pub async fn evaluate_cohort_progression(
        &self,
        ctx: &RequestContext,
        _org_id: Uuid,
        source_period_id: Uuid,
    ) -> Result<Vec<StudentProgressionItem>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentRead)
            .await?;

        let enrollments = self
            .enrollment_repo
            .find_by_period(source_period_id)
            .await?;
        let mut items = Vec::new();

        for enr in enrollments
            .into_iter()
            .filter(|enrollment| enrollment.org_id == ctx.tenant_id)
        {
            let student_opt = self
                .student_repo
                .find_by_id(enr.student_id)
                .await?
                .filter(|student| student.org_id == ctx.tenant_id);
            let student_number = student_opt
                .as_ref()
                .map(|student| student.student_number.clone());
            let (decision, reason) = if let Some(student) = student_opt {
                match student.status {
                    StudentStatus::Active => match student.standing {
                        AcademicStanding::GoodStanding | AcademicStanding::HonorList => (
                            PromotionDecision::Promote,
                            "Eligible for advancement to next grade/level".to_string(),
                        ),
                        AcademicStanding::AcademicWarning1 | AcademicStanding::AcademicWarning2 => {
                            (
                                PromotionDecision::Promote,
                                format!(
                                    "Eligible for advancement under academic warning ({})",
                                    student.standing
                                ),
                            )
                        }
                        AcademicStanding::AcademicWarning3 | AcademicStanding::Probation => (
                            PromotionDecision::Retain,
                            format!(
                                "Academic standing requires retention or review ({})",
                                student.standing
                            ),
                        ),
                        AcademicStanding::Dismissed => (
                            PromotionDecision::Exclude,
                            "Student dismissed for academic reasons".to_string(),
                        ),
                    },
                    StudentStatus::Graduated => (
                        PromotionDecision::Exclude,
                        "Student has already graduated".to_string(),
                    ),
                    StudentStatus::Inactive => (
                        PromotionDecision::Exclude,
                        "Student registration is incomplete".to_string(),
                    ),
                    StudentStatus::Withdrawn
                    | StudentStatus::Expelled
                    | StudentStatus::TransferredOut => (
                        PromotionDecision::Exclude,
                        format!("Student is currently inactive ({})", student.status),
                    ),
                    StudentStatus::Suspended => (
                        PromotionDecision::Retain,
                        "Student is currently suspended — requires review".to_string(),
                    ),
                    StudentStatus::OnLeave => (
                        PromotionDecision::Retain,
                        "Student is currently on leave — requires review before progression"
                            .to_string(),
                    ),
                }
            } else {
                (
                    PromotionDecision::Promote,
                    "Regular progression".to_string(),
                )
            };

            items.push(StudentProgressionItem {
                student_id: enr.student_id,
                student_number,
                enrollment_id: enr.id,
                source_section_id: enr.section_id,
                target_section_id: None,
                decision,
                reason,
            });
        }

        Ok(items)
    }
}
