//! Academic write commands.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateClassCmd {
    pub org_id: String,
    pub cycle_id: String,
    pub org_unit_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub grade_level: String,
    pub capacity: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubjectCmd {
    pub org_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrgUnitCmd {
    pub org_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub parent_id: Option<String>,
}
