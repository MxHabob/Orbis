use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Student-level pass/fail outcome for a period (not per subject).
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "period_overall_results")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub student_id: String,
    pub enrollment_id: String,
    pub period_id: String,
    pub passed: i32,
    pub failed_subject_count: i32,
    pub policy_id: Option<String>,
    pub decision: String,
    pub percentage: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
