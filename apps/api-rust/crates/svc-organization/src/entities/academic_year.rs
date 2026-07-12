use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Academic Year — the top-level time container for all academic data.
///
/// Lives in the unified `orbis.db` (no separate registry or year databases).
///
/// Organization: planning → active → closing → archived
/// Lock: `is_locked` is independent from `status` — allows temporary
///       read-only mode during an active year (e.g., grade review).
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "academic_years")]
#[schema(as = AcademicYear)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub hijri_start: Option<String>,
    pub hijri_end: Option<String>,

    /// Organization status: planning, active, closing, archived
    pub status: String,

    /// Operational lock — independent from status.
    /// An active year can be temporarily locked (e.g., during grade audit).
    pub is_locked: i32,
    pub locked_at: Option<String>,
    pub locked_by: Option<String>,
    pub lock_reason: Option<String>,

    /// Archive audit trail
    pub archived_at: Option<String>,
    pub archived_by: Option<String>,

    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Model {
    /// Is this year writable (not archived and not locked)?
    pub fn is_writable(&self) -> bool {
        self.status != "archived" && self.is_locked == 0
    }

    /// Can students be enrolled in this year?
    pub fn can_enroll(&self) -> bool {
        self.is_writable() && self.status == "active"
    }

    /// Can grades be entered for this year?
    pub fn can_enter_grades(&self) -> bool {
        self.is_writable() && (self.status == "active" || self.status == "closing")
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrgId",
        to = "super::organization::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Organization,

    #[sea_orm(has_many = "super::academic_period::Entity")]
    AcademicPeriods,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::academic_period::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicPeriods.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

