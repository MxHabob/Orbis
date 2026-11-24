use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{
    AcademicCalendarRepository, AcademicPeriodRepository, AssessmentSchemeRepository,
    AttendanceRepository, BehaviorRecordRepository, EmployeeRepository, EnrollmentRepository,
    GpaRecordRepository, GradeRepository, ScheduledLessonRepository, SectionRepository,
    SectionSubjectRepository, SettingsRepository, StudentRepository, SubjectRepository,
};
use crate::services::calculation_engine::{
    AcademicCalculationEngine, AstNode, CalcEngine, apply_rounding,
};
use chrono::{NaiveDate, Utc};
use orbis_domain::entities::academic_period::{AcademicPeriod, PeriodKind};
use orbis_domain::entities::academic_policy::{
    AcademicPolicy, AcademicStatus, ComponentScore, PeriodAssessmentResult, PeriodComponentScores,
    StudentAnnualReportCard, StudentCertificateReportCard, StudentTermReportCard,
    SubjectTermResult,
};
use orbis_domain::entities::assessment_scheme::{
    AssessmentComponent, AssessmentLevel, AssessmentScheme, InstitutionScope,
};
use orbis_domain::entities::attendance::Attendance;
use orbis_domain::entities::behavior_record::BehaviorRecord;
use orbis_domain::entities::enrollment::{Enrollment, EnrollmentStatus};
use orbis_domain::entities::gpa_record::CumulativeGpaRecord;
use orbis_domain::entities::grade::Grade;
use orbis_domain::entities::section_subject::SectionSubject;
use orbis_domain::entities::student::{AcademicStanding, Student};
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::iam::permission::Permission;
use orbis_domain::queries::filters::{AttendanceFilter, GradeFilter, PaginatedList};
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

fn validate_assessment_components(
    scheme_max_score: Decimal,
    components: &[AssessmentComponent],
) -> Result<(), DomainError> {
    if components.is_empty() {
        return Ok(());
    }

    let mut codes = std::collections::HashSet::new();
    let mut sort_orders = std::collections::HashSet::new();
    let mut total_weight = Decimal::ZERO;
    let mut total_max_score = Decimal::ZERO;

    for component in components {
        if component.code.trim().is_empty() {
            return Err(DomainError::Validation(
                "Assessment component code is required".into(),
            ));
        }
        if component.name.trim().is_empty() {
            return Err(DomainError::Validation(
                "Assessment component name is required".into(),
            ));
        }
        if !codes.insert(component.code.trim().to_string()) {
            return Err(DomainError::Validation(
                "Assessment component codes must be unique".into(),
            ));
        }
        if component.sort_order <= 0 {
            return Err(DomainError::Validation(
                "Assessment component sort order must be positive".into(),
            ));
        }
        if !sort_orders.insert(component.sort_order) {
            return Err(DomainError::Validation(
                "Assessment component sort orders must be unique".into(),
            ));
        }
        if component.max_score <= Decimal::ZERO {
            return Err(DomainError::Validation(
                "Assessment component maximum score must be positive".into(),
            ));
        }
        if component.weight < Decimal::ZERO {
            return Err(DomainError::Validation(
                "Assessment component weight cannot be negative".into(),
            ));
        }
        if let Some(min_score) = component.min_score {
            if min_score < Decimal::ZERO || min_score > component.max_score {
                return Err(DomainError::Validation(
                    "Assessment component passing score must be within the component score range"
                        .into(),
                ));
            }
        }
        total_weight += component.weight;
        total_max_score += component.max_score;
    }

    let weighted_sum_valid = (total_weight - Decimal::ONE).abs() <= Decimal::new(1, 3);
    let raw_score_sum_valid = (total_max_score - scheme_max_score).abs() <= Decimal::new(1, 3);

    if !weighted_sum_valid && !raw_score_sum_valid {
        return Err(DomainError::Validation(
            "Assessment components must use a normalized weight sum of 1.0 or a maximum-score sum matching the scheme"
                .into(),
        ));
    }
    Ok(())
}

pub struct BatchGradeItem {
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub section_subject_id: Uuid,
    pub component_code: Option<String>,
    pub component_id: Option<Uuid>,
    pub score: Decimal,
    pub max_score: Option<Decimal>,
    pub is_excused: bool,
    pub is_exempt: bool,
    pub is_absent: bool,
    pub notes: Option<String>,
}

struct ValidatedGradeContext {
    enrollment_id: Uuid,
    component_code: Option<String>,
    component_max_score: Option<Decimal>,
}

fn normalize_grade_component_key(
    component_code: Option<&str>,
    component_id: Option<Uuid>,
) -> Option<String> {
    component_code
        .map(str::trim)
        .filter(|code| !code.is_empty())
        .map(|code| code.to_lowercase())
        .or_else(|| component_id.map(|id| id.to_string()))
}

fn resolve_grade_max_score(
    configured_max_score: Option<Decimal>,
    requested_max_score: Option<Decimal>,
) -> Result<Decimal, DomainError> {
    if let Some(configured_max_score) = configured_max_score {
        if requested_max_score.is_some_and(|requested| requested != configured_max_score) {
            return Err(DomainError::Validation(
                "Grade maximum score must match the assessment scheme component".into(),
            ));
        }
        return Ok(configured_max_score);
    }

    Ok(requested_max_score.unwrap_or_else(|| Decimal::from(100)))
}

pub struct BatchAttendanceItem {
    pub student_id: Uuid,
    pub status: String,
    pub notes: Option<String>,
    pub section_subject_id: Option<Uuid>,
    pub scheduled_lesson_id: Option<Uuid>,
}

fn teacher_is_assigned_to_lesson(
    employee_id: Uuid,
    lesson: &orbis_domain::entities::scheduled_lesson::ScheduledLesson,
    section_subject: &orbis_domain::entities::section_subject::SectionSubject,
) -> bool {
    if let Some(teacher_id) = lesson.teacher_id {
        return teacher_id == employee_id;
    }

    [
        section_subject.instructor_id,
        section_subject.assistant_instructor_id,
        section_subject.substitute_instructor_id,
    ]
    .contains(&Some(employee_id))
}

async fn authorize_grade_access(
    permission_checker: &dyn PermissionCheckerPort,
    ctx: &RequestContext,
    org_id: Uuid,
    permission: Permission,
) -> Result<(), DomainError> {
    permission_checker.check(ctx, permission).await?;
    if ctx.tenant_id != org_id {
        return Err(DomainError::Unauthorized("Organization mismatch".into()));
    }
    Ok(())
}

pub struct AssessmentService {
    pub scheme_repo: Arc<dyn AssessmentSchemeRepository>,
    pub grade_repo: Arc<dyn GradeRepository>,
    pub attendance_repo: Arc<dyn AttendanceRepository>,
    pub behavior_repo: Arc<dyn BehaviorRecordRepository>,
    pub gpa_repo: Arc<dyn GpaRecordRepository>,
    pub section_subject_repo: Arc<dyn SectionSubjectRepository>,
    pub subject_repo: Arc<dyn SubjectRepository>,
    pub period_repo: Option<Arc<dyn AcademicPeriodRepository>>,
    pub section_repo: Option<Arc<dyn SectionRepository>>,
    pub employee_repo: Option<Arc<dyn EmployeeRepository>>,
    pub scheduled_lesson_repo: Option<Arc<dyn ScheduledLessonRepository>>,
    pub student_repo: Option<Arc<dyn StudentRepository>>,
    pub year_repo: Option<Arc<dyn AcademicCalendarRepository>>,
    pub enrollment_repo: Option<Arc<dyn EnrollmentRepository>>,
    pub settings_repo: Option<Arc<dyn SettingsRepository>>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl AssessmentService {
    pub fn new(
        scheme_repo: Arc<dyn AssessmentSchemeRepository>,
        grade_repo: Arc<dyn GradeRepository>,
        attendance_repo: Arc<dyn AttendanceRepository>,
        behavior_repo: Arc<dyn BehaviorRecordRepository>,
        gpa_repo: Arc<dyn GpaRecordRepository>,
        section_subject_repo: Arc<dyn SectionSubjectRepository>,
        subject_repo: Arc<dyn SubjectRepository>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            scheme_repo,
            grade_repo,
            attendance_repo,
            behavior_repo,
            gpa_repo,
            section_subject_repo,
            subject_repo,
            period_repo: None,
            section_repo: None,
            employee_repo: None,
            scheduled_lesson_repo: None,
            student_repo: None,
            year_repo: None,
            enrollment_repo: None,
            settings_repo: None,
            event_bus,
            permission_checker,
        }
    }

    /// Builder method to supply additional repositories needed for calculated results & report cards
    pub fn with_academic_repos(
        mut self,
        period_repo: Option<Arc<dyn AcademicPeriodRepository>>,
        section_repo: Option<Arc<dyn SectionRepository>>,
        student_repo: Option<Arc<dyn StudentRepository>>,
        year_repo: Option<Arc<dyn AcademicCalendarRepository>>,
        enrollment_repo: Option<Arc<dyn EnrollmentRepository>>,
        settings_repo: Option<Arc<dyn SettingsRepository>>,
    ) -> Self {
        self.period_repo = period_repo;
        self.section_repo = section_repo;
        self.student_repo = student_repo;
        self.year_repo = year_repo;
        self.enrollment_repo = enrollment_repo;
        self.settings_repo = settings_repo;
        self
    }

    pub fn with_attendance_scope_repos(
        mut self,
        employee_repo: Arc<dyn EmployeeRepository>,
        scheduled_lesson_repo: Arc<dyn ScheduledLessonRepository>,
    ) -> Self {
        self.employee_repo = Some(employee_repo);
        self.scheduled_lesson_repo = Some(scheduled_lesson_repo);
        self
    }

    /// Helper to resolve the authoritative AcademicPolicy for an organization
    async fn resolve_policy(&self, org_id: Uuid, policy: Option<AcademicPolicy>) -> AcademicPolicy {
        if let Some(p) = policy {
            p
        } else if let Some(sr) = &self.settings_repo {
            sr.get_settings(org_id)
                .await
                .map(|s| s.academic.to_academic_policy(org_id))
                .unwrap_or_default()
        } else {
            AcademicPolicy::default()
        }
    }

    /// Record or update a student grade
    async fn validate_grade_context(
        &self,
        org_id: Uuid,
        period_id: Uuid,
        student_id: Uuid,
        section_subject_id: Uuid,
        component_code: Option<&str>,
        component_id: Option<Uuid>,
        enrollment_id: Option<Uuid>,
    ) -> Result<ValidatedGradeContext, DomainError> {
        if let Some(period_repo) = &self.period_repo {
            let period = period_repo
                .find_by_id(period_id)
                .await?
                .ok_or_else(|| DomainError::NotFound("Academic period not found".into()))?;
            if period.org_id != org_id {
                return Err(DomainError::Unauthorized(
                    "Academic period belongs to another organization".into(),
                ));
            }
            if !period.can_enter_grades() {
                return Err(DomainError::BusinessRule(
                    "Grade entry is not open for this academic period".into(),
                ));
            }
        }

        if let Some(student_repo) = &self.student_repo {
            let student = student_repo
                .find_by_id(student_id)
                .await?
                .ok_or_else(|| DomainError::NotFound("Student not found".into()))?;
            if student.org_id != org_id {
                return Err(DomainError::Unauthorized(
                    "Student belongs to another organization".into(),
                ));
            }
        }

        let section_subject = self
            .section_subject_repo
            .find_by_id(section_subject_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Section subject not found".into()))?;
        if section_subject.org_id != org_id || section_subject.period_id != period_id {
            return Err(DomainError::Unauthorized(
                "Section subject is not valid for this organization and period".into(),
            ));
        }

        let mut resolved_component_code = component_code
            .map(str::trim)
            .filter(|code| !code.is_empty())
            .map(str::to_owned);
        let mut component_max_score = None;
        if section_subject.scheme_id.is_some() {
            let scheme_id = section_subject.scheme_id.expect("checked above");
            let scheme = self
                .scheme_repo
                .find_by_id(scheme_id)
                .await?
                .ok_or_else(|| DomainError::NotFound("Assessment scheme not found".into()))?;
            if scheme.org_id != org_id {
                return Err(DomainError::Unauthorized(
                    "Assessment scheme belongs to another organization".into(),
                ));
            }
            if component_code.is_some() || component_id.is_some() {
                let matched_component = scheme.components.iter().find(|component| {
                    component_code
                        .is_some_and(|code| component.code.trim().eq_ignore_ascii_case(code.trim()))
                        || component_id.is_some_and(|id| {
                            component.code.trim().eq_ignore_ascii_case(&id.to_string())
                        })
                });
                let Some(component) = matched_component else {
                    return Err(DomainError::Validation(
                        "The assessment component is not defined by the section subject's assessment scheme".into(),
                    ));
                };
                if component_code
                    .is_some_and(|code| !component.code.trim().eq_ignore_ascii_case(code.trim()))
                    || component_id.is_some_and(|id| {
                        !component.code.trim().eq_ignore_ascii_case(&id.to_string())
                    })
                {
                    return Err(DomainError::Validation(
                        "Assessment component code and legacy ID do not match".into(),
                    ));
                }
                resolved_component_code = Some(component.code.clone());
                component_max_score = Some(component.max_score);
            }
        }

        if let Some(enrollment_repo) = &self.enrollment_repo {
            let enrollment = if let Some(enrollment_id) = enrollment_id {
                enrollment_repo
                    .find_by_id(enrollment_id)
                    .await?
                    .ok_or_else(|| DomainError::NotFound("Enrollment not found".into()))?
            } else {
                enrollment_repo
                    .find_by_student(student_id)
                    .await?
                    .into_iter()
                    .find(|item| item.period_id == period_id)
                    .ok_or_else(|| {
                        DomainError::NotFound("Active enrollment not found for this period".into())
                    })?
            };

            if enrollment.org_id != org_id
                || enrollment.student_id != student_id
                || enrollment.period_id != period_id
                || enrollment.section_id != Some(section_subject.section_id)
            {
                return Err(DomainError::Unauthorized(
                    "Enrollment does not match the student, period, and section".into(),
                ));
            }
            return Ok(ValidatedGradeContext {
                enrollment_id: enrollment.id,
                component_code: resolved_component_code,
                component_max_score,
            });
        }

        Ok(ValidatedGradeContext {
            enrollment_id: Uuid::nil(),
            component_code: resolved_component_code,
            component_max_score,
        })
    }

    pub async fn record_grade(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        period_id: Uuid,
        student_id: Uuid,
        section_subject_id: Uuid,
        component_code: Option<String>,
        component_id: Option<Uuid>,
        score: Decimal,
        max_score: Option<Decimal>,
        is_final: bool,
    ) -> Result<Grade, DomainError> {
        self.permission_checker
            .check(ctx, Permission::GradeWrite)
            .await?;

        let context = self
            .validate_grade_context(
                org_id,
                period_id,
                student_id,
                section_subject_id,
                component_code.as_deref(),
                component_id,
                None,
            )
            .await?;
        let max_score = resolve_grade_max_score(context.component_max_score, max_score)?;
        if score < Decimal::ZERO || score > max_score {
            return Err(DomainError::Validation(format!(
                "Score {} is outside allowable range [0, {}]",
                score, max_score
            )));
        }

        let now = Utc::now();
        let grade_id = Uuid::new_v4();

        let grade = Grade {
            id: grade_id,
            org_id,
            period_id,
            student_id,
            enrollment_id: Some(context.enrollment_id).filter(|id| *id != Uuid::nil()),
            section_subject_id,
            component_code: context.component_code,
            component_id,
            score,
            max_score,
            weight: None,
            letter_grade: None,
            is_final,
            is_excused: false,
            is_exempt: false,
            is_absent: false,
            change_reason: None,
            recorded_by: ctx.user_id(),
            updated_by: None,
            notes: None,
            created_at: now,
            updated_at: now,
        };

        self.grade_repo.save(&grade).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::GradeRecorded {
                    grade_id,
                    student_id,
                    section_subject_id,
                    score,
                    timestamp: now,
                },
            )
            .await;

        Ok(grade)
    }

    /// Batch record or update student grades
    pub async fn batch_record_grades(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        items: Vec<BatchGradeItem>,
    ) -> Result<Vec<Grade>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::GradeWrite)
            .await?;
        let now = Utc::now();

        // Preload all unique context entities once to avoid N+1 validation queries.
        let mut period_ids: HashSet<Uuid> = HashSet::new();
        let mut student_ids: HashSet<Uuid> = HashSet::new();
        let mut section_subject_ids: HashSet<Uuid> = HashSet::new();
        let mut enrollment_ids: HashSet<Uuid> = HashSet::new();
        for item in &items {
            period_ids.insert(item.period_id);
            student_ids.insert(item.student_id);
            section_subject_ids.insert(item.section_subject_id);
            if let Some(enrollment_id) = item.enrollment_id {
                enrollment_ids.insert(enrollment_id);
            }
        }

        let mut periods: HashMap<Uuid, AcademicPeriod> = HashMap::new();
        if let Some(period_repo) = &self.period_repo {
            for period_id in &period_ids {
                let period = period_repo
                    .find_by_id(*period_id)
                    .await?
                    .ok_or_else(|| DomainError::NotFound("Academic period not found".into()))?;
                periods.insert(*period_id, period);
            }
        }

        let mut students: HashMap<Uuid, Student> = HashMap::new();
        if let Some(student_repo) = &self.student_repo {
            for student_id in &student_ids {
                let student = student_repo
                    .find_by_id(*student_id)
                    .await?
                    .ok_or_else(|| DomainError::NotFound("Student not found".into()))?;
                students.insert(*student_id, student);
            }
        }

        let mut section_subjects: HashMap<Uuid, SectionSubject> = HashMap::new();
        let mut scheme_ids: HashSet<Uuid> = HashSet::new();
        for section_subject_id in &section_subject_ids {
            let section_subject = self
                .section_subject_repo
                .find_by_id(*section_subject_id)
                .await?
                .ok_or_else(|| DomainError::NotFound("Section subject not found".into()))?;
            if let Some(scheme_id) = section_subject.scheme_id {
                scheme_ids.insert(scheme_id);
            }
            section_subjects.insert(*section_subject_id, section_subject);
        }

        let mut schemes: HashMap<Uuid, AssessmentScheme> = HashMap::new();
        for scheme_id in &scheme_ids {
            let scheme = self
                .scheme_repo
                .find_by_id(*scheme_id)
                .await?
                .ok_or_else(|| DomainError::NotFound("Assessment scheme not found".into()))?;
            schemes.insert(*scheme_id, scheme);
        }

        let mut enrollments_by_id: HashMap<Uuid, Enrollment> = HashMap::new();
        let mut enrollments_by_student: HashMap<Uuid, Vec<Enrollment>> = HashMap::new();
        if let Some(enrollment_repo) = &self.enrollment_repo {
            for enrollment_id in &enrollment_ids {
                let enrollment = enrollment_repo
                    .find_by_id(*enrollment_id)
                    .await?
                    .ok_or_else(|| DomainError::NotFound("Enrollment not found".into()))?;
                enrollments_by_id.insert(*enrollment_id, enrollment);
            }
            for student_id in &student_ids {
                let list = enrollment_repo.find_by_student(*student_id).await?;
                enrollments_by_student.insert(*student_id, list);
            }
        }

        let mut grades = Vec::with_capacity(items.len());
        let mut existing_by_key = HashMap::<(Uuid, Uuid, Uuid, Option<String>), Grade>::new();
        for section_subject_id in &section_subject_ids {
            for existing in self
                .grade_repo
                .find_by_org_and_section_subject(org_id, *section_subject_id)
                .await?
            {
                existing_by_key.insert(
                    (
                        existing.student_id,
                        existing.period_id,
                        existing.section_subject_id,
                        normalize_grade_component_key(
                            existing.component_code.as_deref(),
                            existing.component_id,
                        ),
                    ),
                    existing,
                );
            }
        }
        let mut submitted_keys = HashSet::with_capacity(items.len());

        for item in items {
            if let Some(period) = periods.get(&item.period_id) {
                if period.org_id != org_id {
                    return Err(DomainError::Unauthorized(
                        "Academic period belongs to another organization".into(),
                    ));
                }
                if !period.can_enter_grades() {
                    return Err(DomainError::BusinessRule(
                        "Grade entry is not open for this academic period".into(),
                    ));
                }
            }

            if let Some(student) = students.get(&item.student_id) {
                if student.org_id != org_id {
                    return Err(DomainError::Unauthorized(
                        "Student belongs to another organization".into(),
                    ));
                }
            }

            let section_subject = section_subjects
                .get(&item.section_subject_id)
                .ok_or_else(|| DomainError::NotFound("Section subject not found".into()))?;
            if section_subject.org_id != org_id || section_subject.period_id != item.period_id {
                return Err(DomainError::Unauthorized(
                    "Section subject is not valid for this organization and period".into(),
                ));
            }

            let mut resolved_component_code = item
                .component_code
                .as_deref()
                .map(str::trim)
                .filter(|code| !code.is_empty())
                .map(str::to_owned);
            let mut component_max_score = None;
            let mut resolved_component_id = item.component_id;
            if let Some(scheme_id) = section_subject.scheme_id {
                let scheme = schemes
                    .get(&scheme_id)
                    .ok_or_else(|| DomainError::NotFound("Assessment scheme not found".into()))?;
                if scheme.org_id != org_id {
                    return Err(DomainError::Unauthorized(
                        "Assessment scheme belongs to another organization".into(),
                    ));
                }
                if item.component_code.is_some() || item.component_id.is_some() {
                    let matched_component = scheme.components.iter().find(|component| {
                        item.component_code.as_deref().is_some_and(|code| {
                            component.code.trim().eq_ignore_ascii_case(code.trim())
                        }) || item.component_id.is_some_and(|id| {
                            component.code.trim().eq_ignore_ascii_case(&id.to_string())
                        })
                    });
                    let Some(component) = matched_component else {
                        return Err(DomainError::Validation(
                            "The assessment component is not defined by the section subject's assessment scheme".into(),
                        ));
                    };
                    if item.component_code.as_deref().is_some_and(|code| {
                        !component.code.trim().eq_ignore_ascii_case(code.trim())
                    }) || item.component_id.is_some_and(|id| {
                        !component.code.trim().eq_ignore_ascii_case(&id.to_string())
                    }) {
                        return Err(DomainError::Validation(
                            "Assessment component code and legacy ID do not match".into(),
                        ));
                    }
                    resolved_component_code = Some(component.code.clone());
                    component_max_score = Some(component.max_score);
                    // Keep component_id as provided when present; scheme components are identified by code.
                    if resolved_component_id.is_none() {
                        resolved_component_id = item.component_id;
                    }
                }
            }

            let resolved_enrollment_id = if self.enrollment_repo.is_some() {
                let enrollment = if let Some(enrollment_id) = item.enrollment_id {
                    enrollments_by_id
                        .get(&enrollment_id)
                        .ok_or_else(|| DomainError::NotFound("Enrollment not found".into()))?
                } else {
                    enrollments_by_student
                        .get(&item.student_id)
                        .and_then(|list| list.iter().find(|en| en.period_id == item.period_id))
                        .ok_or_else(|| {
                            DomainError::NotFound(
                                "Active enrollment not found for this period".into(),
                            )
                        })?
                };

                if enrollment.org_id != org_id
                    || enrollment.student_id != item.student_id
                    || enrollment.period_id != item.period_id
                    || enrollment.section_id != Some(section_subject.section_id)
                {
                    return Err(DomainError::Unauthorized(
                        "Enrollment does not match the student, period, and section".into(),
                    ));
                }
                Some(enrollment.id)
            } else {
                None
            };

            let max_score = resolve_grade_max_score(component_max_score, item.max_score)?;
            if item.score < Decimal::ZERO || item.score > max_score {
                return Err(DomainError::Validation(format!(
                    "Score {} is outside allowable range [0, {}] for student {}",
                    item.score, max_score, item.student_id
                )));
            }

            let component_key = normalize_grade_component_key(
                resolved_component_code.as_deref(),
                resolved_component_id,
            );
            let key = (
                item.student_id,
                item.period_id,
                item.section_subject_id,
                component_key,
            );
            if !submitted_keys.insert(key.clone()) {
                return Err(DomainError::Validation(
                    "A grade batch cannot contain duplicate student assessment rows".into(),
                ));
            }
            let existing = existing_by_key.get(&key).cloned();
            let grade_id = existing
                .as_ref()
                .map(|grade| grade.id)
                .unwrap_or_else(Uuid::new_v4);
            let created_at = existing
                .as_ref()
                .map(|grade| grade.created_at)
                .unwrap_or(now);
            let grade = Grade {
                id: grade_id,
                org_id,
                period_id: item.period_id,
                student_id: item.student_id,
                enrollment_id: resolved_enrollment_id.filter(|id| *id != Uuid::nil()),
                section_subject_id: item.section_subject_id,
                component_code: resolved_component_code,
                component_id: resolved_component_id,
                score: item.score,
                max_score,
                weight: None,
                letter_grade: None,
                is_final: false,
                is_excused: item.is_excused,
                is_exempt: item.is_exempt,
                is_absent: item.is_absent,
                change_reason: None,
                recorded_by: existing
                    .as_ref()
                    .and_then(|grade| grade.recorded_by)
                    .or_else(|| ctx.user_id()),
                updated_by: ctx.user_id(),
                notes: item.notes,
                created_at,
                updated_at: now,
            };
            existing_by_key.insert(key, grade.clone());
            grades.push(grade);
        }

        self.grade_repo.save_batch(&grades).await?;

        for g in &grades {
            let _ = self
                .event_bus
                .publish(
                    ctx,
                    &DomainEvent::GradeRecorded {
                        grade_id: g.id,
                        student_id: g.student_id,
                        section_subject_id: g.section_subject_id,
                        score: g.score,
                        timestamp: now,
                    },
                )
                .await;
        }

        Ok(grades)
    }

    /// Delete a grade record
    pub async fn delete_grade(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        grade_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::GradeWrite)
            .await?;

        let grade = self
            .grade_repo
            .find_by_id(grade_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Grade {} not found", grade_id)))?;

        if grade.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        self.grade_repo.delete(grade_id).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::GradeDeleted {
                    grade_id,
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(())
    }

    /// Calculate final grade from assessment scheme formula
    pub async fn calculate_scheme_grades(
        &self,
        ctx: &RequestContext,
        scheme_id: Uuid,
        student_id: Uuid,
        section_subject_id: Uuid,
    ) -> Result<Decimal, DomainError> {
        self.authorize_grade_analytics(ctx).await?;
        let org_id = ctx.tenant_id;
        let student_repo = self.student_repo.as_ref().ok_or_else(|| {
            DomainError::Internal("Student repository is required for grade calculations".into())
        })?;
        let student = student_repo
            .find_by_id(student_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Student not found".into()))?;
        if student.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        if !section_subject_id.is_nil() {
            let section_subject = self
                .section_subject_repo
                .find_by_id(section_subject_id)
                .await?
                .ok_or_else(|| DomainError::NotFound("Section subject not found".into()))?;
            if section_subject.org_id != org_id {
                return Err(DomainError::Unauthorized("Organization mismatch".into()));
            }
        }

        let scheme = self
            .scheme_repo
            .find_by_org(org_id)
            .await?
            .into_iter()
            .find(|scheme| scheme.id == scheme_id)
            .ok_or_else(|| DomainError::NotFound("Assessment Scheme not found".into()))?;

        let formula_val = scheme
            .formula
            .ok_or_else(|| DomainError::Validation("Scheme has no calculation formula".into()))?;

        let formula: AstNode = serde_json::from_value(formula_val)
            .map_err(|e| DomainError::Validation(format!("Invalid formula format: {}", e)))?;

        let grades = self
            .grade_repo
            .find_by_org_student_and_subject(org_id, student_id, section_subject_id)
            .await?;

        let mut engine = CalcEngine::new();
        for grade in grades {
            let var_name = grade
                .component_code
                .clone()
                .or_else(|| grade.component_id.map(|cid| cid.to_string()))
                .unwrap_or_else(|| grade.section_subject_id.to_string());
            engine.set_variable(&var_name, grade.score);
        }

        let max_score_decimal = scheme.max_score;
        let final_score = engine.evaluate(&formula)?;
        let final_score = if final_score > max_score_decimal {
            max_score_decimal
        } else {
            final_score
        };

        Ok(final_score)
    }

    /// Calculate and record GPA for a student in a specific academic period
    pub async fn calculate_gpa(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        period_id: Uuid,
        student_id: Uuid,
    ) -> Result<CumulativeGpaRecord, DomainError> {
        authorize_grade_access(
            self.permission_checker.as_ref(),
            ctx,
            org_id,
            Permission::GradeAnalytics,
        )
        .await?;

        let student_repo = self.student_repo.as_ref().ok_or_else(|| {
            DomainError::Internal("Student repository is required for GPA calculation".into())
        })?;
        let student = student_repo
            .find_by_id(student_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Student not found".into()))?;
        if student.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        let period_repo = self.period_repo.as_ref().ok_or_else(|| {
            DomainError::Internal(
                "Academic period repository is required for GPA calculation".into(),
            )
        })?;
        let period = period_repo
            .find_by_id(period_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Academic period not found".into()))?;
        if period.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        let term_grades = self
            .grade_repo
            .find_by_org_student_and_period(org_id, student_id, period_id)
            .await?;
        let past_records = self
            .gpa_repo
            .find_by_student_in_org(org_id, student_id)
            .await?;

        let mut term_attempted = Decimal::ZERO;
        let mut term_earned = Decimal::ZERO;
        let mut term_points = Decimal::ZERO;

        let policy = self.resolve_policy(org_id, None).await;
        let pass_threshold = policy.passing_threshold_percentage;

        for g in &term_grades {
            if g.is_exempt || g.is_excused {
                continue;
            }

            // Resolve actual subject credit hours
            let credit = if let Some(cs) = self
                .section_subject_repo
                .find_by_id(g.section_subject_id)
                .await?
            {
                if cs.org_id != org_id {
                    return Err(DomainError::Unauthorized("Organization mismatch".into()));
                }
                if let Some(subj) = self.subject_repo.find_by_id(cs.subject_id).await? {
                    if subj.org_id != org_id {
                        return Err(DomainError::Unauthorized("Organization mismatch".into()));
                    }
                    subj.credit_units.unwrap_or(Decimal::from(3))
                } else {
                    Decimal::from(3)
                }
            } else {
                Decimal::from(3)
            };

            let pct = if g.max_score > Decimal::ZERO {
                (g.score / g.max_score) * Decimal::from(100)
            } else {
                g.score
            };

            term_attempted += credit;
            if pct >= pass_threshold {
                term_earned += credit;
            }
            let eval = policy.evaluate_percentage(pct);
            term_points += eval.grade_points * credit;
        }

        let term_gpa = if term_attempted > Decimal::ZERO {
            (term_points / term_attempted).round_dp(2)
        } else {
            Decimal::ZERO
        };

        let mut cum_attempted = term_attempted;
        let mut cum_earned = term_earned;
        let mut cum_points = term_points;

        for rec in &past_records {
            if rec.period_id != period_id {
                cum_attempted += rec.term_attempted_credits;
                cum_earned += rec.term_earned_credits;
                cum_points += rec.term_points;
            }
        }

        let cumulative_gpa = if cum_attempted > Decimal::ZERO {
            (cum_points / cum_attempted).round_dp(2)
        } else {
            Decimal::ZERO
        };

        let standing = if cumulative_gpa
            >= Decimal::from_str_exact("3.5").unwrap_or(Decimal::from(3))
        {
            AcademicStanding::HonorList
        } else if cumulative_gpa >= Decimal::from(2) {
            AcademicStanding::GoodStanding
        } else if cumulative_gpa >= Decimal::from_str_exact("1.75").unwrap_or(Decimal::from(1)) {
            AcademicStanding::AcademicWarning1
        } else {
            AcademicStanding::Probation
        };

        let now = Utc::now();
        let record = CumulativeGpaRecord {
            id: Uuid::new_v4(),
            org_id,
            student_id,
            period_id,
            term_attempted_credits: term_attempted,
            term_earned_credits: term_earned,
            term_points,
            term_gpa,
            cumulative_attempted_credits: cum_attempted,
            cumulative_earned_credits: cum_earned,
            cumulative_points: cum_points,
            cumulative_gpa,
            standing,
            calculated_at: now,
        };

        self.gpa_repo.save(&record).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::GpaCalculated {
                    student_id,
                    period_id,
                    gpa: term_gpa,
                    cgpa: cumulative_gpa,
                    timestamp: now,
                },
            )
            .await;

        Ok(record)
    }

    /// Mark attendance record
    pub async fn mark_attendance(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        _period_id: Uuid,
        student_id: Uuid,
        section_id: Option<Uuid>,
        section_subject_id: Option<Uuid>,
        date: NaiveDate,
        status: String,
        notes: Option<String>,
    ) -> Result<Attendance, DomainError> {
        self.permission_checker
            .check(ctx, Permission::AttendanceWrite)
            .await?;
        self.ensure_legacy_attendance_allowed(ctx, org_id).await?;

        let now = Utc::now();
        let att_id = Uuid::new_v4();

        let attendance = Attendance {
            id: att_id,
            student_id,
            section_id,
            section_subject_id,
            scheduled_lesson_id: None,
            attendance_date: date,
            status: status.clone(),
            notes,
            marked_by: ctx.user_id(),
            created_at: now,
            updated_at: now,
        };

        self.attendance_repo.save(&attendance).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::AttendanceMarked {
                    attendance_id: att_id,
                    student_id,
                    section_id,
                    section_subject_id,
                    scheduled_lesson_id: None,
                    status,
                    timestamp: now,
                },
            )
            .await;

        Ok(attendance)
    }

    /// Batch mark attendance records for a class on a date
    pub async fn batch_mark_attendance(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        section_id: Uuid,
        date: NaiveDate,
        items: Vec<BatchAttendanceItem>,
    ) -> Result<Vec<Attendance>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::AttendanceWrite)
            .await?;

        if items.iter().any(|item| item.scheduled_lesson_id.is_some()) {
            self.validate_lesson_attendance_scope(ctx, org_id, section_id, &items)
                .await?;
        } else {
            self.ensure_legacy_attendance_allowed(ctx, org_id).await?;
        }

        let now = Utc::now();
        let existing_attendances = self
            .attendance_repo
            .find_by_section_and_date(section_id, date)
            .await?;
        let mut attendances = Vec::with_capacity(items.len());

        for item in items {
            let existing = existing_attendances.iter().find(|attendance| {
                attendance.student_id == item.student_id
                    && attendance.section_subject_id == item.section_subject_id
                    && attendance.scheduled_lesson_id == item.scheduled_lesson_id
            });
            attendances.push(Attendance {
                id: existing.map_or_else(Uuid::new_v4, |attendance| attendance.id),
                student_id: item.student_id,
                section_id: Some(section_id),
                section_subject_id: item.section_subject_id,
                scheduled_lesson_id: item.scheduled_lesson_id,
                attendance_date: date,
                status: item.status,
                notes: item.notes,
                marked_by: ctx.user_id(),
                created_at: existing.map_or(now, |attendance| attendance.created_at),
                updated_at: now,
            });
        }

        self.attendance_repo.save_batch(&attendances).await?;

        for a in &attendances {
            let _ = self
                .event_bus
                .publish(
                    ctx,
                    &DomainEvent::AttendanceMarked {
                        attendance_id: a.id,
                        student_id: a.student_id,
                        section_id: a.section_id,
                        section_subject_id: a.section_subject_id,
                        scheduled_lesson_id: a.scheduled_lesson_id,
                        status: a.status.clone(),
                        timestamp: now,
                    },
                )
                .await;
        }

        Ok(attendances)
    }

    async fn validate_lesson_attendance_scope(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        section_id: Uuid,
        items: &[BatchAttendanceItem],
    ) -> Result<(), DomainError> {
        let first = items
            .first()
            .ok_or_else(|| DomainError::Validation("Attendance items are required".into()))?;
        let scheduled_lesson_id = first.scheduled_lesson_id.ok_or_else(|| {
            DomainError::Validation(
                "A batch cannot mix lesson-specific and general attendance".into(),
            )
        })?;
        let section_subject_id = first.section_subject_id.ok_or_else(|| {
            DomainError::Validation("A section subject is required for lesson attendance".into())
        })?;
        let mut student_ids = std::collections::HashSet::with_capacity(items.len());
        for item in items {
            if item.scheduled_lesson_id != Some(scheduled_lesson_id)
                || item.section_subject_id != Some(section_subject_id)
            {
                return Err(DomainError::Validation(
                    "All attendance items must belong to the same scheduled lesson".into(),
                ));
            }
            if !student_ids.insert(item.student_id) {
                return Err(DomainError::Validation(
                    "A student can only be included once in an attendance batch".into(),
                ));
            }
        }

        let lesson_repo = self.scheduled_lesson_repo.as_ref().ok_or_else(|| {
            DomainError::Internal("Scheduled lesson repository is not configured".into())
        })?;
        let lesson = lesson_repo
            .find_by_id(scheduled_lesson_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Scheduled lesson not found".into()))?;
        if lesson.section_subject_id != section_subject_id {
            return Err(DomainError::Validation(
                "Scheduled lesson does not belong to the selected section subject".into(),
            ));
        }

        let section_subject = self
            .section_subject_repo
            .find_by_id(section_subject_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Section subject not found".into()))?;
        if section_subject.section_id != section_id
            || section_subject.org_id != org_id
            || !section_subject.is_active
        {
            return Err(DomainError::Forbidden(
                "Scheduled lesson is outside the requested section".into(),
            ));
        }

        let enrollment_repo = self.enrollment_repo.as_ref().ok_or_else(|| {
            DomainError::Internal("Enrollment repository is not configured".into())
        })?;
        let enrolled_students: std::collections::HashSet<Uuid> = enrollment_repo
            .find_by_section(section_id)
            .await?
            .into_iter()
            .filter(|enrollment| {
                enrollment.org_id == org_id
                    && enrollment.period_id == section_subject.period_id
                    && enrollment.status == EnrollmentStatus::Active
            })
            .map(|enrollment| enrollment.student_id)
            .collect();
        if items
            .iter()
            .any(|item| !enrolled_students.contains(&item.student_id))
        {
            return Err(DomainError::Validation(
                "Attendance can only be recorded for students enrolled in this section".into(),
            ));
        }

        self.ensure_teacher_lesson_assignment(ctx, org_id, &lesson, &section_subject)
            .await?;

        Ok(())
    }

    async fn teacher_employee(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
    ) -> Result<Option<orbis_domain::entities::employee::Employee>, DomainError> {
        let Some(user_id) = ctx.user_id() else {
            return Ok(None);
        };
        let employee_repo = self
            .employee_repo
            .as_ref()
            .ok_or_else(|| DomainError::Internal("Employee repository is not configured".into()))?;
        Ok(employee_repo
            .find_by_user_id(org_id, user_id)
            .await?
            .filter(|employee| employee.is_teacher()))
    }

    async fn ensure_legacy_attendance_allowed(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
    ) -> Result<(), DomainError> {
        if self.teacher_employee(ctx, org_id).await?.is_some() {
            return Err(DomainError::Forbidden(
                "Teachers must record attendance against a scheduled lesson".into(),
            ));
        }
        Ok(())
    }

    async fn ensure_teacher_lesson_assignment(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        lesson: &orbis_domain::entities::scheduled_lesson::ScheduledLesson,
        section_subject: &orbis_domain::entities::section_subject::SectionSubject,
    ) -> Result<(), DomainError> {
        if let Some(employee) = self.teacher_employee(ctx, org_id).await? {
            if !employee.is_active()
                || !teacher_is_assigned_to_lesson(employee.id, lesson, section_subject)
            {
                return Err(DomainError::Forbidden(
                    "Teachers may only record attendance for their assigned lesson".into(),
                ));
            }
        }
        Ok(())
    }

    /// Delete an attendance record
    pub async fn delete_attendance(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        attendance_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::AttendanceWrite)
            .await?;

        if let Some(employee) = self.teacher_employee(ctx, org_id).await? {
            let attendance = self
                .attendance_repo
                .find_by_id(attendance_id)
                .await?
                .ok_or_else(|| DomainError::NotFound("Attendance record not found".into()))?;
            let scheduled_lesson_id = attendance.scheduled_lesson_id.ok_or_else(|| {
                DomainError::Forbidden(
                    "Teachers may only delete attendance for their assigned lesson".into(),
                )
            })?;
            let section_subject_id = attendance.section_subject_id.ok_or_else(|| {
                DomainError::Forbidden(
                    "Teachers may only delete attendance for their assigned lesson".into(),
                )
            })?;
            let lesson_repo = self.scheduled_lesson_repo.as_ref().ok_or_else(|| {
                DomainError::Internal("Scheduled lesson repository is not configured".into())
            })?;
            let lesson = lesson_repo
                .find_by_id(scheduled_lesson_id)
                .await?
                .ok_or_else(|| DomainError::NotFound("Scheduled lesson not found".into()))?;
            let section_subject = self
                .section_subject_repo
                .find_by_id(section_subject_id)
                .await?
                .ok_or_else(|| DomainError::NotFound("Section subject not found".into()))?;
            if section_subject.org_id != org_id
                || !section_subject.is_active
                || attendance.section_id != Some(section_subject.section_id)
                || lesson.section_subject_id != section_subject_id
                || !teacher_is_assigned_to_lesson(employee.id, &lesson, &section_subject)
            {
                return Err(DomainError::Forbidden(
                    "Teachers may only delete attendance for their assigned lesson".into(),
                ));
            }
        }

        self.attendance_repo.delete(attendance_id).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::AttendanceDeleted {
                    attendance_id,
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(())
    }

    /// List student attendance records
    pub async fn list_student_attendance(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
    ) -> Result<Vec<Attendance>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::AttendanceRead)
            .await?;
        self.attendance_repo.find_by_student(student_id).await
    }

    /// Create assessment scheme
    pub async fn create_assessment_scheme(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        name: String,
        name_ar: Option<String>,
        description: Option<String>,
        max_score: Decimal,
        formula: Option<serde_json::Value>,
        level: Option<AssessmentLevel>,
        institution_scope: Option<InstitutionScope>,
        components: Option<Vec<AssessmentComponent>>,
        is_default: bool,
    ) -> Result<AssessmentScheme, DomainError> {
        self.permission_checker
            .check(ctx, Permission::AssessmentWrite)
            .await?;

        validate_assessment_components(max_score, components.as_deref().unwrap_or_default())?;

        let now = Utc::now();
        let scheme_id = Uuid::new_v4();
        let scheme = AssessmentScheme {
            id: scheme_id,
            org_id,
            name: name.clone(),
            name_ar,
            description,
            max_score,
            formula,
            level: level.unwrap_or(AssessmentLevel::MonthlyPeriod),
            institution_scope: institution_scope.unwrap_or(InstitutionScope::All),
            components: components.unwrap_or_default(),
            is_default,
            is_system: false,
            source_document: None,
            source_authority: None,
            verification_status: None,
            effective_date: None,
            policy_version: 0,
            min_attendance_ratio: None,
            pass_threshold: None,
            created_at: now,
            updated_at: now,
        };

        self.scheme_repo.save(&scheme).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::AssessmentSchemeCreated {
                    scheme_id,
                    org_id,
                    name,
                    timestamp: now,
                },
            )
            .await;

        Ok(scheme)
    }

    /// Update assessment scheme
    pub async fn update_assessment_scheme(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        scheme_id: Uuid,
        name: Option<String>,
        name_ar: Option<String>,
        description: Option<String>,
        max_score: Option<Decimal>,
        formula: Option<serde_json::Value>,
        level: Option<AssessmentLevel>,
        institution_scope: Option<InstitutionScope>,
        components: Option<Vec<AssessmentComponent>>,
        is_default: Option<bool>,
    ) -> Result<AssessmentScheme, DomainError> {
        self.permission_checker
            .check(ctx, Permission::AssessmentWrite)
            .await?;

        let mut scheme = self
            .scheme_repo
            .find_by_id(scheme_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Assessment Scheme {} not found", scheme_id))
            })?;

        if scheme.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }
        let effective_max_score = max_score.unwrap_or(scheme.max_score);
        if let Some(ref components) = components {
            validate_assessment_components(effective_max_score, components)?;
        }

        if let Some(n) = name {
            scheme.name = n;
        }
        if let Some(nar) = name_ar {
            scheme.name_ar = Some(nar);
        }
        if let Some(d) = description {
            scheme.description = Some(d);
        }
        if let Some(m) = max_score {
            scheme.max_score = m;
        }
        if let Some(f) = formula {
            scheme.formula = Some(f);
        }
        if let Some(lvl) = level {
            scheme.level = lvl;
        }
        if let Some(scope) = institution_scope {
            scheme.institution_scope = scope;
        }
        if let Some(comps) = components {
            scheme.components = comps;
        }
        if let Some(def) = is_default {
            scheme.is_default = def;
        }
        scheme.updated_at = Utc::now();

        self.scheme_repo.save(&scheme).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::AssessmentSchemeUpdated {
                    scheme_id,
                    org_id,
                    timestamp: scheme.updated_at,
                },
            )
            .await;

        Ok(scheme)
    }

    /// Delete assessment scheme
    pub async fn delete_assessment_scheme(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        scheme_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::AssessmentDelete)
            .await?;

        let scheme = self
            .scheme_repo
            .find_by_id(scheme_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Assessment Scheme {} not found", scheme_id))
            })?;

        if scheme.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        self.scheme_repo.delete(scheme_id).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::AssessmentSchemeDeleted {
                    scheme_id,
                    org_id,
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(())
    }

    /// List assessment schemes
    pub async fn list_assessment_schemes(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<AssessmentScheme>, DomainError> {
        self.scheme_repo.find_by_org(org_id).await
    }

    /// List grades
    pub async fn list_grades(
        &self,
        org_id: Uuid,
        filter: GradeFilter,
    ) -> Result<PaginatedList<Grade>, DomainError> {
        self.grade_repo.list(org_id, filter).await
    }

    pub async fn list_grades_for_request(
        &self,
        ctx: &RequestContext,
        student_id: Option<Uuid>,
        section_subject_id: Option<Uuid>,
    ) -> Result<Vec<Grade>, DomainError> {
        authorize_grade_access(
            self.permission_checker.as_ref(),
            ctx,
            ctx.tenant_id,
            Permission::GradeRead,
        )
        .await?;

        if let Some(student_id) = student_id {
            self.grade_repo
                .find_by_org_and_student(ctx.tenant_id, student_id)
                .await
        } else if let Some(section_subject_id) = section_subject_id {
            self.grade_repo
                .find_by_org_and_section_subject(ctx.tenant_id, section_subject_id)
                .await
        } else {
            Ok(vec![])
        }
    }

    pub async fn authorize_grade_analytics(&self, ctx: &RequestContext) -> Result<(), DomainError> {
        authorize_grade_access(
            self.permission_checker.as_ref(),
            ctx,
            ctx.tenant_id,
            Permission::GradeAnalytics,
        )
        .await
    }

    /// List attendance
    pub async fn list_attendance(
        &self,
        org_id: Uuid,
        filter: AttendanceFilter,
    ) -> Result<PaginatedList<Attendance>, DomainError> {
        self.attendance_repo.list(org_id, filter).await
    }

    /// Record behavior for a student
    pub async fn record_behavior(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        student_id: Uuid,
        record_type: String,
        description: String,
        points: i32,
    ) -> Result<BehaviorRecord, DomainError> {
        self.permission_checker
            .check(ctx, Permission::BehaviorWrite)
            .await?;
        let now = Utc::now();
        let record_id = Uuid::new_v4();
        let record = BehaviorRecord {
            id: record_id,
            student_id,
            org_id,
            reporter_id: ctx.user_id().unwrap_or(Uuid::nil()),
            record_type: record_type.clone(),
            description,
            points,
            created_at: now,
            updated_at: now,
        };
        self.behavior_repo.save(&record).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::BehaviorLogged {
                    record_id,
                    student_id,
                    record_type,
                    points,
                    timestamp: now,
                },
            )
            .await;

        Ok(record)
    }

    /// Delete behavior record
    pub async fn delete_behavior(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        behavior_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::BehaviorDelete)
            .await?;

        let rec = self
            .behavior_repo
            .find_by_id(behavior_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Behavior record {} not found", behavior_id))
            })?;

        if rec.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        self.behavior_repo.delete(behavior_id).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::BehaviorDeleted {
                    record_id: behavior_id,
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(())
    }

    /// List behavior records for a student
    pub async fn list_student_behavior_records(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
    ) -> Result<Vec<BehaviorRecord>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::BehaviorRead)
            .await?;
        self.behavior_repo.find_by_student(student_id).await
    }

    /// List all behavior records for an organization
    pub async fn list_behavior_records(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
    ) -> Result<Vec<BehaviorRecord>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::BehaviorRead)
            .await?;
        self.behavior_repo.find_by_org(org_id).await
    }

    /// Calculate comprehensive student term report card
    pub async fn calculate_student_term_report_card(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        student_id: Uuid,
        period_id: Uuid,
        section_id: Option<Uuid>,
        policy: Option<AcademicPolicy>,
    ) -> Result<StudentTermReportCard, DomainError> {
        self.permission_checker
            .check(ctx, Permission::GradeRead)
            .await?;

        let policy = self.resolve_policy(org_id, policy).await;

        // 1. Resolve student details
        let (student_name, student_number) = if let Some(sr) = &self.student_repo {
            if let Some(detail) = sr.find_detail_by_id(student_id).await? {
                (
                    detail.person.full_name(),
                    Some(detail.student.student_number),
                )
            } else if let Some(st) = sr.find_by_id(student_id).await? {
                (
                    format!("Student {}", &st.id.to_string()[..8]),
                    Some(st.student_number),
                )
            } else {
                ("Student".to_string(), None)
            }
        } else {
            ("Student".to_string(), None)
        };

        // 2. Resolve period details and child periods
        let (period_name, child_periods) = if let Some(pr) = &self.period_repo {
            let p = pr.find_by_id(period_id).await?.ok_or_else(|| {
                DomainError::NotFound(format!("Academic period {} not found", period_id))
            })?;
            let all_year_periods = pr.find_by_year(p.year_id).await?;
            let mut children: Vec<_> = all_year_periods
                .into_iter()
                .filter(|cp| cp.parent_id == Some(period_id))
                .collect();
            children.sort_by_key(|cp| cp.sort_order);
            (p.name, children)
        } else {
            ("Term Period".to_string(), vec![])
        };

        // 3. Resolve class
        let resolved_section_id = if let Some(cid) = section_id {
            cid
        } else if let Some(er) = &self.enrollment_repo {
            let enrollments = er.find_by_student(student_id).await?;
            enrollments
                .into_iter()
                .find(|e| e.period_id == period_id || e.status == EnrollmentStatus::Active)
                .and_then(|e| e.section_id)
                .unwrap_or_else(Uuid::nil)
        } else {
            Uuid::nil()
        };

        let class_name = if let Some(cr) = &self.section_repo {
            if let Some(cls) = cr.find_by_id(resolved_section_id).await? {
                cls.name
            } else {
                "Class".to_string()
            }
        } else {
            "Class".to_string()
        };

        // 4. Resolve class subjects
        let section_subjects = if resolved_section_id != Uuid::nil() {
            let cs_list = self
                .section_subject_repo
                .find_by_section_and_period(resolved_section_id, period_id)
                .await?;
            if cs_list.is_empty() {
                self.section_subject_repo
                    .find_by_section(resolved_section_id)
                    .await?
            } else {
                cs_list
            }
        } else {
            vec![]
        };

        // 5. Calculate subject term results
        let mut subject_results = Vec::with_capacity(section_subjects.len());

        for cs in &section_subjects {
            let subject = self.subject_repo.find_by_id(cs.subject_id).await?;
            let (subject_code, subject_name, subject_name_ar, credit_units) = match &subject {
                Some(s) => (
                    s.code.clone(),
                    s.name.clone(),
                    s.name_ar.clone(),
                    s.credit_units.unwrap_or(Decimal::ONE),
                ),
                None => (
                    "SUBJ".to_string(),
                    "Subject".to_string(),
                    None,
                    Decimal::ONE,
                ),
            };

            let all_grades = self
                .grade_repo
                .find_by_student_and_subject(student_id, cs.id)
                .await?;

            let mut monthly_results = Vec::new();
            let mut term_final_exam = None;

            if !child_periods.is_empty() {
                for cp in &child_periods {
                    let is_exam = cp.period_kind == PeriodKind::Examination
                        || cp
                            .code
                            .as_deref()
                            .unwrap_or("")
                            .to_uppercase()
                            .contains("EXAM");

                    let period_grades: Vec<&Grade> =
                        all_grades.iter().filter(|g| g.period_id == cp.id).collect();

                    if is_exam {
                        if let Some(first_grade) = period_grades.first() {
                            term_final_exam = Some(first_grade.score);
                        }
                    } else {
                        // Monthly period
                        let components: Vec<ComponentScore> = if period_grades.is_empty() {
                            vec![]
                        } else {
                            period_grades
                                .iter()
                                .map(|g| ComponentScore {
                                    code: g
                                        .component_code
                                        .clone()
                                        .or_else(|| g.component_id.map(|cid| cid.to_string()))
                                        .unwrap_or_else(|| "TOTAL".into()),
                                    name: g.notes.clone().unwrap_or_else(|| "Score".into()),
                                    name_ar: None,
                                    score: g.score,
                                    max_score: g.max_score,
                                })
                                .collect()
                        };

                        let mut breakdown = PeriodComponentScores {
                            components,
                            total: Decimal::ZERO,
                            total_max: Decimal::from(100),
                        };
                        breakdown.compute_total();

                        monthly_results.push(PeriodAssessmentResult {
                            period_id: cp.id,
                            period_code: cp.code.clone().unwrap_or_default(),
                            period_name: cp.name.clone(),
                            period_name_ar: cp.name_ar.clone(),
                            breakdown,
                        });
                    }
                }
            } else {
                // Flat/direct grades
                let mut direct_components = Vec::new();
                for g in &all_grades {
                    if g.is_final {
                        term_final_exam = Some(g.score);
                    } else {
                        direct_components.push(ComponentScore {
                            code: g
                                .component_code
                                .clone()
                                .or_else(|| g.component_id.map(|cid| cid.to_string()))
                                .unwrap_or_else(|| "TOTAL".into()),
                            name: g.notes.clone().unwrap_or_else(|| "Score".into()),
                            name_ar: None,
                            score: g.score,
                            max_score: g.max_score,
                        });
                    }
                }
                if !direct_components.is_empty() {
                    let mut breakdown = PeriodComponentScores {
                        components: direct_components,
                        total: Decimal::ZERO,
                        total_max: Decimal::from(100),
                    };
                    breakdown.compute_total();
                    monthly_results.push(PeriodAssessmentResult {
                        period_id,
                        period_code: "TERM".into(),
                        period_name: period_name.clone(),
                        period_name_ar: None,
                        breakdown,
                    });
                }
            }

            let res = AcademicCalculationEngine::calculate_subject_term_result(
                cs.subject_id,
                subject_code,
                subject_name,
                subject_name_ar,
                credit_units,
                monthly_results,
                term_final_exam,
                &policy,
            )?;
            subject_results.push(res);
        }

        // 6. Aggregate student term report card
        let total_score: Decimal = subject_results.iter().map(|s| s.term_total).sum();
        let max_possible_score = Decimal::from(subject_results.len()) * policy.term_max_score;
        let raw_pct = if max_possible_score > Decimal::ZERO {
            (total_score / max_possible_score) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };
        let overall_percentage =
            apply_rounding(raw_pct, policy.rounding_mode, policy.decimal_places);

        let failed_count = subject_results
            .iter()
            .filter(|s| s.status == AcademicStatus::Failed)
            .count();
        let any_incomplete = subject_results
            .iter()
            .any(|s| s.status == AcademicStatus::Incomplete);

        let status = if any_incomplete {
            AcademicStatus::Incomplete
        } else if failed_count == 0 {
            if overall_percentage >= policy.honors_threshold_percentage {
                AcademicStatus::PassedWithHonors
            } else if overall_percentage >= policy.passing_threshold_percentage {
                AcademicStatus::Passed
            } else {
                AcademicStatus::Failed
            }
        } else if failed_count <= policy.max_failed_subjects_for_conditional_pass {
            AcademicStatus::ConditionalPass
        } else {
            AcademicStatus::Failed
        };

        Ok(StudentTermReportCard {
            student_id,
            student_name,
            student_number,
            section_id: resolved_section_id,
            class_name,
            period_id,
            period_name,
            subjects: subject_results,
            total_score,
            max_possible_score,
            overall_percentage,
            overall_letter_grade: {
                let eval = policy.evaluate_percentage(overall_percentage);
                Some(eval.letter_grade)
            },
            overall_letter_grade_ar: {
                let eval = policy.evaluate_percentage(overall_percentage);
                Some(eval.letter_grade_ar)
            },
            gpa: None,
            rank_in_class: None,
            rank_in_grade: None,
            total_students_in_class: 0,
            total_students_in_grade: 0,
            status,
            calculated_at: Utc::now(),
        })
    }

    /// Calculate comprehensive student annual report card
    pub async fn calculate_student_annual_report_card(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        student_id: Uuid,
        calendar_id: Uuid,
        section_id: Option<Uuid>,
        policy: Option<AcademicPolicy>,
    ) -> Result<StudentAnnualReportCard, DomainError> {
        self.permission_checker
            .check(ctx, Permission::GradeRead)
            .await?;

        let policy = self.resolve_policy(org_id, policy).await;

        let (year_name, term_periods) = if let Some(pr) = &self.period_repo {
            let all_periods = pr.find_by_year(calendar_id).await?;
            let mut terms: Vec<_> = all_periods
                .into_iter()
                .filter(|p| p.parent_id.is_none())
                .collect();
            terms.sort_by_key(|p| p.sort_order);

            let yr_name = if let Some(yr) = &self.year_repo {
                yr.find_by_id(calendar_id)
                    .await?
                    .map(|y| y.name)
                    .unwrap_or_else(|| "Academic Year".into())
            } else {
                "Academic Year".into()
            };
            (yr_name, terms)
        } else {
            ("Academic Year".into(), vec![])
        };

        let term_1_card = if let Some(t1) = term_periods.first() {
            Some(
                self.calculate_student_term_report_card(
                    ctx,
                    org_id,
                    student_id,
                    t1.id,
                    section_id,
                    Some(policy.clone()),
                )
                .await?,
            )
        } else {
            None
        };

        let term_2_card = if term_periods.len() > 1 {
            let t2 = &term_periods[1];
            Some(
                self.calculate_student_term_report_card(
                    ctx,
                    org_id,
                    student_id,
                    t2.id,
                    section_id,
                    Some(policy.clone()),
                )
                .await?,
            )
        } else {
            None
        };

        let student_name = term_1_card
            .as_ref()
            .map(|c| c.student_name.clone())
            .or_else(|| term_2_card.as_ref().map(|c| c.student_name.clone()))
            .unwrap_or_else(|| "Student".into());

        let student_number = term_1_card
            .as_ref()
            .and_then(|c| c.student_number.clone())
            .or_else(|| term_2_card.as_ref().and_then(|c| c.student_number.clone()));

        let resolved_section_id = section_id.unwrap_or_else(|| {
            term_1_card
                .as_ref()
                .map(|c| c.section_id)
                .or_else(|| term_2_card.as_ref().map(|c| c.section_id))
                .unwrap_or_else(Uuid::nil)
        });

        let class_name = term_1_card
            .as_ref()
            .map(|c| c.class_name.clone())
            .or_else(|| term_2_card.as_ref().map(|c| c.class_name.clone()))
            .unwrap_or_else(|| "Class".into());

        // Match subjects
        let mut subject_map: std::collections::BTreeMap<
            Uuid,
            (Option<SubjectTermResult>, Option<SubjectTermResult>),
        > = std::collections::BTreeMap::new();

        if let Some(t1) = &term_1_card {
            for s in &t1.subjects {
                subject_map.entry(s.subject_id).or_insert((None, None)).0 = Some(s.clone());
            }
        }
        if let Some(t2) = &term_2_card {
            for s in &t2.subjects {
                subject_map.entry(s.subject_id).or_insert((None, None)).1 = Some(s.clone());
            }
        }

        let mut annual_subjects = Vec::with_capacity(subject_map.len());
        for (subject_id, (t1_res, t2_res)) in subject_map {
            let (code, name, name_ar, credit) = if let Some(r) = t1_res.as_ref().or(t2_res.as_ref())
            {
                (
                    r.subject_code.clone(),
                    r.subject_name.clone(),
                    r.subject_name_ar.clone(),
                    r.credit_hours,
                )
            } else {
                ("SUBJ".into(), "Subject".into(), None, Decimal::ONE)
            };

            let annual_res = AcademicCalculationEngine::calculate_subject_annual_result(
                subject_id, code, name, name_ar, credit, t1_res, t2_res, &policy,
            )?;
            annual_subjects.push(annual_res);
        }

        let total_score: Decimal = annual_subjects.iter().map(|s| s.annual_total).sum();
        let max_possible_score = Decimal::from(annual_subjects.len()) * policy.annual_max_score;
        let raw_pct = if max_possible_score > Decimal::ZERO {
            (total_score / max_possible_score) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };
        let overall_percentage =
            apply_rounding(raw_pct, policy.rounding_mode, policy.decimal_places);

        let failed_count = annual_subjects
            .iter()
            .filter(|s| s.status == AcademicStatus::Failed)
            .count();
        let any_incomplete = annual_subjects
            .iter()
            .any(|s| s.status == AcademicStatus::Incomplete);

        let status = if any_incomplete {
            AcademicStatus::Incomplete
        } else if failed_count == 0 {
            if overall_percentage >= policy.honors_threshold_percentage {
                AcademicStatus::PassedWithHonors
            } else if overall_percentage >= policy.passing_threshold_percentage {
                AcademicStatus::Passed
            } else {
                AcademicStatus::Failed
            }
        } else if failed_count <= policy.max_failed_subjects_for_conditional_pass {
            AcademicStatus::ConditionalPass
        } else {
            AcademicStatus::Failed
        };

        Ok(StudentAnnualReportCard {
            student_id,
            student_name,
            student_number,
            section_id: resolved_section_id,
            class_name,
            calendar_id,
            academic_calendar_name: year_name,
            subjects: annual_subjects,
            total_score,
            max_possible_score,
            overall_percentage,
            overall_letter_grade: {
                let eval = policy.evaluate_percentage(overall_percentage);
                Some(eval.letter_grade)
            },
            overall_letter_grade_ar: {
                let eval = policy.evaluate_percentage(overall_percentage);
                Some(eval.letter_grade_ar)
            },
            cumulative_gpa: None,
            rank_in_class: None,
            rank_in_grade: None,
            total_students_in_class: 0,
            total_students_in_grade: 0,
            status,
            calculated_at: Utc::now(),
        })
    }

    /// Calculate class term results with rankings
    pub async fn calculate_class_term_report_cards(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        section_id: Uuid,
        period_id: Uuid,
        policy: Option<AcademicPolicy>,
    ) -> Result<Vec<StudentTermReportCard>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::GradeRead)
            .await?;

        let policy = self.resolve_policy(org_id, policy).await;

        let enrollments = if let Some(er) = &self.enrollment_repo {
            er.find_by_section(section_id).await?
        } else {
            vec![]
        };

        let mut cards = Vec::with_capacity(enrollments.len());
        for en in &enrollments {
            let card = self
                .calculate_student_term_report_card(
                    ctx,
                    org_id,
                    en.student_id,
                    period_id,
                    Some(section_id),
                    Some(policy.clone()),
                )
                .await?;
            cards.push(card);
        }

        let total_students = cards.len();
        if policy.ranking_enabled && !cards.is_empty() {
            let scores: Vec<(Uuid, Decimal)> = cards
                .iter()
                .map(|c| (c.student_id, c.overall_percentage))
                .collect();
            let rankings =
                AcademicCalculationEngine::compute_rankings_legacy(&scores, policy.tie_policy)?;
            let rank_map: std::collections::HashMap<Uuid, u32> = rankings.into_iter().collect();

            for c in &mut cards {
                if let Some(r) = rank_map.get(&c.student_id) {
                    if let Some(max_r) = policy.max_displayed_rank {
                        if *r <= max_r {
                            c.rank_in_class = Some(*r);
                        }
                    } else {
                        c.rank_in_class = Some(*r);
                    }
                }
                c.total_students_in_class = total_students;
            }
        }

        Ok(cards)
    }

    /// Calculate class annual results with rankings
    pub async fn calculate_class_annual_report_cards(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        section_id: Uuid,
        calendar_id: Uuid,
        policy: Option<AcademicPolicy>,
    ) -> Result<Vec<StudentAnnualReportCard>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::GradeRead)
            .await?;

        let policy = self.resolve_policy(org_id, policy).await;

        let enrollments = if let Some(er) = &self.enrollment_repo {
            er.find_by_section(section_id).await?
        } else {
            vec![]
        };

        let mut cards = Vec::with_capacity(enrollments.len());
        for en in &enrollments {
            let card = self
                .calculate_student_annual_report_card(
                    ctx,
                    org_id,
                    en.student_id,
                    calendar_id,
                    Some(section_id),
                    Some(policy.clone()),
                )
                .await?;
            cards.push(card);
        }

        let total_students = cards.len();
        if policy.ranking_enabled && !cards.is_empty() {
            let scores: Vec<(Uuid, Decimal)> = cards
                .iter()
                .map(|c| (c.student_id, c.overall_percentage))
                .collect();
            let rankings =
                AcademicCalculationEngine::compute_rankings_legacy(&scores, policy.tie_policy)?;
            let rank_map: std::collections::HashMap<Uuid, u32> = rankings.into_iter().collect();

            for c in &mut cards {
                if let Some(r) = rank_map.get(&c.student_id) {
                    if let Some(max_r) = policy.max_displayed_rank {
                        if *r <= max_r {
                            c.rank_in_class = Some(*r);
                        }
                    } else {
                        c.rank_in_class = Some(*r);
                    }
                }
                c.total_students_in_class = total_students;
            }
        }

        Ok(cards)
    }

    /// Calculate certificate examination report card (Grade 9 basic / Grade 12 Thanawiya).
    ///
    /// For each subject, the certificate result combines:
    /// - Ministerial exam score (from grades marked as `is_final = true`)
    /// - Cumulative prior-year contribution (from grades with component code "cumulative_prior_years")
    ///
    /// The `certificate_type` parameter should be "basic_grade9" or "secondary_thanawiya".
    pub async fn calculate_student_certificate_report_card(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        student_id: Uuid,
        calendar_id: Uuid,
        section_id: Option<Uuid>,
        certificate_type: String,
        policy: Option<AcademicPolicy>,
    ) -> Result<StudentCertificateReportCard, DomainError> {
        self.permission_checker
            .check(ctx, Permission::GradeRead)
            .await?;

        let policy = self.resolve_policy(org_id, policy).await;

        // 1. Resolve student
        let (student_name, student_number) = if let Some(sr) = &self.student_repo {
            if let Some(detail) = sr.find_detail_by_id(student_id).await? {
                (
                    detail.person.full_name(),
                    Some(detail.student.student_number),
                )
            } else if let Some(st) = sr.find_by_id(student_id).await? {
                (
                    format!("Student {}", &st.id.to_string()[..8]),
                    Some(st.student_number),
                )
            } else {
                ("Student".to_string(), None)
            }
        } else {
            ("Student".to_string(), None)
        };

        // 2. Resolve section
        let resolved_section_id = if let Some(cid) = section_id {
            cid
        } else if let Some(er) = &self.enrollment_repo {
            let enrollments = er.find_by_student(student_id).await?;
            enrollments
                .into_iter()
                .find(|e| e.status == EnrollmentStatus::Active)
                .and_then(|e| e.section_id)
                .unwrap_or_else(Uuid::nil)
        } else {
            Uuid::nil()
        };

        let class_name = if let Some(cr) = &self.section_repo {
            if let Some(cls) = cr.find_by_id(resolved_section_id).await? {
                cls.name
            } else {
                "Class".to_string()
            }
        } else {
            "Class".to_string()
        };

        // 3. Resolve calendar name
        let calendar_name = if let Some(yr) = &self.year_repo {
            if let Some(cal) = yr.find_by_id(calendar_id).await? {
                cal.name
            } else {
                "Academic Year".to_string()
            }
        } else {
            "Academic Year".to_string()
        };

        // 4. Get section subjects
        let section_subjects = if resolved_section_id != Uuid::nil() {
            self.section_subject_repo
                .find_by_section(resolved_section_id)
                .await?
        } else {
            vec![]
        };

        // 5. Calculate certificate result per subject
        let mut subject_results = Vec::with_capacity(section_subjects.len());

        for cs in &section_subjects {
            let subject = self.subject_repo.find_by_id(cs.subject_id).await?;
            let (subject_code, subject_name, subject_name_ar, credit_units) = match &subject {
                Some(s) => (
                    s.code.clone(),
                    s.name.clone(),
                    s.name_ar.clone(),
                    s.credit_units.unwrap_or(Decimal::ONE),
                ),
                None => (
                    "SUBJ".to_string(),
                    "Subject".to_string(),
                    None,
                    Decimal::ONE,
                ),
            };

            // Fetch all grades for this student + subject
            let all_grades = self
                .grade_repo
                .find_by_student_and_subject(student_id, cs.id)
                .await?;

            // Exam score = grade marked as is_final
            let exam_score = all_grades.iter().find(|g| g.is_final).map(|g| g.score);

            // Cumulative score = grade with component matching cumulative pattern
            let cumulative_score = all_grades
                .iter()
                .find(|g| {
                    (g.component_id.is_none() && g.component_code.is_none() && !g.is_final)
                        || g.component_code.as_deref() == Some("cumulative_prior_years")
                        || g.notes
                            .as_deref()
                            .unwrap_or("")
                            .to_lowercase()
                            .contains("cumulative")
                })
                .filter(|g| !g.is_final)
                .map(|g| g.score);

            let res = AcademicCalculationEngine::calculate_certificate_result(
                cs.subject_id,
                subject_code,
                subject_name,
                subject_name_ar,
                credit_units,
                exam_score,
                cumulative_score,
                &policy,
            )?;
            subject_results.push(res);
        }

        // 6. Aggregate
        let total_score: Decimal = subject_results.iter().map(|s| s.term_total).sum();
        let max_possible_score: Decimal = subject_results.iter().map(|s| s.term_max).sum();
        let raw_pct = if max_possible_score > Decimal::ZERO {
            (total_score / max_possible_score) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };
        let overall_percentage =
            apply_rounding(raw_pct, policy.rounding_mode, policy.decimal_places);

        let failed_count = subject_results
            .iter()
            .filter(|s| s.status == AcademicStatus::Failed)
            .count();
        let any_incomplete = subject_results
            .iter()
            .any(|s| s.status == AcademicStatus::Incomplete);

        let status = if any_incomplete {
            AcademicStatus::Incomplete
        } else if failed_count == 0 {
            if overall_percentage >= policy.honors_threshold_percentage {
                AcademicStatus::PassedWithHonors
            } else if overall_percentage >= policy.passing_threshold_percentage {
                AcademicStatus::Passed
            } else {
                AcademicStatus::Failed
            }
        } else if failed_count <= policy.max_failed_subjects_for_conditional_pass {
            AcademicStatus::ConditionalPass
        } else {
            AcademicStatus::Failed
        };

        Ok(StudentCertificateReportCard {
            student_id,
            student_name,
            student_number,
            section_id: resolved_section_id,
            class_name,
            calendar_id,
            academic_calendar_name: calendar_name,
            certificate_type,
            subjects: subject_results,
            total_score,
            max_possible_score,
            overall_percentage,
            overall_letter_grade: {
                let eval = policy.evaluate_percentage(overall_percentage);
                Some(eval.letter_grade)
            },
            overall_letter_grade_ar: {
                let eval = policy.evaluate_percentage(overall_percentage);
                Some(eval.letter_grade_ar)
            },
            rank_in_class: None,
            rank_in_grade: None,
            total_students_in_class: 0,
            total_students_in_grade: 0,
            status,
            calculated_at: Utc::now(),
        })
    }
}

#[cfg(test)]
mod assessment_service_tests {
    use super::*;
    use chrono::Utc;

    struct AllowPermissions;

    #[async_trait::async_trait]
    impl PermissionCheckerPort for AllowPermissions {
        async fn check(
            &self,
            _ctx: &RequestContext,
            _permission: Permission,
        ) -> Result<(), DomainError> {
            Ok(())
        }
    }

    struct DenyPermissions;

    #[async_trait::async_trait]
    impl PermissionCheckerPort for DenyPermissions {
        async fn check(
            &self,
            _ctx: &RequestContext,
            _permission: Permission,
        ) -> Result<(), DomainError> {
            Err(DomainError::Unauthorized("Permission denied".into()))
        }
    }

    fn request_context(tenant_id: Uuid) -> RequestContext {
        RequestContext {
            tenant_id,
            actor: orbis_domain::iam::actor::Actor::User(Uuid::new_v4()),
            request_id: "test".into(),
            source: crate::context::ActionSource::Api,
            org_unit_path: None,
        }
    }

    #[tokio::test]
    async fn grade_access_rejects_callers_without_permission() {
        let ctx = request_context(Uuid::new_v4());
        let result =
            authorize_grade_access(&DenyPermissions, &ctx, ctx.tenant_id, Permission::GradeRead)
                .await;

        assert!(matches!(result, Err(DomainError::Unauthorized(_))));
    }

    #[tokio::test]
    async fn grade_access_rejects_cross_organization_requests() {
        let ctx = request_context(Uuid::new_v4());
        let result = authorize_grade_access(
            &AllowPermissions,
            &ctx,
            Uuid::new_v4(),
            Permission::GradeAnalytics,
        )
        .await;

        assert!(matches!(result, Err(DomainError::Unauthorized(_))));
    }

    fn section_subject(
        instructor_id: Uuid,
    ) -> orbis_domain::entities::section_subject::SectionSubject {
        orbis_domain::entities::section_subject::SectionSubject {
            id: Uuid::new_v4(),
            org_id: Uuid::new_v4(),
            section_id: Uuid::new_v4(),
            subject_id: Uuid::new_v4(),
            period_id: Uuid::new_v4(),
            instructor_id: Some(instructor_id),
            assistant_instructor_id: None,
            substitute_instructor_id: None,
            scheme_id: None,
            weekly_hours: None,
            theory_hours: None,
            practical_hours: None,
            assessment_config: None,
            grade_groups: None,
            capacity: None,
            status: "active".into(),
            is_active: true,
            created_at: None,
            updated_at: None,
        }
    }

    fn lesson(
        teacher_id: Option<Uuid>,
    ) -> orbis_domain::entities::scheduled_lesson::ScheduledLesson {
        orbis_domain::entities::scheduled_lesson::ScheduledLesson {
            id: Uuid::new_v4(),
            section_subject_id: Uuid::new_v4(),
            room_id: None,
            time_slot_id: Uuid::new_v4(),
            teacher_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn scheduled_lesson_teacher_overrides_section_assignment() {
        let assigned_teacher = Uuid::new_v4();
        let substitute_teacher = Uuid::new_v4();
        let section_subject = section_subject(assigned_teacher);
        let lesson = lesson(Some(substitute_teacher));

        assert!(!teacher_is_assigned_to_lesson(
            assigned_teacher,
            &lesson,
            &section_subject
        ));
        assert!(teacher_is_assigned_to_lesson(
            substitute_teacher,
            &lesson,
            &section_subject
        ));
    }

    #[test]
    fn section_subject_assignment_applies_without_lesson_override() {
        let assigned_teacher = Uuid::new_v4();
        let section_subject = section_subject(assigned_teacher);
        let lesson = lesson(None);

        assert!(teacher_is_assigned_to_lesson(
            assigned_teacher,
            &lesson,
            &section_subject
        ));
    }

    #[test]
    fn normalize_grade_component_key_prefers_trimmed_lowercased_code() {
        let key = super::normalize_grade_component_key(Some("  Written_Test  "), None);
        assert_eq!(key.as_deref(), Some("written_test"));
    }

    #[test]
    fn normalize_grade_component_key_falls_back_to_legacy_component_id() {
        let id = Uuid::nil();
        let key = super::normalize_grade_component_key(None, Some(id));
        assert_eq!(key, Some(id.to_string()));
    }

    #[test]
    fn resolve_grade_max_score_uses_configured_scheme_maximum() {
        let configured = Decimal::from(20);
        let resolved = super::resolve_grade_max_score(Some(configured), None).unwrap();
        assert_eq!(resolved, configured);
    }

    #[test]
    fn resolve_grade_max_score_rejects_mismatched_requested_maximum() {
        let err = super::resolve_grade_max_score(Some(Decimal::from(20)), Some(Decimal::from(30)))
            .unwrap_err();
        assert!(matches!(err, DomainError::Validation(_)));
    }

    #[test]
    fn resolve_grade_max_score_defaults_to_hundred_without_scheme_component() {
        let resolved = super::resolve_grade_max_score(None, None).unwrap();
        assert_eq!(resolved, Decimal::from(100));
    }
}
