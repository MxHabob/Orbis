use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::models::person::{self, Entity as PersonEntity};
use orbis_app::ports::repositories::PersonRepository;
use orbis_domain::entities::person::Person;
use orbis_domain::errors::DomainError;

pub struct PersonRepoImpl {
    pub db: DatabaseConnection,
}

fn map_model_to_person(m: person::Model) -> Person {
    Person {
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
        birth_date: m.birth_date,
        place_of_birth: m.place_of_birth,
        nationality: m.nationality,
        blood_type: m.blood_type,
        marital_status: m.marital_status,
        medical_notes: m.medical_notes,
        special_needs_notes: m.special_needs_notes,
        phone: m.phone,
        email: m.email,
        address: m.address,
        avatar_url: m.avatar_url,
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

#[async_trait]
impl PersonRepository for PersonRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Person>, DomainError> {
        let model = PersonEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(map_model_to_person))
    }

    async fn find_by_national_id(
        &self,
        org_id: Option<Uuid>,
        national_id: &str,
    ) -> Result<Option<Person>, DomainError> {
        let mut query = PersonEntity::find().filter(person::Column::NationalId.eq(national_id));
        if let Some(oid) = org_id {
            query = query.filter(person::Column::OrgId.eq(oid));
        }
        let model = query
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_model_to_person))
    }

    async fn find_by_email(
        &self,
        org_id: Option<Uuid>,
        email: &str,
    ) -> Result<Option<Person>, DomainError> {
        let mut query = PersonEntity::find().filter(person::Column::Email.eq(email));
        if let Some(oid) = org_id {
            query = query.filter(person::Column::OrgId.eq(oid));
        }
        let model = query
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_model_to_person))
    }

    async fn find_by_phone(
        &self,
        org_id: Option<Uuid>,
        phone: &str,
    ) -> Result<Option<Person>, DomainError> {
        let mut query = PersonEntity::find().filter(person::Column::Phone.eq(phone));
        if let Some(oid) = org_id {
            query = query.filter(person::Column::OrgId.eq(oid));
        }
        let model = query
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_model_to_person))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<Person>, DomainError> {
        let models = PersonEntity::find()
            .filter(person::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_model_to_person).collect())
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
            birth_date: Set(person.birth_date),
            place_of_birth: Set(person.place_of_birth.clone()),
            nationality: Set(person.nationality.clone()),
            blood_type: Set(person.blood_type.clone()),
            marital_status: Set(person.marital_status.clone()),
            medical_notes: Set(person.medical_notes.clone()),
            special_needs_notes: Set(person.special_needs_notes.clone()),
            phone: Set(person.phone.clone()),
            email: Set(person.email.clone()),
            address: Set(person.address.clone()),
            avatar_url: Set(person.avatar_url.clone()),
            created_at: Set(person.created_at),
            updated_at: Set(person.updated_at),
        };

        if PersonEntity::find_by_id(person.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active_model
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active_model
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }

        Ok(())
    }
}
