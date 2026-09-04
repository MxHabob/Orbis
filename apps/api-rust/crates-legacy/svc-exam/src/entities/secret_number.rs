use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "secret_numbers")]
#[schema(as = SecretNumber)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub exam_session_id: String,
    pub enrollment_id: String,
    pub student_id: String,
    pub secret_number: String,
    pub secret_hash: String,
    pub barcode_data: Option<String>,
    pub qr_data: Option<String>,
    pub is_active: i32,
    pub is_revealed: i32,
    pub revealed_at: Option<String>,
    pub revealed_by: Option<String>,
    pub generated_at: String,
    pub generated_by: String,
    pub generation_batch: String,
    pub previous_secret_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
