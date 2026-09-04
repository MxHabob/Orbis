//! Exam schedule generation operations.

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use crate::entities::exam_schedule;

pub async fn generate(
    _ctx: &ServiceContext,
    _session_id: &str,
) -> Result<serde_json::Value, AppError> {
    Err(AppError::internal("Exam schedule generation is not implemented"))
}

pub async fn get(
    ctx: &ServiceContext,
    session_id: &str,
) -> Result<Vec<exam_schedule::Model>, AppError> {
    crate::ops::exam_schedule::list(ctx, session_id, None).await
}
