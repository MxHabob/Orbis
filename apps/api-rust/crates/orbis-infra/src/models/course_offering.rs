use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "course_offerings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub course_id: Uuid,
    pub period_id: Option<Uuid>,
    pub section_code: Option<String>,
    pub name: Option<String>,
    pub status: String,
    pub capacity: Option<i32>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
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
        belongs_to = "super::course::Entity",
        from = "Column::CourseId",
        to = "super::course::Column::Id"
    )]
    Course,
    #[sea_orm(
        belongs_to = "super::academic_calendar_period::Entity",
        from = "Column::PeriodId",
        to = "super::academic_calendar_period::Column::Id"
    )]
    Period,
}

impl Related<super::course::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Course.def()
    }
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::academic_calendar_period::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Period.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
