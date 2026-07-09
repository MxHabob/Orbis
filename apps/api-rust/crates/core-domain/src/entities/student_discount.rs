use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "student_discounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub enrollment_id: String,
    pub discount_template_id: String,
    pub applied_amount: f64,
    pub applied_to_fee_id: Option<String>,
    pub status: String,
    pub reason: Option<String>,
    pub evidence_urls: Option<String>,
    pub applied_by: Option<String>,
    pub applied_at: Option<String>,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
    pub approval_notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
