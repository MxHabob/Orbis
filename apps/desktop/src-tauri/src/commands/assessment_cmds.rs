use tauri::State;
use crate::state::AppState;
use orbis_app::usecases::assessment::{
    assessment_scheme::CreateAssessmentSchemeUseCase,
    grade::RecordGradeUseCase,
    attendance::MarkAttendanceUseCase,
    calculate_grades::CalculateGradesUseCase,
};
use orbis_app::context::RequestContext;
use uuid::Uuid;
use chrono::NaiveDate;

#[tauri::command]
pub async fn create_assessment_scheme(
    org_id: String,
    name: String,
    description: Option<String>,
    max_score: f64,
    pass_score: f64,
    formula: Option<serde_json::Value>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_org_id = Uuid::parse_str(&org_id).map_err(|_| "Invalid org ID")?;

    let ctx = RequestContext {
        tenant_id: parsed_org_id,
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = CreateAssessmentSchemeUseCase {
        scheme_repo: state.assessment_scheme_repo.clone(),
    };

    match usecase.execute(&ctx, parsed_org_id, name, description, max_score, pass_score, formula).await {
        Ok(scheme) => Ok(scheme.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn record_grade(
    student_id: String,
    class_subject_id: String,
    scheme_id: String,
    score: f64,
    is_excused: bool,
    notes: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_student_id = Uuid::parse_str(&student_id).map_err(|_| "Invalid student ID")?;
    let parsed_class_subject_id = Uuid::parse_str(&class_subject_id).map_err(|_| "Invalid class subject ID")?;
    let parsed_scheme_id = Uuid::parse_str(&scheme_id).map_err(|_| "Invalid scheme ID")?;

    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = RecordGradeUseCase {
        grade_repo: state.grade_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, parsed_student_id, parsed_class_subject_id, parsed_scheme_id, score, is_excused, notes).await {
        Ok(grade) => Ok(grade.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn mark_attendance(
    student_id: String,
    class_id: String,
    date: String,
    status: String,
    notes: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_student_id = Uuid::parse_str(&student_id).map_err(|_| "Invalid student ID")?;
    let parsed_class_id = Uuid::parse_str(&class_id).map_err(|_| "Invalid class ID")?;
    
    let parsed_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|_| "Invalid date format")?;

    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = MarkAttendanceUseCase {
        attendance_repo: state.attendance_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, parsed_student_id, parsed_class_id, parsed_date, status, notes).await {
        Ok(att) => Ok(att.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn calculate_grades(
    scheme_id: String,
    student_id: String,
    state: State<'_, AppState>,
) -> Result<f64, String> {
    let parsed_scheme_id = Uuid::parse_str(&scheme_id).map_err(|_| "Invalid scheme ID")?;
    let parsed_student_id = Uuid::parse_str(&student_id).map_err(|_| "Invalid student ID")?;

    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = CalculateGradesUseCase {
        scheme_repo: state.assessment_scheme_repo.clone(),
        grade_repo: state.grade_repo.clone(),
    };

    match usecase.execute(&ctx, parsed_scheme_id, parsed_student_id).await {
        Ok(score) => Ok(score),
        Err(e) => Err(e.to_string()),
    }
}
