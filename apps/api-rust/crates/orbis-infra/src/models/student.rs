use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Student record — links person to institutional student identity.
/// Personal data (names, IDs) lives in `persons`; this table holds
/// student-specific attributes only.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "students")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub person_id: Uuid,
    pub user_id: Option<Uuid>,
    pub student_number: String,
    /// active, suspended, graduated, withdrawn, expelled
    pub status: String,
    /// good, probation, warning, dismissed
    pub standing: String,
    pub admitted_at: DateTimeUtc,
    pub graduated_at: Option<DateTimeUtc>,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTimeUtc>,
    pub meta: Option<Json>,
    pub org_unit_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::person::Entity",
        from = "Column::PersonId",
        to = "super::person::Column::Id"
    )]
    Person,
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrgId",
        to = "super::organization::Column::Id"
    )]
    Organization,
    #[sea_orm(has_many = "super::enrollment::Entity")]
    Enrollments,
}

impl Related<super::person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Person.def()
    }
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::enrollment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Enrollments.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
