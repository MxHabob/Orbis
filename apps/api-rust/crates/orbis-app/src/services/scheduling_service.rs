use chrono::{NaiveDate, NaiveTime, Utc};
use orbis_domain::entities::exam_proctor::ExamProctor;
use orbis_domain::entities::exam_schedule::ExamSchedule;
use orbis_domain::entities::exam_session::ExamSession;
use orbis_domain::entities::room::Room;
use orbis_domain::entities::scheduled_lesson::ScheduledLesson;
use orbis_domain::entities::seating_number::SeatingNumber;
use orbis_domain::entities::secret_number::SecretNumber;
use orbis_domain::entities::time_slot::TimeSlot;
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::iam::permission::Permission;
use std::collections::HashSet;
use std::sync::Arc;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{
    ExamProctorRepository, ExamScheduleRepository, ExamSessionRepository, OrgUnitRepository,
    RoomRepository, ScheduledLessonRepository, SeatingNumberRepository, SecretNumberRepository,
    TimeSlotRepository,
};

pub struct SchedulingService {
    pub room_repo: Arc<dyn RoomRepository>,
    pub org_unit_repo: Arc<dyn OrgUnitRepository>,
    pub time_slot_repo: Arc<dyn TimeSlotRepository>,
    pub lesson_repo: Arc<dyn ScheduledLessonRepository>,
    pub exam_session_repo: Arc<dyn ExamSessionRepository>,
    pub exam_schedule_repo: Arc<dyn ExamScheduleRepository>,
    pub seating_number_repo: Arc<dyn SeatingNumberRepository>,
    pub secret_number_repo: Arc<dyn SecretNumberRepository>,
    pub exam_proctor_repo: Arc<dyn ExamProctorRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

fn validate_exam_session(
    session_type: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<(), DomainError> {
    if !["final", "midterm", "quiz", "practical"].contains(&session_type) {
        return Err(DomainError::Validation("Invalid exam session type".into()));
    }
    if end_date < start_date {
        return Err(DomainError::Validation(
            "Exam session end date must be on or after its start date".into(),
        ));
    }
    Ok(())
}

impl SchedulingService {
    pub fn new(
        room_repo: Arc<dyn RoomRepository>,
        org_unit_repo: Arc<dyn OrgUnitRepository>,
        time_slot_repo: Arc<dyn TimeSlotRepository>,
        lesson_repo: Arc<dyn ScheduledLessonRepository>,
        exam_session_repo: Arc<dyn ExamSessionRepository>,
        exam_schedule_repo: Arc<dyn ExamScheduleRepository>,
        seating_number_repo: Arc<dyn SeatingNumberRepository>,
        secret_number_repo: Arc<dyn SecretNumberRepository>,
        exam_proctor_repo: Arc<dyn ExamProctorRepository>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            room_repo,
            org_unit_repo,
            time_slot_repo,
            lesson_repo,
            exam_session_repo,
            exam_schedule_repo,
            seating_number_repo,
            secret_number_repo,
            exam_proctor_repo,
            event_bus,
            permission_checker,
        }
    }

    // ── Rooms ────────────────────────────────────────────────────────

    pub async fn create_room(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        name: String,
        name_ar: Option<String>,
        capacity: i32,
        room_type: String,
        floor: Option<i32>,
        org_unit_id: Option<Uuid>,
    ) -> Result<Room, DomainError> {
        self.permission_checker
            .check(ctx, Permission::ScheduleWrite)
            .await?;

        if let Some(unit_id) = org_unit_id {
            self.validate_room_org_unit(org_id, unit_id).await?;
        }

        let now = Utc::now();
        let room_id = Uuid::new_v4();

        let room = Room {
            id: room_id,
            org_id,
            name,
            name_ar,
            capacity,
            room_type,
            is_exam_hall: false,
            floor,
            org_unit_id,
            created_at: now,
            updated_at: now,
        };

        self.room_repo.save(&room).await?;

        let event = DomainEvent::RoomCreated {
            room_id,
            org_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(room)
    }

    pub async fn update_room(
        &self,
        ctx: &RequestContext,
        id: Uuid,
        name: Option<String>,
        name_ar: Option<String>,
        capacity: Option<i32>,
        room_type: Option<String>,
        is_exam_hall: Option<bool>,
        floor: Option<i32>,
        org_unit_id: Option<Option<Uuid>>,
    ) -> Result<Room, DomainError> {
        self.permission_checker
            .check(ctx, Permission::ScheduleWrite)
            .await?;

        let mut room = self
            .room_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Room not found".into()))?;

        if let Some(Some(unit_id)) = org_unit_id {
            self.validate_room_org_unit(room.org_id, unit_id).await?;
        }

        let now = Utc::now();
        if let Some(n) = name {
            room.name = n;
        }
        if let Some(n) = name_ar {
            room.name_ar = Some(n);
        }
        if let Some(c) = capacity {
            room.capacity = c;
        }
        if let Some(rt) = room_type {
            room.room_type = rt;
        }
        if let Some(is_exam_hall) = is_exam_hall {
            room.is_exam_hall = is_exam_hall;
        }
        if let Some(f) = floor {
            room.floor = Some(f);
        }
        if let Some(ou_id) = org_unit_id {
            room.org_unit_id = ou_id;
        }
        room.updated_at = now;

        self.room_repo.save(&room).await?;

        let event = DomainEvent::RoomUpdated {
            room_id: id,
            org_id: room.org_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(room)
    }

    pub async fn delete_room(&self, ctx: &RequestContext, id: Uuid) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::ScheduleWrite)
            .await?;

        let room = self
            .room_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Room not found".into()))?;

        self.room_repo.delete(id).await?;

        let event = DomainEvent::RoomDeleted {
            room_id: id,
            org_id: room.org_id,
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(())
    }

    pub async fn list_rooms(&self, org_id: Uuid) -> Result<Vec<Room>, DomainError> {
        self.room_repo.find_by_org(org_id).await
    }

    pub async fn get_room(&self, id: Uuid) -> Result<Option<Room>, DomainError> {
        self.room_repo.find_by_id(id).await
    }

    async fn validate_room_org_unit(
        &self,
        org_id: Uuid,
        org_unit_id: Uuid,
    ) -> Result<(), DomainError> {
        match self.org_unit_repo.find_by_id(org_unit_id).await? {
            Some(unit)
                if unit.org_id == org_id
                    && unit.is_active
                    && ["branch", "building", "facility"]
                        .iter()
                        .any(|kind| unit.unit_type.eq_ignore_ascii_case(kind)) =>
            {
                Ok(())
            }
            _ => Err(DomainError::Validation("Invalid room location".into())),
        }
    }

    // ── Time Slots ──────────────────────────────────────────────────

    pub async fn create_time_slot(
        &self,
        ctx: &RequestContext,
        period_id: Uuid,
        day_of_week: i16,
        slot_index: i32,
        name: Option<String>,
        name_ar: Option<String>,
        start_time: NaiveTime,
        end_time: NaiveTime,
        is_break: bool,
        break_type: Option<String>,
    ) -> Result<TimeSlot, DomainError> {
        self.permission_checker
            .check(ctx, Permission::ScheduleWrite)
            .await?;

        let now = Utc::now();
        let slot_id = Uuid::new_v4();

        let slot = TimeSlot {
            id: slot_id,
            period_id,
            day_of_week,
            slot_index,
            name,
            name_ar,
            start_time,
            end_time,
            is_break,
            break_type,
            created_at: now,
            updated_at: now,
        };

        self.time_slot_repo.save(&slot).await?;

        let event = DomainEvent::TimeSlotCreated {
            time_slot_id: slot_id,
            period_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(slot)
    }

    pub async fn update_time_slot(
        &self,
        ctx: &RequestContext,
        id: Uuid,
        day_of_week: Option<i16>,
        slot_index: Option<i32>,
        name: Option<Option<String>>,
        name_ar: Option<Option<String>>,
        start_time: Option<NaiveTime>,
        end_time: Option<NaiveTime>,
        is_break: Option<bool>,
        break_type: Option<Option<String>>,
    ) -> Result<TimeSlot, DomainError> {
        self.permission_checker
            .check(ctx, Permission::ScheduleWrite)
            .await?;

        let mut slot = self
            .time_slot_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Time slot not found".into()))?;

        let now = Utc::now();
        if let Some(dow) = day_of_week {
            slot.day_of_week = dow;
        }
        if let Some(si) = slot_index {
            slot.slot_index = si;
        }
        if let Some(n) = name {
            slot.name = n;
        }
        if let Some(na) = name_ar {
            slot.name_ar = na;
        }
        if let Some(st) = start_time {
            slot.start_time = st;
        }
        if let Some(et) = end_time {
            slot.end_time = et;
        }
        if let Some(ib) = is_break {
            slot.is_break = ib;
        }
        if let Some(bt) = break_type {
            slot.break_type = bt;
        }
        slot.updated_at = now;

        self.time_slot_repo.save(&slot).await?;

        let event = DomainEvent::TimeSlotUpdated {
            time_slot_id: id,
            period_id: slot.period_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(slot)
    }

    pub async fn delete_time_slot(
        &self,
        ctx: &RequestContext,
        id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::ScheduleWrite)
            .await?;

        let slot = self
            .time_slot_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Time slot not found".into()))?;

        self.time_slot_repo.delete(id).await?;

        let event = DomainEvent::TimeSlotDeleted {
            time_slot_id: id,
            period_id: slot.period_id,
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(())
    }

    pub async fn batch_create_time_slots(
        &self,
        ctx: &RequestContext,
        period_id: Uuid,
        items: Vec<(
            i16,
            i32,
            Option<String>,
            Option<String>,
            NaiveTime,
            NaiveTime,
            bool,
            Option<String>,
        )>,
    ) -> Result<Vec<TimeSlot>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::ScheduleWrite)
            .await?;

        let now = Utc::now();
        let mut slots = Vec::with_capacity(items.len());
        for (day_of_week, slot_index, name, name_ar, start_time, end_time, is_break, break_type) in
            items
        {
            let slot = TimeSlot {
                id: Uuid::new_v4(),
                period_id,
                day_of_week,
                slot_index,
                name,
                name_ar,
                start_time,
                end_time,
                is_break,
                break_type,
                created_at: now,
                updated_at: now,
            };
            slots.push(slot);
        }

        self.time_slot_repo.save_bulk(&slots).await?;

        for s in &slots {
            let event = DomainEvent::TimeSlotCreated {
                time_slot_id: s.id,
                period_id,
                timestamp: now,
            };
            self.event_bus.publish(ctx, &event).await?;
        }

        Ok(slots)
    }

    pub async fn list_time_slots(&self, period_id: Uuid) -> Result<Vec<TimeSlot>, DomainError> {
        self.time_slot_repo.find_by_period(period_id).await
    }

    pub async fn get_time_slot(&self, id: Uuid) -> Result<Option<TimeSlot>, DomainError> {
        self.time_slot_repo.find_by_id(id).await
    }

    // ── Scheduled Lessons ───────────────────────────────────────────

    pub async fn schedule_lesson(
        &self,
        ctx: &RequestContext,
        section_subject_id: Uuid,
        time_slot_id: Uuid,
        room_id: Option<Uuid>,
        teacher_id: Option<Uuid>,
    ) -> Result<ScheduledLesson, DomainError> {
        self.permission_checker
            .check(ctx, Permission::ScheduleWrite)
            .await?;

        let now = Utc::now();
        let lesson = ScheduledLesson {
            id: Uuid::new_v4(),
            section_subject_id,
            time_slot_id,
            room_id,
            teacher_id,
            created_at: now,
            updated_at: now,
        };

        if self
            .lesson_repo
            .has_conflicts(std::slice::from_ref(&lesson), &[])
            .await?
        {
            return Err(DomainError::Conflict(
                "The section, teacher, or room is already scheduled in this time slot".into(),
            ));
        }

        self.lesson_repo.save(&lesson).await?;

        let event = DomainEvent::LessonScheduled {
            lesson_id: lesson.id,
            section_subject_id,
            time_slot_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(lesson)
    }

    pub async fn update_scheduled_lesson(
        &self,
        ctx: &RequestContext,
        id: Uuid,
        section_subject_id: Uuid,
        time_slot_id: Uuid,
        room_id: Option<Uuid>,
        teacher_id: Option<Uuid>,
    ) -> Result<ScheduledLesson, DomainError> {
        self.permission_checker
            .check(ctx, Permission::ScheduleWrite)
            .await?;

        let mut lesson = self
            .lesson_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Scheduled lesson not found".into()))?;
        lesson.section_subject_id = section_subject_id;
        lesson.time_slot_id = time_slot_id;
        lesson.room_id = room_id;
        lesson.teacher_id = teacher_id;
        lesson.updated_at = Utc::now();

        if self
            .lesson_repo
            .has_conflicts(std::slice::from_ref(&lesson), &[id])
            .await?
        {
            return Err(DomainError::Conflict(
                "The section, teacher, or room is already scheduled in this time slot".into(),
            ));
        }

        self.lesson_repo.save(&lesson).await?;
        let event = DomainEvent::LessonUpdated {
            lesson_id: id,
            section_subject_id,
            time_slot_id,
            timestamp: lesson.updated_at,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(lesson)
    }

    pub async fn delete_scheduled_lesson(
        &self,
        ctx: &RequestContext,
        id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::ScheduleWrite)
            .await?;

        self.lesson_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Scheduled lesson not found".into()))?;

        self.lesson_repo.delete(id).await?;

        let event = DomainEvent::LessonDeleted {
            lesson_id: id,
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(())
    }

    pub async fn bulk_schedule_lessons(
        &self,
        ctx: &RequestContext,
        items: Vec<(Uuid, Uuid, Option<Uuid>, Option<Uuid>)>,
    ) -> Result<Vec<ScheduledLesson>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::ScheduleWrite)
            .await?;

        let now = Utc::now();
        let mut lessons = Vec::with_capacity(items.len());
        for (section_subject_id, time_slot_id, room_id, teacher_id) in items {
            let lesson = ScheduledLesson {
                id: Uuid::new_v4(),
                section_subject_id,
                time_slot_id,
                room_id,
                teacher_id,
                created_at: now,
                updated_at: now,
            };
            lessons.push(lesson);
        }

        if self.lesson_repo.has_conflicts(&lessons, &[]).await? {
            return Err(DomainError::Conflict(
                "The section, teacher, or room is already scheduled in this time slot".into(),
            ));
        }

        self.lesson_repo.save_bulk(&lessons).await?;

        for l in &lessons {
            let event = DomainEvent::LessonScheduled {
                lesson_id: l.id,
                section_subject_id: l.section_subject_id,
                time_slot_id: l.time_slot_id,
                timestamp: now,
            };
            self.event_bus.publish(ctx, &event).await?;
        }

        Ok(lessons)
    }

    pub async fn list_lessons_by_class(
        &self,
        section_id: Uuid,
    ) -> Result<Vec<ScheduledLesson>, DomainError> {
        self.lesson_repo.find_by_section(section_id).await
    }

    pub async fn list_lessons_by_teacher(
        &self,
        teacher_id: Uuid,
    ) -> Result<Vec<ScheduledLesson>, DomainError> {
        self.lesson_repo.find_by_teacher(teacher_id).await
    }

    pub async fn list_lessons_by_section_subject(
        &self,
        section_subject_id: Uuid,
    ) -> Result<Vec<ScheduledLesson>, DomainError> {
        self.lesson_repo
            .find_by_section_subject(section_subject_id)
            .await
    }

    pub async fn list_lessons_by_period(
        &self,
        period_id: Uuid,
    ) -> Result<Vec<ScheduledLesson>, DomainError> {
        self.lesson_repo.find_by_period(period_id).await
    }

    // ── Exam Sessions ───────────────────────────────────────────────

    pub async fn create_exam_session(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        period_id: Uuid,
        name: String,
        name_ar: Option<String>,
        session_type: String,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<ExamSession, DomainError> {
        self.permission_checker
            .check(ctx, Permission::ExamSessionWrite)
            .await?;
        validate_exam_session(&session_type, start_date, end_date)?;

        let now = Utc::now();
        let session_id = Uuid::new_v4();

        let session = ExamSession {
            id: session_id,
            org_id,
            period_id,
            name,
            name_ar,
            session_type,
            start_date,
            end_date,
            status: "draft".to_string(),
            created_at: now,
            updated_at: now,
        };

        self.exam_session_repo.save(&session).await?;

        let event = DomainEvent::ExamSessionCreated {
            session_id,
            org_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(session)
    }

    pub async fn list_exam_sessions(
        &self,
        period_id: Uuid,
    ) -> Result<Vec<ExamSession>, DomainError> {
        self.exam_session_repo.find_by_period(period_id).await
    }

    pub async fn get_exam_session(&self, id: Uuid) -> Result<Option<ExamSession>, DomainError> {
        self.exam_session_repo.find_by_id(id).await
    }

    // ── Exam Schedules ──────────────────────────────────────────────

    pub async fn schedule_exam(
        &self,
        ctx: &RequestContext,
        session_id: Uuid,
        section_subject_id: Uuid,
        exam_date: NaiveDate,
        start_time: NaiveTime,
        end_time: NaiveTime,
    ) -> Result<ExamSchedule, DomainError> {
        self.permission_checker
            .check(ctx, Permission::ExamScheduleWrite)
            .await?;

        let now = Utc::now();
        let schedule_id = Uuid::new_v4();

        let schedule = ExamSchedule {
            id: schedule_id,
            exam_session_id: session_id,
            section_subject_id,
            exam_date,
            start_time,
            end_time,
            created_at: now,
            updated_at: now,
        };

        self.exam_schedule_repo.save(&schedule).await?;

        let event = DomainEvent::ExamScheduled {
            schedule_id,
            session_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(schedule)
    }

    pub async fn list_exam_schedules(
        &self,
        session_id: Uuid,
    ) -> Result<Vec<ExamSchedule>, DomainError> {
        self.exam_schedule_repo.find_by_session(session_id).await
    }

    pub async fn get_exam_schedule(&self, id: Uuid) -> Result<Option<ExamSchedule>, DomainError> {
        self.exam_schedule_repo.find_by_id(id).await
    }

    // ── Seating Numbers ─────────────────────────────────────────────

    pub async fn assign_seating_number(
        &self,
        ctx: &RequestContext,
        session_id: Uuid,
        student_id: Uuid,
        seating_number: String,
        room_id: Option<Uuid>,
    ) -> Result<SeatingNumber, DomainError> {
        self.permission_checker
            .check(ctx, Permission::SeatingNumberGenerate)
            .await?;

        let now = Utc::now();
        let seating_id = Uuid::new_v4();

        let seating = SeatingNumber {
            id: seating_id,
            exam_session_id: session_id,
            student_id,
            room_id,
            seating_number: seating_number.clone(),
            created_at: now,
            updated_at: now,
        };

        self.seating_number_repo.save(&seating).await?;

        let event = DomainEvent::SeatingNumberAssigned {
            student_id,
            session_id,
            seating_number,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(seating)
    }

    pub async fn list_seating_numbers(
        &self,
        session_id: Uuid,
    ) -> Result<Vec<SeatingNumber>, DomainError> {
        self.seating_number_repo.find_by_session(session_id).await
    }

    pub async fn get_student_seating(
        &self,
        session_id: Uuid,
        student_id: Uuid,
    ) -> Result<Option<SeatingNumber>, DomainError> {
        self.seating_number_repo
            .find_by_session_and_student(session_id, student_id)
            .await
    }

    pub async fn generate_seating_numbers(
        &self,
        ctx: &RequestContext,
        session_id: Uuid,
        student_ids: Vec<Uuid>,
        room_ids: Vec<Uuid>,
        prefix: Option<String>,
        start_number: Option<u32>,
    ) -> Result<Vec<SeatingNumber>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::SeatingNumberGenerate)
            .await?;

        let now = Utc::now();
        let prefix = prefix.unwrap_or_else(|| "SEAT-".to_string());
        let mut next_number = start_number.unwrap_or(1);
        let existing = self.seating_number_repo.find_by_session(session_id).await?;
        let assigned_students: HashSet<Uuid> =
            existing.iter().map(|seat| seat.student_id).collect();
        let mut used_numbers: HashSet<String> = existing
            .iter()
            .map(|seat| seat.seating_number.clone())
            .collect();
        let mut requested_students = HashSet::new();
        let student_ids: Vec<Uuid> = student_ids
            .into_iter()
            .filter(|student_id| {
                requested_students.insert(*student_id) && !assigned_students.contains(student_id)
            })
            .collect();

        let mut room_capacity = Vec::with_capacity(room_ids.len());
        let unique_room_ids: HashSet<Uuid> = room_ids.iter().copied().collect();
        if unique_room_ids.len() != room_ids.len() {
            return Err(DomainError::Validation(
                "Exam hall list contains duplicate rooms".into(),
            ));
        }
        for room_id in room_ids {
            let room = self
                .room_repo
                .find_by_id(room_id)
                .await?
                .ok_or_else(|| DomainError::NotFound(format!("Room {room_id} not found")))?;
            if !room.is_exam_hall {
                return Err(DomainError::Validation(format!(
                    "Room {} is not enabled as an exam hall",
                    room.name
                )));
            }
            let used = existing
                .iter()
                .filter(|seat| seat.room_id == Some(room_id))
                .count() as i32;
            room_capacity.push((room_id, room.capacity.max(0), used));
        }

        let mut next_room = 0;
        let mut seatings = Vec::with_capacity(student_ids.len());
        for student_id in student_ids {
            let room_id = if room_capacity.is_empty() {
                None
            } else {
                let available_room = (0..room_capacity.len())
                    .map(|offset| (next_room + offset) % room_capacity.len())
                    .find(|&index| room_capacity[index].2 < room_capacity[index].1)
                    .ok_or_else(|| {
                        DomainError::Validation(
                            "Selected exam halls do not have enough remaining capacity".into(),
                        )
                    })?;
                room_capacity[available_room].2 += 1;
                next_room = (available_room + 1) % room_capacity.len();
                Some(room_capacity[available_room].0)
            };

            let seating_number = loop {
                let candidate = format!("{prefix}{next_number:04}");
                next_number = next_number.checked_add(1).ok_or_else(|| {
                    DomainError::Validation("Seating number range is exhausted".into())
                })?;
                if used_numbers.insert(candidate.clone()) {
                    break candidate;
                }
            };
            let seating = SeatingNumber {
                id: Uuid::new_v4(),
                exam_session_id: session_id,
                student_id,
                room_id,
                seating_number,
                created_at: now,
                updated_at: now,
            };
            seatings.push(seating);
        }

        if !seatings.is_empty() {
            self.seating_number_repo.save_bulk(&seatings).await?;
        }

        for s in &seatings {
            let event = DomainEvent::SeatingNumberAssigned {
                student_id: s.student_id,
                session_id: s.exam_session_id,
                seating_number: s.seating_number.clone(),
                timestamp: now,
            };
            self.event_bus.publish(ctx, &event).await?;
        }

        Ok(seatings)
    }

    // ── Secret Numbers ──────────────────────────────────────────────

    pub async fn assign_secret_number(
        &self,
        ctx: &RequestContext,
        schedule_id: Uuid,
        student_id: Uuid,
        secret_number: String,
    ) -> Result<SecretNumber, DomainError> {
        self.permission_checker
            .check(ctx, Permission::SecretNumberGenerate)
            .await?;

        let now = Utc::now();
        let id = Uuid::new_v4();

        let secret = SecretNumber {
            id,
            exam_schedule_id: schedule_id,
            student_id,
            secret_number,
            created_at: now,
            updated_at: now,
        };

        self.secret_number_repo.save(&secret).await?;
        Ok(secret)
    }

    pub async fn list_secret_numbers(
        &self,
        schedule_id: Uuid,
    ) -> Result<Vec<SecretNumber>, DomainError> {
        self.secret_number_repo.find_by_schedule(schedule_id).await
    }

    pub async fn generate_secret_numbers(
        &self,
        ctx: &RequestContext,
        schedule_id: Uuid,
        student_ids: Vec<Uuid>,
    ) -> Result<Vec<SecretNumber>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::SecretNumberGenerate)
            .await?;

        self.exam_schedule_repo
            .find_by_id(schedule_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Exam schedule not found".to_string()))?;

        let now = Utc::now();
        let existing = self
            .secret_number_repo
            .find_by_schedule(schedule_id)
            .await?;
        let mut assigned_students: HashSet<Uuid> =
            existing.iter().map(|secret| secret.student_id).collect();
        let mut used_secrets: HashSet<String> = existing
            .iter()
            .map(|secret| secret.secret_number.clone())
            .collect();
        let mut requested_students = HashSet::new();
        let student_ids: Vec<Uuid> = student_ids
            .into_iter()
            .filter(|student_id| {
                requested_students.insert(*student_id) && !assigned_students.contains(student_id)
            })
            .collect();
        let mut secrets = Vec::with_capacity(student_ids.len());

        for student_id in student_ids {
            let mut candidate;
            loop {
                let u = Uuid::new_v4();
                let num = (u.as_u128() % 900_000) + 100_000;
                candidate = format!("{:06}", num);
                if used_secrets.insert(candidate.clone()) {
                    break;
                }
            }
            assigned_students.insert(student_id);

            secrets.push(SecretNumber {
                id: Uuid::new_v4(),
                exam_schedule_id: schedule_id,
                student_id,
                secret_number: candidate,
                created_at: now,
                updated_at: now,
            });
        }

        if !secrets.is_empty() {
            self.secret_number_repo.save_bulk(&secrets).await?;
        }
        Ok(secrets)
    }

    // ── Exam Proctors ───────────────────────────────────────────────

    pub async fn assign_proctor(
        &self,
        ctx: &RequestContext,
        schedule_id: Uuid,
        room_id: Uuid,
        teacher_id: Uuid,
        is_chief: bool,
    ) -> Result<ExamProctor, DomainError> {
        self.permission_checker
            .check(ctx, Permission::ProctorAssign)
            .await?;

        self.exam_schedule_repo
            .find_by_id(schedule_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Exam schedule not found".to_string()))?;
        let room = self
            .room_repo
            .find_by_id(room_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Room not found".to_string()))?;
        if !room.is_exam_hall {
            return Err(DomainError::Validation(
                "Room is not enabled as an exam hall".into(),
            ));
        }
        let assigned = self.exam_proctor_repo.find_by_schedule(schedule_id).await?;
        if assigned
            .iter()
            .any(|proctor| proctor.teacher_id == teacher_id && proctor.room_id == room_id)
        {
            return Err(DomainError::Validation(
                "Teacher is already assigned to this exam hall".into(),
            ));
        }
        if is_chief
            && assigned
                .iter()
                .any(|proctor| proctor.room_id == room_id && proctor.is_chief)
        {
            return Err(DomainError::Validation(
                "This exam hall already has a chief proctor".into(),
            ));
        }

        let now = Utc::now();
        let id = Uuid::new_v4();

        let proctor = ExamProctor {
            id,
            exam_schedule_id: schedule_id,
            room_id,
            teacher_id,
            is_chief,
            created_at: now,
            updated_at: now,
        };

        self.exam_proctor_repo.save(&proctor).await?;
        Ok(proctor)
    }

    pub async fn update_exam_session(
        &self,
        ctx: &RequestContext,
        id: Uuid,
        name: Option<String>,
        name_ar: Option<String>,
        session_type: Option<String>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        status: Option<String>,
    ) -> Result<ExamSession, DomainError> {
        self.permission_checker
            .check(ctx, Permission::ExamSessionWrite)
            .await?;
        let mut session = self
            .exam_session_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Exam session not found".to_string()))?;

        if let Some(n) = name {
            session.name = n;
        }
        if let Some(n) = name_ar {
            session.name_ar = Some(n);
        }
        if let Some(session_type) = session_type {
            session.session_type = session_type;
        }
        if let Some(s) = start_date {
            session.start_date = s;
        }
        if let Some(e) = end_date {
            session.end_date = e;
        }
        if let Some(st) = status {
            session.status = st;
        }
        validate_exam_session(&session.session_type, session.start_date, session.end_date)?;
        session.updated_at = Utc::now();

        self.exam_session_repo.save(&session).await?;

        let event = DomainEvent::ExamSessionUpdated {
            session_id: session.id,
            org_id: session.org_id,
            timestamp: session.updated_at,
        };
        self.event_bus.publish(ctx, &event).await?;
        Ok(session)
    }

    pub async fn delete_exam_session(
        &self,
        ctx: &RequestContext,
        id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::ExamSessionDelete)
            .await?;
        let session = self
            .exam_session_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Exam session not found".to_string()))?;

        self.exam_session_repo.delete(id).await?;

        let event = DomainEvent::ExamSessionDeleted {
            session_id: id,
            org_id: session.org_id,
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;
        Ok(())
    }

    pub async fn delete_exam_schedule(
        &self,
        ctx: &RequestContext,
        id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::ExamScheduleWrite)
            .await?;
        let schedule = self
            .exam_schedule_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Exam schedule not found".to_string()))?;

        self.exam_schedule_repo.delete(id).await?;

        let event = DomainEvent::ExamScheduleDeleted {
            schedule_id: id,
            session_id: schedule.exam_session_id,
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;
        Ok(())
    }

    pub async fn delete_seating_number(
        &self,
        ctx: &RequestContext,
        id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::SeatingNumberGenerate)
            .await?;
        self.seating_number_repo.delete(id).await?;
        let event = DomainEvent::SeatingNumberDeleted {
            seating_id: id,
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;
        Ok(())
    }

    pub async fn delete_secret_number(
        &self,
        ctx: &RequestContext,
        id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::SecretNumberGenerate)
            .await?;
        self.secret_number_repo.delete(id).await?;
        let event = DomainEvent::SecretNumberDeleted {
            secret_id: id,
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;
        Ok(())
    }

    pub async fn delete_proctor(&self, ctx: &RequestContext, id: Uuid) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::ProctorAssign)
            .await?;
        self.exam_proctor_repo.delete(id).await?;
        let event = DomainEvent::ProctorDeleted {
            proctor_id: id,
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;
        Ok(())
    }

    pub async fn list_proctors(&self, schedule_id: Uuid) -> Result<Vec<ExamProctor>, DomainError> {
        self.exam_proctor_repo.find_by_schedule(schedule_id).await
    }
}

#[cfg(test)]
mod exam_session_validation_tests {
    use super::validate_exam_session;
    use chrono::NaiveDate;
    use orbis_domain::errors::DomainError;

    #[test]
    fn accepts_supported_types_and_valid_date_ranges() {
        let start = NaiveDate::from_ymd_opt(2026, 10, 16).unwrap();
        let end = NaiveDate::from_ymd_opt(2026, 10, 28).unwrap();
        for session_type in ["final", "midterm", "quiz", "practical"] {
            assert!(validate_exam_session(session_type, start, end).is_ok());
        }
    }

    #[test]
    fn rejects_unknown_types_and_end_dates_before_start() {
        let start = NaiveDate::from_ymd_opt(2026, 10, 16).unwrap();
        let earlier = NaiveDate::from_ymd_opt(2026, 10, 15).unwrap();
        assert!(matches!(
            validate_exam_session("unknown", start, start),
            Err(DomainError::Validation(_))
        ));
        assert!(matches!(
            validate_exam_session("final", start, earlier),
            Err(DomainError::Validation(_))
        ));
    }
}
