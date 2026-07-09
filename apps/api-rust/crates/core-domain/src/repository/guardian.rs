use crate::entities::student_guardian;
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait};

impl super::Repository {
    pub async fn get_guardians_by_student(
        &self,
        student_id: &str,
    ) -> Result<Vec<student_guardian::Model>, DbErr> {
        student_guardian::Entity::find()
            .filter(student_guardian::Column::StudentId.eq(student_id))
            .all(&self.db)
            .await
    }

    pub async fn get_students_by_guardian(
        &self,
        guardian_id: &str,
    ) -> Result<Vec<student_guardian::Model>, DbErr> {
        student_guardian::Entity::find()
            .filter(student_guardian::Column::GuardianId.eq(guardian_id))
            .all(&self.db)
            .await
    }

    pub async fn get_guardian_link_by_id(
        &self,
        id: &str,
    ) -> Result<Option<student_guardian::Model>, DbErr> {
        student_guardian::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    pub async fn insert_guardian_link(
        &self,
        link: student_guardian::Model,
    ) -> Result<student_guardian::Model, DbErr> {
        let active = student_guardian::ActiveModel {
            id: Set(link.id),
            student_id: Set(link.student_id),
            guardian_id: Set(link.guardian_id),
            relationship: Set(link.relationship),
            relationship_ar: Set(link.relationship_ar),
            is_primary: Set(link.is_primary),
            is_emergency_contact: Set(link.is_emergency_contact),
            is_authorized_pickup: Set(link.is_authorized_pickup),
            can_view_grades: Set(link.can_view_grades),
            can_view_attendance: Set(link.can_view_attendance),
            can_view_behavior: Set(link.can_view_behavior),
            can_view_finance: Set(link.can_view_finance),
            can_view_schedule: Set(link.can_view_schedule),
            can_receive_notifications: Set(link.can_receive_notifications),
            is_payer: Set(link.is_payer),
            payment_share_percent: Set(link.payment_share_percent),
            occupation: Set(link.occupation),
            employer: Set(link.employer),
            work_phone: Set(link.work_phone),
            notes: Set(link.notes),
            created_at: Set(link.created_at),
            updated_at: Set(link.updated_at),
        };
        active.insert(&self.db).await
    }
}
