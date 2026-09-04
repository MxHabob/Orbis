use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ai_insights")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub insight_type: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub scope: String,
    pub title: String,
    pub title_ar: Option<String>,
    pub summary: String,
    pub summary_ar: Option<String>,
    pub details: Option<String>,
    /// "info" | "suggestion" | "warning" | "alert" | "critical"
    pub severity: String,
    pub confidence: f64,
    pub impact_score: Option<f64>,
    pub suggested_action: Option<String>,
    pub suggested_params: Option<String>,
    /// "new" | "seen" | "acknowledged" | "acted_upon" | "dismissed" | "expired"
    pub status: String,
    pub acknowledged_by: Option<String>,
    pub acknowledged_at: Option<String>,
    pub generated_by: String,
    pub valid_from: String,
    pub valid_until: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub mod severity {
    pub const INFO: &str = "info";
    pub const SUGGESTION: &str = "suggestion";
    pub const WARNING: &str = "warning";
    pub const ALERT: &str = "alert";
    pub const CRITICAL: &str = "critical";
}

pub mod insight_status {
    pub const NEW: &str = "new";
    pub const SEEN: &str = "seen";
    pub const ACKNOWLEDGED: &str = "acknowledged";
    pub const ACTED_UPON: &str = "acted_upon";
    pub const DISMISSED: &str = "dismissed";
    pub const EXPIRED: &str = "expired";
}
