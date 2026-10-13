use async_trait::async_trait;
use orbis_app::ports::repositories::AcademicFrameworkRepository;
use orbis_domain::entities::academic_calendar::{
    AcademicCalendar, AcademicPeriod, AcademicCalendarStatus, PeriodType,
};
use orbis_domain::entities::subject::Course;
use orbis_domain::entities::enrollment::Enrollment;
use orbis_domain::entities::section_subject::SectionSubject;
use orbis_domain::errors::DomainError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::models::{
    academic_calendar::{self, Entity as CalendarEntity},
    academic_period::{self, Entity as CalendarPeriodEntity},
    period_type::{self, Entity as PeriodTypeEntity},
    subject::{self, Entity as SubjectEntity},
    enrollment::{self, Entity as EnrollmentEntity},
    section_subject::{self, Entity as SectionSubjectEntity},
};

pub struct AcademicFrameworkRepoImpl {
    pub db: DatabaseConnection,
}

macro_rules! save_active_model {
    ($repo:expr, $entity:ty, $active:expr, $id:expr, $org_id:expr) => {
        if let Some(existing) = <$entity>::find_by_id($id)
            .one(&$repo.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?
        {
            if existing.org_id != $org_id {
                return Err(DomainError::Forbidden(
                    "Cannot update academic data owned by another organization".into(),
                ));
            }
            $active
                .update(&$repo.db)
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;
        } else {
            $active
                .insert(&$repo.db)
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;
        }
    };
}

fn map_calendar(m: academic_calendar::Model) -> Result<AcademicCalendar, DomainError> {
    let status = match m.status.as_str() {
        "planning" => AcademicCalendarStatus::Planning,
        "active" => AcademicCalendarStatus::Active,
        "closed" => AcademicCalendarStatus::Closed,
        "archived" => AcademicCalendarStatus::Archived,
        other => {
            return Err(DomainError::Internal(format!(
                "Unknown academic calendar status '{}'",
                other
            )));
        }
    };
    Ok(AcademicCalendar {
        id: m.id,
        org_id: m.org_id,
        code: m.code,
        name: m.name,
        name_ar: m.name_ar,
        start_date: m.start_date,
        end_date: m.end_date,
        starts_at: Some(m.start_date),
        ends_at: Some(m.end_date),
        hijri_start: m.hijri_start,
        hijri_end: m.hijri_end,
        status,
        is_locked: m.is_locked,
        locked_at: m.locked_at,
        locked_by: m.locked_by,
        lock_reason: m.lock_reason,
        current_period_id: m.current_period_id,
        archived_at: m.archived_at,
        archived_by: m.archived_by,
        notes: m.notes,
        created_at: m.created_at,
        updated_at: m.updated_at,
    })
}

fn map_period_type(m: period_type::Model) -> PeriodType {
    PeriodType {
        id: m.id,
        org_id: m.org_id,
        code: m.code,
        name: m.name,
        name_ar: m.name_ar,
        sort_order: m.sort_order,
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

fn map_calendar_period(m: academic_period::Model) -> AcademicPeriod {
    AcademicPeriod {
        id: m.id,
        org_id: m.org_id,
        calendar_id: m.calendar_id,
        parent_id: m.parent_id,
        period_type_id: m.period_type_id,
        code: m.code,
        name: m.name,
        name_ar: m.name_ar,
        starts_at: m.start_date,
        ends_at: m.end_date,
        sort_order: m.sort_order.unwrap_or(0),
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

fn map_course(m: subject::Model) -> Course {
    Course {
        id: m.id,
        org_id: m.org_id,
        unit_id: m.unit_id,
        code: m.code,
        name: m.name,
        name_ar: m.name_ar,
        description: m.description,
        subject_type: m.subject_type,
        is_core: m.is_core,
        is_elective: m.is_elective,
        credit_units: m.credit_units,
        workload_units: m.workload_units,
        theory_hours: m.theory_hours,
        practical_hours: m.practical_hours,
        is_lab_required: m.is_lab_required,
        max_students_per_section: m.max_students_per_section.map(|v| v as u32),
        color: m.color,
        icon: m.icon,
        sort_order: m.sort_order,
        is_active: m.is_active,
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

fn map_offering(m: section_subject::Model) -> Result<SectionSubject, DomainError> {
    Ok(SectionSubject {
        id: m.id,
        org_id: m.org_id,
        section_id: m.section_id,
        subject_id: m.subject_id,
        period_id: m.period_id,
        instructor_id: m.instructor_id,
        assistant_instructor_id: m.assistant_instructor_id,
        substitute_instructor_id: m.substitute_instructor_id,
        scheme_id: m.scheme_id,
        weekly_hours: m.weekly_hours,
        theory_hours: m.theory_hours,
        practical_hours: m.practical_hours,
        assessment_config: m.assessment_config,
        grade_groups: m.grade_groups,
        capacity: m.capacity,
        status: m.status,
        is_active: m.is_active,
        created_at: Some(m.created_at),
        updated_at: Some(m.updated_at),
    })
}

fn map_enrollment(m: enrollment::Model) -> Result<Enrollment, DomainError> {
    Ok(Enrollment {
        id: m.id,
        org_id: m.org_id,
        period_id: m.period_id,
        student_id: m.student_id,
        section_id: m.section_id,
        attempt_number: m.attempt_number,
        student_number: m.student_number,
        study_mode: match m.study_mode.as_deref().unwrap_or("regular") {
            "homeschooled" => orbis_domain::entities::enrollment::StudyMode::Homeschooled,
            "auditor" => orbis_domain::entities::enrollment::StudyMode::Auditor,
            "visiting" => orbis_domain::entities::enrollment::StudyMode::Visiting,
            _ => orbis_domain::entities::enrollment::StudyMode::Regular,
        },
        status: match m.status.as_str() {
            "pending_approval" => orbis_domain::entities::enrollment::EnrollmentStatus::PendingApproval,
            "withdrawn" => orbis_domain::entities::enrollment::EnrollmentStatus::Withdrawn,
            "graduated" => orbis_domain::entities::enrollment::EnrollmentStatus::Graduated,
            "suspended" => orbis_domain::entities::enrollment::EnrollmentStatus::Suspended,
            _ => orbis_domain::entities::enrollment::EnrollmentStatus::Active,
        },
        is_attendance_exempt: m.is_attendance_exempt.unwrap_or(false),
        is_grade_exempt: m.is_grade_exempt.unwrap_or(false),
        enrolled_at: m.enrolled_at.into(),
        withdrawn_at: m.withdrawn_at.map(Into::into),
        withdrawal_reason: m.withdrawal_reason,
        meta: m.meta.unwrap_or_default(),
        created_by: m.created_by,
        updated_by: m.updated_by,
        created_at: m.created_at.into(),
        updated_at: m.updated_at.into(),
    })
}

#[async_trait]
impl AcademicFrameworkRepository for AcademicFrameworkRepoImpl {
    async fn list_calendars(&self, org_id: Uuid) -> Result<Vec<AcademicCalendar>, DomainError> {
        CalendarEntity::find()
            .filter(academic_calendar::Column::OrgId.eq(org_id))
            .order_by_asc(academic_calendar::Column::StartDate)
            .all(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?
            .into_iter()
            .map(map_calendar)
            .collect()
    }

    async fn find_calendar(
        &self,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<Option<AcademicCalendar>, DomainError> {
        let model = CalendarEntity::find_by_id(id)
            .filter(academic_calendar::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        model.map(map_calendar).transpose()
    }

    async fn find_calendar_by_code(
        &self,
        org_id: Uuid,
        code: &str,
    ) -> Result<Option<AcademicCalendar>, DomainError> {
        let model = CalendarEntity::find()
            .filter(academic_calendar::Column::OrgId.eq(org_id))
            .filter(academic_calendar::Column::Code.eq(code))
            .one(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        model.map(map_calendar).transpose()
    }

    async fn save_calendar(&self, calendar: &AcademicCalendar) -> Result<(), DomainError> {
        let active = academic_calendar::ActiveModel {
            id: Set(calendar.id),
            org_id: Set(calendar.org_id),
            code: Set(calendar.code.clone()),
            name: Set(calendar.name.clone()),
            name_ar: Set(calendar.name_ar.clone()),
            start_date: Set(calendar.start_date),
            end_date: Set(calendar.end_date),
            hijri_start: Set(calendar.hijri_start.clone()),
            hijri_end: Set(calendar.hijri_end.clone()),
            status: Set(calendar.status.to_string()),
            is_locked: Set(calendar.is_locked),
            locked_at: Set(calendar.locked_at),
            locked_by: Set(calendar.locked_by),
            lock_reason: Set(calendar.lock_reason.clone()),
            current_period_id: Set(calendar.current_period_id),
            archived_at: Set(calendar.archived_at),
            archived_by: Set(calendar.archived_by),
            notes: Set(calendar.notes.clone()),
            metadata: Set(None),
            created_at: Set(calendar.created_at),
            updated_at: Set(calendar.updated_at),
        };
        save_active_model!(self, CalendarEntity, active, calendar.id, calendar.org_id);
        Ok(())
    }

    async fn list_period_types(&self, org_id: Uuid) -> Result<Vec<PeriodType>, DomainError> {
        PeriodTypeEntity::find()
            .filter(period_type::Column::OrgId.eq(org_id))
            .order_by_asc(period_type::Column::SortOrder)
            .all(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))
            .map(|models| models.into_iter().map(map_period_type).collect())
    }

    async fn find_period_type(
        &self,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<Option<PeriodType>, DomainError> {
        let model = PeriodTypeEntity::find_by_id(id)
            .filter(period_type::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        Ok(model.map(map_period_type))
    }

    async fn find_period_type_by_code(
        &self,
        org_id: Uuid,
        code: &str,
    ) -> Result<Option<PeriodType>, DomainError> {
        let model = PeriodTypeEntity::find()
            .filter(period_type::Column::OrgId.eq(org_id))
            .filter(period_type::Column::Code.eq(code))
            .one(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        Ok(model.map(map_period_type))
    }

    async fn save_period_type(&self, period_type: &PeriodType) -> Result<(), DomainError> {
        let active = period_type::ActiveModel {
            id: Set(period_type.id),
            org_id: Set(period_type.org_id),
            code: Set(period_type.code.clone()),
            name: Set(period_type.name.clone()),
            name_ar: Set(period_type.name_ar.clone()),
            sort_order: Set(period_type.sort_order),
            created_at: Set(period_type.created_at),
            updated_at: Set(period_type.updated_at),
        };
        save_active_model!(
            self,
            PeriodTypeEntity,
            active,
            period_type.id,
            period_type.org_id
        );
        Ok(())
    }

    async fn find_calendar_period(
        &self,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<Option<AcademicPeriod>, DomainError> {
        let model = CalendarPeriodEntity::find_by_id(id)
            .filter(academic_period::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        Ok(model.map(map_calendar_period))
    }

    async fn save_calendar_period(
        &self,
        period: &AcademicPeriod,
    ) -> Result<(), DomainError> {
        let active = academic_period::ActiveModel {
            id: Set(period.id),
            org_id: Set(period.org_id),
            calendar_id: Set(period.calendar_id),
            parent_id: Set(period.parent_id),
            period_type_id: Set(period.period_type_id),
            code: Set(period.code.clone()),
            name: Set(period.name.clone()),
            name_ar: Set(period.name_ar.clone()),
            period_kind: Set("teaching".to_string()),
            start_date: Set(period.starts_at),
            end_date: Set(period.ends_at),
            hijri_start: Set(None),
            hijri_end: Set(None),
            enrollment_starts_at: Set(None),
            enrollment_ends_at: Set(None),
            is_enrollment_open: Set(false),
            grade_entry_starts_at: Set(None),
            grade_entry_ends_at: Set(None),
            is_grade_entry_open: Set(false),
            exam_starts_at: Set(None),
            exam_ends_at: Set(None),
            status: Set("active".to_string()),
            is_locked: Set(false),
            locked_at: Set(None),
            locked_by: Set(None),
            sort_order: Set(Some(period.sort_order)),
            metadata: Set(None),
            created_at: Set(period.created_at),
            updated_at: Set(period.updated_at),
        };
        save_active_model!(self, CalendarPeriodEntity, active, period.id, period.org_id);
        Ok(())
    }

    async fn list_calendar_periods(
        &self,
        org_id: Uuid,
        calendar_id: Uuid,
    ) -> Result<Vec<AcademicPeriod>, DomainError> {
        CalendarPeriodEntity::find()
            .filter(academic_period::Column::OrgId.eq(org_id))
            .filter(academic_period::Column::CalendarId.eq(calendar_id))
            .order_by_asc(academic_period::Column::SortOrder)
            .all(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))
            .map(|models| models.into_iter().map(map_calendar_period).collect())
    }

    async fn find_course(&self, org_id: Uuid, id: Uuid) -> Result<Option<Course>, DomainError> {
        let model = SubjectEntity::find_by_id(id)
            .filter(subject::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        Ok(model.map(map_course))
    }

    async fn list_courses(&self, org_id: Uuid) -> Result<Vec<Course>, DomainError> {
        SubjectEntity::find()
            .filter(subject::Column::OrgId.eq(org_id))
            .order_by_asc(subject::Column::Code)
            .all(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))
            .map(|models| models.into_iter().map(map_course).collect())
    }

    async fn find_course_by_code(
        &self,
        org_id: Uuid,
        code: &str,
    ) -> Result<Option<Course>, DomainError> {
        let model = SubjectEntity::find()
            .filter(subject::Column::OrgId.eq(org_id))
            .filter(subject::Column::Code.eq(code))
            .one(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        Ok(model.map(map_course))
    }

    async fn save_course(&self, course: &Course) -> Result<(), DomainError> {
        let active = subject::ActiveModel {
            id: Set(course.id),
            org_id: Set(course.org_id),
            unit_id: Set(course.unit_id),
            code: Set(course.code.clone()),
            name: Set(course.name.clone()),
            name_ar: Set(course.name_ar.clone()),
            description: Set(course.description.clone()),
            subject_type: Set(course.subject_type.clone()),
            is_core: Set(course.is_core),
            is_elective: Set(course.is_elective),
            credit_units: Set(course.credit_units),
            workload_units: Set(course.workload_units),
            theory_hours: Set(course.theory_hours),
            practical_hours: Set(course.practical_hours),
            is_lab_required: Set(course.is_lab_required),
            max_students_per_section: Set(course.max_students_per_section.map(|v| v as i32)),
            color: Set(course.color.clone()),
            icon: Set(course.icon.clone()),
            sort_order: Set(course.sort_order),
            is_active: Set(course.is_active),
            created_at: Set(course.created_at),
            updated_at: Set(course.updated_at),
        };
        save_active_model!(self, SubjectEntity, active, course.id, course.org_id);
        Ok(())
    }

    async fn find_section_subject(
        &self,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<Option<SectionSubject>, DomainError> {
        let model = SectionSubjectEntity::find_by_id(id)
            .filter(section_subject::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        model.map(map_offering).transpose()
    }

    async fn list_section_subjects(
        &self,
        org_id: Uuid,
        course_id: Option<Uuid>,
    ) -> Result<Vec<SectionSubject>, DomainError> {
        let mut query = SectionSubjectEntity::find()
            .filter(section_subject::Column::OrgId.eq(org_id));
        if let Some(course_id) = course_id {
            query = query.filter(section_subject::Column::SubjectId.eq(course_id));
        }
        query
            .order_by_asc(section_subject::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?
            .into_iter()
            .map(map_offering)
            .collect()
    }

    async fn save_section_subject(&self, offering: &SectionSubject) -> Result<(), DomainError> {
        let now = chrono::Utc::now();
        let active = section_subject::ActiveModel {
            id: Set(offering.id),
            org_id: Set(offering.org_id),
            section_id: Set(offering.section_id),
            subject_id: Set(offering.subject_id),
            period_id: Set(offering.period_id),
            instructor_id: Set(offering.instructor_id),
            assistant_instructor_id: Set(offering.assistant_instructor_id),
            substitute_instructor_id: Set(offering.substitute_instructor_id),
            scheme_id: Set(offering.scheme_id),
            weekly_hours: Set(offering.weekly_hours),
            theory_hours: Set(offering.theory_hours),
            practical_hours: Set(offering.practical_hours),
            assessment_config: Set(offering.assessment_config.clone()),
            grade_groups: Set(offering.grade_groups.clone()),
            capacity: Set(offering.capacity),
            status: Set(offering.status.clone()),
            is_active: Set(offering.is_active),
            created_at: Set(offering.created_at.unwrap_or(now)),
            updated_at: Set(offering.updated_at.unwrap_or(now)),
        };
        save_active_model!(
            self,
            SectionSubjectEntity,
            active,
            offering.id,
            offering.org_id
        );
        Ok(())
    }

    async fn find_enrollment_attempt(
        &self,
        org_id: Uuid,
        offering_id: Uuid,
        student_id: Uuid,
        attempt_number: u32,
    ) -> Result<Option<Enrollment>, DomainError> {
        let attempt_number = i32::try_from(attempt_number)
            .map_err(|error| DomainError::Validation(error.to_string()))?;
        let model = EnrollmentEntity::find()
            .filter(enrollment::Column::OrgId.eq(org_id))
            .filter(enrollment::Column::SectionId.eq(Some(offering_id)))
            .filter(enrollment::Column::StudentId.eq(student_id))
            .filter(enrollment::Column::AttemptNumber.eq(attempt_number))
            .one(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        model.map(map_enrollment).transpose()
    }

    async fn list_enrollments(
        &self,
        org_id: Uuid,
        offering_id: Uuid,
    ) -> Result<Vec<Enrollment>, DomainError> {
        EnrollmentEntity::find()
            .filter(enrollment::Column::OrgId.eq(org_id))
            .filter(enrollment::Column::SectionId.eq(Some(offering_id)))
            .order_by_asc(enrollment::Column::EnrolledAt)
            .all(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?
            .into_iter()
            .map(map_enrollment)
            .collect()
    }

    async fn save_enrollment(
        &self,
        enrollment: &Enrollment,
    ) -> Result<(), DomainError> {
        let active = enrollment::ActiveModel {
            id: Set(enrollment.id),
            org_id: Set(enrollment.org_id),
            period_id: Set(enrollment.period_id),
            student_id: Set(enrollment.student_id),
            section_id: Set(enrollment.section_id),
            student_number: Set(enrollment.student_number.clone()),
            status: Set(enrollment.status.to_string()),
            study_mode: Set(Some(enrollment.study_mode.to_string())),
            attempt_number: Set(enrollment.attempt_number),
            enrollment_type: Set(None),
            is_attendance_exempt: Set(Some(enrollment.is_attendance_exempt)),
            is_grade_exempt: Set(Some(enrollment.is_grade_exempt)),
            enrolled_at: Set(enrollment.enrolled_at.into()),
            withdrawn_at: Set(enrollment.withdrawn_at.map(Into::into)),
            withdrawal_reason: Set(enrollment.withdrawal_reason.clone()),
            meta: Set(Some(enrollment.meta.clone())),
            created_by: Set(enrollment.created_by),
            updated_by: Set(enrollment.updated_by),
            created_at: Set(enrollment.created_at.into()),
            updated_at: Set(enrollment.updated_at.into()),
        };
        save_active_model!(
            self,
            EnrollmentEntity,
            active,
            enrollment.id,
            enrollment.org_id
        );
        Ok(())
    }
}
