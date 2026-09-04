use axum::{extract::State, Json, routing::post, Router};
use serde::{Deserialize, Serialize};
use orbis_app::context::RequestContext;
use orbis_app::usecases::student::{
    enrollment::EnrollStudentUseCase,
    student_guardian::AssignGuardianUseCase,
};
use uuid::Uuid;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct EnrollStudentRequest {
    pub org_id: Uuid,
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub class_id: Uuid,
    pub student_number: String,
    pub enrollment_type: String,
}

#[derive(Serialize)]
pub struct EnrollmentResponse {
    pub id: Uuid,
}

#[derive(Deserialize)]
pub struct AssignGuardianRequest {
    pub student_id: Uuid,
    pub guardian_id: Uuid,
    pub relationship: String,
    pub is_primary: bool,
}

#[derive(Serialize)]
pub struct GuardianResponse {
    pub id: Uuid,
}

async fn enroll_student(
    State(state): State<AppState>,
    Json(payload): Json<EnrollStudentRequest>,
) -> Result<Json<EnrollmentResponse>, String> {
    let ctx = RequestContext {
        tenant_id: payload.org_id,
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = EnrollStudentUseCase {
        enrollment_repo: state.enrollment_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match use_case.execute(&ctx, payload.org_id, payload.period_id, payload.student_id, payload.class_id, payload.student_number, payload.enrollment_type).await {
        Ok(enrollment) => Ok(Json(EnrollmentResponse {
            id: enrollment.id,
        })),
        Err(e) => Err(e.to_string()),
    }
}

async fn assign_guardian(
    State(state): State<AppState>,
    Json(payload): Json<AssignGuardianRequest>,
) -> Result<Json<GuardianResponse>, String> {
    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(), // Should be derived from session
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = AssignGuardianUseCase {
        student_guardian_repo: state.student_guardian_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match use_case.execute(&ctx, payload.student_id, payload.guardian_id, payload.relationship, payload.is_primary).await {
        Ok(guardian) => Ok(Json(GuardianResponse {
            id: guardian.id,
        })),
        Err(e) => Err(e.to_string()),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/student/enroll", post(enroll_student))
        .route("/student/guardian", post(assign_guardian))
}
