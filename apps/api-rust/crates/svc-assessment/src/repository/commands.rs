//! Assessment command repository — write operations.

use crate::entities::{grade, attendance, attendance_daily, attendance_summary, behavior_record};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr};
use super::AssessmentCommandRepo;

impl AssessmentCommandRepo {
    // ── Behavior Commands ───────────────────────────────────────

    pub async fn create_behavior_record(&self, b: behavior_record::Model) -> Result<behavior_record::Model, DbErr> {
        let active = behavior_record::ActiveModel {
            id: Set(b.id),
            org_id: Set(b.org_id),
            period_id: Set(b.period_id),
            enrollment_id: Set(b.enrollment_id),
            behavior_type: Set(b.behavior_type),
            severity: Set(b.severity),
            category: Set(b.category),
            title: Set(b.title),
            title_ar: Set(b.title_ar),
            description: Set(b.description),
            incident_date: Set(b.incident_date),
            incident_time: Set(b.incident_time),
            location: Set(b.location),
            points: Set(b.points),
            action_taken: Set(b.action_taken),
            action_details: Set(b.action_details),
            action_completed: Set(b.action_completed),
            reported_by: Set(b.reported_by),
            reviewed_by: Set(b.reviewed_by),
            reviewed_at: Set(b.reviewed_at),
            witnesses: Set(b.witnesses),
            parent_notified: Set(b.parent_notified),
            parent_notified_at: Set(b.parent_notified_at),
            parent_response: Set(b.parent_response),
            evidence_urls: Set(b.evidence_urls),
            status: Set(b.status),
            resolution_notes: Set(b.resolution_notes),
            resolved_at: Set(b.resolved_at),
            resolved_by: Set(b.resolved_by),
            is_private: Set(b.is_private),
            ai_source: Set(b.ai_source),
            ai_confidence: Set(b.ai_confidence),
            ai_model_version: Set(b.ai_model_version),
            camera_id: Set(b.camera_id),
            created_at: Set(b.created_at),
            updated_at: Set(b.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_behavior_record(&self, active: behavior_record::ActiveModel) -> Result<behavior_record::Model, DbErr> {
        active.update(&self.db).await
    }

    // ── Grade Commands ──────────────────────────────────────────

    pub async fn create_grade(&self, g: grade::Model) -> Result<grade::Model, DbErr> {
        let active = grade::ActiveModel {
            id: Set(g.id),
            org_id: Set(g.org_id),
            period_id: Set(g.period_id),
            enrollment_id: Set(g.enrollment_id),
            assessment_id: Set(g.assessment_id),
            class_subject_id: Set(g.class_subject_id),
            subject_name: Set(g.subject_name),
            subject_name_ar: Set(g.subject_name_ar),
            score: Set(g.score),
            max_score: Set(g.max_score),
            weight: Set(g.weight),
            grade_letter: Set(g.grade_letter),
            grade_points: Set(g.grade_points),
            exam_type: Set(g.exam_type),
            semester: Set(g.semester),
            notes: Set(g.notes),
            graded_by: Set(g.graded_by),
            graded_at: Set(g.graded_at),
            is_final: Set(g.is_final),
            created_at: Set(g.created_at),
            updated_at: Set(g.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_grade(&self, active: grade::ActiveModel) -> Result<grade::Model, DbErr> {
        active.update(&self.db).await
    }



    // ── Attendance Commands ─────────────────────────────────────

    pub async fn create_attendance(&self, att: attendance::Model) -> Result<attendance::Model, DbErr> {
        let active = attendance::ActiveModel {
            id: Set(att.id),
            org_id: Set(att.org_id),
            period_id: Set(att.period_id),
            enrollment_id: Set(att.enrollment_id),
            class_id: Set(att.class_id),
            date: Set(att.date),
            status: Set(att.status),
            check_in_time: Set(att.check_in_time),
            check_out_time: Set(att.check_out_time),
            lesson_slot: Set(att.lesson_slot),
            subject_name: Set(att.subject_name),
            recorded_by: Set(att.recorded_by),
            notes: Set(att.notes),
            excuse_type: Set(att.excuse_type),
            excuse_document: Set(att.excuse_document),
            is_excused: Set(att.is_excused),
            created_at: Set(att.created_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_attendance(&self, active: attendance::ActiveModel) -> Result<attendance::Model, DbErr> {
        active.update(&self.db).await
    }

    pub async fn create_attendance_daily(&self, ad: attendance_daily::Model) -> Result<attendance_daily::Model, DbErr> {
        let active = attendance_daily::ActiveModel {
            id: Set(ad.id),
            org_id: Set(ad.org_id),
            period_id: Set(ad.period_id),
            enrollment_id: Set(ad.enrollment_id),
            date: Set(ad.date),
            status: Set(ad.status),
            lessons: Set(ad.lessons),
            total_lessons: Set(ad.total_lessons),
            present_lessons: Set(ad.present_lessons),
            absent_lessons: Set(ad.absent_lessons),
            late_lessons: Set(ad.late_lessons),
            excused_lessons: Set(ad.excused_lessons),
            late_minutes: Set(ad.late_minutes),
            excuse_type: Set(ad.excuse_type),
            excuse_document: Set(ad.excuse_document),
            excuse_notes: Set(ad.excuse_notes),
            is_verified: Set(ad.is_verified),
            verified_by: Set(ad.verified_by),
            verified_at: Set(ad.verified_at),
            marked_by: Set(ad.marked_by),
            marked_at: Set(ad.marked_at),
            mark_method: Set(ad.mark_method),
            device_id: Set(ad.device_id),
            location: Set(ad.location),
            notes: Set(ad.notes),
            created_at: Set(ad.created_at),
            updated_at: Set(ad.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_attendance_daily(&self, active: attendance_daily::ActiveModel) -> Result<attendance_daily::Model, DbErr> {
        active.update(&self.db).await
    }

    pub async fn upsert_attendance_summary(&self, s: attendance_summary::Model) -> Result<attendance_summary::Model, DbErr> {
        // Since we don't have direct check, we can check if it exists or use save/insert
        let active = attendance_summary::ActiveModel {
            id: Set(s.id),
            org_id: Set(s.org_id),
            enrollment_id: Set(s.enrollment_id),
            period_id: Set(s.period_id),
            total_lessons: Set(s.total_lessons),
            present_lessons: Set(s.present_lessons),
            absent_lessons: Set(s.absent_lessons),
            late_lessons: Set(s.late_lessons),
            excused_lessons: Set(s.excused_lessons),
            late_minutes: Set(s.late_minutes),
            attendance_rate: Set(s.attendance_rate),
            created_at: Set(s.created_at),
            updated_at: Set(s.updated_at),
        };
        // In Sea-ORM, we can do insert/update. We can use insert for simplicity.
        active.insert(&self.db).await
    }
}
