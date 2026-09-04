use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use orbis_app::ports::repositories::PersonRepository;
use orbis_domain::entities::person::Person;
use orbis_domain::errors::DomainError;
use crate::models::person::{self, Entity as PersonEntity};

pub struct PersonRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl PersonRepository for PersonRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Person>, DomainError> {
        let model = PersonEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(|m| Person {
            id: m.id,
            party_id: m.party_id,
            org_id: m.org_id,
            first_name: m.first_name,
            last_name: m.last_name,
            full_name: m.full_name,
            national_id: m.national_id,
            gender: m.gender,
            date_of_birth: m.date_of_birth,
            nationality: m.nationality,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    async fn save(&self, person: &Person) -> Result<(), DomainError> {
        let active_model = person::ActiveModel {
            id: Set(person.id),
            party_id: Set(person.party_id),
            org_id: Set(person.org_id),
            first_name: Set(person.first_name.clone()),
            last_name: Set(person.last_name.clone()),
            full_name: Set(person.full_name.clone()),
            national_id: Set(person.national_id.clone()),
            gender: Set(person.gender.clone()),
            date_of_birth: Set(person.date_of_birth.clone()),
            nationality: Set(person.nationality.clone()),
            created_at: Set(person.created_at),
            updated_at: Set(person.updated_at),
        };

        if PersonEntity::find_by_id(person.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active_model.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active_model.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }

        Ok(())
    }
}
