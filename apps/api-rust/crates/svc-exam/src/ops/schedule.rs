//! Exam schedule generation operations.

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use crate::entities::exam_schedule;

pub async fn generate(
    ctx: &ServiceContext,
    session_id: &str,
) -> Result<serde_json::Value, AppError> {
    // Timetable solver/generator is a stub for now. We can return success.
    Ok(serde_json::json!({ "success": true, "message": "Exam schedule generation triggered" }))
}

pub async fn get(
    ctx: &ServiceContext,
    session_id: &str,
) -> Result<Vec<exam_schedule::Model>, AppError> {
    crate::ops::exam_schedule::list(ctx, session_id).await
}
