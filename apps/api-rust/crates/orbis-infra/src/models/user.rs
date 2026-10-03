use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub person_id: Uuid,
    pub org_id: Uuid,
    pub email: Option<String>,
    pub is_email_verified: bool,
    pub phone: Option<String>,
    pub is_phone_verified: bool,
    pub password_hash: Option<String>,
    pub is_mfa_enabled: bool,
    pub status: String,
    pub login_count: i32,
    pub failed_login_attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub is_deleted: bool,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrgId",
        to = "super::organization::Column::Id"
    )]
    Organization,
    #[sea_orm(
        belongs_to = "super::person::Entity",
        from = "Column::PersonId",
        to = "super::person::Column::Id"
    )]
    Person,
    #[sea_orm(has_many = "super::user_session::Entity")]
    UserSessions,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Person.def()
    }
}

impl Related<super::user_session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserSessions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
