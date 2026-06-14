use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "student_guardians")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub student_id: String,
    pub guardian_id: String,
    pub relationship: String,
    pub relationship_ar: Option<String>,
    pub is_primary: i32,
    pub is_emergency_contact: i32,
    pub is_authorized_pickup: i32,
    pub can_view_grades: i32,
    pub can_view_attendance: i32,
    pub can_view_behavior: i32,
    pub can_view_finance: i32,
    pub can_view_schedule: i32,
    pub can_receive_notifications: i32,
    pub is_payer: i32,
    pub payment_share_percent: Option<f64>,
    pub occupation: Option<String>,
    pub employer: Option<String>,
    pub work_phone: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
