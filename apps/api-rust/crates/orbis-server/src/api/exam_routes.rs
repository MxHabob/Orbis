use axum::{extract::State, Json, routing::post, Router};
use serde::{Deserialize, Serialize};
use orbis_app::context::RequestContext;
use orbis_app::usecases::exam::{
    exam_session::CreateExamSessionUseCase,
    exam_schedule::ScheduleExamUseCase,
    seating_number::AssignSeatingNumberUseCase,
};
use uuid::Uuid;
use chrono::{NaiveDate, NaiveTime};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateExamSessionRequest {
    pub org_id: Uuid,
    pub academic_period_id: Uuid,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Serialize)]
pub struct ExamSessionResponse {
    pub id: Uuid,
}

#[derive(Deserialize)]
pub struct ScheduleExamRequest {
    pub exam_session_id: Uuid,
    pub class_subject_id: Uuid,
    pub exam_date: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Serialize)]
pub struct ExamScheduleResponse {
    pub id: Uuid,
}

async fn create_exam_session(
    State(state): State<AppState>,
    Json(payload): Json<CreateExamSessionRequest>,
) -> Result<Json<ExamSessionResponse>, String> {
    let ctx = RequestContext {
        tenant_id: payload.org_id,
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let start_date = NaiveDate::parse_from_str(&payload.start_date, "%Y-%m-%d").map_err(|_| "Invalid start date")?;
    let end_date = NaiveDate::parse_from_str(&payload.end_date, "%Y-%m-%d").map_err(|_| "Invalid end date")?;

    let use_case = CreateExamSessionUseCase {
        session_repo: state.exam_session_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match use_case.execute(&ctx, payload.org_id, payload.academic_period_id, payload.name, start_date, end_date).await {
        Ok(session) => Ok(Json(ExamSessionResponse {
            id: session.id,
        })),
        Err(e) => Err(e.to_string()),
    }
}

async fn schedule_exam(
    State(state): State<AppState>,
    Json(payload): Json<ScheduleExamRequest>,
) -> Result<Json<ExamScheduleResponse>, String> {
    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let exam_date = NaiveDate::parse_from_str(&payload.exam_date, "%Y-%m-%d").map_err(|_| "Invalid date format")?;
    let start_time = NaiveTime::parse_from_str(&payload.start_time, "%H:%M:%S").map_err(|_| "Invalid start time")?;
    let end_time = NaiveTime::parse_from_str(&payload.end_time, "%H:%M:%S").map_err(|_| "Invalid end time")?;

    let use_case = ScheduleExamUseCase {
        schedule_repo: state.exam_schedule_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match use_case.execute(&ctx, payload.exam_session_id, payload.class_subject_id, exam_date, start_time, end_time).await {
        Ok(schedule) => Ok(Json(ExamScheduleResponse {
            id: schedule.id,
        })),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Deserialize)]
pub struct AssignSeatingRequest {
    pub exam_session_id: Uuid,
    pub student_id: Uuid,
    pub seating_number: String,
    pub room_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct SeatingResponse {
    pub id: Uuid,
}

async fn assign_seating(
    State(state): State<AppState>,
    Json(payload): Json<AssignSeatingRequest>,
) -> Result<Json<SeatingResponse>, String> {
    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = AssignSeatingNumberUseCase {
        seating_repo: state.seating_number_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match use_case.execute(&ctx, payload.exam_session_id, payload.student_id, payload.seating_number, payload.room_id).await {
        Ok(seating) => Ok(Json(SeatingResponse { id: seating.id })),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Deserialize)]
pub struct AssignSecretRequest {
    pub exam_schedule_id: Uuid,
    pub student_id: Uuid,
    pub secret_number: String,
}

#[derive(Serialize)]
pub struct SecretResponse {
    pub id: Uuid,
}

async fn assign_secret(
    State(state): State<AppState>,
    Json(payload): Json<AssignSecretRequest>,
) -> Result<Json<SecretResponse>, String> {
    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = orbis_app::usecases::exam::secret_number::AssignSecretNumberUseCase {
        secret_repo: state.secret_number_repo.clone(),
    };

    match use_case.execute(&ctx, payload.exam_schedule_id, payload.student_id, payload.secret_number).await {
        Ok(secret) => Ok(Json(SecretResponse { id: secret.id })),
        Err(e) => Err(e.to_string()),
    }
}

#[derive(Deserialize)]
pub struct AssignProctorRequest {
    pub exam_schedule_id: Uuid,
    pub room_id: Uuid,
    pub teacher_id: Uuid,
    pub is_chief: bool,
}

#[derive(Serialize)]
pub struct ProctorResponse {
    pub id: Uuid,
}

async fn assign_proctor(
    State(state): State<AppState>,
    Json(payload): Json<AssignProctorRequest>,
) -> Result<Json<ProctorResponse>, String> {
    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor_id: Uuid::new_v4(),
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = orbis_app::usecases::exam::exam_proctor::AssignProctorUseCase {
        proctor_repo: state.exam_proctor_repo.clone(),
    };

    match use_case.execute(&ctx, payload.exam_schedule_id, payload.room_id, payload.teacher_id, payload.is_chief).await {
        Ok(proctor) => Ok(Json(ProctorResponse { id: proctor.id })),
        Err(e) => Err(e.to_string()),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/exam/sessions", post(create_exam_session))
        .route("/exam/schedules", post(schedule_exam))
        .route("/exam/seating", post(assign_seating))
        .route("/exam/secrets", post(assign_secret))
        .route("/exam/proctors", post(assign_proctor))
}
