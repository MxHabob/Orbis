use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct OrbisError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

pub mod iam_routes;
pub mod org_routes;
pub mod calendar_routes;
pub mod academic_routes;
pub mod student_routes;
pub mod scheduling_routes;
pub mod assessment_routes;
pub mod exam_routes;
pub mod finance_routes;
pub mod document_routes;
pub mod integration_routes;
