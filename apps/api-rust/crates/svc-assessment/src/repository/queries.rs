//! Assessment query repository — read operations.

use crate::entities::{grade, assessment, attendance, behavior_record};
use crate::params::{BehaviorParams, GradeParams, AttendanceParams};
use svc_academic::entities::enrollment;
use sea_orm::{entity::prelude::*, QueryFilter, QueryOrder, QuerySelect, EntityTrait, ColumnTrait};

use super::AssessmentQueryRepo;

impl AssessmentQueryRepo {
    // ── Behavior Queries ────────────────────────────────────────

    pub async fn get_behavior_record_by_id(&self, id: &str) -> Result<Option<behavior_record::Model>, DbErr> {
        behavior_record::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    pub async fn get_behavior_records(
        &self,
        org_id: &str,
        params: BehaviorParams,
    ) -> Result<Vec<behavior_record::Model>, DbErr> {
        let mut query = behavior_record::Entity::find()
            .filter(behavior_record::Column::OrgId.eq(org_id));

        if let Some(ref student_id) = params.student_id {
            if !student_id.trim().is_empty() {
                let enrollment_ids = enrollment::Entity::find()
                    .filter(enrollment::Column::StudentId.eq(student_id))
                    .all(&self.db)
                    .await?
                    .into_iter()
                    .map(|e| e.id)
                    .collect::<Vec<String>>();
                query = query.filter(behavior_record::Column::EnrollmentId.is_in(enrollment_ids));
            }
        }

        if let Some(ref severity) = params.severity {
            if !severity.trim().is_empty() {
                query = query.filter(behavior_record::Column::Severity.eq(severity));
            }
        }

        if let Some(ref status) = params.status {
            if !status.trim().is_empty() {
                query = query.filter(behavior_record::Column::Status.eq(status));
            }
        }

        if let Some(ref search) = params.q {
            if !search.trim().is_empty() {
                let s = format!("%{}%", search);
                query = query.filter(
                    behavior_record::Column::Title.like(&s)
                        .or(behavior_record::Column::Description.like(&s))
                        .or(behavior_record::Column::ReportedBy.like(&s))
                );
            }
        }

        query = query.order_by_desc(behavior_record::Column::CreatedAt);

        if let Some(limit) = params.limit {
            query = query.limit(limit);
        }
        if let Some(offset) = params.offset {
            query = query.offset(offset);
        }

        query.all(&self.db).await
    }

    // ── Grade Queries ───────────────────────────────────────────

    pub async fn get_grade_by_id(&self, id: &str) -> Result<Option<grade::Model>, DbErr> {
        grade::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    pub async fn get_grades(
        &self,
        org_id: &str,
        params: GradeParams,
    ) -> Result<Vec<grade::Model>, DbErr> {
        let mut query = grade::Entity::find()
            .filter(grade::Column::OrgId.eq(org_id));

        if let Some(ref enrollment_id) = params.enrollment_id {
            if !enrollment_id.trim().is_empty() {
                query = query.filter(grade::Column::EnrollmentId.eq(enrollment_id));
            }
        }

        if let Some(ref period_id) = params.period_id {
            if !period_id.trim().is_empty() {
                query = query.filter(grade::Column::PeriodId.eq(period_id));
            }
        }

        if let Some(ref subject_name) = params.subject_name {
            if !subject_name.trim().is_empty() {
                query = query.filter(grade::Column::SubjectName.eq(subject_name));
            }
        }

        if let Some(ref exam_type) = params.exam_type {
            if !exam_type.trim().is_empty() {
                query = query.filter(grade::Column::ExamType.eq(exam_type));
            }
        }

        if let Some(ref semester) = params.semester {
            if !semester.trim().is_empty() {
                query = query.filter(grade::Column::Semester.eq(semester));
            }
        }

        if let Some(is_final) = params.is_final {
            query = query.filter(grade::Column::IsFinal.eq(is_final));
        }

        if let Some(ref assessment_id) = params.assessment_id {
            if !assessment_id.trim().is_empty() {
                query = query.filter(grade::Column::AssessmentId.eq(assessment_id));
            }
        }

        if let Some(ref class_subject_id) = params.class_subject_id {
            if !class_subject_id.trim().is_empty() {
                query = query.filter(grade::Column::ClassSubjectId.eq(class_subject_id));
            }
        }

        if let Some(ref class_id) = params.class_id {
            if !class_id.trim().is_empty() {
                let enrollment_ids: Vec<String> = enrollment::Entity::find()
                    .filter(enrollment::Column::ClassId.eq(class_id))
                    .all(&self.db)
                    .await?
                    .into_iter()
                    .map(|e| e.id)
                    .collect();
                query = query.filter(grade::Column::EnrollmentId.is_in(enrollment_ids));
            }
        }

        query = query.order_by_desc(grade::Column::CreatedAt);

        if let Some(limit) = params.limit {
            query = query.limit(limit);
        }
        if let Some(offset) = params.offset {
            query = query.offset(offset);
        }

        query.all(&self.db).await
    }

    pub async fn get_grades_by_enrollment(&self, enrollment_id: &str) -> Result<Vec<grade::Model>, DbErr> {
        grade::Entity::find()
            .filter(grade::Column::EnrollmentId.eq(enrollment_id))
            .order_by_asc(grade::Column::SubjectName)
            .all(&self.db)
            .await
    }

    pub async fn get_grades_by_enrollment_final(&self, enrollment_id: &str) -> Result<Vec<grade::Model>, DbErr> {
        grade::Entity::find()
            .filter(grade::Column::EnrollmentId.eq(enrollment_id))
            .filter(grade::Column::IsFinal.eq(1))
            .order_by_asc(grade::Column::SubjectName)
            .all(&self.db)
            .await
    }

    pub async fn get_grades_by_assessment(&self, assessment_id: &str) -> Result<Vec<grade::Model>, DbErr> {
        grade::Entity::find()
            .filter(grade::Column::AssessmentId.eq(assessment_id))
            .order_by_asc(grade::Column::EnrollmentId)
            .all(&self.db)
            .await
    }

    pub async fn get_grade_matrix(
        &self,
        class_subject_id: &str,
        period_id: &str,
    ) -> Result<Vec<grade::Model>, DbErr> {
        grade::Entity::find()
            .filter(grade::Column::ClassSubjectId.eq(class_subject_id))
            .filter(grade::Column::PeriodId.eq(period_id))
            .order_by_asc(grade::Column::EnrollmentId)
            .order_by_asc(grade::Column::ExamType)
            .all(&self.db)
            .await
    }

    pub async fn find_grade_by_enrollment_assessment(
        &self,
        enrollment_id: &str,
        assessment_id: &str,
    ) -> Result<Option<grade::Model>, DbErr> {
        grade::Entity::find()
            .filter(grade::Column::EnrollmentId.eq(enrollment_id))
            .filter(grade::Column::AssessmentId.eq(assessment_id))
            .one(&self.db)
            .await
    }

    // ── Assessment Queries ──────────────────────────────────────

    pub async fn get_assessments(
        &self,
        class_subject_id: &str,
        period_id: &str,
    ) -> Result<Vec<assessment::Model>, DbErr> {
        assessment::Entity::find()
            .filter(assessment::Column::ClassSubjectId.eq(class_subject_id))
            .filter(assessment::Column::PeriodId.eq(period_id))
            .all(&self.db)
            .await
    }

    pub async fn get_assessment_by_id(&self, id: &str) -> Result<Option<assessment::Model>, DbErr> {
        assessment::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    // ── Attendance Queries ──────────────────────────────────────

    pub async fn get_attendance(
        &self,
        org_id: &str,
        params: AttendanceParams,
    ) -> Result<Vec<attendance::Model>, DbErr> {
        let mut query = attendance::Entity::find()
            .filter(attendance::Column::OrgId.eq(org_id));

        if let Some(ref enrollment_id) = params.enrollment_id {
            if !enrollment_id.trim().is_empty() {
                query = query.filter(attendance::Column::EnrollmentId.eq(enrollment_id));
            }
        }

        if let Some(ref class_id) = params.class_id {
            if !class_id.trim().is_empty() {
                query = query.filter(attendance::Column::ClassId.eq(class_id));
            }
        }

        if let Some(ref period_id) = params.period_id {
            if !period_id.trim().is_empty() {
                query = query.filter(attendance::Column::PeriodId.eq(period_id));
            }
        }

        if let Some(ref date) = params.date {
            if !date.trim().is_empty() {
                query = query.filter(attendance::Column::Date.eq(date));
            }
        }

        if let Some(ref date_from) = params.date_from {
            if !date_from.trim().is_empty() {
                query = query.filter(attendance::Column::Date.gte(date_from));
            }
        }

        if let Some(ref date_to) = params.date_to {
            if !date_to.trim().is_empty() {
                query = query.filter(attendance::Column::Date.lte(date_to));
            }
        }

        if let Some(ref status) = params.status {
            if !status.trim().is_empty() {
                query = query.filter(attendance::Column::Status.eq(status));
            }
        }

        query = query.order_by_desc(attendance::Column::Date);

        if let Some(limit) = params.limit {
            query = query.limit(limit);
        }
        if let Some(offset) = params.offset {
            query = query.offset(offset);
        }

        query.all(&self.db).await
    }

    pub async fn get_attendance_by_enrollment(&self, enrollment_id: &str) -> Result<Vec<attendance::Model>, DbErr> {
        attendance::Entity::find()
            .filter(attendance::Column::EnrollmentId.eq(enrollment_id))
            .order_by_desc(attendance::Column::Date)
            .all(&self.db)
            .await
    }

    pub async fn get_attendance_by_class_date(&self, class_id: &str, date: &str) -> Result<Vec<attendance::Model>, DbErr> {
        attendance::Entity::find()
            .filter(attendance::Column::ClassId.eq(class_id))
            .filter(attendance::Column::Date.eq(date))
            .all(&self.db)
            .await
    }

    pub async fn get_attendance_count_by_status(&self, enrollment_id: &str, status: &str) -> Result<u64, DbErr> {
        attendance::Entity::find()
            .filter(attendance::Column::EnrollmentId.eq(enrollment_id))
            .filter(attendance::Column::Status.eq(status))
            .count(&self.db)
            .await
    }

    pub async fn get_attendance_total(&self, enrollment_id: &str) -> Result<u64, DbErr> {
        attendance::Entity::find()
            .filter(attendance::Column::EnrollmentId.eq(enrollment_id))
            .count(&self.db)
            .await
    }
}
