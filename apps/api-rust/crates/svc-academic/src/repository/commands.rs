//! Academic command repository — write operations.

use crate::entities::{class, subject, class_subject, enrollment};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr};
use super::AcademicCommandRepo;

impl AcademicCommandRepo {
    // ── Class Commands ──────────────────────────────────────────

    pub async fn create_class(&self, c: class::Model) -> Result<class::Model, DbErr> {
        let active = class::ActiveModel {
            id: Set(c.id), org_id: Set(c.org_id), unit_id: Set(c.unit_id),
            period_id: Set(c.period_id), name: Set(c.name), name_ar: Set(c.name_ar),
            level: Set(c.level), division: Set(c.division),
            homeroom_teacher_id: Set(c.homeroom_teacher_id),
            capacity: Set(c.capacity), current_students: Set(c.current_students),
            room_id: Set(c.room_id), color: Set(c.color),
            is_active: Set(c.is_active), is_deleted: Set(c.is_deleted),
            deleted_at: Set(c.deleted_at),
            created_at: Set(c.created_at), updated_at: Set(c.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_class(&self, model: class::ActiveModel) -> Result<class::Model, DbErr> {
        model.update(&self.db).await
    }

    // ── Subject Commands ────────────────────────────────────────

    pub async fn create_subject(&self, s: subject::Model) -> Result<subject::Model, DbErr> {
        let active = subject::ActiveModel {
            id: Set(s.id), org_id: Set(s.org_id), unit_id: Set(s.unit_id),
            code: Set(s.code), name: Set(s.name), name_ar: Set(s.name_ar),
            name_en: Set(s.name_en), description: Set(s.description),
            short_description: Set(s.short_description),
            is_core: Set(s.is_core), is_elective: Set(s.is_elective),
            credit_hours: Set(s.credit_hours),
            requires_lab: Set(s.requires_lab), requires_practical: Set(s.requires_practical),
            max_students_per_class: Set(s.max_students_per_class),
            default_grade_groups: Set(s.default_grade_groups),
            prerequisites: Set(s.prerequisites), learning_outcomes: Set(s.learning_outcomes),
            color: Set(s.color), icon: Set(s.icon), cover_image: Set(s.cover_image),
            sort_order: Set(s.sort_order), is_active: Set(s.is_active),
            created_at: Set(s.created_at), updated_at: Set(s.updated_at),
        };
        active.insert(&self.db).await
    }

    // ── Enrollment Commands ─────────────────────────────────────

    pub async fn create_enrollment(&self, e: enrollment::Model) -> Result<enrollment::Model, DbErr> {
        let active = enrollment::ActiveModel {
            id: Set(e.id), org_id: Set(e.org_id), period_id: Set(e.period_id),
            student_id: Set(e.student_id), class_id: Set(e.class_id),
            student_number: Set(e.student_number), status: Set(e.status),
            enrollment_date: Set(e.enrollment_date), enrollment_type: Set(e.enrollment_type),
            withdrawal_date: Set(e.withdrawal_date), withdrawal_reason: Set(e.withdrawal_reason),
            final_grade: Set(e.final_grade), final_score: Set(e.final_score),
            final_rank: Set(e.final_rank), is_promoted: Set(e.is_promoted),
            promotion_decision: Set(e.promotion_decision), promotion_notes: Set(e.promotion_notes),
            meta: Set(e.meta),
            created_at: Set(e.created_at), updated_at: Set(e.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_enrollment(&self, model: enrollment::ActiveModel) -> Result<enrollment::Model, DbErr> {
        model.update(&self.db).await
    }

    // ── Class Subject Commands ──────────────────────────────────

    pub async fn assign_subject_to_class(&self, cs: class_subject::Model) -> Result<class_subject::Model, DbErr> {
        let active = class_subject::ActiveModel {
            id: Set(cs.id), class_id: Set(cs.class_id), subject_id: Set(cs.subject_id),
            period_id: Set(cs.period_id), primary_teacher_id: Set(cs.primary_teacher_id),
            assistant_teacher_id: Set(cs.assistant_teacher_id),
            substitute_teacher_id: Set(cs.substitute_teacher_id),
            grade_groups: Set(cs.grade_groups), weekly_hours: Set(cs.weekly_hours),
            theory_hours: Set(cs.theory_hours), practical_hours: Set(cs.practical_hours),
            assessment_config: Set(cs.assessment_config), is_active: Set(cs.is_active),
            created_at: Set(cs.created_at), updated_at: Set(cs.updated_at),
        };
        active.insert(&self.db).await
    }

}
