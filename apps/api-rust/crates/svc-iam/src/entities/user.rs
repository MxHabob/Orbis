use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "users")]
#[schema(as = User)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub person_id: String,
    pub party_id: Option<String>,
    pub org_id: String,
    pub email: Option<String>,
    pub email_verified: i32,
    pub email_verified_at: Option<String>,
    pub phone: Option<String>,
    pub phone_verified: i32,
    pub phone_verified_at: Option<String>,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub password_changed_at: Option<String>,
    pub password_expires_at: Option<String>,
    pub mfa_enabled: i32,
    #[serde(skip_serializing)]
    pub mfa_secret: Option<String>,
    #[serde(skip_serializing)]
    pub mfa_backup_codes: Option<String>,
    pub full_name: String,
    pub primary_role: String,
    pub status: String,
    pub last_login_at: Option<String>,
    pub last_login_ip: Option<String>,
    pub login_count: Option<i32>,
    pub failed_logins: Option<i32>,
    pub locked_until: Option<String>,
    pub prefs: String,
    pub meta: String,
    pub is_deleted: i32,
    pub deleted_at: Option<String>,
    pub deleted_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub created_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_session::Entity")]
    UserSession,
}

impl Related<super::user_session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserSession.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
