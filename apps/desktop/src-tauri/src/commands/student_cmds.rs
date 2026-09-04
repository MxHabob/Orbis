use tauri::State;
use crate::state::AppState;
use orbis_app::usecases::student::{
    enrollment::EnrollStudentUseCase,
    student_guardian::AssignGuardianUseCase,
};
use orbis_app::context::RequestContext;
use uuid::Uuid;

#[tauri::command]
pub async fn enroll_student(
    org_id: String,
    period_id: String,
    student_id: String,
    class_id: String,
    student_number: String,
    enrollment_type: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_org_id = Uuid::parse_str(&org_id).map_err(|_| "Invalid org ID")?;
    let parsed_period_id = Uuid::parse_str(&period_id).map_err(|_| "Invalid period ID")?;
    let parsed_student_id = Uuid::parse_str(&student_id).map_err(|_| "Invalid student ID")?;
    let parsed_class_id = Uuid::parse_str(&class_id).map_err(|_| "Invalid class ID")?;

    let ctx = RequestContext {
        tenant_id: parsed_org_id,
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = EnrollStudentUseCase {
        enrollment_repo: state.enrollment_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, parsed_org_id, parsed_period_id, parsed_student_id, parsed_class_id, student_number, enrollment_type).await {
        Ok(enrollment) => Ok(enrollment.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn assign_guardian(
    student_id: String,
    guardian_id: String,
    relationship: String,
    is_primary: bool,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_student_id = Uuid::parse_str(&student_id).map_err(|_| "Invalid student ID")?;
    let parsed_guardian_id = Uuid::parse_str(&guardian_id).map_err(|_| "Invalid guardian ID")?;

    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(), // From session
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = AssignGuardianUseCase {
        student_guardian_repo: state.student_guardian_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, parsed_student_id, parsed_guardian_id, relationship, is_primary).await {
        Ok(guardian) => Ok(guardian.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}
