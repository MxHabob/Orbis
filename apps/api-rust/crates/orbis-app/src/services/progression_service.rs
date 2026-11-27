use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{
    AcademicPeriodRepository, AttendanceRepository, EnrollmentRepository, GpaRecordRepository,
    GradeRepository, OrganizationRepository, PolicyRepository, PromotionRepository,
    SectionRepository, StudentFeeRepository, StudentRepository,
};
use crate::services::finance_service::FinanceService;
use chrono::Utc;
use orbis_domain::entities::academic_period::AcademicPeriod;
use orbis_domain::entities::country_profile::CountryProfile;
use orbis_domain::entities::enrollment::{Enrollment, EnrollmentStatus};
use orbis_domain::entities::organization::InstitutionType;
use orbis_domain::entities::progression::{
    ProgressionDetail, ProgressionJob, ProgressionJobStatus, PromotionDecision,
};
use orbis_domain::entities::section::Section;
use orbis_domain::entities::settings::{
    AcademicSettings, FinancialClearanceMode, OrganizationSettings,
};
use orbis_domain::entities::student::{AcademicStanding, Student, StudentStatus};
use orbis_domain::entities::student_fee::FeeStatus;
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::iam::permission::Permission;
use orbis_domain::queries::filters::{AttendanceFilter, GradeFilter, PaginationParams};
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

pub struct ProgressionService {
    pub progression_repo: Arc<dyn PromotionRepository>,
    pub enrollment_repo: Arc<dyn EnrollmentRepository>,
    pub student_repo: Arc<dyn StudentRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
    pub org_repo: Option<Arc<dyn OrganizationRepository>>,
    pub period_repo: Option<Arc<dyn AcademicPeriodRepository>>,
    pub section_repo: Option<Arc<dyn SectionRepository>>,
    pub policy_repo: Option<Arc<dyn PolicyRepository>>,
    pub grade_repo: Option<Arc<dyn GradeRepository>>,
    pub attendance_repo: Option<Arc<dyn AttendanceRepository>>,
    pub gpa_repo: Option<Arc<dyn GpaRecordRepository>>,
    pub fee_repo: Option<Arc<dyn StudentFeeRepository>>,
    pub finance_service: Option<Arc<FinanceService>>,
}

#[derive(Debug, Clone)]
pub struct StudentProgressionItem {
    pub student_id: Uuid,
    pub student_number: Option<String>,
    pub student_name: Option<String>,
    pub student_name_ar: Option<String>,
    pub enrollment_id: Uuid,
    pub source_section_id: Option<Uuid>,
    pub target_section_id: Option<Uuid>,
    pub decision: PromotionDecision,
    pub reason: String,
    pub final_gpa: Option<Decimal>,
    pub overall_percentage: Option<Decimal>,
    pub attendance_rate: Option<Decimal>,
    pub failed_subjects_count: u32,
    pub financial_balance_due: Decimal,
    pub has_financial_hold: bool,
    pub already_enrolled_in_target: bool,
    pub manual_override: bool,
    pub override_reason: Option<String>,
}

struct EvaluatedMetrics {
    final_gpa: Option<Decimal>,
    overall_percentage: Option<Decimal>,
    attendance_rate: Option<Decimal>,
    failed_subjects_count: u32,
    financial_balance_due: Decimal,
    has_financial_hold: bool,
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
            org_repo: None,
            period_repo: None,
            section_repo: None,
            policy_repo: None,
            grade_repo: None,
            attendance_repo: None,
            gpa_repo: None,
            fee_repo: None,
            finance_service: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_erp_dependencies(
        mut self,
        org_repo: Arc<dyn OrganizationRepository>,
        period_repo: Arc<dyn AcademicPeriodRepository>,
        section_repo: Arc<dyn SectionRepository>,
        policy_repo: Arc<dyn PolicyRepository>,
        grade_repo: Arc<dyn GradeRepository>,
        attendance_repo: Arc<dyn AttendanceRepository>,
        gpa_repo: Arc<dyn GpaRecordRepository>,
        fee_repo: Arc<dyn StudentFeeRepository>,
        finance_service: Arc<FinanceService>,
    ) -> Self {
        self.org_repo = Some(org_repo);
        self.period_repo = Some(period_repo);
        self.section_repo = Some(section_repo);
        self.policy_repo = Some(policy_repo);
        self.grade_repo = Some(grade_repo);
        self.attendance_repo = Some(attendance_repo);
        self.gpa_repo = Some(gpa_repo);
        self.fee_repo = Some(fee_repo);
        self.finance_service = Some(finance_service);
        self
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
        self.promote_cohort_with_options(
            ctx,
            org_id,
            source_period_id,
            target_period_id,
            job_name,
            items,
            None,
        )
        .await
    }

    /// Execute atomic bulk cohort promotion with explicit ERP options (such as fee assessment).
    #[allow(clippy::too_many_arguments)]
    pub async fn promote_cohort_with_options(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        source_period_id: Uuid,
        target_period_id: Uuid,
        job_name: String,
        items: Vec<StudentProgressionItem>,
        auto_assess_fees_override: Option<bool>,
    ) -> Result<ProgressionJob, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentUpdate)
            .await?;

        if org_id != ctx.tenant_id || source_period_id == target_period_id {
            return Err(DomainError::Validation(
                "Invalid promotion scope or target period".into(),
            ));
        }
        if job_name.trim().is_empty() {
            return Err(DomainError::Validation(
                "Promotion job name is required".into(),
            ));
        }
        if items.is_empty() {
            return Err(DomainError::Validation(
                "At least one student progression item is required".into(),
            ));
        }

        let settings = self.load_settings(org_id).await;
        let auto_assess_fees =
            auto_assess_fees_override.unwrap_or(settings.promotion.auto_assess_target_fees);

        // Batch-load source enrollments to avoid N+1 queries during commit preparation.
        let source_enrollments: HashMap<Uuid, Enrollment> = self
            .enrollment_repo
            .find_by_period(source_period_id)
            .await?
            .into_iter()
            .filter(|e| e.org_id == org_id)
            .map(|e| (e.id, e))
            .collect();

        let now = Utc::now();
        let job_id = Uuid::new_v4();

        let mut promoted_count = 0;
        let mut retained_count = 0;
        let mut graduated_count = 0;
        let mut excluded_count = 0;

        let mut seen_students = HashSet::new();
        let mut new_enrollments = Vec::new();
        let mut details = Vec::new();

        for item in items {
            if !seen_students.insert(item.student_id) {
                return Err(DomainError::Validation(
                    "Duplicate student in promotion payload".into(),
                ));
            }
            let enrollment = source_enrollments
                .get(&item.enrollment_id)
                .filter(|enrollment| {
                    enrollment.org_id == org_id
                        && enrollment.period_id == source_period_id
                        && enrollment.student_id == item.student_id
                        && enrollment.section_id == item.source_section_id
                })
                .ok_or_else(|| DomainError::NotFound("Source enrollment not found".into()))?
                .clone();

            let resolved_target_section = match item.decision {
                PromotionDecision::Promote => {
                    promoted_count += 1;
                    let mut next_enrollment = Enrollment::new(
                        org_id,
                        target_period_id,
                        enrollment.student_id,
                        item.target_section_id,
                        enrollment.student_number.clone(),
                        enrollment.study_mode,
                    );
                    next_enrollment.attempt_number = 1;
                    next_enrollment.enrollment_type = Some("promotion".to_string());
                    next_enrollment.created_by = ctx.user_id();
                    next_enrollment.updated_by = ctx.user_id();
                    new_enrollments.push(next_enrollment);
                    item.target_section_id
                }
                PromotionDecision::Retain => {
                    retained_count += 1;
                    let target_section = item.target_section_id.or(enrollment.section_id);
                    let mut next_enrollment = Enrollment::new(
                        org_id,
                        target_period_id,
                        enrollment.student_id,
                        target_section,
                        enrollment.student_number.clone(),
                        enrollment.study_mode,
                    );
                    next_enrollment.attempt_number = enrollment.attempt_number.saturating_add(1);
                    next_enrollment.enrollment_type = Some("repeat".to_string());
                    next_enrollment.created_by = ctx.user_id();
                    next_enrollment.updated_by = ctx.user_id();
                    new_enrollments.push(next_enrollment);
                    target_section
                }
                PromotionDecision::Graduate => {
                    graduated_count += 1;
                    None
                }
                PromotionDecision::Exclude => {
                    excluded_count += 1;
                    None
                }
            };

            details.push(ProgressionDetail {
                id: Uuid::new_v4(),
                job_id,
                student_id: item.student_id,
                enrollment_id: item.enrollment_id,
                source_section_id: item.source_section_id,
                target_section_id: resolved_target_section,
                decision: item.decision,
                reason: item.reason,
                manual_override: item.manual_override,
                override_by: item.manual_override.then(|| ctx.user_id()).flatten(),
                override_reason: item.override_reason.filter(|s| !s.trim().is_empty()),
                final_gpa: item.final_gpa,
                attendance_rate: item.attendance_rate,
                created_at: now,
            });
        }

        let total_students = details.len() as u32;
        let job = ProgressionJob {
            id: job_id,
            org_id,
            source_period_id,
            target_period_id: Some(target_period_id),
            name: job_name.trim().to_string(),
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

        // Post-commit ERP hook: assess mandatory target period fees via FinanceService
        // so every generated StudentFee has an atomic double-entry JournalEntry.
        if auto_assess_fees && !new_enrollments.is_empty() {
            if let Some(finance_service) = &self.finance_service {
                let sections_map: HashMap<Uuid, Section> =
                    if let Some(sec_repo) = &self.section_repo {
                        sec_repo
                            .find_by_org(org_id)
                            .await
                            .unwrap_or_default()
                            .into_iter()
                            .map(|s| (s.id, s))
                            .collect()
                    } else {
                        HashMap::new()
                    };
                for new_enr in &new_enrollments {
                    let sec = new_enr.section_id.and_then(|id| sections_map.get(&id));
                    let _ = finance_service
                        .assess_enrollment_fees(
                            ctx,
                            org_id,
                            target_period_id,
                            new_enr.student_id,
                            new_enr.id,
                            sec.and_then(|s| s.level.as_deref()),
                            new_enr.study_mode,
                            sec.and_then(|s| s.org_unit_id),
                        )
                        .await;
                }
            }
        }

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

    pub async fn get_job_details(
        &self,
        ctx: &RequestContext,
        job_id: Uuid,
    ) -> Result<Vec<ProgressionDetail>, DomainError> {
        let _job = self.get_job(ctx, job_id).await?;
        self.progression_repo.find_details_by_job(job_id).await
    }

    pub async fn rollback_job(
        &self,
        ctx: &RequestContext,
        job_id: Uuid,
    ) -> Result<ProgressionJob, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentUpdate)
            .await?;
        let now = Utc::now();
        let rolled_back = self
            .progression_repo
            .rollback_promotion(job_id, ctx.tenant_id, now)
            .await?;
        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::CohortPromotionRolledBack {
                    job_id,
                    org_id: ctx.tenant_id,
                    timestamp: now,
                },
            )
            .await;
        Ok(rolled_back)
    }

    /// Evaluate cohort progression eligibility (Dry-run preview without target period).
    pub async fn evaluate_cohort_progression(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        source_period_id: Uuid,
    ) -> Result<Vec<StudentProgressionItem>, DomainError> {
        self.evaluate_cohort_with_target(ctx, org_id, source_period_id, None)
            .await
    }

    /// Evaluate cohort progression eligibility with full academic, attendance, financial,
    /// institution-type, level-ordering, and target-section resolution.
    pub async fn evaluate_cohort_with_target(
        &self,
        ctx: &RequestContext,
        _org_id: Uuid,
        source_period_id: Uuid,
        target_period_id: Option<Uuid>,
    ) -> Result<Vec<StudentProgressionItem>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentRead)
            .await?;

        let org_id = ctx.tenant_id;
        let settings = self.load_settings(org_id).await;
        let institution_type = self.load_institution_type(org_id).await;
        let source_period = self.load_period(org_id, source_period_id).await?;

        // Batch-load all required cohort datasets in O(1) queries per entity type.
        let enrollments = self
            .enrollment_repo
            .find_by_period(source_period_id)
            .await?;

        let target_enrolled_students: HashSet<Uuid> = if let Some(target_id) = target_period_id {
            self.enrollment_repo
                .find_by_period(target_id)
                .await?
                .into_iter()
                .filter(|e| {
                    e.org_id == org_id
                        && matches!(
                            e.status,
                            EnrollmentStatus::Active | EnrollmentStatus::PendingApproval
                        )
                })
                .map(|e| e.student_id)
                .collect()
        } else {
            HashSet::new()
        };

        let student_details_by_id = self.load_student_details_map(org_id).await;
        let sections_by_id = self.load_sections_map(org_id).await;
        let grades_by_student = self
            .load_grades_by_student(org_id, source_period.as_ref(), source_period_id)
            .await;
        let attendance_by_student = self
            .load_attendance_by_student(org_id, source_period.as_ref())
            .await;
        let unpaid_balance_by_student = self
            .load_unpaid_balances_by_student(org_id, source_period_id)
            .await;

        let level_order = resolve_level_order(&settings);
        let mut items = Vec::new();

        for enr in enrollments.into_iter().filter(|e| e.org_id == org_id) {
            let detail_opt = student_details_by_id.get(&enr.student_id);
            let student_opt = detail_opt.map(|d| &d.student);
            let person_opt = detail_opt.map(|d| &d.person);
            let student_number = student_opt
                .map(|s| s.student_number.clone())
                .or_else(|| Some(enr.student_number.clone()));
            let student_name = person_opt.map(|p| p.full_name());
            let student_name_ar = person_opt.and_then(|p| p.full_name_ar());

            let source_section = enr.section_id.and_then(|id| sections_by_id.get(&id));
            let already_enrolled_in_target = target_enrolled_students.contains(&enr.student_id);

            let (metrics, decision, reason) = self
                .evaluate_single_student(
                    org_id,
                    institution_type.clone(),
                    &settings,
                    &level_order,
                    &enr,
                    student_opt,
                    source_section,
                    grades_by_student.get(&enr.student_id),
                    attendance_by_student.get(&enr.student_id),
                    unpaid_balance_by_student
                        .get(&enr.student_id)
                        .copied()
                        .unwrap_or(Decimal::ZERO),
                    already_enrolled_in_target,
                )
                .await;

            let target_section_id = match decision {
                PromotionDecision::Promote => {
                    suggest_target_section(source_section, &level_order, sections_by_id.values())
                }
                PromotionDecision::Retain => enr.section_id,
                PromotionDecision::Graduate | PromotionDecision::Exclude => None,
            };

            items.push(StudentProgressionItem {
                student_id: enr.student_id,
                student_number,
                student_name,
                student_name_ar,
                enrollment_id: enr.id,
                source_section_id: enr.section_id,
                target_section_id,
                decision,
                reason,
                final_gpa: metrics.final_gpa,
                overall_percentage: metrics.overall_percentage,
                attendance_rate: metrics.attendance_rate,
                failed_subjects_count: metrics.failed_subjects_count,
                financial_balance_due: metrics.financial_balance_due,
                has_financial_hold: metrics.has_financial_hold,
                already_enrolled_in_target,
                manual_override: false,
                override_reason: None,
            });
        }

        Ok(items)
    }

    #[allow(clippy::too_many_arguments)]
    async fn evaluate_single_student(
        &self,
        org_id: Uuid,
        institution_type: InstitutionType,
        settings: &OrganizationSettings,
        level_order: &[String],
        enrollment: &Enrollment,
        student_opt: Option<&Student>,
        source_section: Option<&Section>,
        grades_opt: Option<&Vec<orbis_domain::entities::grade::Grade>>,
        attendance_opt: Option<&Vec<orbis_domain::entities::attendance::Attendance>>,
        financial_balance_due: Decimal,
        already_enrolled_in_target: bool,
    ) -> (EvaluatedMetrics, PromotionDecision, String) {
        let attendance_rate = if enrollment.is_attendance_exempt
            || enrollment.study_mode.default_attendance_exempt()
        {
            None
        } else {
            compute_attendance_rate(attendance_opt)
        };

        let (overall_percentage, failed_subjects_count) =
            if enrollment.is_grade_exempt || enrollment.study_mode.default_grade_exempt() {
                (None, 0)
            } else {
                compute_grade_summary(grades_opt, &settings.academic)
            };

        let final_gpa = self
            .resolve_student_gpa(org_id, enrollment.student_id, enrollment.period_id)
            .await;

        let has_financial_hold = financial_balance_due > Decimal::ZERO
            && settings.promotion.financial_clearance_mode != FinancialClearanceMode::Ignore;

        let metrics = EvaluatedMetrics {
            final_gpa,
            overall_percentage,
            attendance_rate,
            failed_subjects_count,
            financial_balance_due,
            has_financial_hold,
        };

        if already_enrolled_in_target {
            return (
                metrics,
                PromotionDecision::Exclude,
                "Student already has an active enrollment in target period".into(),
            );
        }

        if matches!(
            enrollment.status,
            EnrollmentStatus::Withdrawn
                | EnrollmentStatus::Transferred
                | EnrollmentStatus::Graduated
        ) {
            return (
                metrics,
                PromotionDecision::Exclude,
                format!("Enrollment is finalized ({})", enrollment.status),
            );
        }

        if matches!(
            enrollment.status,
            EnrollmentStatus::Suspended
                | EnrollmentStatus::OnHold
                | EnrollmentStatus::PendingApproval
        ) {
            return (
                metrics,
                PromotionDecision::Retain,
                format!("Enrollment status '{}' requires review", enrollment.status),
            );
        }

        if let Some(student) = student_opt {
            match student.status {
                StudentStatus::Active => {}
                StudentStatus::Graduated => {
                    return (
                        metrics,
                        PromotionDecision::Exclude,
                        "Student has already graduated".into(),
                    );
                }
                StudentStatus::Inactive => {
                    return (
                        metrics,
                        PromotionDecision::Exclude,
                        "Student registration is incomplete".into(),
                    );
                }
                StudentStatus::Withdrawn
                | StudentStatus::Expelled
                | StudentStatus::TransferredOut => {
                    return (
                        metrics,
                        PromotionDecision::Exclude,
                        format!("Student is inactive ({})", student.status),
                    );
                }
                StudentStatus::Suspended | StudentStatus::OnLeave => {
                    return (
                        metrics,
                        PromotionDecision::Retain,
                        format!(
                            "Student status '{}' requires review before progression",
                            student.status
                        ),
                    );
                }
            }

            if matches!(student.standing, AcademicStanding::Dismissed) {
                return (
                    metrics,
                    PromotionDecision::Exclude,
                    "Student dismissed for academic reasons".into(),
                );
            }
        }

        // Financial clearance check
        if financial_balance_due > Decimal::ZERO
            && settings.promotion.financial_clearance_mode == FinancialClearanceMode::Block
        {
            return (
                metrics,
                PromotionDecision::Retain,
                format!(
                    "Financial clearance hold: unpaid balance of {}",
                    financial_balance_due
                ),
            );
        }

        // Attendance threshold check
        if settings.promotion.check_attendance {
            if let Some(rate) = attendance_rate {
                if rate < settings.academic.min_attendance_rate {
                    return (
                        metrics,
                        PromotionDecision::Retain,
                        format!(
                            "Attendance rate ({}%) is below minimum ({}%)",
                            rate, settings.academic.min_attendance_rate
                        ),
                    );
                }
            }
        }

        // Institution-aware academic decision
        let (passed, pass_reason) = evaluate_academic_rule(
            institution_type,
            &settings.academic,
            student_opt,
            overall_percentage,
            failed_subjects_count,
            final_gpa,
        );

        if !passed {
            return (metrics, PromotionDecision::Retain, pass_reason);
        }

        // Check final-level graduation when enabled
        if settings.academic.auto_graduate_final_level
            && is_final_level(source_section, level_order)
        {
            return (
                metrics,
                PromotionDecision::Graduate,
                "Completed final level and eligible for graduation".into(),
            );
        }

        let mut final_reason = pass_reason;
        if has_financial_hold {
            final_reason = format!(
                "{} (Warning: unpaid balance {})",
                final_reason, financial_balance_due
            );
        }

        (metrics, PromotionDecision::Promote, final_reason)
    }

    async fn load_settings(&self, org_id: Uuid) -> OrganizationSettings {
        if let Some(repo) = &self.policy_repo {
            repo.get_settings(org_id).await.unwrap_or_default()
        } else {
            OrganizationSettings::default()
        }
    }

    async fn load_institution_type(&self, org_id: Uuid) -> InstitutionType {
        if let Some(repo) = &self.org_repo {
            if let Ok(Some(org)) = repo.find_by_id(org_id).await {
                return InstitutionType::from_str_loose(&org.institution_type);
            }
        }
        InstitutionType::School
    }

    async fn load_period(
        &self,
        org_id: Uuid,
        period_id: Uuid,
    ) -> Result<Option<AcademicPeriod>, DomainError> {
        let Some(repo) = &self.period_repo else {
            return Ok(None);
        };
        let period = repo.find_by_id(period_id).await?;
        if let Some(ref p) = period {
            if p.org_id != org_id {
                return Err(DomainError::Unauthorized(
                    "Academic period belongs to another organization".into(),
                ));
            }
        }
        Ok(period)
    }

    async fn load_student_details_map(
        &self,
        org_id: Uuid,
    ) -> HashMap<Uuid, orbis_domain::entities::student::StudentDetail> {
        if let Ok(details) = self.student_repo.find_active_details_by_org(org_id).await {
            if !details.is_empty() {
                return details.into_iter().map(|d| (d.student.id, d)).collect();
            }
        }
        HashMap::new()
    }

    async fn load_sections_map(&self, org_id: Uuid) -> HashMap<Uuid, Section> {
        if let Some(repo) = &self.section_repo {
            repo.find_by_org(org_id)
                .await
                .unwrap_or_default()
                .into_iter()
                .filter(|s| !s.is_deleted)
                .map(|s| (s.id, s))
                .collect()
        } else {
            HashMap::new()
        }
    }

    async fn load_grades_by_student(
        &self,
        org_id: Uuid,
        source_period: Option<&AcademicPeriod>,
        source_period_id: Uuid,
    ) -> HashMap<Uuid, Vec<orbis_domain::entities::grade::Grade>> {
        let Some(repo) = &self.grade_repo else {
            return HashMap::new();
        };

        let mut period_ids = vec![source_period_id];
        if let (Some(period), Some(period_repo)) = (source_period, &self.period_repo) {
            if period.period_type.is_annual() {
                if let Ok(year_periods) = period_repo.find_by_year(period.year_id).await {
                    for yp in year_periods {
                        if !period_ids.contains(&yp.id) {
                            period_ids.push(yp.id);
                        }
                    }
                }
            }
        }

        let mut grouped: HashMap<Uuid, Vec<orbis_domain::entities::grade::Grade>> = HashMap::new();
        for pid in period_ids {
            let mut page = 1u64;
            loop {
                let Ok(list) = repo
                    .list(
                        org_id,
                        GradeFilter {
                            period_id: Some(pid),
                            pagination: Some(PaginationParams {
                                page: Some(page),
                                page_size: Some(PaginationParams::MAX_PAGE_SIZE),
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                    )
                    .await
                else {
                    break;
                };
                let count = list.items.len();
                for g in list.items {
                    grouped.entry(g.student_id).or_default().push(g);
                }
                if (count as u64) < PaginationParams::MAX_PAGE_SIZE {
                    break;
                }
                page += 1;
            }
        }
        grouped
    }

    async fn load_attendance_by_student(
        &self,
        org_id: Uuid,
        source_period: Option<&AcademicPeriod>,
    ) -> HashMap<Uuid, Vec<orbis_domain::entities::attendance::Attendance>> {
        let Some(repo) = &self.attendance_repo else {
            return HashMap::new();
        };
        let mut grouped: HashMap<Uuid, Vec<orbis_domain::entities::attendance::Attendance>> =
            HashMap::new();
        let mut page = 1u64;
        loop {
            let Ok(list) = repo
                .list(
                    org_id,
                    AttendanceFilter {
                        pagination: Some(PaginationParams {
                            page: Some(page),
                            page_size: Some(PaginationParams::MAX_PAGE_SIZE),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                )
                .await
            else {
                break;
            };
            let count = list.items.len();
            for att in list.items {
                if let Some(period) = source_period {
                    let start = period.start_date.date_naive();
                    let end = period.end_date.date_naive();
                    if att.attendance_date < start || att.attendance_date > end {
                        continue;
                    }
                }
                grouped.entry(att.student_id).or_default().push(att);
            }
            if (count as u64) < PaginationParams::MAX_PAGE_SIZE {
                break;
            }
            page += 1;
        }
        grouped
    }

    async fn load_unpaid_balances_by_student(
        &self,
        org_id: Uuid,
        source_period_id: Uuid,
    ) -> HashMap<Uuid, Decimal> {
        let Some(repo) = &self.fee_repo else {
            return HashMap::new();
        };
        let Ok(fees) = repo.find_by_org(org_id).await else {
            return HashMap::new();
        };
        let mut balances: HashMap<Uuid, Decimal> = HashMap::new();
        for fee in fees {
            if fee.period_id == source_period_id
                && !matches!(
                    fee.status,
                    FeeStatus::Cancelled | FeeStatus::Refunded | FeeStatus::Paid
                )
                && fee.balance_due > Decimal::ZERO
            {
                *balances.entry(fee.student_id).or_default() += fee.balance_due;
            }
        }
        balances
    }

    async fn resolve_student_gpa(
        &self,
        org_id: Uuid,
        student_id: Uuid,
        period_id: Uuid,
    ) -> Option<Decimal> {
        let repo = self.gpa_repo.as_ref()?;
        let records = repo.find_by_student_in_org(org_id, student_id).await.ok()?;
        records
            .iter()
            .find(|r| r.period_id == period_id)
            .or_else(|| records.last())
            .map(|r| r.cumulative_gpa)
    }
}

fn compute_attendance_rate(
    records: Option<&Vec<orbis_domain::entities::attendance::Attendance>>,
) -> Option<Decimal> {
    let records = records?;
    if records.is_empty() {
        return None;
    }
    let mut total = 0u32;
    let mut present = 0u32;
    for rec in records {
        total += 1;
        let status = rec.status.trim().to_ascii_lowercase();
        if matches!(status.as_str(), "present" | "late" | "excused") {
            present += 1;
        }
    }
    if total == 0 {
        return None;
    }
    let pct = (Decimal::from(present) / Decimal::from(total)) * Decimal::from(100);
    Some(pct.round_dp(2))
}

fn compute_grade_summary(
    grades: Option<&Vec<orbis_domain::entities::grade::Grade>>,
    academic: &AcademicSettings,
) -> (Option<Decimal>, u32) {
    let Some(grades) = grades else {
        return (None, 0);
    };
    let mut by_subject: HashMap<Uuid, (Decimal, Decimal)> = HashMap::new();
    for g in grades {
        if g.is_exempt || g.is_excused || g.max_score <= Decimal::ZERO {
            continue;
        }
        let entry = by_subject
            .entry(g.section_subject_id)
            .or_insert((Decimal::ZERO, Decimal::ZERO));
        entry.0 += g.score;
        entry.1 += g.max_score;
    }
    if by_subject.is_empty() {
        return (None, 0);
    }

    let mut total_pct = Decimal::ZERO;
    let mut failed_subjects = 0u32;
    for (score, max_score) in by_subject.values() {
        if *max_score <= Decimal::ZERO {
            continue;
        }
        let pct = (*score / *max_score) * Decimal::from(100);
        total_pct += pct;
        if pct < academic.passing_threshold_percentage {
            failed_subjects += 1;
        }
    }
    let avg = (total_pct / Decimal::from(by_subject.len() as u32)).round_dp(2);
    (Some(avg), failed_subjects)
}

fn evaluate_academic_rule(
    institution_type: InstitutionType,
    academic: &AcademicSettings,
    student_opt: Option<&Student>,
    overall_percentage: Option<Decimal>,
    failed_subjects_count: u32,
    final_gpa: Option<Decimal>,
) -> (bool, String) {
    if let Some(student) = student_opt {
        if matches!(
            student.standing,
            AcademicStanding::AcademicWarning3 | AcademicStanding::Probation
        ) {
            return (
                false,
                format!(
                    "Academic standing requires retention or review ({})",
                    student.standing
                ),
            );
        }
    }

    match institution_type {
        InstitutionType::University => {
            if let Some(gpa) = final_gpa {
                if gpa <= Decimal::ZERO && overall_percentage.is_none() {
                    return (
                        false,
                        "GPA is zero — review before academic progression".into(),
                    );
                }
            }
            if let Some(avg) = overall_percentage {
                if avg < academic.passing_threshold_percentage {
                    return (
                        false,
                        format!(
                            "Overall average ({}%) is below passing threshold ({}%)",
                            avg, academic.passing_threshold_percentage
                        ),
                    );
                }
            }
            (
                true,
                "Eligible for university level/term progression".into(),
            )
        }
        InstitutionType::School | InstitutionType::Institute => {
            if failed_subjects_count as usize > academic.max_failed_subjects_for_conditional_pass {
                return (
                    false,
                    format!(
                        "Failed {} subject(s) (maximum allowed: {})",
                        failed_subjects_count, academic.max_failed_subjects_for_conditional_pass
                    ),
                );
            }
            if let Some(avg) = overall_percentage {
                if avg < academic.passing_threshold_percentage {
                    return (
                        false,
                        format!(
                            "Overall average ({}%) is below passing threshold ({}%)",
                            avg, academic.passing_threshold_percentage
                        ),
                    );
                }
            }
            if failed_subjects_count > 0 {
                (
                    true,
                    format!(
                        "Eligible for conditional promotion ({} failed subject(s))",
                        failed_subjects_count
                    ),
                )
            } else {
                (true, "Eligible for advancement to next level".into())
            }
        }
    }
}

fn resolve_level_order(settings: &OrganizationSettings) -> Vec<String> {
    if !settings.promotion.level_progression_order.is_empty() {
        return settings
            .promotion
            .level_progression_order
            .iter()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();
    }
    let profile = CountryProfile::yemen();
    let mut levels: Vec<String> = profile
        .stages
        .into_iter()
        .flat_map(|stage| stage.levels.into_iter().map(|l| l.code))
        .collect();
    // Deduplicate while preserving order
    let mut seen = HashSet::new();
    levels.retain(|l| seen.insert(l.to_ascii_lowercase()));
    levels
}

fn is_final_level(source_section: Option<&Section>, level_order: &[String]) -> bool {
    let Some(level) = source_section.and_then(|s| s.level.as_deref()) else {
        return false;
    };
    let normalized = level.trim().to_ascii_lowercase();
    level_order
        .last()
        .is_some_and(|last| last.trim().to_ascii_lowercase() == normalized)
}

fn suggest_target_section<'a>(
    source_section: Option<&Section>,
    level_order: &[String],
    all_sections: impl Iterator<Item = &'a Section>,
) -> Option<Uuid> {
    let source = source_section?;
    let source_level = source.level.as_deref()?.trim().to_ascii_lowercase();
    let current_idx = level_order
        .iter()
        .position(|l| l.trim().to_ascii_lowercase() == source_level)?;
    let next_level = level_order
        .get(current_idx + 1)?
        .trim()
        .to_ascii_lowercase();

    let candidates: Vec<&Section> = all_sections
        .filter(|s| {
            s.is_active
                && !s.is_deleted
                && s.level
                    .as_deref()
                    .is_some_and(|l| l.trim().to_ascii_lowercase() == next_level)
        })
        .collect();

    // Prefer same org_unit_id + same track, then same track, then any active section in next level.
    candidates
        .iter()
        .find(|s| s.org_unit_id == source.org_unit_id && s.track == source.track)
        .or_else(|| candidates.iter().find(|s| s.track == source.track))
        .or_else(|| candidates.first())
        .map(|s| s.id)
}
