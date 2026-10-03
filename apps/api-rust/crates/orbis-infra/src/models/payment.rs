use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "payments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub fee_id: Option<Uuid>,
    pub enrollment_id: Option<Uuid>,
    pub student_id: Option<Uuid>,
    pub account_id: Uuid,
    pub amount: Decimal,
    pub currency_code: Option<String>,
    pub exchange_rate: Option<Decimal>,
    pub base_amount: Option<Decimal>,
    pub method: String,
    pub reference: Option<String>,
    pub receipt_number: Option<String>,
    pub journal_entry_id: Option<Uuid>,
    pub payer_name: Option<String>,
    pub payer_phone: Option<String>,
    pub notes: Option<String>,
    pub paid_at: DateTimeWithTimeZone,
    pub created_by: Option<Uuid>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub org_unit_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
