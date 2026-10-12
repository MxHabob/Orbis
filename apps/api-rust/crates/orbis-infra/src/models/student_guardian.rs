use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "student_guardians")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub student_id: Uuid,
    pub guardian_id: Uuid,
    pub relationship: String,
    pub relationship_ar: Option<String>,
    pub is_primary: bool,
    pub is_emergency_contact: bool,
    pub is_authorized_pickup: bool,
    pub is_payer: bool,
    pub payment_share_percent: Option<f64>,
    pub occupation: Option<String>,
    pub employer: Option<String>,
    pub work_phone: Option<String>,
    pub notes: Option<String>,
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
        belongs_to = "super::student::Entity",
        from = "Column::StudentId",
        to = "super::student::Column::Id"
    )]
    Student,
    #[sea_orm(
        belongs_to = "super::person::Entity",
        from = "Column::GuardianId",
        to = "super::person::Column::Id"
    )]
    Guardian,
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

impl Related<super::person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Guardian.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
