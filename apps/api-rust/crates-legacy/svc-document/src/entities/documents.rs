use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "documents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub uploaded_by: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub asset_type: String,
    pub school_year: Option<String>,
    pub term: Option<String>,
    pub tags: Option<String>,
    pub visibility: String,
    pub size: Option<i32>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub storage_provider: String,
    pub path: String,
    pub url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub preview_path: Option<String>,
    pub deleted_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
