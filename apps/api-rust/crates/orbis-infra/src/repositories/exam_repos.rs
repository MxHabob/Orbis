use async_trait::async_trait;
use orbis_app::ports::repositories::{
    ExamSessionRepository, ExamScheduleRepository, SeatingNumberRepository,
    SecretNumberRepository, ExamProctorRepository
};
use orbis_domain::entities::exam_session::ExamSession as DomainExamSession;
use orbis_domain::entities::exam_schedule::ExamSchedule as DomainExamSchedule;
use orbis_domain::entities::seating_number::SeatingNumber as DomainSeatingNumber;
use orbis_domain::entities::secret_number::SecretNumber as DomainSecretNumber;
use orbis_domain::entities::exam_proctor::ExamProctor as DomainExamProctor;
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::models::{
    exam_session::{self, Entity as ExamSessionEntity},
    exam_schedule::{self, Entity as ExamScheduleEntity},
    seating_number::{self, Entity as SeatingNumberEntity},
    secret_number::{self, Entity as SecretNumberEntity},
    exam_proctor::{self, Entity as ExamProctorEntity},
};

pub struct ExamRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl ExamSessionRepository for ExamRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainExamSession>, DomainError> {
        let model = ExamSessionEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainExamSession {
            id: m.id,
            org_id: m.org_id,
            period_id: m.period_id,
            name: m.name,
            start_date: m.start_date,
            end_date: m.end_date,
            status: m.status,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    async fn find_by_period(&self, period_id: Uuid) -> Result<Vec<DomainExamSession>, DomainError> {
        let models = ExamSessionEntity::find()
            .filter(exam_session::Column::PeriodId.eq(period_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainExamSession {
            id: m.id,
            org_id: m.org_id,
            period_id: m.period_id,
            name: m.name,
            start_date: m.start_date,
            end_date: m.end_date,
            status: m.status,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }).collect())
    }

    async fn save(&self, session: &DomainExamSession) -> Result<(), DomainError> {
        let active = exam_session::ActiveModel {
            id: Set(session.id),
            org_id: Set(session.org_id),
            period_id: Set(session.period_id),
            name: Set(session.name.clone()),
            start_date: Set(session.start_date),
            end_date: Set(session.end_date),
            status: Set(session.status.clone()),
            created_at: Set(session.created_at),
            updated_at: Set(session.updated_at),
        };

        if ExamSessionEntity::find_by_id(session.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        ExamSessionEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl ExamScheduleRepository for ExamRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainExamSchedule>, DomainError> {
        let model = ExamScheduleEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainExamSchedule {
            id: m.id, exam_session_id: m.exam_session_id, class_subject_id: m.class_subject_id,
            exam_date: m.exam_date, start_time: m.start_time, end_time: m.end_time,
            created_at: m.created_at, updated_at: m.updated_at
        }))
    }

    async fn find_by_session(&self, session_id: Uuid) -> Result<Vec<DomainExamSchedule>, DomainError> {
        let models = ExamScheduleEntity::find()
            .filter(exam_schedule::Column::ExamSessionId.eq(session_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainExamSchedule {
            id: m.id, exam_session_id: m.exam_session_id, class_subject_id: m.class_subject_id,
            exam_date: m.exam_date, start_time: m.start_time, end_time: m.end_time,
            created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }

    async fn save(&self, schedule: &DomainExamSchedule) -> Result<(), DomainError> {
        let active = exam_schedule::ActiveModel {
            id: Set(schedule.id), exam_session_id: Set(schedule.exam_session_id),
            class_subject_id: Set(schedule.class_subject_id), exam_date: Set(schedule.exam_date),
            start_time: Set(schedule.start_time), end_time: Set(schedule.end_time),
            created_at: Set(schedule.created_at), updated_at: Set(schedule.updated_at),
        };

        if ExamScheduleEntity::find_by_id(schedule.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        ExamScheduleEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl SeatingNumberRepository for ExamRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainSeatingNumber>, DomainError> {
        let model = SeatingNumberEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainSeatingNumber {
            id: m.id, exam_session_id: m.exam_session_id,
            student_id: m.student_id, seating_number: m.seating_number,
            room_id: m.room_id, created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    async fn find_by_session(&self, session_id: Uuid) -> Result<Vec<DomainSeatingNumber>, DomainError> {
        let models = SeatingNumberEntity::find()
            .filter(seating_number::Column::ExamSessionId.eq(session_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainSeatingNumber {
            id: m.id, exam_session_id: m.exam_session_id,
            student_id: m.student_id, seating_number: m.seating_number,
            room_id: m.room_id, created_at: m.created_at,
            updated_at: m.updated_at,
        }).collect())
    }

    async fn find_by_session_and_student(&self, session_id: Uuid, student_id: Uuid) -> Result<Option<DomainSeatingNumber>, DomainError> {
        let model = SeatingNumberEntity::find()
            .filter(seating_number::Column::ExamSessionId.eq(session_id))
            .filter(seating_number::Column::StudentId.eq(student_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainSeatingNumber {
            id: m.id, exam_session_id: m.exam_session_id,
            student_id: m.student_id, seating_number: m.seating_number,
            room_id: m.room_id, created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    async fn save(&self, seating: &DomainSeatingNumber) -> Result<(), DomainError> {
        let active = seating_number::ActiveModel {
            id: Set(seating.id), exam_session_id: Set(seating.exam_session_id),
            student_id: Set(seating.student_id), seating_number: Set(seating.seating_number.clone()),
            room_id: Set(seating.room_id), created_at: Set(seating.created_at),
            updated_at: Set(seating.updated_at),
        };

        if SeatingNumberEntity::find_by_id(seating.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn save_bulk(&self, seatings: &[DomainSeatingNumber]) -> Result<(), DomainError> {
        for s in seatings {
            SeatingNumberRepository::save(self, s).await?;
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        SeatingNumberEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }

    async fn delete_by_session(&self, session_id: Uuid) -> Result<(), DomainError> {
        SeatingNumberEntity::delete_many()
            .filter(seating_number::Column::ExamSessionId.eq(session_id))
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl SecretNumberRepository for ExamRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainSecretNumber>, DomainError> {
        let model = SecretNumberEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainSecretNumber {
            id: m.id, exam_schedule_id: m.exam_schedule_id,
            student_id: m.student_id, secret_number: m.secret_number,
            created_at: m.created_at, updated_at: m.updated_at,
        }))
    }

    async fn find_by_schedule(&self, schedule_id: Uuid) -> Result<Vec<DomainSecretNumber>, DomainError> {
        let models = SecretNumberEntity::find()
            .filter(secret_number::Column::ExamScheduleId.eq(schedule_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainSecretNumber {
            id: m.id, exam_schedule_id: m.exam_schedule_id,
            student_id: m.student_id, secret_number: m.secret_number,
            created_at: m.created_at, updated_at: m.updated_at,
        }).collect())
    }

    async fn find_by_schedule_and_seating(&self, schedule_id: Uuid, seating_id: Uuid) -> Result<Option<DomainSecretNumber>, DomainError> {
        let model = SecretNumberEntity::find()
            .filter(secret_number::Column::ExamScheduleId.eq(schedule_id))
            .filter(secret_number::Column::StudentId.eq(seating_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainSecretNumber {
            id: m.id, exam_schedule_id: m.exam_schedule_id,
            student_id: m.student_id, secret_number: m.secret_number,
            created_at: m.created_at, updated_at: m.updated_at,
        }))
    }

    async fn save(&self, secret: &DomainSecretNumber) -> Result<(), DomainError> {
        let active = secret_number::ActiveModel {
            id: Set(secret.id), exam_schedule_id: Set(secret.exam_schedule_id),
            student_id: Set(secret.student_id), secret_number: Set(secret.secret_number.clone()),
            created_at: Set(secret.created_at), updated_at: Set(secret.updated_at),
        };

        if SecretNumberEntity::find_by_id(secret.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn save_bulk(&self, secrets: &[DomainSecretNumber]) -> Result<(), DomainError> {
        for s in secrets {
            SecretNumberRepository::save(self, s).await?;
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        SecretNumberEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }

    async fn delete_by_schedule(&self, schedule_id: Uuid) -> Result<(), DomainError> {
        SecretNumberEntity::delete_many()
            .filter(secret_number::Column::ExamScheduleId.eq(schedule_id))
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl ExamProctorRepository for ExamRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainExamProctor>, DomainError> {
        let model = ExamProctorEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainExamProctor {
            id: m.id, exam_schedule_id: m.exam_schedule_id,
            room_id: m.room_id, teacher_id: m.teacher_id,
            is_chief: m.is_chief, created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    async fn find_by_schedule(&self, schedule_id: Uuid) -> Result<Vec<DomainExamProctor>, DomainError> {
        let models = ExamProctorEntity::find()
            .filter(exam_proctor::Column::ExamScheduleId.eq(schedule_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainExamProctor {
            id: m.id, exam_schedule_id: m.exam_schedule_id,
            room_id: m.room_id, teacher_id: m.teacher_id,
            is_chief: m.is_chief, created_at: m.created_at,
            updated_at: m.updated_at,
        }).collect())
    }

    async fn save(&self, proctor: &DomainExamProctor) -> Result<(), DomainError> {
        let active = exam_proctor::ActiveModel {
            id: Set(proctor.id), exam_schedule_id: Set(proctor.exam_schedule_id),
            room_id: Set(proctor.room_id), teacher_id: Set(proctor.teacher_id),
            is_chief: Set(proctor.is_chief), created_at: Set(proctor.created_at),
            updated_at: Set(proctor.updated_at),
        };

        if ExamProctorEntity::find_by_id(proctor.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        ExamProctorEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }

    async fn delete_by_schedule(&self, schedule_id: Uuid) -> Result<(), DomainError> {
        ExamProctorEntity::delete_many()
            .filter(exam_proctor::Column::ExamScheduleId.eq(schedule_id))
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

