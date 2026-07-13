use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "organizations")]
#[schema(as = Organization)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub name_ar: Option<String>,
    #[sea_orm(unique)]
    pub slug: Option<String>,
    #[sea_orm(unique)]
    pub code: Option<String>,
    pub timezone: String,
    pub locale: String,
    pub currency: String,
    pub date_format: Option<String>,
    pub hijri_enabled: i32,
    pub max_users: i32,
    pub max_students: i32,
    pub max_storage_gb: i32,
    pub settings: Option<String>,
    pub features: Option<String>,
    pub subscription_plan: String,
    pub subscription_expires_at: Option<String>,
    pub logo: Option<String>,
    pub favicon: Option<String>,
    pub primary_color: Option<String>,
    pub is_active: i32,
    pub is_verified: i32,
    pub is_deleted: i32,
    pub deleted_at: Option<String>,
    pub deleted_by: Option<String>,

    /// FK to the currently active academic year.
    /// Switching years = UPDATE organizations SET current_period_id = ?
    pub current_period_id: Option<String>,

    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::academic_year::Entity")]
    AcademicYears,

    #[sea_orm(has_many = "super::academic_period::Entity")]
    AcademicPeriods,

    #[sea_orm(has_one = "super::academic_policy::Entity")]
    AcademicPolicy,
}

impl Related<super::academic_year::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicYears.def()
    }
}

impl Related<super::academic_period::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicPeriods.def()
    }
}

impl Related<super::academic_policy::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicPolicy.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
