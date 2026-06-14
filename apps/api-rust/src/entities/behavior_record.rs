use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "behavior_records")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub cycle_id: String,
    pub enrollment_id: String,
    #[sea_orm(column_name = "type")]
    pub behavior_type: String,
    pub severity: String,
    pub category: Option<String>,
    pub title: String,
    pub title_ar: Option<String>,
    pub description: String,
    pub incident_date: String,
    pub incident_time: Option<String>,
    pub location: Option<String>,
    pub points: Option<i32>,
    pub action_taken: Option<String>,
    pub action_details: Option<String>,
    pub action_completed: Option<i32>,
    pub reported_by: String,
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<String>,
    pub witnesses: Option<String>,
    pub parent_notified: i32,
    pub parent_notified_at: Option<String>,
    pub parent_response: Option<String>,
    pub evidence_urls: Option<String>,
    pub status: String,
    pub resolution_notes: Option<String>,
    pub resolved_at: Option<String>,
    pub resolved_by: Option<String>,
    pub is_private: i32,
    pub ai_source: Option<String>,
    pub ai_confidence: Option<f64>,
    pub ai_model_version: Option<String>,
    pub camera_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::enrollment::Entity",
        from = "Column::EnrollmentId",
        to = "super::enrollment::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Enrollment,
}

impl Related<super::enrollment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Enrollment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
