use async_trait::async_trait;
use orbis_app::ports::repositories::{EnrollmentRepository, StudentGuardianRepository};
use orbis_domain::entities::enrollment::Enrollment as DomainEnrollment;
use orbis_domain::entities::student_guardian::StudentGuardian as DomainStudentGuardian;
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::models::{
    enrollment::{self, Entity as EnrollmentEntity},
    student_guardian::{self, Entity as StudentGuardianEntity},
};

pub struct StudentRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl EnrollmentRepository for StudentRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainEnrollment>, DomainError> {
        let model = EnrollmentEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainEnrollment {
            id: m.id, org_id: m.org_id, period_id: m.period_id, student_id: m.student_id,
            class_id: m.class_id, student_number: m.student_number, status: m.status,
            enrollment_date: m.enrollment_date, enrollment_type: m.enrollment_type,
            withdrawal_date: m.withdrawal_date, withdrawal_reason: m.withdrawal_reason,
            meta: m.meta, created_at: m.created_at, updated_at: m.updated_at
        }))
    }

    async fn find_by_student(&self, student_id: Uuid) -> Result<Vec<DomainEnrollment>, DomainError> {
        let models = EnrollmentEntity::find().filter(enrollment::Column::StudentId.eq(student_id)).all(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainEnrollment {
            id: m.id, org_id: m.org_id, period_id: m.period_id, student_id: m.student_id,
            class_id: m.class_id, student_number: m.student_number, status: m.status,
            enrollment_date: m.enrollment_date, enrollment_type: m.enrollment_type,
            withdrawal_date: m.withdrawal_date, withdrawal_reason: m.withdrawal_reason,
            meta: m.meta, created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }

    async fn find_by_class(&self, class_id: Uuid) -> Result<Vec<DomainEnrollment>, DomainError> {
        let models = EnrollmentEntity::find()
            .filter(enrollment::Column::ClassId.eq(class_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainEnrollment {
            id: m.id, org_id: m.org_id, period_id: m.period_id, student_id: m.student_id,
            class_id: m.class_id, student_number: m.student_number, status: m.status,
            enrollment_date: m.enrollment_date, enrollment_type: m.enrollment_type,
            withdrawal_date: m.withdrawal_date, withdrawal_reason: m.withdrawal_reason,
            meta: m.meta, created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainEnrollment>, DomainError> {
        let models = EnrollmentEntity::find()
            .filter(enrollment::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainEnrollment {
            id: m.id, org_id: m.org_id, period_id: m.period_id, student_id: m.student_id,
            class_id: m.class_id, student_number: m.student_number, status: m.status,
            enrollment_date: m.enrollment_date, enrollment_type: m.enrollment_type,
            withdrawal_date: m.withdrawal_date, withdrawal_reason: m.withdrawal_reason,
            meta: m.meta, created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }


    async fn save(&self, enrollment: &DomainEnrollment) -> Result<(), DomainError> {
        let active = enrollment::ActiveModel {
            id: Set(enrollment.id), org_id: Set(enrollment.org_id),
            period_id: Set(enrollment.period_id), student_id: Set(enrollment.student_id),
            class_id: Set(enrollment.class_id), student_number: Set(enrollment.student_number.clone()),
            status: Set(enrollment.status.clone()), enrollment_date: Set(enrollment.enrollment_date),
            enrollment_type: Set(enrollment.enrollment_type.clone()),
            withdrawal_date: Set(enrollment.withdrawal_date),
            withdrawal_reason: Set(enrollment.withdrawal_reason.clone()),
            meta: Set(enrollment.meta.clone()), created_at: Set(enrollment.created_at),
            updated_at: Set(enrollment.updated_at),
        };

        if EnrollmentEntity::find_by_id(enrollment.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl StudentGuardianRepository for StudentRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainStudentGuardian>, DomainError> {
        let model = StudentGuardianEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainStudentGuardian {
            id: m.id, student_id: m.student_id, guardian_id: m.guardian_id,
            relationship: m.relationship, relationship_ar: m.relationship_ar,
            is_primary: m.is_primary, is_emergency_contact: m.is_emergency_contact,
            is_authorized_pickup: m.is_authorized_pickup, can_view_grades: m.can_view_grades,
            can_view_attendance: m.can_view_attendance, can_view_behavior: m.can_view_behavior,
            can_view_finance: m.can_view_finance, can_view_schedule: m.can_view_schedule,
            can_receive_notifications: m.can_receive_notifications, is_payer: m.is_payer,
            payment_share_percent: m.payment_share_percent, occupation: m.occupation,
            employer: m.employer, work_phone: m.work_phone, notes: m.notes,
            created_at: m.created_at, updated_at: m.updated_at
        }))
    }

    async fn find_by_student(&self, student_id: Uuid) -> Result<Vec<DomainStudentGuardian>, DomainError> {
        let models = StudentGuardianEntity::find().filter(student_guardian::Column::StudentId.eq(student_id)).all(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainStudentGuardian {
            id: m.id, student_id: m.student_id, guardian_id: m.guardian_id,
            relationship: m.relationship, relationship_ar: m.relationship_ar,
            is_primary: m.is_primary, is_emergency_contact: m.is_emergency_contact,
            is_authorized_pickup: m.is_authorized_pickup, can_view_grades: m.can_view_grades,
            can_view_attendance: m.can_view_attendance, can_view_behavior: m.can_view_behavior,
            can_view_finance: m.can_view_finance, can_view_schedule: m.can_view_schedule,
            can_receive_notifications: m.can_receive_notifications, is_payer: m.is_payer,
            payment_share_percent: m.payment_share_percent, occupation: m.occupation,
            employer: m.employer, work_phone: m.work_phone, notes: m.notes,
            created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }

    async fn save(&self, guardian: &DomainStudentGuardian) -> Result<(), DomainError> {
        let active = student_guardian::ActiveModel {
            id: Set(guardian.id), student_id: Set(guardian.student_id),
            guardian_id: Set(guardian.guardian_id), relationship: Set(guardian.relationship.clone()),
            relationship_ar: Set(guardian.relationship_ar.clone()), is_primary: Set(guardian.is_primary),
            is_emergency_contact: Set(guardian.is_emergency_contact),
            is_authorized_pickup: Set(guardian.is_authorized_pickup),
            can_view_grades: Set(guardian.can_view_grades),
            can_view_attendance: Set(guardian.can_view_attendance),
            can_view_behavior: Set(guardian.can_view_behavior),
            can_view_finance: Set(guardian.can_view_finance),
            can_view_schedule: Set(guardian.can_view_schedule),
            can_receive_notifications: Set(guardian.can_receive_notifications),
            is_payer: Set(guardian.is_payer), payment_share_percent: Set(guardian.payment_share_percent),
            occupation: Set(guardian.occupation.clone()), employer: Set(guardian.employer.clone()),
            work_phone: Set(guardian.work_phone.clone()), notes: Set(guardian.notes.clone()),
            created_at: Set(guardian.created_at), updated_at: Set(guardian.updated_at),
        };

        if StudentGuardianEntity::find_by_id(guardian.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}
