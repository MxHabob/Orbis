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
            org_id: m.org_id,
            first_name: m.first_name,
            last_name: m.last_name,
            full_name: m.full_name,
            name_ar: m.name_ar,
            national_id: m.national_id,
            passport_number: m.passport_number,
            gender: m.gender,
            date_of_birth: m.date_of_birth.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            place_of_birth: m.place_of_birth,
            nationality: m.nationality,
            blood_type: m.blood_type,
            medical_notes: m.medical_notes,
            special_needs_notes: m.special_needs_notes,
            phone: m.phone,
            email: m.email,
            address: m.address,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    async fn save(&self, person: &Person) -> Result<(), DomainError> {
        let active_model = person::ActiveModel {
            id: Set(person.id),
            org_id: Set(person.org_id),
            first_name: Set(person.first_name.clone()),
            last_name: Set(person.last_name.clone()),
            full_name: Set(person.full_name.clone()),
            name_ar: Set(person.name_ar.clone()),
            national_id: Set(person.national_id.clone()),
            passport_number: Set(person.passport_number.clone()),
            gender: Set(person.gender.clone()),
            date_of_birth: Set(person.date_of_birth.map(|d| d.to_string())),
            place_of_birth: Set(person.place_of_birth.clone()),
            nationality: Set(person.nationality.clone()),
            blood_type: Set(person.blood_type.clone()),
            medical_notes: Set(person.medical_notes.clone()),
            special_needs_notes: Set(person.special_needs_notes.clone()),
            phone: Set(person.phone.clone()),
            email: Set(person.email.clone()),
            address: Set(person.address.clone()),
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
