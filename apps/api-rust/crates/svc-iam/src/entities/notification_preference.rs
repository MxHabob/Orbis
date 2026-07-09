use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "notification_preferences")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub user_id: String,
    pub notification_type: String,
    pub in_app_enabled: i32,
    pub email_enabled: i32,
    pub sms_enabled: i32,
    pub push_enabled: i32,
    pub whatsapp_enabled: i32,
    pub quiet_hours_enabled: i32,
    pub quiet_hours_start: Option<String>,
    pub quiet_hours_end: Option<String>,
    pub digest_mode: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
