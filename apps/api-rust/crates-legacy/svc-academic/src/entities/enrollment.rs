use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "enrollments")]
#[schema(as = Enrollment)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub period_id: String,
    pub student_id: String,
    pub class_id: String,
    #[sea_orm(unique)]
    pub student_number: String,
    pub status: String,
    pub enrollment_date: String,
    pub enrollment_type: String,
    pub withdrawal_date: Option<String>,
    pub withdrawal_reason: Option<String>,
    pub meta: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::class::Entity",
        from = "Column::ClassId",
        to = "super::class::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Class,
}

impl Related<super::class::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Class.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
