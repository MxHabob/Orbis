use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Organization hierarchy node — represents branches, departments, divisions, etc.
///
/// Uses a hybrid Adjacency List + Materialized Path model for efficient
/// tree queries on SQLite.
///
/// # Hierarchy Rules
///
/// - `path` is auto-generated: `"/root/branch-ryd/dept-cs"`
/// - `depth` is auto-calculated from `path`
/// - `settings` and `local_policies` support inheritance (child overrides parent)
/// - `data_isolation` controls data visibility between nodes
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "org_units")]
#[schema(as = OrgUnit)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub parent_id: Option<String>,

    // ── Materialized Path ──
    pub path: String,
    pub depth: i32,

    // ── Metadata ──
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    #[sea_orm(column_name = "unit_type")]
    pub unit_type: String, // organization, branch, department, division, unit
    pub head_id: Option<String>,

    // ── Physical Location ──
    pub capacity: Option<i32>,
    pub room_number: Option<String>,
    pub floor: Option<i32>,
    pub building: Option<String>,

    // ── Settings & Policy Inheritance ──
    pub settings: Option<String>,        // JSON: local settings overrides
    pub local_policies: Option<String>,  // JSON: local policy overrides

    // ── Data Isolation ──
    pub data_isolation: String, // shared, isolated, read_only

    // ── Denormalized Counts ──
    pub children_count: i32,
    pub descendant_count: i32,

    // ── Status ──
    pub sort_order: Option<i32>,
    pub is_active: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
