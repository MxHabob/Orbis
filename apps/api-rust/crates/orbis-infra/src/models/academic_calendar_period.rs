use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "academic_calendar_periods")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub calendar_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub period_type_id: Uuid,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub sort_order: i32,
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
        belongs_to = "super::academic_calendar::Entity",
        from = "Column::CalendarId",
        to = "super::academic_calendar::Column::Id"
    )]
    Calendar,
    #[sea_orm(
        belongs_to = "super::academic_period_type::Entity",
        from = "Column::PeriodTypeId",
        to = "super::academic_period_type::Column::Id"
    )]
    PeriodType,
    #[sea_orm(belongs_to = "Entity", from = "Column::ParentId", to = "Column::Id")]
    Parent,
}

impl Related<super::academic_calendar::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Calendar.def()
    }
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::academic_period_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PeriodType.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
