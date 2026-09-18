use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "organizations")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    #[sea_orm(unique)]
    pub slug: String,
    #[sea_orm(unique)]
    pub code: Option<String>,
    pub timezone: String,
    pub locale: String,
    pub date_format: Option<String>,
    pub is_hijri_enabled: bool,
    pub base_currency_code: Option<String>,
    pub settings: Option<serde_json::Value>,
    pub features: Option<serde_json::Value>,
    pub logo: Option<String>,
    pub favicon: Option<String>,
    pub primary_color: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
    pub current_period_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::role::Entity")]
    Roles,
    #[sea_orm(has_many = "super::autopilot_task::Entity")]
    AutopilotTasks,
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Roles.def()
    }
}

impl Related<super::autopilot_task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AutopilotTasks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
