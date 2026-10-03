use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "academic_years")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub hijri_start: Option<String>,
    pub hijri_end: Option<String>,
    pub status: String,
    pub is_locked: bool,
    pub locked_at: Option<DateTime<Utc>>,
    pub locked_by: Option<Uuid>,
    pub lock_reason: Option<String>,
    pub current_period_id: Option<Uuid>,
    pub archived_at: Option<DateTime<Utc>>,
    pub archived_by: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
