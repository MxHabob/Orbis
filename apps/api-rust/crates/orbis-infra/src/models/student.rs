use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "students")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub person_id: Uuid,
    pub user_id: Option<Uuid>,
    pub student_number: String,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub name_ar: Option<String>,
    pub gender: String,
    pub date_of_birth: Option<Date>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub status: String,
    pub standing: String,
    pub admitted_at: DateTimeUtc,
    pub graduated_at: Option<DateTimeUtc>,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTimeUtc>,
    pub meta: Option<Json>,
    pub org_unit_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::person::Entity",
        from = "Column::PersonId",
        to = "super::person::Column::Id"
    )]
    Person,
    #[sea_orm(has_many = "super::enrollment::Entity")]
    Enrollment,
}

impl Related<super::person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Person.def()
    }
}

impl Related<super::enrollment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Enrollment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

