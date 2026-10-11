use async_trait::async_trait;
use orbis_app::ports::repositories::AcademicFrameworkRepository;
use orbis_domain::entities::academic_calendar::{
    AcademicCalendar, AcademicCalendarPeriod, AcademicCalendarStatus, AcademicPeriodType,
};
use orbis_domain::entities::course::Course;
use orbis_domain::entities::course_enrollment::{CourseEnrollment, CourseEnrollmentStatus};
use orbis_domain::entities::course_offering::{CourseOffering, CourseOfferingStatus};
use orbis_domain::errors::DomainError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::models::{
    academic_calendar::{self, Entity as CalendarEntity},
    academic_calendar_period::{self, Entity as CalendarPeriodEntity},
    academic_period_type::{self, Entity as PeriodTypeEntity},
    course::{self, Entity as CourseEntity},
    course_enrollment::{self, Entity as CourseEnrollmentEntity},
    course_offering::{self, Entity as CourseOfferingEntity},
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
        starts_at: m.starts_at,
        ends_at: m.ends_at,
        status,
        created_at: m.created_at,
        updated_at: m.updated_at,
    })
}

fn map_period_type(m: academic_period_type::Model) -> AcademicPeriodType {
    AcademicPeriodType {
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

fn map_calendar_period(m: academic_calendar_period::Model) -> AcademicCalendarPeriod {
    AcademicCalendarPeriod {
        id: m.id,
        org_id: m.org_id,
        calendar_id: m.calendar_id,
        parent_id: m.parent_id,
        period_type_id: m.period_type_id,
        code: m.code,
        name: m.name,
        name_ar: m.name_ar,
        starts_at: m.starts_at,
        ends_at: m.ends_at,
        sort_order: m.sort_order,
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

fn map_course(m: course::Model) -> Course {
    Course {
        id: m.id,
        org_id: m.org_id,
        unit_id: m.unit_id,
        code: m.code,
        name: m.name,
        name_ar: m.name_ar,
        description: m.description,
        credit_units: m.credit_units,
        workload_units: m.workload_units,
        is_active: m.is_active,
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

fn map_offering(m: course_offering::Model) -> Result<CourseOffering, DomainError> {
    let status = match m.status.as_str() {
        "draft" => CourseOfferingStatus::Draft,
        "open" => CourseOfferingStatus::Open,
        "in_progress" => CourseOfferingStatus::InProgress,
        "completed" => CourseOfferingStatus::Completed,
        "cancelled" => CourseOfferingStatus::Cancelled,
        other => {
            return Err(DomainError::Internal(format!(
                "Unknown course offering status '{}'",
                other
            )));
        }
    };
    let capacity = m
        .capacity
        .map(u32::try_from)
        .transpose()
        .map_err(|error| DomainError::Internal(error.to_string()))?;
    Ok(CourseOffering {
        id: m.id,
        org_id: m.org_id,
        course_id: m.course_id,
        period_id: m.period_id,
        section_code: m.section_code,
        name: m.name,
        status,
        capacity,
        starts_at: m.starts_at,
        ends_at: m.ends_at,
        created_at: m.created_at,
        updated_at: m.updated_at,
    })
}

fn map_course_enrollment(m: course_enrollment::Model) -> Result<CourseEnrollment, DomainError> {
    let status = match m.status.as_str() {
        "pending" => CourseEnrollmentStatus::Pending,
        "active" => CourseEnrollmentStatus::Active,
        "withdrawn" => CourseEnrollmentStatus::Withdrawn,
        "completed" => CourseEnrollmentStatus::Completed,
        "failed" => CourseEnrollmentStatus::Failed,
        "exempt" => CourseEnrollmentStatus::Exempt,
        other => {
            return Err(DomainError::Internal(format!(
                "Unknown course enrollment status '{}'",
                other
            )));
        }
    };
    let attempt_number = u32::try_from(m.attempt_number)
        .map_err(|error| DomainError::Internal(error.to_string()))?;
    Ok(CourseEnrollment {
        id: m.id,
        org_id: m.org_id,
        offering_id: m.offering_id,
        student_id: m.student_id,
        attempt_number,
        status,
        enrolled_at: m.enrolled_at,
        withdrawn_at: m.withdrawn_at,
        withdrawal_reason: m.withdrawal_reason,
        created_by: m.created_by,
        updated_by: m.updated_by,
        created_at: m.created_at,
        updated_at: m.updated_at,
    })
}

#[async_trait]
impl AcademicFrameworkRepository for AcademicFrameworkRepoImpl {
    async fn list_calendars(&self, org_id: Uuid) -> Result<Vec<AcademicCalendar>, DomainError> {
        CalendarEntity::find()
            .filter(academic_calendar::Column::OrgId.eq(org_id))
            .order_by_asc(academic_calendar::Column::StartsAt)
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
            starts_at: Set(calendar.starts_at),
            ends_at: Set(calendar.ends_at),
            status: Set(calendar.status.to_string()),
            created_at: Set(calendar.created_at),
            updated_at: Set(calendar.updated_at),
        };
        save_active_model!(self, CalendarEntity, active, calendar.id, calendar.org_id);
        Ok(())
    }

    async fn list_period_types(&self, org_id: Uuid) -> Result<Vec<AcademicPeriodType>, DomainError> {
        PeriodTypeEntity::find()
            .filter(academic_period_type::Column::OrgId.eq(org_id))
            .order_by_asc(academic_period_type::Column::SortOrder)
            .all(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))
            .map(|models| models.into_iter().map(map_period_type).collect())
    }

    async fn find_period_type(
        &self,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<Option<AcademicPeriodType>, DomainError> {
        let model = PeriodTypeEntity::find_by_id(id)
            .filter(academic_period_type::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        Ok(model.map(map_period_type))
    }

    async fn find_period_type_by_code(
        &self,
        org_id: Uuid,
        code: &str,
    ) -> Result<Option<AcademicPeriodType>, DomainError> {
        let model = PeriodTypeEntity::find()
            .filter(academic_period_type::Column::OrgId.eq(org_id))
            .filter(academic_period_type::Column::Code.eq(code))
            .one(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        Ok(model.map(map_period_type))
    }

    async fn save_period_type(&self, period_type: &AcademicPeriodType) -> Result<(), DomainError> {
        let active = academic_period_type::ActiveModel {
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
    ) -> Result<Option<AcademicCalendarPeriod>, DomainError> {
        let model = CalendarPeriodEntity::find_by_id(id)
            .filter(academic_calendar_period::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        Ok(model.map(map_calendar_period))
    }

    async fn save_calendar_period(
        &self,
        period: &AcademicCalendarPeriod,
    ) -> Result<(), DomainError> {
        let active = academic_calendar_period::ActiveModel {
            id: Set(period.id),
            org_id: Set(period.org_id),
            calendar_id: Set(period.calendar_id),
            parent_id: Set(period.parent_id),
            period_type_id: Set(period.period_type_id),
            code: Set(period.code.clone()),
            name: Set(period.name.clone()),
            name_ar: Set(period.name_ar.clone()),
            starts_at: Set(period.starts_at),
            ends_at: Set(period.ends_at),
            sort_order: Set(period.sort_order),
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
    ) -> Result<Vec<AcademicCalendarPeriod>, DomainError> {
        CalendarPeriodEntity::find()
            .filter(academic_calendar_period::Column::OrgId.eq(org_id))
            .filter(academic_calendar_period::Column::CalendarId.eq(calendar_id))
            .order_by_asc(academic_calendar_period::Column::SortOrder)
            .all(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))
            .map(|models| models.into_iter().map(map_calendar_period).collect())
    }

    async fn find_course(&self, org_id: Uuid, id: Uuid) -> Result<Option<Course>, DomainError> {
        let model = CourseEntity::find_by_id(id)
            .filter(course::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        Ok(model.map(map_course))
    }

    async fn list_courses(&self, org_id: Uuid) -> Result<Vec<Course>, DomainError> {
        CourseEntity::find()
            .filter(course::Column::OrgId.eq(org_id))
            .order_by_asc(course::Column::Code)
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
        let model = CourseEntity::find()
            .filter(course::Column::OrgId.eq(org_id))
            .filter(course::Column::Code.eq(code))
            .one(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        Ok(model.map(map_course))
    }

    async fn save_course(&self, course: &Course) -> Result<(), DomainError> {
        let active = course::ActiveModel {
            id: Set(course.id),
            org_id: Set(course.org_id),
            unit_id: Set(course.unit_id),
            code: Set(course.code.clone()),
            name: Set(course.name.clone()),
            name_ar: Set(course.name_ar.clone()),
            description: Set(course.description.clone()),
            credit_units: Set(course.credit_units),
            workload_units: Set(course.workload_units),
            is_active: Set(course.is_active),
            created_at: Set(course.created_at),
            updated_at: Set(course.updated_at),
        };
        save_active_model!(self, CourseEntity, active, course.id, course.org_id);
        Ok(())
    }

    async fn find_course_offering(
        &self,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<Option<CourseOffering>, DomainError> {
        let model = CourseOfferingEntity::find_by_id(id)
            .filter(course_offering::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        model.map(map_offering).transpose()
    }

    async fn list_course_offerings(
        &self,
        org_id: Uuid,
        course_id: Option<Uuid>,
    ) -> Result<Vec<CourseOffering>, DomainError> {
        let mut query = CourseOfferingEntity::find()
            .filter(course_offering::Column::OrgId.eq(org_id));
        if let Some(course_id) = course_id {
            query = query.filter(course_offering::Column::CourseId.eq(course_id));
        }
        query
            .order_by_asc(course_offering::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?
            .into_iter()
            .map(map_offering)
            .collect()
    }

    async fn save_course_offering(&self, offering: &CourseOffering) -> Result<(), DomainError> {
        let capacity = offering
            .capacity
            .map(i32::try_from)
            .transpose()
            .map_err(|error| DomainError::Validation(error.to_string()))?;
        let active = course_offering::ActiveModel {
            id: Set(offering.id),
            org_id: Set(offering.org_id),
            course_id: Set(offering.course_id),
            period_id: Set(offering.period_id),
            section_code: Set(offering.section_code.clone()),
            name: Set(offering.name.clone()),
            status: Set(offering.status.to_string()),
            capacity: Set(capacity),
            starts_at: Set(offering.starts_at),
            ends_at: Set(offering.ends_at),
            created_at: Set(offering.created_at),
            updated_at: Set(offering.updated_at),
        };
        save_active_model!(
            self,
            CourseOfferingEntity,
            active,
            offering.id,
            offering.org_id
        );
        Ok(())
    }

    async fn find_course_enrollment_attempt(
        &self,
        org_id: Uuid,
        offering_id: Uuid,
        student_id: Uuid,
        attempt_number: u32,
    ) -> Result<Option<CourseEnrollment>, DomainError> {
        let attempt_number = i32::try_from(attempt_number)
            .map_err(|error| DomainError::Validation(error.to_string()))?;
        let model = CourseEnrollmentEntity::find()
            .filter(course_enrollment::Column::OrgId.eq(org_id))
            .filter(course_enrollment::Column::OfferingId.eq(offering_id))
            .filter(course_enrollment::Column::StudentId.eq(student_id))
            .filter(course_enrollment::Column::AttemptNumber.eq(attempt_number))
            .one(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        model.map(map_course_enrollment).transpose()
    }

    async fn list_course_enrollments(
        &self,
        org_id: Uuid,
        offering_id: Uuid,
    ) -> Result<Vec<CourseEnrollment>, DomainError> {
        CourseEnrollmentEntity::find()
            .filter(course_enrollment::Column::OrgId.eq(org_id))
            .filter(course_enrollment::Column::OfferingId.eq(offering_id))
            .order_by_asc(course_enrollment::Column::EnrolledAt)
            .all(&self.db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?
            .into_iter()
            .map(map_course_enrollment)
            .collect()
    }

    async fn save_course_enrollment(
        &self,
        enrollment: &CourseEnrollment,
    ) -> Result<(), DomainError> {
        let attempt_number = i32::try_from(enrollment.attempt_number)
            .map_err(|error| DomainError::Validation(error.to_string()))?;
        let active = course_enrollment::ActiveModel {
            id: Set(enrollment.id),
            org_id: Set(enrollment.org_id),
            offering_id: Set(enrollment.offering_id),
            student_id: Set(enrollment.student_id),
            attempt_number: Set(attempt_number),
            status: Set(enrollment.status.to_string()),
            enrolled_at: Set(enrollment.enrolled_at),
            withdrawn_at: Set(enrollment.withdrawn_at),
            withdrawal_reason: Set(enrollment.withdrawal_reason.clone()),
            created_by: Set(enrollment.created_by),
            updated_by: Set(enrollment.updated_by),
            created_at: Set(enrollment.created_at),
            updated_at: Set(enrollment.updated_at),
        };
        save_active_model!(
            self,
            CourseEnrollmentEntity,
            active,
            enrollment.id,
            enrollment.org_id
        );
        Ok(())
    }
}
