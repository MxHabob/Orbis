use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "documents")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub owner_id: String,
    pub uploaded_by: Option<String>,
    pub name: String,
    pub title: Option<String>,
    pub description: Option<String>,
    #[sea_orm(column_name = "type")]
    pub item_type: String,
    pub visibility: String,
    pub size: Option<i32>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub storage_provider: String,
    pub bucket: Option<String>,
    pub key: String,
    pub url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub parent_id: Option<String>,
    pub path: Option<String>,
    pub depth: Option<i32>,
    pub shared_with: Option<String>,
    pub share_link: Option<String>,
    pub share_link_expires_at: Option<String>,
    pub share_password: Option<String>,
    pub version: Option<i32>,
    pub previous_version_id: Option<String>,
    pub is_latest_version: i32,
    pub processing_status: Option<String>,
    pub ocr_text: Option<String>,
    pub extracted_data: Option<String>,
    pub is_pinned: i32,
    pub is_starred: i32,
    pub is_template: i32,
    pub tags: Option<String>,
    pub meta: Option<String>,
    pub deleted_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
