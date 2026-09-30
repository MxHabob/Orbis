use async_trait::async_trait;
use chrono::Utc;
use orbis_app::ports::repositories::SequenceCounterRepository;
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::models::sequence_counter::{self, Entity as SequenceCounterEntity};

pub struct SequenceCounterRepoImpl {
    pub db: DatabaseConnection,
}

impl SequenceCounterRepoImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SequenceCounterRepository for SequenceCounterRepoImpl {
    async fn next_sequence(&self, org_id: Uuid, sequence_type: &str, academic_year: i32) -> Result<u64, DomainError> {
        let now = Utc::now();
        let existing = SequenceCounterEntity::find()
            .filter(sequence_counter::Column::OrgId.eq(org_id))
            .filter(sequence_counter::Column::SequenceType.eq(sequence_type))
            .filter(sequence_counter::Column::AcademicYear.eq(academic_year))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if let Some(model) = existing {
            let next_val = model.current_val + 1;
            let mut active: sequence_counter::ActiveModel = model.into();
            active.current_val = Set(next_val);
            active.updated_at = Set(now);
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
            Ok(next_val as u64)
        } else {
            let next_val = 1i64;
            let active = sequence_counter::ActiveModel {
                id: Set(Uuid::new_v4()),
                org_id: Set(org_id),
                sequence_type: Set(sequence_type.to_string()),
                academic_year: Set(academic_year),
                current_val: Set(next_val),
                updated_at: Set(now),
            };
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
            Ok(next_val as u64)
        }
    }
}
