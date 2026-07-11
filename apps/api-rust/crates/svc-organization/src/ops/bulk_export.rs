//! Bulk export operations.

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;

pub async fn students(
    _ctx: &ServiceContext,
) -> Result<serde_json::Value, AppError> {
    // Basic stub/mock for student bulk export
    Ok(serde_json::json!({
        "success": true,
        "message": "Bulk export of students completed successfully",
        "data": []
    }))
}

pub async fn grades(
    _ctx: &ServiceContext,
) -> Result<serde_json::Value, AppError> {
    // Basic stub/mock for grade bulk export
    Ok(serde_json::json!({
        "success": true,
        "message": "Bulk export of grades completed successfully",
        "data": []
    }))
}
