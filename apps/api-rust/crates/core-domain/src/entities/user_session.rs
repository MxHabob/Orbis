use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "user_sessions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub user_id: String,
    #[sea_orm(unique)]
    pub token: String,
    #[sea_orm(unique)]
    pub refresh_token: Option<String>,
    pub device_id: Option<String>,
    pub device_type: Option<String>,
    pub device_name: Option<String>,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub is_current: i32,
    pub is_trusted: i32,
    pub mfa_verified: i32,
    pub expires_at: String,
    pub refresh_expires_at: Option<String>,
    pub last_active_at: Option<String>,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
