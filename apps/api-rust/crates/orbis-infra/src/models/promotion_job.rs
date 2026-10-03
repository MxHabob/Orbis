use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "promotion_jobs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub source_period_id: Uuid,
    pub target_period_id: Option<Uuid>,
    pub name: String,
    pub status: String,
    pub total_students: i32,
    pub promoted_count: i32,
    pub retained_count: i32,
    pub graduated_count: i32,
    pub excluded_count: i32,
    pub executed_by: Option<Uuid>,
    pub executed_at: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
