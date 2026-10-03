use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "subjects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub is_core: bool,
    pub is_elective: bool,
    pub credit_hours: Option<Decimal>,
    pub theory_hours: Option<Decimal>,
    pub practical_hours: Option<Decimal>,
    pub is_lab_required: bool,
    pub max_students_per_class: Option<i32>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
