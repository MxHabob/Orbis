use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Subject prerequisite rules.
/// Links a subject to its prerequisite subjects with optional grade requirements.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "prerequisites")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub subject_id: Uuid,
    pub prerequisite_subject_id: Uuid,
    /// mandatory, recommended, corequisite
    pub requirement_type: String,
    pub min_grade: Option<String>,
    pub min_score: Option<Decimal>,
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
        belongs_to = "super::subject::Entity",
        from = "Column::SubjectId",
        to = "super::subject::Column::Id"
    )]
    Subject,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::subject::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subject.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
