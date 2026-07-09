use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "audit_log")]
#[schema(as = AuditLog)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub user_id: Option<String>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: String,
    pub old_data: Option<String>,
    pub new_data: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub session_id: Option<String>,
    pub metadata: String,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
