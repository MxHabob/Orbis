use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "user_accounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub user_id: String,
    pub provider: String,
    pub provider_account_id: String,
    pub provider_email: Option<String>,
    pub provider_name: Option<String>,
    pub provider_picture: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
    pub expires_at: Option<String>,
    pub scope: Option<String>,
    pub token_type: Option<String>,
    pub meta: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
