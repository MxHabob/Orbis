use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Tracks a single promotion operation (year-end academic promotion).
/// Lives in `orbis_registry.db`.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "promotion_jobs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub source_period_id: String,
    pub target_period_id: Option<String>,
    pub status: String,
    pub initiated_by: String,
    pub initiated_by_name: Option<String>,

    /// Serialized PromotionConfig JSON
    pub config: String,

    /// Serialized PromotionPreview JSON (populated after dry_run)
    pub dry_run_result: Option<String>,
    pub dry_run_at: Option<String>,

    // Execution statistics
    pub total_students: Option<i32>,
    pub promoted_count: Option<i32>,
    pub retained_count: Option<i32>,
    pub graduated_count: Option<i32>,
    pub withdrawn_count: Option<i32>,
    pub excluded_count: Option<i32>,

    // Safety & recovery
    pub error_log: Option<String>,
    pub rollback_available: i32,

    // Timestamps
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub rolled_back_at: Option<String>,
    pub rolled_back_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
