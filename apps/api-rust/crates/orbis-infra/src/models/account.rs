use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "accounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<Uuid>,
    pub currency_code: String,
    pub is_active: bool,
    pub balances: Json,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub version: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
