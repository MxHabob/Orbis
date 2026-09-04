use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pass_fail_policies")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub assessment_scheme_id: Uuid,
    #[sea_orm(column_type = "Double")]
    pub pass_score_threshold: f64,
    #[sea_orm(column_type = "Double", nullable)]
    pub attendance_requirement: Option<f64>,
    pub rules_json: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
