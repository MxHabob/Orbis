//! Dashboard stats operations.

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;

pub async fn get_stats(
    _ctx: &ServiceContext,
) -> Result<serde_json::Value, AppError> {
    // Return mock/dummy dashboard stats for now
    Ok(serde_json::json!({
        "students_count": 150,
        "classes_count": 12,
        "teachers_count": 8,
        "active_cycle": "2025/2026"
    }))
}
