use tauri::State;
use crate::state::AppState;
use orbis_app::usecases::academic::{
    subject::CreateSubjectUseCase,
    class::CreateClassUseCase,
    class_subject::AssignClassSubjectUseCase,
};
use orbis_app::context::RequestContext;
use uuid::Uuid;

#[tauri::command]
pub async fn create_subject(
    org_id: String,
    unit_id: Option<String>,
    code: String,
    name: String,
    credit_hours: Option<i32>,
    is_core: bool,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_org_id = Uuid::parse_str(&org_id).map_err(|_| "Invalid org ID")?;
    let parsed_unit_id = unit_id.map(|id| Uuid::parse_str(&id).unwrap());

    let ctx = RequestContext {
        tenant_id: parsed_org_id,
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = CreateSubjectUseCase {
        subject_repo: state.subject_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, parsed_org_id, parsed_unit_id, code, name, credit_hours, is_core).await {
        Ok(subject) => Ok(subject.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn create_class(
    org_id: String,
    unit_id: String,
    period_id: String,
    name: String,
    level: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_org_id = Uuid::parse_str(&org_id).map_err(|_| "Invalid org ID")?;
    let parsed_unit_id = Uuid::parse_str(&unit_id).map_err(|_| "Invalid unit ID")?;
    let parsed_period_id = Uuid::parse_str(&period_id).map_err(|_| "Invalid period ID")?;

    let ctx = RequestContext {
        tenant_id: parsed_org_id,
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = CreateClassUseCase {
        class_repo: state.class_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, parsed_org_id, parsed_unit_id, parsed_period_id, name, level).await {
        Ok(class) => Ok(class.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn assign_class_subject(
    class_id: String,
    subject_id: String,
    period_id: String,
    primary_teacher_id: String,
    weekly_hours: Option<i32>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_class_id = Uuid::parse_str(&class_id).map_err(|_| "Invalid class ID")?;
    let parsed_subject_id = Uuid::parse_str(&subject_id).map_err(|_| "Invalid subject ID")?;
    let parsed_period_id = Uuid::parse_str(&period_id).map_err(|_| "Invalid period ID")?;
    let parsed_primary_teacher_id = Uuid::parse_str(&primary_teacher_id).map_err(|_| "Invalid primary teacher ID")?;

    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(), // From session
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = AssignClassSubjectUseCase {
        class_subject_repo: state.class_subject_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, parsed_class_id, parsed_subject_id, parsed_period_id, parsed_primary_teacher_id, weekly_hours).await {
        Ok(class_subject) => Ok(class_subject.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}
