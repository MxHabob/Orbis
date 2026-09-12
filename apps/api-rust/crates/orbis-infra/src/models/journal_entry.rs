use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "journal_entries")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub reference_number: String,
    pub accounting_date: Date,
    pub amount: Decimal,
    pub currency_code: String,
    pub exchange_rate: Decimal,
    pub base_amount: Decimal,
    pub debit_account_id: Uuid,
    pub credit_account_id: Uuid,
    pub description: String,
    pub description_ar: Option<String>,
    pub source_type: Option<String>,
    pub source_id: Option<Uuid>,
    pub status: Option<String>,
    pub is_reversed: Option<bool>,
    pub reversed_by: Option<Uuid>,
    pub reversal_reason: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
