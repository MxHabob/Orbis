use crate::entities::{class, enrollment, academic_cycle};
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait};

impl super::Repository {
    pub async fn get_classes(&self, org_id: &str, params: crate::params::ClassParams) -> Result<Vec<class::Model>, DbErr> {
        let mut query = class::Entity::find()
            .filter(class::Column::OrgId.eq(org_id))
            .filter(class::Column::IsDeleted.eq(0));

        if let Some(ref search) = params.q {
            if !search.trim().is_empty() {
                let s = format!("%{}%", search);
                query = query.filter(
                    class::Column::Name.like(&s)
                        .or(class::Column::Level.like(&s))
                );
            }
        }

        if let Some(active) = params.is_active {
            query = query.filter(class::Column::IsActive.eq(active));
        }

        query.all(&self.db).await
    }

    pub async fn create_class(&self, c: class::Model) -> Result<class::Model, DbErr> {
        let active_model = class::ActiveModel {
            id: Set(c.id),
            org_id: Set(c.org_id),
            unit_id: Set(c.unit_id),
            cycle_id: Set(c.cycle_id),
            name: Set(c.name),
            name_ar: Set(c.name_ar),
            level: Set(c.level),
            division: Set(c.division),
            homeroom_teacher_id: Set(c.homeroom_teacher_id),
            capacity: Set(c.capacity),
            current_students: Set(c.current_students),
            room_id: Set(c.room_id),
            color: Set(c.color),
            schedule_template: Set(c.schedule_template),
            is_active: Set(c.is_active),
            is_deleted: Set(c.is_deleted),
            deleted_at: Set(c.deleted_at),
            created_at: Set(c.created_at),
            updated_at: Set(c.updated_at),
        };
        active_model.insert(&self.db).await
    }

    pub async fn get_enrollment_by_student_id(&self, student_id: &str, cycle_id: &str) -> Result<Option<enrollment::Model>, DbErr> {
        enrollment::Entity::find()
            .filter(enrollment::Column::StudentId.eq(student_id))
            .filter(enrollment::Column::CycleId.eq(cycle_id))
            .filter(enrollment::Column::Status.eq("active"))
            .one(&self.db)
            .await
    }

    pub async fn create_enrollment(&self, e: enrollment::Model) -> Result<enrollment::Model, DbErr> {
        let active_model = enrollment::ActiveModel {
            id: Set(e.id),
            org_id: Set(e.org_id),
            cycle_id: Set(e.cycle_id),
            student_id: Set(e.student_id),
            class_id: Set(e.class_id),
            student_number: Set(e.student_number),
            status: Set(e.status),
            enrollment_date: Set(e.enrollment_date),
            enrollment_type: Set(e.enrollment_type),
            withdrawal_date: Set(e.withdrawal_date),
            withdrawal_reason: Set(e.withdrawal_reason),
            final_grade: Set(e.final_grade),
            final_score: Set(e.final_score),
            final_rank: Set(e.final_rank),
            is_promoted: Set(e.is_promoted),
            promotion_decision: Set(e.promotion_decision),
            promotion_notes: Set(e.promotion_notes),
            meta: Set(e.meta),
            created_at: Set(e.created_at),
            updated_at: Set(e.updated_at),
        };
        active_model.insert(&self.db).await
    }

    pub async fn get_current_cycle(&self, org_id: &str) -> Result<Option<academic_cycle::Model>, DbErr> {
        academic_cycle::Entity::find()
            .filter(academic_cycle::Column::OrgId.eq(org_id))
            .filter(academic_cycle::Column::IsCurrent.eq(1))
            .filter(academic_cycle::Column::CycleType.eq("year"))
            .one(&self.db)
            .await
    }

    pub async fn create_academic_cycle(&self, ac: academic_cycle::Model) -> Result<academic_cycle::Model, DbErr> {
        let active_model = academic_cycle::ActiveModel {
            id: Set(ac.id),
            org_id: Set(ac.org_id),
            name: Set(ac.name),
            name_ar: Set(ac.name_ar),
            code: Set(ac.code),
            cycle_type: Set(ac.cycle_type),
            parent_id: Set(ac.parent_id),
            start_date: Set(ac.start_date),
            end_date: Set(ac.end_date),
            hijri_start_date: Set(ac.hijri_start_date),
            hijri_end_date: Set(ac.hijri_end_date),
            exam_start_date: Set(ac.exam_start_date),
            exam_end_date: Set(ac.exam_end_date),
            weight: Set(ac.weight),
            grading_system: Set(ac.grading_system),
            passing_grade: Set(ac.passing_grade),
            max_grade: Set(ac.max_grade),
            is_current: Set(ac.is_current),
            is_locked: Set(ac.is_locked),
            is_enrollment_open: Set(ac.is_enrollment_open),
            settings: Set(ac.settings),
            sort_order: Set(ac.sort_order),
            created_at: Set(ac.created_at),
            updated_at: Set(ac.updated_at),
        };
        active_model.insert(&self.db).await
    }
}

