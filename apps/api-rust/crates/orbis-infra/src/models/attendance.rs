use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "attendances")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub student_id: Uuid,
    pub class_id: Option<Uuid>,
    pub class_subject_id: Option<Uuid>,
    pub scheduled_lesson_id: Option<Uuid>,
    pub date: NaiveDate,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
