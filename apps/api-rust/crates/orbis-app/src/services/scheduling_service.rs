use std::sync::Arc;
use uuid::Uuid;
use chrono::{Utc, NaiveDate, NaiveTime};
use orbis_domain::entities::room::Room;
use orbis_domain::entities::time_slot::TimeSlot;
use orbis_domain::entities::scheduled_lesson::ScheduledLesson;
use orbis_domain::entities::exam_session::ExamSession;
use orbis_domain::entities::exam_schedule::ExamSchedule;
use orbis_domain::entities::seating_number::SeatingNumber;
use orbis_domain::entities::secret_number::SecretNumber;
use orbis_domain::entities::exam_proctor::ExamProctor;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use orbis_domain::iam::permission::Permission;

use crate::context::RequestContext;
use crate::ports::repositories::{
    RoomRepository, TimeSlotRepository, ScheduledLessonRepository,
    ExamSessionRepository, ExamScheduleRepository, SeatingNumberRepository,
    SecretNumberRepository, ExamProctorRepository,
};
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;

pub struct SchedulingService {
    pub room_repo: Arc<dyn RoomRepository>,
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

impl SchedulingService {
    pub fn new(
        room_repo: Arc<dyn RoomRepository>,
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
        capacity: i32,
        room_type: String,
    ) -> Result<Room, DomainError> {
        self.permission_checker.check(ctx, Permission::ScheduleWrite).await?;

        let now = Utc::now();
        let room_id = Uuid::new_v4();

        let room = Room {
            id: room_id,
            org_id,
            name,
            capacity,
            room_type,
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

    pub async fn list_rooms(&self, org_id: Uuid) -> Result<Vec<Room>, DomainError> {
        self.room_repo.find_by_org(org_id).await
    }

    pub async fn get_room(&self, id: Uuid) -> Result<Option<Room>, DomainError> {
        self.room_repo.find_by_id(id).await
    }

    // ── Time Slots ──────────────────────────────────────────────────

    pub async fn create_time_slot(
        &self,
        ctx: &RequestContext,
        period_id: Uuid,
        day_of_week: i16,
        start_time: NaiveTime,
        end_time: NaiveTime,
        slot_index: i32,
    ) -> Result<TimeSlot, DomainError> {
        self.permission_checker.check(ctx, Permission::ScheduleWrite).await?;

        let now = Utc::now();
        let slot_id = Uuid::new_v4();

        let slot = TimeSlot {
            id: slot_id,
            period_id,
            day_of_week,
            start_time,
            end_time,
            slot_index,
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
        class_subject_id: Uuid,
        time_slot_id: Uuid,
        room_id: Option<Uuid>,
        teacher_id: Option<Uuid>,
    ) -> Result<ScheduledLesson, DomainError> {
        self.permission_checker.check(ctx, Permission::ScheduleWrite).await?;

        if let Some(tid) = teacher_id {
            let teacher_lessons = self.lesson_repo.find_by_teacher(tid).await?;
            if teacher_lessons.iter().any(|l| l.time_slot_id == time_slot_id) {
                return Err(DomainError::Conflict(
                    "Teacher is already scheduled for another lesson in this time slot".into(),
                ));
            }
        }

        let now = Utc::now();
        let lesson_id = Uuid::new_v4();

        let lesson = ScheduledLesson {
            id: lesson_id,
            class_subject_id,
            time_slot_id,
            room_id,
            teacher_id,
            created_at: now,
            updated_at: now,
        };

        self.lesson_repo.save(&lesson).await?;

        let event = DomainEvent::LessonScheduled {
            lesson_id,
            class_subject_id,
            time_slot_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(lesson)
    }

    pub async fn list_lessons_by_class(&self, class_id: Uuid) -> Result<Vec<ScheduledLesson>, DomainError> {
        self.lesson_repo.find_by_class(class_id).await
    }

    pub async fn list_lessons_by_teacher(&self, teacher_id: Uuid) -> Result<Vec<ScheduledLesson>, DomainError> {
        self.lesson_repo.find_by_teacher(teacher_id).await
    }

    pub async fn list_lessons_by_class_subject(&self, class_subject_id: Uuid) -> Result<Vec<ScheduledLesson>, DomainError> {
        self.lesson_repo.find_by_class_subject(class_subject_id).await
    }

    pub async fn list_lessons_by_period(&self, period_id: Uuid) -> Result<Vec<ScheduledLesson>, DomainError> {
        self.lesson_repo.find_by_period(period_id).await
    }

    // ── Exam Sessions ───────────────────────────────────────────────

    pub async fn create_exam_session(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        period_id: Uuid,
        name: String,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<ExamSession, DomainError> {
        self.permission_checker.check(ctx, Permission::ExamSessionWrite).await?;

        let now = Utc::now();
        let session_id = Uuid::new_v4();

        let session = ExamSession {
            id: session_id,
            org_id,
            period_id,
            name,
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

    pub async fn list_exam_sessions(&self, period_id: Uuid) -> Result<Vec<ExamSession>, DomainError> {
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
        class_subject_id: Uuid,
        exam_date: NaiveDate,
        start_time: NaiveTime,
        end_time: NaiveTime,
    ) -> Result<ExamSchedule, DomainError> {
        self.permission_checker.check(ctx, Permission::ExamScheduleWrite).await?;

        let now = Utc::now();
        let schedule_id = Uuid::new_v4();

        let schedule = ExamSchedule {
            id: schedule_id,
            exam_session_id: session_id,
            class_subject_id,
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

    pub async fn list_exam_schedules(&self, session_id: Uuid) -> Result<Vec<ExamSchedule>, DomainError> {
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
        self.permission_checker.check(ctx, Permission::SeatingNumberGenerate).await?;

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

    pub async fn list_seating_numbers(&self, session_id: Uuid) -> Result<Vec<SeatingNumber>, DomainError> {
        self.seating_number_repo.find_by_session(session_id).await
    }

    pub async fn get_student_seating(&self, session_id: Uuid, student_id: Uuid) -> Result<Option<SeatingNumber>, DomainError> {
        self.seating_number_repo.find_by_session_and_student(session_id, student_id).await
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
        self.permission_checker.check(ctx, Permission::SeatingNumberGenerate).await?;

        let now = Utc::now();
        let prefix = prefix.unwrap_or_else(|| "SEAT-".to_string());
        let start = start_number.unwrap_or(1);

        let mut seatings = Vec::with_capacity(student_ids.len());
        for (i, &student_id) in student_ids.iter().enumerate() {
            let seat_num = format!("{}{:04}", prefix, start + (i as u32));
            let room_id = if !room_ids.is_empty() {
                Some(room_ids[i % room_ids.len()])
            } else {
                None
            };

            let seating = SeatingNumber {
                id: Uuid::new_v4(),
                exam_session_id: session_id,
                student_id,
                room_id,
                seating_number: seat_num.clone(),
                created_at: now,
                updated_at: now,
            };
            seatings.push(seating);
        }

        self.seating_number_repo.save_bulk(&seatings).await?;

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
        self.permission_checker.check(ctx, Permission::SecretNumberGenerate).await?;

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

    pub async fn list_secret_numbers(&self, schedule_id: Uuid) -> Result<Vec<SecretNumber>, DomainError> {
        self.secret_number_repo.find_by_schedule(schedule_id).await
    }

    pub async fn generate_secret_numbers(
        &self,
        ctx: &RequestContext,
        schedule_id: Uuid,
        student_ids: Vec<Uuid>,
    ) -> Result<Vec<SecretNumber>, DomainError> {
        self.permission_checker.check(ctx, Permission::SecretNumberGenerate).await?;

        let now = Utc::now();
        let mut used_secrets = std::collections::HashSet::new();
        let mut secrets = Vec::with_capacity(student_ids.len());

        for &student_id in &student_ids {
            let mut candidate;
            loop {
                let u = Uuid::new_v4();
                let num = (u.as_u128() % 900_000) + 100_000;
                candidate = format!("{:06}", num);
                if used_secrets.insert(candidate.clone()) {
                    break;
                }
            }

            secrets.push(SecretNumber {
                id: Uuid::new_v4(),
                exam_schedule_id: schedule_id,
                student_id,
                secret_number: candidate,
                created_at: now,
                updated_at: now,
            });
        }

        self.secret_number_repo.save_bulk(&secrets).await?;
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
        self.permission_checker.check(ctx, Permission::ProctorAssign).await?;

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

    pub async fn list_proctors(&self, schedule_id: Uuid) -> Result<Vec<ExamProctor>, DomainError> {
        self.exam_proctor_repo.find_by_schedule(schedule_id).await
    }
}
