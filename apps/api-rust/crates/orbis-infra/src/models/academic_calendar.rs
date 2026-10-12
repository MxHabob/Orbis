use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

/// Unified academic calendar — the top-level temporal container.
/// Replaces both `academic_calendars` and old `academic_calendars`.
/// Works for any institution type:
/// - Schools: "2025-2026 Academic Year"
/// - Universities: "Fall 2025 Semester Calendar"
/// - Institutes: "2025 Training Cycle"
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "academic_calendars")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub hijri_start: Option<String>,
    pub hijri_end: Option<String>,
    /// planning, active, completed, archived
    pub status: String,
    pub is_locked: bool,
    pub locked_at: Option<DateTime<Utc>>,
    pub locked_by: Option<Uuid>,
    pub lock_reason: Option<String>,
    pub current_period_id: Option<Uuid>,
    pub archived_at: Option<DateTime<Utc>>,
    pub archived_by: Option<Uuid>,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
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
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
