use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "persons")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    pub org_id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub full_name: Option<String>,
    pub national_id: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
