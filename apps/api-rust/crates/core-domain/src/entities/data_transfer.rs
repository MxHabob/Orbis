use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "data_transfers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    #[sea_orm(column_name = "type")]
    pub item_type: String,
    pub entity_type: String,
    pub file_name: Option<String>,
    pub file_url: Option<String>,
    pub file_size: Option<i32>,
    pub file_format: Option<String>,
    pub total_records: Option<i32>,
    pub success_records: Option<i32>,
    pub failed_records: Option<i32>,
    pub skipped_records: Option<i32>,
    pub error_log: Option<String>,
    pub warning_log: Option<String>,
    pub status: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub cancelled_at: Option<String>,
    pub cancel_reason: Option<String>,
    pub initiated_by: Option<String>,
    pub config: Option<String>,
    pub mapping: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
