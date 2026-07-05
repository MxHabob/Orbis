use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "promotion_policies")]
#[schema(as = PromotionPolicy)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,

    // Scoring
    pub passing_score: i32,
    pub min_attendance_rate: f64,

    // Graduation
    pub auto_graduate_final_level: i32,
    pub final_level: Option<String>,

    // Exclusions
    pub exclude_withdrawn: i32,

    // Fee carry-over policy: ignore | carry_all | carry_selected | require_approval
    pub fee_carry_over_mode: String,
    pub fee_carry_over_categories: Option<String>,

    // Data carry-over
    pub carry_over_guardians: i32,
    pub carry_over_time_slots: i32,
    pub carry_over_class_subjects: i32,

    // Auto-create academic structure
    pub auto_create_cycles: i32,
    pub cycle_template: Option<String>,

    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
