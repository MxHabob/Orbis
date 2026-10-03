use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "autopilot_tasks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub task_type: String,
    pub status: String,
    pub target_id: Option<Uuid>,
    pub target_type: Option<String>,
    pub risk_level: String,
    pub reason: Option<String>,
    pub parameters: Option<serde_json::Value>,
    pub scheduled_at: DateTimeUtc,
    pub executed_at: Option<DateTimeUtc>,
    pub attempts: i32,
    pub max_retries: i32,
    pub error_log: Option<String>,
    pub idempotency_key: Option<String>,
    pub triggered_by: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrgId",
        to = "super::organization::Column::Id"
    )]
    Organization,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

