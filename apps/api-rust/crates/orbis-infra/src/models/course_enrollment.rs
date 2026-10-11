use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "course_enrollments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub offering_id: Uuid,
    pub student_id: Uuid,
    pub attempt_number: i32,
    pub status: String,
    pub enrolled_at: DateTime<Utc>,
    pub withdrawn_at: Option<DateTime<Utc>>,
    pub withdrawal_reason: Option<String>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
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
        belongs_to = "super::course_offering::Entity",
        from = "Column::OfferingId",
        to = "super::course_offering::Column::Id"
    )]
    Offering,
    #[sea_orm(
        belongs_to = "super::student::Entity",
        from = "Column::StudentId",
        to = "super::student::Column::Id"
    )]
    Student,
}

impl Related<super::course_offering::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Offering.def()
    }
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::student::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
