use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "assessment_schemes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub max_score: f64,
    pub formula: Option<serde_json::Value>,
    /// Assessment level: monthly_period, term, annual, coursework, midterm, final, module_assessment, certificate, certificate_cumulative
    pub level: String,
    /// Institution scope: school, university, institute, all
    pub institution_scope: String,
    /// JSON array of AssessmentComponent objects
    pub components_json: String,
    pub is_default: bool,
    pub is_system: bool,
    // ── Policy Provenance ─────────────────────────────────────
    pub source_document: Option<String>,
    pub source_authority: Option<String>,
    pub verification_status: Option<String>,
    pub effective_date: Option<DateTime<Utc>>,
    pub policy_version: i32,
    pub min_attendance_ratio: Option<f64>,
    pub pass_threshold: Option<f64>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
