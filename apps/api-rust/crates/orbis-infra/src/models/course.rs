use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "courses")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub credit_units: Option<Decimal>,
    pub workload_units: Option<Decimal>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrgId",
        to = "super::organization::Column::Id"
    )]
    Organization,
    #[sea_orm(
        belongs_to = "super::org_unit::Entity",
        from = "Column::UnitId",
        to = "super::org_unit::Column::Id"
    )]
    Unit,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::org_unit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Unit.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
