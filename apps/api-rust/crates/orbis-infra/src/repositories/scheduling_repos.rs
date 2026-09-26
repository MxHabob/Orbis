use async_trait::async_trait;
use orbis_app::ports::repositories::{RoomRepository, TimeSlotRepository, ScheduledLessonRepository};
use orbis_domain::entities::room::Room as DomainRoom;
use orbis_domain::entities::time_slot::TimeSlot as DomainTimeSlot;
use orbis_domain::entities::scheduled_lesson::ScheduledLesson as DomainScheduledLesson;
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::models::{
    room::{self, Entity as RoomEntity},
    time_slot::{self, Entity as TimeSlotEntity},
    scheduled_lesson::{self, Entity as ScheduledLessonEntity},
};

pub struct SchedulingRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl RoomRepository for SchedulingRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainRoom>, DomainError> {
        let model = RoomEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainRoom {
            id: m.id, org_id: m.org_id, name: m.name, capacity: m.capacity,
            room_type: m.room_type, org_unit_id: m.org_unit_id, created_at: m.created_at, updated_at: m.updated_at
        }))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainRoom>, DomainError> {
        let models = RoomEntity::find().filter(room::Column::OrgId.eq(org_id)).all(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainRoom {
            id: m.id, org_id: m.org_id, name: m.name, capacity: m.capacity,
            room_type: m.room_type, org_unit_id: m.org_unit_id, created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }

    async fn save(&self, room: &DomainRoom) -> Result<(), DomainError> {
        let active = room::ActiveModel {
            id: Set(room.id), org_id: Set(room.org_id), name: Set(room.name.clone()),
            capacity: Set(room.capacity), room_type: Set(room.room_type.clone()),
            org_unit_id: Set(room.org_unit_id),
            created_at: Set(room.created_at), updated_at: Set(room.updated_at),
        };

        if RoomEntity::find_by_id(room.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl TimeSlotRepository for SchedulingRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainTimeSlot>, DomainError> {
        let model = TimeSlotEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainTimeSlot {
            id: m.id, period_id: m.period_id, day_of_week: m.day_of_week,
            slot_index: m.slot_index, name: m.name, name_ar: m.name_ar,
            start_time: m.start_time, end_time: m.end_time,
            is_break: m.is_break, break_type: m.break_type,
            created_at: m.created_at, updated_at: m.updated_at
        }))
    }

    async fn find_by_period(&self, period_id: Uuid) -> Result<Vec<DomainTimeSlot>, DomainError> {
        let models = TimeSlotEntity::find().filter(time_slot::Column::PeriodId.eq(period_id)).all(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainTimeSlot {
            id: m.id, period_id: m.period_id, day_of_week: m.day_of_week,
            slot_index: m.slot_index, name: m.name, name_ar: m.name_ar,
            start_time: m.start_time, end_time: m.end_time,
            is_break: m.is_break, break_type: m.break_type,
            created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }

    async fn save(&self, slot: &DomainTimeSlot) -> Result<(), DomainError> {
        let active = time_slot::ActiveModel {
            id: Set(slot.id), period_id: Set(slot.period_id), day_of_week: Set(slot.day_of_week),
            slot_index: Set(slot.slot_index), name: Set(slot.name.clone()), name_ar: Set(slot.name_ar.clone()),
            start_time: Set(slot.start_time), end_time: Set(slot.end_time),
            is_break: Set(slot.is_break), break_type: Set(slot.break_type.clone()),
            created_at: Set(slot.created_at), updated_at: Set(slot.updated_at),
        };

        if TimeSlotEntity::find_by_id(slot.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl ScheduledLessonRepository for SchedulingRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainScheduledLesson>, DomainError> {
        let model = ScheduledLessonEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainScheduledLesson {
            id: m.id, class_subject_id: m.class_subject_id, room_id: m.room_id,
            time_slot_id: m.time_slot_id, teacher_id: m.teacher_id,
            created_at: m.created_at, updated_at: m.updated_at
        }))
    }

    async fn find_by_class_subject(&self, class_subject_id: Uuid) -> Result<Vec<DomainScheduledLesson>, DomainError> {
        let models = ScheduledLessonEntity::find()
            .filter(scheduled_lesson::Column::ClassSubjectId.eq(class_subject_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainScheduledLesson {
            id: m.id, class_subject_id: m.class_subject_id, room_id: m.room_id,
            time_slot_id: m.time_slot_id, teacher_id: m.teacher_id,
            created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }

    async fn find_by_class(&self, class_id: Uuid) -> Result<Vec<DomainScheduledLesson>, DomainError> {
        let class_subjects = crate::models::class_subject::Entity::find()
            .filter(crate::models::class_subject::Column::ClassId.eq(class_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let cs_ids: Vec<Uuid> = class_subjects.into_iter().map(|cs| cs.id).collect();
        if cs_ids.is_empty() {
            return Ok(Vec::new());
        }
        let models = ScheduledLessonEntity::find()
            .filter(scheduled_lesson::Column::ClassSubjectId.is_in(cs_ids))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainScheduledLesson {
            id: m.id, class_subject_id: m.class_subject_id, room_id: m.room_id,
            time_slot_id: m.time_slot_id, teacher_id: m.teacher_id,
            created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }

    async fn find_by_teacher(&self, teacher_id: Uuid) -> Result<Vec<DomainScheduledLesson>, DomainError> {
        let models = ScheduledLessonEntity::find()
            .filter(scheduled_lesson::Column::TeacherId.eq(teacher_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainScheduledLesson {
            id: m.id, class_subject_id: m.class_subject_id, room_id: m.room_id,
            time_slot_id: m.time_slot_id, teacher_id: m.teacher_id,
            created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }

    async fn find_by_period(&self, period_id: Uuid) -> Result<Vec<DomainScheduledLesson>, DomainError> {
        let slots = TimeSlotEntity::find()
            .filter(time_slot::Column::PeriodId.eq(period_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let slot_ids: Vec<Uuid> = slots.into_iter().map(|s| s.id).collect();

        if slot_ids.is_empty() {
            return Ok(Vec::new());
        }

        let models = ScheduledLessonEntity::find()
            .filter(scheduled_lesson::Column::TimeSlotId.is_in(slot_ids))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainScheduledLesson {
            id: m.id, class_subject_id: m.class_subject_id, room_id: m.room_id,
            time_slot_id: m.time_slot_id, teacher_id: m.teacher_id,
            created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }

    async fn save(&self, lesson: &DomainScheduledLesson) -> Result<(), DomainError> {

        let active = scheduled_lesson::ActiveModel {
            id: Set(lesson.id), class_subject_id: Set(lesson.class_subject_id),
            room_id: Set(lesson.room_id), time_slot_id: Set(lesson.time_slot_id),
            teacher_id: Set(lesson.teacher_id), created_at: Set(lesson.created_at),
            updated_at: Set(lesson.updated_at),
        };

        if ScheduledLessonEntity::find_by_id(lesson.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}
