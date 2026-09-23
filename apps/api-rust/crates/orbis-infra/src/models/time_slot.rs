use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc, NaiveTime};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "time_slots")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub period_id: Uuid,
    pub day_of_week: i16,
    pub slot_index: i32,
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub is_break: bool,
    pub break_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
