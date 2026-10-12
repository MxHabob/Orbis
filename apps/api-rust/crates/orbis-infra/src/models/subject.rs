use sea_orm::entity::prelude::*;

/// Unified subject/course/module catalog.
/// Works for all institution types through subject_type:
/// - Schools: "Mathematics" (subject_type = subject)
/// - Universities: "CS101 Data Structures" (subject_type = course)
/// - Institutes: "Welding Module 1" (subject_type = module)
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
    /// subject, course, module
    pub subject_type: String,
    pub is_core: bool,
    pub is_elective: bool,
    pub credit_units: Option<Decimal>,
    pub workload_units: Option<Decimal>,
    pub theory_hours: Option<Decimal>,
    pub practical_hours: Option<Decimal>,
    pub is_lab_required: bool,
    pub max_students_per_section: Option<i32>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
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
