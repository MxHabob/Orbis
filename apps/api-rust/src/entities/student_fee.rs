use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "student_fees")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub cycle_id: String,
    pub enrollment_id: String,
    pub fee_template_id: String,
    pub original_amount: f64,
    pub discount_amount: f64,
    pub late_fee_amount: f64,
    pub total_amount: f64,
    pub paid_amount: f64,
    pub balance: f64,
    pub applied_discounts: Option<String>,
    pub payment_plan_id: Option<String>,
    pub status: String,
    pub due_date: String,
    pub is_overdue: i32,
    pub overdue_since: Option<String>,
    pub overdue_days: Option<i32>,
    pub is_installment: i32,
    pub parent_fee_id: Option<String>,
    pub installment_number: Option<i32>,
    pub total_installments: Option<i32>,
    pub reminder_sent: Option<i32>,
    pub reminder_sent_at: Option<String>,
    pub final_notice_sent: Option<i32>,
    pub final_notice_sent_at: Option<String>,
    pub is_locked: i32,
    pub locked_at: Option<String>,
    pub locked_by: Option<String>,
    pub notes: Option<String>,
    pub meta: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
