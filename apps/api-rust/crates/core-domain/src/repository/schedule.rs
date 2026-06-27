use crate::entities::{time_slot, class_subject, scheduled_lesson, room, teacher_availability};
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait, QueryOrder, sea_query::Expr};

#[derive(Debug, Clone, Default)]
pub struct RoomUpdate {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub capacity: Option<i32>,
    pub room_type: Option<String>,
    pub building: Option<String>,
    pub floor: Option<i32>,
    pub is_active: Option<i32>,
}

impl super::Repository {
    // ── Time Slots ───────────────────────────────────────────────────────────

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

    pub async fn insert_time_slot(&self, slot: time_slot::Model) -> Result<time_slot::Model, DbErr> {
        let active = time_slot::ActiveModel {
            id: Set(slot.id),
            org_id: Set(slot.org_id),
            cycle_id: Set(slot.cycle_id),
            slot_number: Set(slot.slot_number),
            name: Set(slot.name),
            name_ar: Set(slot.name_ar),
            start_time: Set(slot.start_time),
            end_time: Set(slot.end_time),
            duration_minutes: Set(slot.duration_minutes),
            is_break: Set(slot.is_break),
            break_type: Set(slot.break_type),
            applicable_days: Set(slot.applicable_days),
            sort_order: Set(slot.sort_order),
            is_active: Set(slot.is_active),
            created_at: Set(slot.created_at),
            updated_at: Set(slot.updated_at),
        };
        active.insert(&self.db).await
    }

    // ── Class Subjects ───────────────────────────────────────────────────────

    pub async fn get_class_subjects(
        &self,
        class_id: Option<&str>,
        cycle_id: Option<&str>,
    ) -> Result<Vec<class_subject::Model>, DbErr> {
        let mut query = class_subject::Entity::find()
            .filter(class_subject::Column::IsActive.eq(1));

        if let Some(cid) = class_id {
            query = query.filter(class_subject::Column::ClassId.eq(cid));
        }
        if let Some(cyc) = cycle_id {
            query = query.filter(class_subject::Column::CycleId.eq(cyc));
        }

        query.all(&self.db).await
    }

    pub async fn get_class_subject_by_id(&self, id: &str) -> Result<Option<class_subject::Model>, DbErr> {
        class_subject::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    pub async fn find_class_subject(
        &self,
        class_id: &str,
        subject_id: &str,
        cycle_id: &str,
    ) -> Result<Option<class_subject::Model>, DbErr> {
        class_subject::Entity::find()
            .filter(class_subject::Column::ClassId.eq(class_id))
            .filter(class_subject::Column::SubjectId.eq(subject_id))
            .filter(class_subject::Column::CycleId.eq(cycle_id))
            .one(&self.db)
            .await
    }

    pub async fn insert_class_subject(&self, cs: class_subject::Model) -> Result<class_subject::Model, DbErr> {
        let active = class_subject::ActiveModel {
            id: Set(cs.id),
            class_id: Set(cs.class_id),
            subject_id: Set(cs.subject_id),
            cycle_id: Set(cs.cycle_id),
            primary_teacher_id: Set(cs.primary_teacher_id),
            assistant_teacher_id: Set(cs.assistant_teacher_id),
            substitute_teacher_id: Set(cs.substitute_teacher_id),
            grade_groups: Set(cs.grade_groups),
            weekly_hours: Set(cs.weekly_hours),
            theory_hours: Set(cs.theory_hours),
            practical_hours: Set(cs.practical_hours),
            assessment_config: Set(cs.assessment_config),
            is_active: Set(cs.is_active),
            created_at: Set(cs.created_at),
            updated_at: Set(cs.updated_at),
        };
        active.insert(&self.db).await
    }

    // ── Scheduled Lessons ────────────────────────────────────────────────────

    pub async fn get_scheduled_lessons(
        &self,
        org_id: &str,
        cycle_id: &str,
        class_id: Option<&str>,
        teacher_id: Option<&str>,
    ) -> Result<Vec<scheduled_lesson::Model>, DbErr> {
        let mut query = scheduled_lesson::Entity::find()
            .filter(scheduled_lesson::Column::OrgId.eq(org_id))
            .filter(scheduled_lesson::Column::CycleId.eq(cycle_id))
            .filter(scheduled_lesson::Column::Status.ne("cancelled"));

        if let Some(cid) = class_id {
            query = query.filter(scheduled_lesson::Column::ClassId.eq(cid));
        }
        if let Some(tid) = teacher_id {
            query = query.filter(scheduled_lesson::Column::TeacherId.eq(tid));
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

    pub async fn insert_scheduled_lesson(
        &self,
        lesson: scheduled_lesson::Model,
    ) -> Result<scheduled_lesson::Model, DbErr> {
        let active = scheduled_lesson::ActiveModel {
            id: Set(lesson.id),
            org_id: Set(lesson.org_id),
            cycle_id: Set(lesson.cycle_id),
            class_id: Set(lesson.class_id),
            class_subject_id: Set(lesson.class_subject_id),
            subject_id: Set(lesson.subject_id),
            teacher_id: Set(lesson.teacher_id),
            substitute_teacher_id: Set(lesson.substitute_teacher_id),
            time_slot_id: Set(lesson.time_slot_id),
            day_of_week: Set(lesson.day_of_week),
            room_id: Set(lesson.room_id),
            room_name: Set(lesson.room_name),
            status: Set(lesson.status),
            is_substituted: Set(lesson.is_substituted),
            substitution_reason: Set(lesson.substitution_reason),
            topic: Set(lesson.topic),
            topic_ar: Set(lesson.topic_ar),
            objectives: Set(lesson.objectives),
            materials: Set(lesson.materials),
            homework: Set(lesson.homework),
            homework_due_date: Set(lesson.homework_due_date),
            attendance_taken: Set(lesson.attendance_taken),
            attendance_taken_at: Set(lesson.attendance_taken_at),
            attendance_taken_by: Set(lesson.attendance_taken_by),
            attendance_method: Set(lesson.attendance_method),
            notes: Set(lesson.notes),
            cancellation_reason: Set(lesson.cancellation_reason),
            created_at: Set(lesson.created_at),
            updated_at: Set(lesson.updated_at),
        };
        active.insert(&self.db).await
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

    pub async fn cancel_lessons_for_cycle(
        &self,
        org_id: &str,
        cycle_id: &str,
    ) -> Result<u64, DbErr> {
        use sea_orm::{ColumnTrait, QueryFilter};
        let now = chrono::Utc::now().to_rfc3339();
        let result = scheduled_lesson::Entity::update_many()
            .col_expr(scheduled_lesson::Column::Status, Expr::value("cancelled"))
            .col_expr(scheduled_lesson::Column::UpdatedAt, Expr::value(now))
            .filter(scheduled_lesson::Column::OrgId.eq(org_id))
            .filter(scheduled_lesson::Column::CycleId.eq(cycle_id))
            .filter(scheduled_lesson::Column::Status.ne("cancelled"))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    pub async fn get_class_subjects_for_classes(
        &self,
        class_ids: &[String],
        cycle_id: &str,
    ) -> Result<Vec<class_subject::Model>, DbErr> {
        if class_ids.is_empty() {
            return Ok(vec![]);
        }
        class_subject::Entity::find()
            .filter(class_subject::Column::CycleId.eq(cycle_id))
            .filter(class_subject::Column::IsActive.eq(1))
            .filter(class_subject::Column::ClassId.is_in(class_ids.to_vec()))
            .all(&self.db)
            .await
    }

    // ── Rooms (canonical scheduling resource — see schema comment) ───────────

    pub async fn get_rooms(&self, org_id: &str) -> Result<Vec<room::Model>, DbErr> {
        room::Entity::find()
            .filter(room::Column::OrgId.eq(org_id))
            .filter(room::Column::IsActive.eq(1))
            .order_by_asc(room::Column::Name)
            .all(&self.db)
            .await
    }

    pub async fn get_room_by_id(&self, id: &str) -> Result<Option<room::Model>, DbErr> {
        room::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    pub async fn insert_room(&self, model: room::Model) -> Result<room::Model, DbErr> {
        let active = room::ActiveModel {
            id: Set(model.id),
            org_id: Set(model.org_id),
            name: Set(model.name),
            name_ar: Set(model.name_ar),
            capacity: Set(model.capacity),
            room_type: Set(model.room_type),
            building: Set(model.building),
            floor: Set(model.floor),
            is_active: Set(model.is_active),
            created_at: Set(model.created_at),
            updated_at: Set(model.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_room(&self, id: &str, req: RoomUpdate) -> Result<room::Model, DbErr> {
        let existing = room::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("room not found".to_string()))?;

        let mut active: room::ActiveModel = existing.into();
        if let Some(v) = req.name { active.name = Set(v); }
        if let Some(v) = req.name_ar { active.name_ar = Set(Some(v)); }
        if let Some(v) = req.capacity { active.capacity = Set(Some(v)); }
        if let Some(v) = req.room_type { active.room_type = Set(v); }
        if let Some(v) = req.building { active.building = Set(Some(v)); }
        if let Some(v) = req.floor { active.floor = Set(Some(v)); }
        if let Some(v) = req.is_active { active.is_active = Set(v); }
        active.updated_at = Set(chrono::Utc::now().to_rfc3339());
        active.update(&self.db).await
    }

    pub async fn soft_delete_room(&self, id: &str) -> Result<(), DbErr> {
        let existing = room::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("room not found".to_string()))?;

        let mut active: room::ActiveModel = existing.into();
        active.is_active = Set(0);
        active.updated_at = Set(chrono::Utc::now().to_rfc3339());
        active.update(&self.db).await?;
        Ok(())
    }

    // ── Teacher availability (unavailability records for solver) ───────────────

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

    // ── Solve jobs ───────────────────────────────────────────────────────────

    
}
