use chrono::Utc;
use orbis_domain::entities::academic_calendar::{
    AcademicCalendar, AcademicPeriod, AcademicCalendarStatus, PeriodType,
};
use orbis_domain::entities::subject::Course;
use orbis_domain::entities::enrollment::{Enrollment, Enrollmentstatus};
use orbis_domain::entities::section_subject::{SectionSubject, SectionSubjectstatus};
use orbis_domain::errors::DomainError;
use orbis_domain::iam::permission::Permission;
use std::sync::Arc;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{
    AcademicFrameworkRepository, OrgUnitRepository, StudentRepository,
};

pub struct AcademicFrameworkService {
    pub framework_repo: Arc<dyn AcademicFrameworkRepository>,
    pub student_repo: Arc<dyn StudentRepository>,
    pub org_unit_repo: Arc<dyn OrgUnitRepository>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl AcademicFrameworkService {
    pub fn new(
        framework_repo: Arc<dyn AcademicFrameworkRepository>,
        student_repo: Arc<dyn StudentRepository>,
        org_unit_repo: Arc<dyn OrgUnitRepository>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            framework_repo,
            student_repo,
            org_unit_repo,
            permission_checker,
        }
    }

    pub async fn list_calendars(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
    ) -> Result<Vec<AcademicCalendar>, DomainError> {
        self.require_org_permission(ctx, org_id, Permission::PeriodRead)
            .await?;
        self.framework_repo.list_calendars(org_id).await
    }

    pub async fn list_period_types(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
    ) -> Result<Vec<PeriodType>, DomainError> {
        self.require_org_permission(ctx, org_id, Permission::PeriodRead)
            .await?;
        self.framework_repo.list_period_types(org_id).await
    }

    pub async fn list_calendar_periods(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        calendar_id: Uuid,
    ) -> Result<Vec<AcademicPeriod>, DomainError> {
        self.require_org_permission(ctx, org_id, Permission::PeriodRead)
            .await?;
        if self
            .framework_repo
            .find_calendar(org_id, calendar_id)
            .await?
            .is_none()
        {
            return Err(DomainError::NotFound("Academic calendar not found".into()));
        }
        self.framework_repo
            .list_calendar_periods(org_id, calendar_id)
            .await
    }

    pub async fn list_courses(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
    ) -> Result<Vec<Course>, DomainError> {
        self.require_org_permission(ctx, org_id, Permission::SubjectRead)
            .await?;
        self.framework_repo.list_courses(org_id).await
    }

    pub async fn list_section_subjects(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        course_id: Option<Uuid>,
    ) -> Result<Vec<SectionSubject>, DomainError> {
        self.require_org_permission(ctx, org_id, Permission::SubjectRead)
            .await?;
        if let Some(course_id) = course_id
            && self.framework_repo.find_course(org_id, course_id).await?.is_none()
        {
            return Err(DomainError::NotFound("Course not found".into()));
        }
        self.framework_repo
            .list_section_subjects(org_id, course_id)
            .await
    }

    pub async fn list_enrollments(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        offering_id: Uuid,
    ) -> Result<Vec<Enrollment>, DomainError> {
        self.require_org_permission(ctx, org_id, Permission::EnrollmentRead)
            .await?;
        if self
            .framework_repo
            .find_section_subject(org_id, offering_id)
            .await?
            .is_none()
        {
            return Err(DomainError::NotFound("Course offering not found".into()));
        }
        self.framework_repo
            .list_enrollments(org_id, offering_id)
            .await
    }

    pub async fn create_calendar(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        code: Option<String>,
        name: String,
        name_ar: Option<String>,
        starts_at: Option<chrono::DateTime<Utc>>,
        ends_at: Option<chrono::DateTime<Utc>>,
    ) -> Result<AcademicCalendar, DomainError> {
        self.require_org_permission(ctx, org_id, Permission::PeriodWrite)
            .await?;
        validate_name(&name, "Calendar")?;
        validate_range(starts_at, ends_at, "Calendar")?;

        let normalized_code = normalize_optional_code(code, "Calendar")?;
        if let Some(code) = normalized_code.as_deref()
            && self
                .framework_repo
                .find_calendar_by_code(org_id, code)
                .await?
                .is_some()
        {
            return Err(DomainError::Conflict(format!(
                "Academic calendar code '{}' already exists",
                code
            )));
        }

        let now = Utc::now();
        let calendar = AcademicCalendar {
            id: Uuid::new_v4(),
            org_id,
            code: normalized_code,
            name: name.trim().to_string(),
            name_ar: normalize_optional_name(name_ar),
            starts_at,
            ends_at,
            status: AcademicCalendarStatus::Planning,
            created_at: now,
            updated_at: now,
        };
        self.framework_repo.save_calendar(&calendar).await?;
        Ok(calendar)
    }

    pub async fn create_period_type(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        code: String,
        name: String,
        name_ar: Option<String>,
        sort_order: i32,
    ) -> Result<PeriodType, DomainError> {
        self.require_org_permission(ctx, org_id, Permission::PeriodWrite)
            .await?;
        validate_name(&name, "Period type")?;
        let normalized_code = normalize_code(code, "Period type")?;
        if self
            .framework_repo
            .find_period_type_by_code(org_id, &normalized_code)
            .await?
            .is_some()
        {
            return Err(DomainError::Conflict(format!(
                "Academic period type code '{}' already exists",
                normalized_code
            )));
        }

        let now = Utc::now();
        let period_type = PeriodType {
            id: Uuid::new_v4(),
            org_id,
            code: normalized_code,
            name: name.trim().to_string(),
            name_ar: normalize_optional_name(name_ar),
            sort_order,
            created_at: now,
            updated_at: now,
        };
        self.framework_repo.save_period_type(&period_type).await?;
        Ok(period_type)
    }

    pub async fn create_calendar_period(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        calendar_id: Uuid,
        parent_id: Option<Uuid>,
        period_type_id: Uuid,
        code: Option<String>,
        name: String,
        name_ar: Option<String>,
        starts_at: Option<chrono::DateTime<Utc>>,
        ends_at: Option<chrono::DateTime<Utc>>,
        sort_order: i32,
    ) -> Result<AcademicPeriod, DomainError> {
        self.require_org_permission(ctx, org_id, Permission::PeriodWrite)
            .await?;
        validate_name(&name, "Academic period")?;
        validate_range(starts_at, ends_at, "Academic period")?;

        let calendar = self
            .framework_repo
            .find_calendar(org_id, calendar_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Academic calendar not found".into()))?;
        ensure_range_within(
            starts_at,
            ends_at,
            calendar.starts_at,
            calendar.ends_at,
            "Academic period",
        )?;
        if self
            .framework_repo
            .find_period_type(org_id, period_type_id)
            .await?
            .is_none()
        {
            return Err(DomainError::NotFound(
                "Academic period type not found".into(),
            ));
        }
        if let Some(parent_id) = parent_id {
            let parent = self
                .framework_repo
                .find_calendar_period(org_id, parent_id)
                .await?
                .ok_or_else(|| DomainError::NotFound("Parent period not found".into()))?;
            ensure_parent_calendar(parent.calendar_id, calendar_id)?;
            ensure_range_within(
                starts_at,
                ends_at,
                parent.starts_at,
                parent.ends_at,
                "Academic period",
            )?;
        }

        let now = Utc::now();
        let period = AcademicPeriod {
            id: Uuid::new_v4(),
            org_id,
            calendar_id,
            parent_id,
            period_type_id,
            code: normalize_optional_code(code, "Academic period")?,
            name: name.trim().to_string(),
            name_ar: normalize_optional_name(name_ar),
            starts_at,
            ends_at,
            sort_order,
            created_at: now,
            updated_at: now,
        };
        self.framework_repo.save_calendar_period(&period).await?;
        Ok(period)
    }

    pub async fn create_course(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        unit_id: Option<Uuid>,
        code: String,
        name: String,
        name_ar: Option<String>,
        description: Option<String>,
        credit_units: Option<rust_decimal::Decimal>,
        workload_units: Option<rust_decimal::Decimal>,
    ) -> Result<Course, DomainError> {
        self.require_org_permission(ctx, org_id, Permission::SubjectWrite)
            .await?;
        validate_name(&name, "Course")?;
        if let Some(unit_id) = unit_id {
            let unit = self
                .org_unit_repo
                .find_by_id(unit_id)
                .await?
                .ok_or_else(|| DomainError::NotFound("Organizational unit not found".into()))?;
            if unit.org_id != org_id {
                return Err(DomainError::Validation(
                    "Course organizational unit must belong to the same organization".into(),
                ));
            }
        }
        let normalized_code = normalize_code(code, "Course")?;
        if self
            .framework_repo
            .find_course_by_code(org_id, &normalized_code)
            .await?
            .is_some()
        {
            return Err(DomainError::Conflict(format!(
                "Course code '{}' already exists",
                normalized_code
            )));
        }
        for (label, units) in [("Credit", credit_units), ("Workload", workload_units)] {
            if units.is_some_and(|value| value <= rust_decimal::Decimal::ZERO) {
                return Err(DomainError::Validation(format!(
                    "{label} units must be greater than zero when specified"
                )));
            }
        }

        let now = Utc::now();
        let course = Course {
            id: Uuid::new_v4(),
            org_id,
            unit_id,
            code: normalized_code,
            name: name.trim().to_string(),
            name_ar: normalize_optional_name(name_ar),
            description: normalize_optional_name(description),
            subject_type: "course".to_string(),
            is_core: false,
            is_elective: false,
            credit_units,
            workload_units,
            theory_hours: None,
            practical_hours: None,
            is_lab_required: false,
            max_students_per_section: None,
            color: None,
            icon: None,
            sort_order: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        };
        self.framework_repo.save_course(&course).await?;
        Ok(course)
    }

    pub async fn create_section_subject(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        course_id: Uuid,
        period_id: Option<Uuid>,
        section_code: Option<String>,
        name: Option<String>,
        capacity: Option<u32>,
        starts_at: Option<chrono::DateTime<Utc>>,
        ends_at: Option<chrono::DateTime<Utc>>,
    ) -> Result<SectionSubject, DomainError> {
        self.require_org_permission(ctx, org_id, Permission::SubjectWrite)
            .await?;
        if self
            .framework_repo
            .find_course(org_id, course_id)
            .await?
            .filter(|course| course.is_active)
            .is_none()
        {
            return Err(DomainError::NotFound(
                "Active course not found in this organization".into(),
            ));
        }
        validate_range(starts_at, ends_at, "Course offering")?;
        if capacity == Some(0) {
            return Err(DomainError::Validation(
                "Offering capacity must be greater than zero".into(),
            ));
        }
        if let Some(period_id) = period_id {
            let period = self
                .framework_repo
                .find_calendar_period(org_id, period_id)
                .await?
                .ok_or_else(|| DomainError::NotFound("Academic period not found".into()))?;
            ensure_range_within(
                starts_at,
                ends_at,
                period.starts_at,
                period.ends_at,
                "Course offering",
            )?;
        }

        let now = Utc::now();
        let offering = SectionSubject {
            id: Uuid::new_v4(),
            org_id,
            section_id: Uuid::new_v4(),
            subject_id: course_id,
            period_id: period_id.unwrap_or_else(Uuid::nil),
            instructor_id: None,
            assistant_instructor_id: None,
            scheme_id: None,
            weekly_hours: None,
            status: "draft".to_string(),
            is_active: true,
        };
        self.framework_repo.save_section_subject(&offering).await?;
        Ok(offering)
    }

    pub async fn enroll_student(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        offering_id: Uuid,
        student_id: Uuid,
        attempt_number: u32,
    ) -> Result<Enrollment, DomainError> {
        self.require_org_permission(ctx, org_id, Permission::EnrollmentWrite)
            .await?;
        if attempt_number == 0 || i32::try_from(attempt_number).is_err() {
            return Err(DomainError::Validation(
                "Attempt number must be within the supported positive integer range".into(),
            ));
        }
        let student = self
            .student_repo
            .find_by_id(student_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Student not found".into()))?;
        if student.org_id != org_id {
            return Err(DomainError::Validation(
                "Student must belong to the same organization as the course offering".into(),
            ));
        }
        let offering = self
            .framework_repo
            .find_section_subject(org_id, offering_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Course offering not found".into()))?;
        if offering.status != "open" {
            return Err(DomainError::BusinessRule(
                "Course offering is not open for enrollment".into(),
            ));
        }
        if self
            .framework_repo
            .find_enrollment_attempt(org_id, offering_id, student_id, attempt_number)
            .await?
            .is_some()
        {
            return Err(DomainError::Conflict(
                "This learner already has a record for the specified course attempt".into(),
            ));
        }

        let now = Utc::now();
        let enrollment = Enrollment {
            id: Uuid::new_v4(),
            org_id,
            period_id: Uuid::nil(),
            student_id,
            section_id: Some(offering_id),
            attempt_number: attempt_number as i32,
            student_number: String::new(),
            study_mode: orbis_domain::entities::enrollment::StudyMode::Regular,
            status: orbis_domain::entities::enrollment::EnrollmentStatus::Active,
            is_attendance_exempt: false,
            is_grade_exempt: false,
            enrolled_at: now,
            withdrawn_at: None,
            withdrawal_reason: None,
            meta: serde_json::json!({}),
            created_by: ctx.actor.id(),
            updated_by: None,
            created_at: now,
            updated_at: now,
        };
        self.framework_repo
            .save_enrollment(&enrollment)
            .await?;
        Ok(enrollment)
    }

    async fn require_org_permission(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        permission: Permission,
    ) -> Result<(), DomainError> {
        if ctx.tenant_id != org_id {
            return Err(DomainError::Forbidden(
                "The requested academic data belongs to another organization".into(),
            ));
        }
        self.permission_checker.check(ctx, permission).await
    }
}

fn validate_name(name: &str, label: &str) -> Result<(), DomainError> {
    if name.trim().is_empty() {
        return Err(DomainError::Validation(format!("{label} name is required")));
    }
    Ok(())
}

fn normalize_code(code: String, label: &str) -> Result<String, DomainError> {
    let normalized = code.trim().to_uppercase();
    if normalized.is_empty() {
        return Err(DomainError::Validation(format!("{label} code is required")));
    }
    Ok(normalized)
}

fn normalize_optional_code(
    code: Option<String>,
    label: &str,
) -> Result<Option<String>, DomainError> {
    code.map(|value| normalize_code(value, label)).transpose()
}

fn normalize_optional_name(name: Option<String>) -> Option<String> {
    name.map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn validate_range(
    starts_at: Option<chrono::DateTime<Utc>>,
    ends_at: Option<chrono::DateTime<Utc>>,
    label: &str,
) -> Result<(), DomainError> {
    if starts_at
        .zip(ends_at)
        .is_some_and(|(start, end)| end < start)
    {
        return Err(DomainError::Validation(format!(
            "{label} end must not precede its start"
        )));
    }
    Ok(())
}

fn ensure_range_within(
    child_start: Option<chrono::DateTime<Utc>>,
    child_end: Option<chrono::DateTime<Utc>>,
    parent_start: Option<chrono::DateTime<Utc>>,
    parent_end: Option<chrono::DateTime<Utc>>,
    label: &str,
) -> Result<(), DomainError> {
    if child_start
        .zip(parent_start)
        .is_some_and(|(child, parent)| child < parent)
        || child_end
            .zip(parent_end)
            .is_some_and(|(child, parent)| child > parent)
    {
        return Err(DomainError::Validation(format!(
            "{label} dates must be within their parent dates"
        )));
    }
    Ok(())
}

fn ensure_parent_calendar(parent_calendar_id: Uuid, calendar_id: Uuid) -> Result<(), DomainError> {
    if parent_calendar_id != calendar_id {
        return Err(DomainError::Validation(
            "Parent period must belong to the same academic calendar".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        AcademicFrameworkService, ensure_parent_calendar, ensure_range_within, normalize_code,
        validate_range,
    };
    use crate::{
        context::{ActionSource, RequestContext},
        ports::{
            iam::PermissionCheckerPort,
            repositories::{AcademicFrameworkRepository, OrgUnitRepository, StudentRepository},
        },
    };
    use async_trait::async_trait;
    use chrono::{TimeZone, Utc};
    use orbis_domain::{
        entities::{
            academic_calendar::{AcademicCalendar, AcademicPeriod, PeriodType},
            course::Course,
            enrollment::{Enrollment, Enrollmentstatus},
            section_subject::{SectionSubject, SectionSubjectstatus},
            org_unit::OrgUnit,
            student::{AcademicStanding, Student, StudentDetail, StudentStatus},
        },
        errors::DomainError,
        iam::{actor::Actor, permission::Permission},
        queries::filters::{PaginatedList, StudentFilter},
    };
    use std::sync::{Arc, Mutex};
    use uuid::Uuid;

    fn at(day: u32) -> chrono::DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 9, day, 0, 0, 0).unwrap()
    }

    #[test]
    fn calendar_period_ranges_are_optional_but_validated_when_present() {
        assert!(validate_range(None, None, "Calendar").is_ok());
        assert!(validate_range(Some(at(1)), Some(at(2)), "Calendar").is_ok());
        assert!(validate_range(Some(at(3)), Some(at(2)), "Calendar").is_err());
    }

    #[test]
    fn child_range_must_fit_parent_range() {
        assert!(
            ensure_range_within(Some(at(2)), Some(at(8)), Some(at(1)), Some(at(9)), "Period")
                .is_ok()
        );
        assert!(
            ensure_range_within(Some(at(1)), Some(at(8)), Some(at(2)), Some(at(9)), "Period")
                .is_err()
        );
    }

    #[test]
    fn parent_period_must_belong_to_the_same_calendar() {
        let calendar_id = Uuid::new_v4();
        assert!(ensure_parent_calendar(calendar_id, calendar_id).is_ok());
        assert!(ensure_parent_calendar(Uuid::new_v4(), calendar_id).is_err());
    }

    #[test]
    fn codes_are_normalized_and_empty_codes_rejected() {
        assert_eq!(
            normalize_code("  module-a ".into(), "Period").unwrap(),
            "MODULE-A"
        );
        assert!(normalize_code("  ".into(), "Period").is_err());
    }

    struct TestFrameworkRepo {
        org_id: Uuid,
        course_id: Uuid,
        offering: Option<SectionSubject>,
        duplicate_attempt: bool,
        saved_offering: Mutex<Option<SectionSubject>>,
    }

    #[async_trait]
    impl AcademicFrameworkRepository for TestFrameworkRepo {
        async fn list_calendars(
            &self,
            _: Uuid,
        ) -> Result<Vec<AcademicCalendar>, DomainError> {
            Ok(vec![])
        }
        async fn find_calendar(
            &self,
            _: Uuid,
            _: Uuid,
        ) -> Result<Option<AcademicCalendar>, DomainError> {
            Ok(None)
        }
        async fn find_calendar_by_code(
            &self,
            _: Uuid,
            _: &str,
        ) -> Result<Option<AcademicCalendar>, DomainError> {
            Ok(None)
        }
        async fn save_calendar(&self, _: &AcademicCalendar) -> Result<(), DomainError> {
            Ok(())
        }
        async fn list_period_types(
            &self,
            _: Uuid,
        ) -> Result<Vec<PeriodType>, DomainError> {
            Ok(vec![])
        }
        async fn find_period_type(
            &self,
            _: Uuid,
            _: Uuid,
        ) -> Result<Option<PeriodType>, DomainError> {
            Ok(None)
        }
        async fn find_period_type_by_code(
            &self,
            _: Uuid,
            _: &str,
        ) -> Result<Option<PeriodType>, DomainError> {
            Ok(None)
        }
        async fn save_period_type(&self, _: &PeriodType) -> Result<(), DomainError> {
            Ok(())
        }
        async fn list_calendar_periods(
            &self,
            _: Uuid,
            _: Uuid,
        ) -> Result<Vec<AcademicPeriod>, DomainError> {
            Ok(vec![])
        }
        async fn find_calendar_period(
            &self,
            _: Uuid,
            _: Uuid,
        ) -> Result<Option<AcademicPeriod>, DomainError> {
            Ok(None)
        }
        async fn save_calendar_period(
            &self,
            _: &AcademicPeriod,
        ) -> Result<(), DomainError> {
            Ok(())
        }
        async fn find_course(&self, org_id: Uuid, id: Uuid) -> Result<Option<Course>, DomainError> {
            Ok(
                (org_id == self.org_id && id == self.course_id).then(|| Course {
                    id,
                    org_id,
                    unit_id: None,
                    code: "COURSE-1".into(),
                    name: "Course".into(),
                    name_ar: None,
                    description: None,
                    credit_units: None,
                    workload_units: None,
                    is_active: true,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                }),
            )
        }
        async fn list_courses(&self, _: Uuid) -> Result<Vec<Course>, DomainError> {
            Ok(vec![])
        }
        async fn find_course_by_code(
            &self,
            _: Uuid,
            _: &str,
        ) -> Result<Option<Course>, DomainError> {
            Ok(None)
        }
        async fn save_course(&self, _: &Course) -> Result<(), DomainError> {
            Ok(())
        }
        async fn find_section_subject(
            &self,
            org_id: Uuid,
            id: Uuid,
        ) -> Result<Option<SectionSubject>, DomainError> {
            Ok(self
                .offering
                .clone()
                .filter(|item| item.org_id == org_id && item.id == id))
        }
        async fn list_section_subjects(
            &self,
            _: Uuid,
            _: Option<Uuid>,
        ) -> Result<Vec<SectionSubject>, DomainError> {
            Ok(vec![])
        }
        async fn save_section_subject(&self, offering: &SectionSubject) -> Result<(), DomainError> {
            *self.saved_offering.lock().unwrap() = Some(offering.clone());
            Ok(())
        }
        async fn find_enrollment_attempt(
            &self,
            org_id: Uuid,
            _: Uuid,
            _: Uuid,
            _: u32,
        ) -> Result<Option<Enrollment>, DomainError> {
            Ok(self.duplicate_attempt.then(|| Enrollment {
                id: Uuid::new_v4(),
                org_id,
                offering_id: Uuid::new_v4(),
                student_id: Uuid::new_v4(),
                attempt_number: 1,
                status: Enrollmentstatus::Active,
                enrolled_at: Utc::now(),
                withdrawn_at: None,
                withdrawal_reason: None,
                created_by: None,
                updated_by: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }))
        }
        async fn list_enrollments(
            &self,
            _: Uuid,
            _: Uuid,
        ) -> Result<Vec<Enrollment>, DomainError> {
            Ok(vec![])
        }
        async fn save_enrollment(&self, _: &Enrollment) -> Result<(), DomainError> {
            Ok(())
        }
    }

    struct TestStudentRepo(Uuid);

    #[async_trait]
    impl StudentRepository for TestStudentRepo {
        async fn find_by_id(&self, id: Uuid) -> Result<Option<Student>, DomainError> {
            Ok(Some(Student {
                id,
                org_id: self.0,
                person_id: Uuid::new_v4(),
                user_id: None,
                student_number: "S-1".into(),
                status: StudentStatus::Active,
                standing: AcademicStanding::GoodStanding,
                admitted_at: Utc::now(),
                graduated_at: None,
                is_deleted: false,
                deleted_at: None,
                meta: serde_json::Value::Null,
                org_unit_id: None,
                created_by: None,
                updated_by: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }))
        }
        async fn find_detail_by_id(&self, _: Uuid) -> Result<Option<StudentDetail>, DomainError> {
            Ok(None)
        }
        async fn find_by_org(&self, _: Uuid) -> Result<Vec<Student>, DomainError> {
            Ok(vec![])
        }
        async fn list(
            &self,
            _: Uuid,
            _: StudentFilter,
        ) -> Result<PaginatedList<StudentDetail>, DomainError> {
            Ok(PaginatedList::empty(20))
        }
        async fn find_by_student_number(
            &self,
            _: Uuid,
            _: &str,
        ) -> Result<Option<Student>, DomainError> {
            Ok(None)
        }
        async fn find_by_national_id(
            &self,
            _: Uuid,
            _: &str,
        ) -> Result<Option<Student>, DomainError> {
            Ok(None)
        }
        async fn save(&self, _: &Student) -> Result<(), DomainError> {
            Ok(())
        }
        async fn update_status(&self, _: Uuid, _: &str) -> Result<(), DomainError> {
            Ok(())
        }
        async fn update_status_batch(&self, _: &[Uuid], _: &str) -> Result<usize, DomainError> {
            Ok(0)
        }
        async fn delete(&self, _: Uuid) -> Result<(), DomainError> {
            Ok(())
        }
    }

    struct TestOrgUnitRepo;

    #[async_trait]
    impl OrgUnitRepository for TestOrgUnitRepo {
        async fn find_by_id(&self, _: Uuid) -> Result<Option<OrgUnit>, DomainError> {
            Ok(None)
        }
        async fn find_by_org(&self, _: Uuid) -> Result<Vec<OrgUnit>, DomainError> {
            Ok(vec![])
        }
        async fn find_children(&self, _: Uuid) -> Result<Vec<OrgUnit>, DomainError> {
            Ok(vec![])
        }
        async fn find_descendants(&self, _: Uuid, _: &str) -> Result<Vec<OrgUnit>, DomainError> {
            Ok(vec![])
        }
        async fn count_branches(&self, _: Uuid) -> Result<u32, DomainError> {
            Ok(0)
        }
        async fn save(&self, _: &OrgUnit) -> Result<(), DomainError> {
            Ok(())
        }
        async fn delete(&self, _: Uuid) -> Result<(), DomainError> {
            Ok(())
        }
    }

    struct AllowPermissions;

    #[async_trait]
    impl PermissionCheckerPort for AllowPermissions {
        async fn check(&self, _: &RequestContext, _: Permission) -> Result<(), DomainError> {
            Ok(())
        }
    }

    fn context(org_id: Uuid) -> RequestContext {
        RequestContext {
            tenant_id: org_id,
            actor: Actor::System,
            request_id: "test".into(),
            source: ActionSource::System,
            org_unit_path: None,
        }
    }

    fn test_service(
        org_id: Uuid,
        course_id: Uuid,
        offering: Option<SectionSubject>,
        duplicate_attempt: bool,
        student_org_id: Uuid,
    ) -> (AcademicFrameworkService, Arc<TestFrameworkRepo>) {
        let repo = Arc::new(TestFrameworkRepo {
            org_id,
            course_id,
            offering,
            duplicate_attempt,
            saved_offering: Mutex::new(None),
        });
        let service = AcademicFrameworkService::new(
            repo.clone(),
            Arc::new(TestStudentRepo(student_org_id)),
            Arc::new(TestOrgUnitRepo),
            Arc::new(AllowPermissions),
        );
        (service, repo)
    }

    #[tokio::test]
    async fn offering_can_be_created_without_an_academic_period() {
        let org_id = Uuid::new_v4();
        let course_id = Uuid::new_v4();
        let (service, repo) = test_service(org_id, course_id, None, false, org_id);

        service
            .create_section_subject(
                &context(org_id),
                org_id,
                course_id,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .unwrap();

        assert_eq!(
            repo.saved_offering
                .lock()
                .unwrap()
                .as_ref()
                .unwrap()
                .period_id,
            None
        );
    }

    #[tokio::test]
    async fn enrollment_rejects_a_student_from_another_organization() {
        let org_id = Uuid::new_v4();
        let offering_id = Uuid::new_v4();
        let offering = SectionSubject {
            id: offering_id,
            org_id,
            course_id: Uuid::new_v4(),
            period_id: None,
            section_code: None,
            name: None,
            status: SectionSubjectstatus::Open,
            capacity: None,
            starts_at: None,
            ends_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let (service, _) = test_service(
            org_id,
            offering.course_id,
            Some(offering),
            false,
            Uuid::new_v4(),
        );

        let result = service
            .enroll_student(&context(org_id), org_id, offering_id, Uuid::new_v4(), 1)
            .await;

        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[tokio::test]
    async fn enrollment_rejects_a_duplicate_course_attempt() {
        let org_id = Uuid::new_v4();
        let offering_id = Uuid::new_v4();
        let course_id = Uuid::new_v4();
        let offering = SectionSubject {
            id: offering_id,
            org_id,
            course_id,
            period_id: None,
            section_code: None,
            name: None,
            status: SectionSubjectstatus::Open,
            capacity: None,
            starts_at: None,
            ends_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let (service, _) = test_service(org_id, course_id, Some(offering), true, org_id);

        let result = service
            .enroll_student(&context(org_id), org_id, offering_id, Uuid::new_v4(), 1)
            .await;

        assert!(matches!(result, Err(DomainError::Conflict(_))));
    }
}
