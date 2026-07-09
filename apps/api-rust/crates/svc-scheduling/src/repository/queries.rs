//! Scheduling query repository — read operations.

use crate::entities::{time_slot, scheduled_lesson, room, teacher_availability, solve_job};
use crate::params::{ScheduleParams, RoomParams};
use sea_orm::{entity::prelude::*, QueryFilter, QueryOrder, EntityTrait, ColumnTrait};
use super::SchedulingQueryRepo;

impl SchedulingQueryRepo {
    // ── Time Slots ──────────────────────────────────────────────

    pub async fn get_time_slots(
        &self,
        org_id: &str,
        cycle_id: &str,
    ) -> Result<Vec<time_slot::Model>, DbErr> {
        time_slot::Entity::find()
            .filter(time_slot::Column::OrgId.eq(org_id))
            .filter(time_slot::Column::CycleId.eq(cycle_id))
            .filter(time_slot::Column::IsActive.eq(1))
            .order_by_asc(time_slot::Column::SortOrder)
            .order_by_asc(time_slot::Column::SlotNumber)
            .all(&self.db)
            .await
    }

    pub async fn get_time_slot_by_id(&self, id: &str) -> Result<Option<time_slot::Model>, DbErr> {
        time_slot::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    // ── Scheduled Lessons ────────────────────────────────────────

    pub async fn get_scheduled_lessons(
        &self,
        org_id: &str,
        params: ScheduleParams,
    ) -> Result<Vec<scheduled_lesson::Model>, DbErr> {
        let mut query = scheduled_lesson::Entity::find()
            .filter(scheduled_lesson::Column::OrgId.eq(org_id))
            .filter(scheduled_lesson::Column::Status.ne("cancelled"));

        if let Some(ref cycle_id) = params.cycle_id {
            query = query.filter(scheduled_lesson::Column::CycleId.eq(cycle_id));
        }
        if let Some(ref class_id) = params.class_id {
            query = query.filter(scheduled_lesson::Column::ClassId.eq(class_id));
        }
        if let Some(ref teacher_id) = params.teacher_id {
            query = query.filter(scheduled_lesson::Column::TeacherId.eq(teacher_id));
        }

        query
            .order_by_asc(scheduled_lesson::Column::DayOfWeek)
            .order_by_asc(scheduled_lesson::Column::TimeSlotId)
            .all(&self.db)
            .await
    }

    pub async fn get_scheduled_lesson_by_id(&self, id: &str) -> Result<Option<scheduled_lesson::Model>, DbErr> {
        scheduled_lesson::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    pub async fn find_lesson_conflicts(
        &self,
        org_id: &str,
        cycle_id: &str,
        time_slot_id: &str,
        day_of_week: i32,
        exclude_id: Option<&str>,
        teacher_id: &str,
        class_id: &str,
        room_id: Option<&str>,
    ) -> Result<Vec<scheduled_lesson::Model>, DbErr> {
        let mut query = scheduled_lesson::Entity::find()
            .filter(scheduled_lesson::Column::OrgId.eq(org_id))
            .filter(scheduled_lesson::Column::CycleId.eq(cycle_id))
            .filter(scheduled_lesson::Column::TimeSlotId.eq(time_slot_id))
            .filter(scheduled_lesson::Column::DayOfWeek.eq(day_of_week))
            .filter(scheduled_lesson::Column::Status.ne("cancelled"));

        if let Some(eid) = exclude_id {
            query = query.filter(scheduled_lesson::Column::Id.ne(eid));
        }

        let lessons = query.all(&self.db).await?;
        let mut conflicts = Vec::new();

        for lesson in lessons {
            if lesson.teacher_id == teacher_id {
                conflicts.push(lesson.clone());
                continue;
            }
            if lesson.class_id == class_id {
                conflicts.push(lesson.clone());
                continue;
            }
            if let Some(rid) = room_id {
                if lesson.room_id.as_deref() == Some(rid) {
                    conflicts.push(lesson);
                }
            }
        }

        Ok(conflicts)
    }

    pub async fn get_all_scheduled_lessons_for_cycle(
        &self,
        org_id: &str,
        cycle_id: &str,
    ) -> Result<Vec<scheduled_lesson::Model>, DbErr> {
        scheduled_lesson::Entity::find()
            .filter(scheduled_lesson::Column::OrgId.eq(org_id))
            .filter(scheduled_lesson::Column::CycleId.eq(cycle_id))
            .filter(scheduled_lesson::Column::Status.ne("cancelled"))
            .order_by_asc(scheduled_lesson::Column::DayOfWeek)
            .order_by_asc(scheduled_lesson::Column::TimeSlotId)
            .all(&self.db)
            .await
    }

    // ── Rooms ───────────────────────────────────────────────────

    pub async fn get_rooms(&self, org_id: &str, params: RoomParams) -> Result<Vec<room::Model>, DbErr> {
        let mut query = room::Entity::find()
            .filter(room::Column::OrgId.eq(org_id));

        if let Some(active) = params.is_active {
            query = query.filter(room::Column::IsActive.eq(active));
        }
        if let Some(ref building) = params.building {
            query = query.filter(room::Column::Building.eq(building));
        }
        if let Some(ref rtype) = params.room_type {
            query = query.filter(room::Column::RoomType.eq(rtype));
        }

        query.order_by_asc(room::Column::Name).all(&self.db).await
    }

    pub async fn get_room_by_id(&self, id: &str) -> Result<Option<room::Model>, DbErr> {
        room::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    // ── Teacher Availability ────────────────────────────────────

    pub async fn get_teacher_unavailability(
        &self,
        org_id: &str,
        cycle_id: &str,
    ) -> Result<Vec<teacher_availability::Model>, DbErr> {
        teacher_availability::Entity::find()
            .filter(teacher_availability::Column::OrgId.eq(org_id))
            .filter(teacher_availability::Column::CycleId.eq(cycle_id))
            .filter(teacher_availability::Column::IsAvailable.eq(0))
            .all(&self.db)
            .await
    }

    // ── Solve Jobs ──────────────────────────────────────────────

    pub async fn get_solve_job_by_id(&self, id: &str) -> Result<Option<solve_job::Model>, DbErr> {
        solve_job::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    pub async fn get_solve_jobs(
        &self,
        org_id: &str,
        job_type: &str,
        target_id: Option<&str>,
    ) -> Result<Vec<solve_job::Model>, DbErr> {
        let mut query = solve_job::Entity::find()
            .filter(solve_job::Column::OrgId.eq(org_id))
            .filter(solve_job::Column::JobType.eq(job_type))
            .order_by_desc(solve_job::Column::CreatedAt);

        if let Some(tgt) = target_id {
            query = query.filter(solve_job::Column::TargetId.eq(tgt));
        }

        query.all(&self.db).await
    }

    pub async fn count_active_solve_jobs(&self, org_id: &str, job_type: &str) -> Result<u64, DbErr> {
        solve_job::Entity::find()
            .filter(solve_job::Column::OrgId.eq(org_id))
            .filter(solve_job::Column::JobType.eq(job_type))
            .filter(solve_job::Column::Status.is_in(["pending", "running"]))
            .count(&self.db)
            .await
    }
}
