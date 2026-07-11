//! Bulk import operations.

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;

pub async fn students(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<serde_json::Value, AppError> {
    use crate::services::student_registration::{StudentRegistrationService, RegisterStudentRequest};
    use crate::services::settings::OrganizationSettingsService;
    use crate::repository::LifecycleQueryRepo;
    
    let students_list = data.get("students")
        .and_then(|v| v.as_array())
        .ok_or_else(|| AppError::validation("Missing 'students' array in import payload"))?;

    let settings_repo = std::sync::Arc::new(LifecycleQueryRepo::new(ctx.db.clone()));
    let settings_svc = std::sync::Arc::new(OrganizationSettingsService::new(settings_repo));
    let reg_svc = StudentRegistrationService::new(settings_svc);

    let mut imported_count = 0;
    for student_val in students_list {
        let req: RegisterStudentRequest = serde_json::from_value(student_val.clone())
            .map_err(|e| AppError::validation(format!("Invalid student record: {}", e)))?;
            
        reg_svc.register_student(&ctx.db, &ctx.org_id, req, Some(ctx.actor.as_str())).await?;
        imported_count += 1;
    }

    Ok(serde_json::json!({
        "success": true,
        "message": format!("Bulk import of {} students completed successfully", imported_count),
        "imported_count": imported_count
    }))
}

pub async fn grades(
    _ctx: &ServiceContext,
    _data: serde_json::Value,
) -> Result<serde_json::Value, AppError> {
    Ok(serde_json::json!({
        "success": true,
        "message": "Bulk import of grades stub",
        "imported_count": 0
    }))
}
