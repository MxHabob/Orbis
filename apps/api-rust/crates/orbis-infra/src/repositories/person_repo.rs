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
            middle_name: m.middle_name,
            last_name: m.last_name,
            first_name_ar: m.first_name_ar,
            middle_name_ar: m.middle_name_ar,
            last_name_ar: m.last_name_ar,
            display_name: m.display_name,
            national_id: m.national_id,
            passport_number: m.passport_number,
            gender: m.gender,
            birth_date: m.birth_date.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            place_of_birth: m.place_of_birth,
            nationality: m.nationality,
            blood_type: m.blood_type,
            medical_notes: m.medical_notes,
            special_needs_notes: m.special_needs_notes,
            phone: m.phone,
            email: m.email,
            address: m.address,
            avatar_url: m.avatar_url,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    async fn save(&self, person: &Person) -> Result<(), DomainError> {
        let active_model = person::ActiveModel {
            id: Set(person.id),
            org_id: Set(person.org_id),
            first_name: Set(person.first_name.clone()),
            middle_name: Set(person.middle_name.clone()),
            last_name: Set(person.last_name.clone()),
            first_name_ar: Set(person.first_name_ar.clone()),
            middle_name_ar: Set(person.middle_name_ar.clone()),
            last_name_ar: Set(person.last_name_ar.clone()),
            display_name: Set(person.display_name.clone()),
            national_id: Set(person.national_id.clone()),
            passport_number: Set(person.passport_number.clone()),
            gender: Set(person.gender.clone()),
            birth_date: Set(person.birth_date.map(|d| d.to_string())),
            place_of_birth: Set(person.place_of_birth.clone()),
            nationality: Set(person.nationality.clone()),
            blood_type: Set(person.blood_type.clone()),
            medical_notes: Set(person.medical_notes.clone()),
            special_needs_notes: Set(person.special_needs_notes.clone()),
            phone: Set(person.phone.clone()),
            email: Set(person.email.clone()),
            address: Set(person.address.clone()),
            avatar_url: Set(person.avatar_url.clone()),
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
