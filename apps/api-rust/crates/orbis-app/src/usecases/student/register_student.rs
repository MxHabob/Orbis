use orbis_domain::entities::student::{Student, Gender, StudentStatus, AcademicStanding};
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::StudentRepository;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use orbis_domain::iam::permission::Permission;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{Utc, NaiveDate};

pub struct RegisterStudentUseCase {
    pub student_repo: Arc<dyn StudentRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl RegisterStudentUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        person_id: Uuid,
        user_id: Option<Uuid>,
        student_number: String,
        _national_id: Option<String>,
        _passport_number: Option<String>,
        _first_name: String,
        _last_name: String,
        _name_ar: Option<String>,
        _gender: Gender,
        _date_of_birth: Option<NaiveDate>,
        _place_of_birth: Option<String>,
        _nationality: Option<String>,
        _blood_type: Option<String>,
        _medical_notes: Option<String>,
        _special_needs_notes: Option<String>,
    ) -> Result<Student, DomainError> {
        self.permission_checker.check(ctx, Permission::StudentCreate).await?;

        let now = Utc::now();
        let student_id = Uuid::new_v4();

        let student = Student {
            id: student_id,
            org_id,
            person_id,
            user_id,
            student_number: student_number.clone(),
            status: StudentStatus::Active,
            standing: AcademicStanding::GoodStanding,
            admitted_at: now,
            graduated_at: None,
            is_deleted: false,
            deleted_at: None,
            meta: serde_json::json!({}),
            created_at: now,
            updated_at: now,
        };

        self.student_repo.save(&student).await?;

        let event = DomainEvent::StudentRegistered {
            student_id,
            org_id,
            student_number,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(student)
    }
}
