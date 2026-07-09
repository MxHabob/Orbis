use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "academic_calendar_events")]
#[schema(as = AcademicCalendarEvent)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub cycle_id: String,

    // Event identity
    pub event_type: String,
    pub title: String,
    pub title_ar: Option<String>,
    pub description: Option<String>,

    // Timing
    pub start_date: String,
    pub end_date: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,

    // Recurrence
    pub is_recurring: i32,
    pub recurrence_rule: Option<String>,

    // Display
    pub color: Option<String>,
    pub is_active: i32,
    pub sort_order: Option<i32>,
    pub created_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::academic_cycle::Entity",
        from = "Column::CycleId",
        to = "super::academic_cycle::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    AcademicCycle,
}

impl Related<super::academic_cycle::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicCycle.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
