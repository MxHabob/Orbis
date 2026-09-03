use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "academic_override_requests")]
#[schema(as = AcademicOverrideRequest)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,

    // Request context
    pub override_type: String,
    pub risk_level: String,

    // What is being overridden
    pub entity_type: String,
    pub entity_id: String,
    pub field_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,

    // Request info
    pub reason: String,
    pub requested_by: String,
    pub requested_by_name: Option<String>,
    pub requested_at: String,

    // Current approval state
    pub status: String,
    pub current_approver_role: Option<String>,

    // Approval chain (JSON)
    pub approval_chain: String,

    // Resolution
    pub resolved_by: Option<String>,
    pub resolved_by_name: Option<String>,
    pub resolved_at: Option<String>,
    pub resolution_notes: Option<String>,

    // Expiry
    pub expires_at: Option<String>,

    // Metadata
    pub metadata: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
