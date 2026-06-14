use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "users")]
#[schema(as = User)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub email: Option<String>,
    pub email_verified: i32,
    pub email_verified_at: Option<String>,
    pub phone: Option<String>,
    pub phone_verified: i32,
    pub phone_verified_at: Option<String>,
    pub password_hash: Option<String>,
    pub password_changed_at: Option<String>,
    pub password_expires_at: Option<String>,
    pub mfa_enabled: i32,
    pub mfa_secret: Option<String>,
    pub mfa_backup_codes: Option<String>,
    pub full_name: String,
    pub name_ar: Option<String>,
    pub name_en: Option<String>,
    pub avatar: Option<String>,
    pub sex: Option<String>,
    pub date_of_birth: Option<String>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub emergency_contact: Option<String>,
    pub blood_type: Option<String>,
    pub allergies: Option<String>,
    pub medical_conditions: Option<String>,
    pub medical_notes: Option<String>,
    pub social_links: Option<String>,
    pub bio: Option<String>,
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
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrgId",
        to = "super::organization::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Organization,
    #[sea_orm(has_many = "super::user_session::Entity")]
    UserSession,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::user_session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserSession.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
