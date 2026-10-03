use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "employees")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub person_id: Uuid,
    pub user_id: Option<Uuid>,
    pub employee_number: String,
    pub employee_type: String,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub org_unit_id: Option<Uuid>,
    pub hire_date: String, // Stored as ISO YYYY-MM-DD string for DB compatibility
    pub status: String,
    pub employment_type: String,
    pub work_shift: Option<String>,
    pub notes: Option<String>,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTimeUtc>,
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
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::org_unit::Entity",
        from = "Column::OrgUnitId",
        to = "super::org_unit::Column::Id"
    )]
    OrgUnit,
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrgId",
        to = "super::organization::Column::Id"
    )]
    Organization,
}

impl Related<super::person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Person.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::org_unit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrgUnit.def()
    }
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
