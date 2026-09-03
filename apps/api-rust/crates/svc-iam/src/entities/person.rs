use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "persons")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub party_id: String,
    pub org_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub full_name: Option<String>,
    pub name_ar: Option<String>,
    pub name_en: Option<String>,
    pub father_name: Option<String>,
    pub grandfather_name: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<String>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    pub emergency_contact: Option<String>,
    pub blood_type: Option<String>,
    pub allergies: Option<String>,
    pub medical_conditions: Option<String>,
    pub medical_notes: Option<String>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub social_links: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::party::Entity",
        from = "Column::PartyId",
        to = "super::party::Column::Id"
    )]
    Party,
}

impl Related<super::party::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Party.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
