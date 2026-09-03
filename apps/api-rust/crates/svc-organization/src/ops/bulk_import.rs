//! Bulk import operations.

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::Permission;

pub async fn students(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<serde_json::Value, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ImportData).await?;

    use svc_enrollment_workflow::ops::student_registration::{register_student, RegisterStudentRequest};
    
    let students_list = data.get("students")
        .and_then(|v| v.as_array())
        .ok_or_else(|| AppError::validation("Missing 'students' array in import payload"))?;

    let mut imported_count = 0;
    for student_val in students_list {
        let req: RegisterStudentRequest = serde_json::from_value(student_val.clone())
            .map_err(|e| AppError::validation(format!("Invalid student record: {}", e)))?;
            
        register_student(ctx, req).await?;
        imported_count += 1;
    }

    Ok(serde_json::json!({
        "success": true,
        "message": format!("Bulk import of {} students completed successfully", imported_count),
        "imported_count": imported_count
    }))
}

pub async fn grades(
    ctx: &ServiceContext,
    _data: serde_json::Value,
) -> Result<serde_json::Value, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ImportData).await?;
    Err(AppError::internal("Bulk import of grades is not implemented"))
}
