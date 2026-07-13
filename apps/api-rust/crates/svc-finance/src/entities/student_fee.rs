use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "student_fees")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    #[sea_orm(column_name = "year_id")]
    pub period_id: String,
    pub enrollment_id: String,
    pub fee_template_id: String,
    pub original_amount: f64,
    pub discount_amount: f64,
    pub net_amount: f64,
    pub status: String,
    pub due_date: String,
    pub discount_details: Option<String>,
    pub parent_fee_id: Option<String>,
    pub installment_number: Option<i32>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
