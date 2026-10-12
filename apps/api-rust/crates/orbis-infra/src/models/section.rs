use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

/// Unified grouping entity for all institution types.
/// - Schools: a stable class/division (e.g. "Grade 1 / Section A")
/// - Universities: a course section (e.g. "CS101 - Section 01")
/// - Institutes: a cohort or training group (e.g. "Cohort 2025-A")
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sections")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub org_unit_id: Option<Uuid>,
    /// Type discriminator: class, course_section, cohort, group
    pub section_type: String,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    /// Academic level (e.g. grade_1, year_2, level_3) — configurable per institution
    pub level: Option<String>,
    /// Optional subdivision (e.g. science_track, literary_track)
    pub track: Option<String>,
    pub capacity: Option<i32>,
    pub color: Option<String>,
    pub is_active: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
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
        from = "Column::OrgUnitId",
        to = "super::org_unit::Column::Id"
    )]
    OrgUnit,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::org_unit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrgUnit.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
