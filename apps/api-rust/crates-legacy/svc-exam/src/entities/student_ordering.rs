use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "student_ordering")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub period_id: String,
    #[sea_orm(unique)]
    pub enrollment_id: String,
    pub student_id: String,
    pub class_id: String,
    pub sort_key: String,
    pub order_in_class: i32,
    pub order_in_division: Option<i32>,
    pub order_in_level: Option<i32>,
    pub order_in_org: Option<i32>,
    pub level: String,
    pub division: Option<String>,
    pub student_name_ar: Option<String>,
    pub student_sex: Option<String>,
    pub academic_number: Option<String>,
    pub is_frozen: i32,
    pub frozen_at: Option<String>,
    pub frozen_by: Option<String>,
    pub strategy: String,
    pub version: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
