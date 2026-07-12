//! Exam query repository — read operations.

use crate::entities::{
    exam_session, exam_subject, exam_schedule,
    seating_number, secret_number,
    exam_hall, exam_hall_allocation,
    exam_proctor, teacher_exam_availability,
    student_ordering,
};
use crate::params::{
    ExamSessionParams, ExamScheduleParams, ExamHallParams,
    SeatingNumberParams, SecretNumberParams, ExamProctorParams,
    TeacherExamAvailabilityParams,
};
use sea_orm::{entity::prelude::*, QueryFilter, QueryOrder, EntityTrait, ColumnTrait};
use super::ExamQueryRepo;

use svc_academic::entities::{enrollment, class};
use svc_organization::entities::academic_period;
use svc_iam::entities::user;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnrollmentOrderingRow {
    pub enrollment_id: String,
    pub student_id: String,
    pub class_id: String,
    pub level: String,
    pub division: Option<String>,
    pub student_name_ar: Option<String>,
    pub student_sex: Option<String>,
    pub academic_number: Option<String>,
}

impl ExamQueryRepo {
    pub async fn get_enrollments_for_ordering(
        &self,
        org_id: &str,
        cycle_id: &str,
    ) -> Result<Vec<EnrollmentOrderingRow>, DbErr> {
        let mut list = enrollment::Entity::find()
            .filter(enrollment::Column::OrgId.eq(org_id))
            .filter(enrollment::Column::CycleId.eq(cycle_id))
            .filter(enrollment::Column::Status.eq("active"))
            .all(&self.db)
            .await?;

        if list.is_empty() {
            if let Some(cycle) = academic_period::Entity::find_by_id(cycle_id.to_string()).one(&self.db).await? {
                if let Some(parent_id) = cycle.parent_id {
                    if let Ok(parent_list) = enrollment::Entity::find()
                        .filter(enrollment::Column::OrgId.eq(org_id))
                        .filter(enrollment::Column::CycleId.eq(parent_id))
                        .filter(enrollment::Column::Status.eq("active"))
                        .all(&self.db)
                        .await
                    {
                        list = parent_list;
                    }
                }
            }
        }

        let mut rows = Vec::with_capacity(list.len());
        for enr in &list {
            let student = user::Entity::find_by_id(enr.student_id.clone())
                .one(&self.db)
                .await?;
            let cls = class::Entity::find_by_id(enr.class_id.clone())
                .one(&self.db)
                .await?;

            if let (Some(s), Some(c)) = (student, cls) {
                rows.push(EnrollmentOrderingRow {
                    enrollment_id: enr.id.clone(),
                    student_id: s.id.clone(),
                    class_id: c.id.clone(),
                    level: c.level.clone(),
                    division: c.division.clone(),
                    student_name_ar: s.name_ar.clone(),
                    student_sex: s.sex.clone(),
                    academic_number: Some(enr.student_number.clone()),
                });
            }
        }
        Ok(rows)
    }

    // ── Student Ordering ──

    pub async fn get_student_ordering(&self, org_id: &str, cycle_id: &str, class_id: Option<&str>) -> Result<Vec<student_ordering::Model>, DbErr> {
        let mut query = student_ordering::Entity::find()
            .filter(student_ordering::Column::OrgId.eq(org_id))
            .filter(student_ordering::Column::CycleId.eq(cycle_id));

        if let Some(cid) = class_id {
            query = query.filter(student_ordering::Column::ClassId.eq(cid));
        }

        query.order_by_asc(student_ordering::Column::OrderInClass).all(&self.db).await
    }

    pub async fn get_student_order_by_enrollment(&self, enrollment_id: &str) -> Result<Option<student_ordering::Model>, DbErr> {
        student_ordering::Entity::find()
            .filter(student_ordering::Column::EnrollmentId.eq(enrollment_id))
            .one(&self.db)
            .await
    }

    // ── Exam Sessions ──

    pub async fn get_exam_sessions(&self, org_id: &str, params: ExamSessionParams) -> Result<Vec<exam_session::Model>, DbErr> {
        let mut query = exam_session::Entity::find()
            .filter(exam_session::Column::OrgId.eq(org_id));

        if let Some(ref cid) = params.cycle_id {
            query = query.filter(exam_session::Column::CycleId.eq(cid));
        }
        if let Some(ref status) = params.status {
            query = query.filter(exam_session::Column::Status.eq(status));
        }

        query.order_by_desc(exam_session::Column::CreatedAt).all(&self.db).await
    }

    pub async fn get_exam_session_by_id(&self, id: &str) -> Result<Option<exam_session::Model>, DbErr> {
        exam_session::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    // ── Exam Subjects ──

    pub async fn get_exam_subjects(&self, exam_session_id: &str) -> Result<Vec<exam_subject::Model>, DbErr> {
        exam_subject::Entity::find()
            .filter(exam_subject::Column::ExamSessionId.eq(exam_session_id))
            .order_by_asc(exam_subject::Column::SortOrder)
            .all(&self.db)
            .await
    }

    pub async fn get_exam_subject_by_id(&self, id: &str) -> Result<Option<exam_subject::Model>, DbErr> {
        exam_subject::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    // ── Exam Schedules ──

    pub async fn get_exam_schedules(&self, exam_session_id: &str, params: ExamScheduleParams) -> Result<Vec<exam_schedule::Model>, DbErr> {
        let mut query = exam_schedule::Entity::find()
            .filter(exam_schedule::Column::ExamSessionId.eq(exam_session_id));

        if let Some(ref exam_date) = params.exam_date {
            query = query.filter(exam_schedule::Column::ExamDate.eq(exam_date));
        }
        if let Some(ref status) = params.status {
            query = query.filter(exam_schedule::Column::Status.eq(status));
        }

        query.order_by_asc(exam_schedule::Column::ExamDate)
            .order_by_asc(exam_schedule::Column::StartTime)
            .all(&self.db)
            .await
    }

    pub async fn get_exam_schedule_by_id(&self, id: &str) -> Result<Option<exam_schedule::Model>, DbErr> {
        exam_schedule::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    // ── Seating Numbers ──

    pub async fn get_seating_numbers(&self, exam_session_id: &str, params: SeatingNumberParams) -> Result<Vec<seating_number::Model>, DbErr> {
        let mut query = seating_number::Entity::find()
            .filter(seating_number::Column::ExamSessionId.eq(exam_session_id));

        if let Some(ref class_id) = params.class_id {
            query = query.filter(seating_number::Column::ClassId.eq(class_id));
        }
        if let Some(active) = params.is_active {
            query = query.filter(seating_number::Column::IsActive.eq(active));
        }

        query.order_by_asc(seating_number::Column::SequenceNumber).all(&self.db).await
    }

    // ── Secret Numbers ──

    pub async fn get_secret_numbers(&self, exam_session_id: &str, params: SecretNumberParams) -> Result<Vec<secret_number::Model>, DbErr> {
        let mut query = secret_number::Entity::find()
            .filter(secret_number::Column::ExamSessionId.eq(exam_session_id));

        if let Some(active) = params.is_active {
            query = query.filter(secret_number::Column::IsActive.eq(active));
        }
        if let Some(revealed) = params.is_revealed {
            query = query.filter(secret_number::Column::IsRevealed.eq(revealed));
        }

        query.all(&self.db).await
    }

    pub async fn lookup_by_secret(&self, exam_session_id: &str, secret_hash: &str) -> Result<Option<secret_number::Model>, DbErr> {
        secret_number::Entity::find()
            .filter(secret_number::Column::ExamSessionId.eq(exam_session_id))
            .filter(secret_number::Column::SecretHash.eq(secret_hash))
            .filter(secret_number::Column::IsActive.eq(1))
            .one(&self.db)
            .await
    }

    // ── Exam Halls ──

    pub async fn get_exam_halls(&self, exam_session_id: &str, params: ExamHallParams) -> Result<Vec<exam_hall::Model>, DbErr> {
        let mut query = exam_hall::Entity::find()
            .filter(exam_hall::Column::ExamSessionId.eq(exam_session_id))
            .filter(exam_hall::Column::Status.eq("active"));

        if let Some(ref rid) = params.room_id {
            query = query.filter(exam_hall::Column::RoomId.eq(rid));
        }

        query.order_by_asc(exam_hall::Column::HallNumber).all(&self.db).await
    }

    pub async fn get_exam_hall_by_id(&self, id: &str) -> Result<Option<exam_hall::Model>, DbErr> {
        exam_hall::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    // ── Exam Hall Allocations ──

    pub async fn get_hall_allocations(&self, exam_session_id: &str, hall_id: Option<&str>) -> Result<Vec<exam_hall_allocation::Model>, DbErr> {
        let mut query = exam_hall_allocation::Entity::find()
            .filter(exam_hall_allocation::Column::ExamSessionId.eq(exam_session_id));

        if let Some(hid) = hall_id {
            query = query.filter(exam_hall_allocation::Column::ExamHallId.eq(hid));
        }

        query.all(&self.db).await
    }

    // ── Exam Proctors ──

    pub async fn get_exam_proctors(&self, exam_session_id: &str, params: ExamProctorParams) -> Result<Vec<exam_proctor::Model>, DbErr> {
        let mut query = exam_proctor::Entity::find()
            .filter(exam_proctor::Column::ExamSessionId.eq(exam_session_id));

        if let Some(ref schedule_id) = params.exam_schedule_id {
            query = query.filter(exam_proctor::Column::ExamScheduleId.eq(schedule_id));
        }
        if let Some(ref tid) = params.teacher_id {
            query = query.filter(exam_proctor::Column::TeacherId.eq(tid));
        }

        query.all(&self.db).await
    }

    // ── Teacher Exam Availability ──

    pub async fn get_teacher_exam_availability(&self, exam_session_id: &str, params: TeacherExamAvailabilityParams) -> Result<Vec<teacher_exam_availability::Model>, DbErr> {
        let mut query = teacher_exam_availability::Entity::find()
            .filter(teacher_exam_availability::Column::ExamSessionId.eq(exam_session_id));

        if let Some(ref tid) = params.teacher_id {
            query = query.filter(teacher_exam_availability::Column::TeacherId.eq(tid));
        }
        if let Some(ref date) = params.date {
            query = query.filter(teacher_exam_availability::Column::Date.eq(date));
        }

        query.all(&self.db).await
    }

    pub async fn get_available_teachers_for_date(&self, org_id: &str, exam_session_id: &str, date: &str) -> Result<Vec<String>, DbErr> {
        let teachers = user::Entity::find()
            .filter(user::Column::OrgId.eq(org_id))
            .filter(user::Column::PrimaryRole.eq("teacher"))
            .filter(user::Column::Status.eq("active"))
            .filter(user::Column::IsDeleted.eq(0))
            .all(&self.db)
            .await?;

        let teacher_ids: Vec<String> = teachers.iter().map(|t| t.id.clone()).collect();

        let unavailable = teacher_exam_availability::Entity::find()
            .filter(teacher_exam_availability::Column::ExamSessionId.eq(exam_session_id))
            .filter(teacher_exam_availability::Column::Date.eq(date))
            .filter(teacher_exam_availability::Column::IsAvailable.eq(0))
            .all(&self.db)
            .await?;

        let unavailable_ids: std::collections::HashSet<String> =
            unavailable.iter().map(|u| u.teacher_id.clone()).collect();

        Ok(teacher_ids
            .into_iter()
            .filter(|id| !unavailable_ids.contains(id))
            .collect())
    }

    // ── Solve Jobs ──

    pub async fn get_solve_job_by_id(&self, id: &str) -> Result<Option<svc_scheduling::entities::solve_job::Model>, DbErr> {
        svc_scheduling::entities::solve_job::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    pub async fn get_solve_jobs(
        &self,
        org_id: &str,
        job_type: &str,
        target_id: Option<&str>,
    ) -> Result<Vec<svc_scheduling::entities::solve_job::Model>, DbErr> {
        let mut query = svc_scheduling::entities::solve_job::Entity::find()
            .filter(svc_scheduling::entities::solve_job::Column::OrgId.eq(org_id))
            .filter(svc_scheduling::entities::solve_job::Column::JobType.eq(job_type))
            .order_by_desc(svc_scheduling::entities::solve_job::Column::CreatedAt);

        if let Some(tgt) = target_id {
            query = query.filter(svc_scheduling::entities::solve_job::Column::TargetId.eq(tgt));
        }

        query.all(&self.db).await
    }

    pub async fn count_active_solve_jobs(&self, org_id: &str, job_type: &str) -> Result<u64, DbErr> {
        svc_scheduling::entities::solve_job::Entity::find()
            .filter(svc_scheduling::entities::solve_job::Column::OrgId.eq(org_id))
            .filter(svc_scheduling::entities::solve_job::Column::JobType.eq(job_type))
            .filter(svc_scheduling::entities::solve_job::Column::Status.is_in(["pending", "running"]))
            .count(&self.db)
            .await
    }
}
