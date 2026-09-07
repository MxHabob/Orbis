use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "academic_override_requests")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub override_type: String,
    pub risk_level: String,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub field_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub reason: String,
    pub requested_by: Uuid,
    pub requested_by_name: Option<String>,
    pub status: String,
    pub current_approver_role: Option<String>,
    pub approval_chain: Json,
    pub resolved_by: Option<Uuid>,
    pub resolved_by_name: Option<String>,
    pub resolved_at: Option<DateTimeUtc>,
    pub resolution_notes: Option<String>,
    pub expires_at: Option<DateTimeUtc>,
    pub metadata: Option<Json>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
