//! Seating operations.

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use crate::entities::seating_number;

pub async fn generate(
    ctx: &ServiceContext,
    session_id: &str,
) -> Result<serde_json::Value, AppError> {
    crate::ops::ordering::generate_ordering(ctx, session_id).await?;
    Ok(serde_json::json!({ "success": true }))
}

pub async fn list(
    ctx: &ServiceContext,
    session_id: &str,
) -> Result<Vec<seating_number::Model>, AppError> {
    crate::ops::ordering::list_seating(ctx, session_id).await
}
