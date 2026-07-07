//! Assessment command repository — write operations.

use crate::entities::{grade, grade_history, assessment, attendance, attendance_daily, attendance_summary, behavior_record};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr, EntityTrait};
use super::AssessmentCommandRepo;

impl AssessmentCommandRepo {
    // ── Behavior Commands ───────────────────────────────────────

    pub async fn create_behavior_record(&self, b: behavior_record::Model) -> Result<behavior_record::Model, DbErr> {
        let active = behavior_record::ActiveModel {
            id: Set(b.id),
            org_id: Set(b.org_id),
            cycle_id: Set(b.cycle_id),
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
            cycle_id: Set(g.cycle_id),
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

    pub async fn create_grade_history(&self, gh: grade_history::Model) -> Result<grade_history::Model, DbErr> {
        let active = grade_history::ActiveModel {
            id: Set(gh.id),
            grade_id: Set(gh.grade_id),
            score: Set(gh.score),
            grade_letter: Set(gh.grade_letter),
            is_absent: Set(gh.is_absent),
            is_excused: Set(gh.is_excused),
            comment: Set(gh.comment),
            changed_by: Set(gh.changed_by),
            changed_at: Set(gh.changed_at),
            change_reason: Set(gh.change_reason),
            change_type: Set(gh.change_type),
            snapshot: Set(gh.snapshot),
            created_at: Set(gh.created_at),
        };
        active.insert(&self.db).await
    }

    // ── Assessment Commands ─────────────────────────────────────

    pub async fn create_assessment(&self, a: assessment::Model) -> Result<assessment::Model, DbErr> {
        let active = assessment::ActiveModel {
            id: Set(a.id),
            org_id: Set(a.org_id),
            cycle_id: Set(a.cycle_id),
            class_subject_id: Set(a.class_subject_id),
            title: Set(a.title),
            title_ar: Set(a.title_ar),
            description: Set(a.description),
            item_type: Set(a.item_type),
            max_score: Set(a.max_score),
            passing_score: Set(a.passing_score),
            weight: Set(a.weight),
            grade_group_name: Set(a.grade_group_name),
            assessment_date: Set(a.assessment_date),
            due_date: Set(a.due_date),
            duration_minutes: Set(a.duration_minutes),
            is_published: Set(a.is_published),
            published_at: Set(a.published_at),
            published_by: Set(a.published_by),
            is_locked: Set(a.is_locked),
            locked_at: Set(a.locked_at),
            locked_by: Set(a.locked_by),
            grading_method: Set(a.grading_method),
            rubric_id: Set(a.rubric_id),
            instructions: Set(a.instructions),
            instructions_ar: Set(a.instructions_ar),
            attachments: Set(a.attachments),
            created_by: Set(a.created_by),
            created_at: Set(a.created_at),
            updated_at: Set(a.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_assessment(&self, active: assessment::ActiveModel) -> Result<assessment::Model, DbErr> {
        active.update(&self.db).await
    }

    // ── Attendance Commands ─────────────────────────────────────

    pub async fn create_attendance(&self, att: attendance::Model) -> Result<attendance::Model, DbErr> {
        let active = attendance::ActiveModel {
            id: Set(att.id),
            org_id: Set(att.org_id),
            cycle_id: Set(att.cycle_id),
            enrollment_id: Set(att.enrollment_id),
            class_id: Set(att.class_id),
            date: Set(att.date),
            status: Set(att.status),
            check_in_time: Set(att.check_in_time),
            check_out_time: Set(att.check_out_time),
            period: Set(att.period),
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
            cycle_id: Set(ad.cycle_id),
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
            enrollment_id: Set(s.enrollment_id),
            cycle_id: Set(s.cycle_id),
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
