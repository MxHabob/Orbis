use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "fee_templates")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub category: Option<String>,
    pub default_amount: Decimal,
    pub currency_code: Option<String>,
    pub revenue_account_id: Uuid,
    pub receivable_account_id: Uuid,
    pub applicable_levels: Option<Json>,
    pub applicable_study_modes: Option<Json>,
    pub frequency: Option<String>,
    pub is_mandatory: bool,
    pub is_active: Option<bool>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
