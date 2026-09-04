use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "notifications")]
#[schema(as = Notification)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub user_id: String,
    pub title: String,
    pub title_ar: Option<String>,
    pub message: String,
    pub message_ar: Option<String>,
    #[sea_orm(column_name = "type")]
    pub notification_type: String,
    pub priority: String,
    pub is_read: i32,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
