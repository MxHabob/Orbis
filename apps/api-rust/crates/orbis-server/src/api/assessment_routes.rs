use axum::{extract::State, Json, routing::post, Router};
use serde::{Deserialize, Serialize};
use orbis_app::context::RequestContext;
use orbis_app::usecases::assessment::{
    assessment_scheme::CreateAssessmentSchemeUseCase,
    grade::RecordGradeUseCase,
    attendance::MarkAttendanceUseCase,
    calculate_grades::CalculateGradesUseCase,
};
use uuid::Uuid;
use chrono::NaiveDate;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateAssessmentSchemeRequest {
    pub org_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub max_score: f64,
    pub formula: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct AssessmentSchemeResponse {
    pub id: Uuid,
}

#[derive(Deserialize)]
pub struct RecordGradeRequest {
    pub student_id: Uuid,
    pub class_subject_id: Uuid,
    pub scheme_id: Uuid,
    pub score: f64,
    pub is_excused: bool,
    pub notes: Option<String>,
}

#[derive(Serialize)]
pub struct GradeResponse {
    pub id: Uuid,
}

#[derive(Deserialize)]
pub struct MarkAttendanceRequest {
    pub student_id: Uuid,
    pub class_id: Uuid,
    pub date: String,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Serialize)]
pub struct AttendanceResponse {
    pub id: Uuid,
}

async fn create_assessment_scheme(
    State(state): State<AppState>,
    Json(payload): Json<CreateAssessmentSchemeRequest>,
) -> Result<Json<AssessmentSchemeResponse>, String> {
    let ctx = RequestContext {
        tenant_id: payload.org_id,
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = CreateAssessmentSchemeUseCase {
        scheme_repo: state.assessment_scheme_repo.clone(),
    };

    match use_case.execute(&ctx, payload.org_id, payload.name, payload.description, payload.max_score, payload.formula).await {
        Ok(scheme) => Ok(Json(AssessmentSchemeResponse {
            id: scheme.id,
        })),
        Err(e) => Err(e.to_string()),
    }
}

async fn record_grade(
    State(state): State<AppState>,
    Json(payload): Json<RecordGradeRequest>,
) -> Result<Json<GradeResponse>, String> {
    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = RecordGradeUseCase {
        grade_repo: state.grade_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match use_case.execute(&ctx, payload.student_id, payload.class_subject_id, payload.scheme_id, payload.score, payload.is_excused, payload.notes).await {
        Ok(grade) => Ok(Json(GradeResponse {
            id: grade.id,
        })),
        Err(e) => Err(e.to_string()),
    }
}

async fn mark_attendance(
    State(state): State<AppState>,
    Json(payload): Json<MarkAttendanceRequest>,
) -> Result<Json<AttendanceResponse>, String> {
    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let date = NaiveDate::parse_from_str(&payload.date, "%Y-%m-%d").map_err(|_| "Invalid date format")?;

    let use_case = MarkAttendanceUseCase {
        attendance_repo: state.attendance_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match use_case.execute(&ctx, payload.student_id, payload.class_id, date, payload.status, payload.notes).await {
        Ok(att) => Ok(Json(AttendanceResponse {
            id: att.id,
        })),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Deserialize)]
pub struct CalculateGradesRequest {
    pub scheme_id: Uuid,
    pub student_id: Uuid,
}

#[derive(Serialize)]
pub struct CalculateGradesResponse {
    pub final_score: f64,
}

async fn calculate_grades_handler(
    State(state): State<AppState>,
    Json(payload): Json<CalculateGradesRequest>,
) -> Result<Json<CalculateGradesResponse>, String> {
    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(), // Mocked
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = CalculateGradesUseCase {
        scheme_repo: state.assessment_scheme_repo.clone(),
        grade_repo: state.grade_repo.clone(),
    };

    match use_case.execute(&ctx, payload.scheme_id, payload.student_id).await {
        Ok(score) => Ok(Json(CalculateGradesResponse { final_score: score })),
        Err(e) => Err(e.to_string()),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/assessment/schemes", post(create_assessment_scheme))
        .route("/assessment/grades", post(record_grade))
        .route("/assessment/attendance", post(mark_attendance))
        .route("/assessment/calculate", post(calculate_grades_handler))
}
