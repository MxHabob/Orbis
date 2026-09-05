//! Exam routes — Sessions, Schedules, Seating, Secrets, Proctors.

use axum::{extract::{State, Path, Query}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_app::usecases::exam::{
    exam_session::CreateExamSessionUseCase,
    exam_schedule::ScheduleExamUseCase,
    seating::AssignSeatingNumberUseCase,
    secret_number::AssignSecretNumberUseCase,
    proctor::AssignProctorUseCase,
};

#[derive(Serialize, ToSchema)]
pub struct ExamSessionDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub period_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: String,
    pub is_active: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateExamSessionRequest {
    pub period_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
pub struct ExamScheduleDto {
    pub id: Uuid,
    pub session_id: Uuid,
    pub subject_id: Uuid,
    pub date: NaiveDate,
    pub max_score: f64,
    pub passing_score: f64,
}

#[derive(Deserialize, ToSchema)]
pub struct ScheduleExamRequest {
    pub session_id: Uuid,
    pub subject_id: Uuid,
    pub date: NaiveDate,
    pub max_score: f64,
    pub passing_score: f64,
}

#[derive(Serialize, ToSchema)]
pub struct SeatingDto { pub student_id: Uuid, pub session_id: Uuid, pub seating_number: String }

#[derive(Deserialize, ToSchema)]
pub struct AssignSeatingRequest { pub student_id: Uuid, pub session_id: Uuid, pub seating_number: String }

#[derive(Serialize, ToSchema)]
pub struct SecretDto { pub student_id: Uuid, pub schedule_id: Uuid, pub secret_number: String }

#[derive(Deserialize, ToSchema)]
pub struct AssignSecretRequest { pub student_id: Uuid, pub schedule_id: Uuid, pub secret_number: String }

#[derive(Serialize, ToSchema)]
pub struct ProctorDto { pub teacher_id: Uuid, pub schedule_id: Uuid, pub room_id: Uuid }

#[derive(Deserialize, ToSchema)]
pub struct AssignProctorRequest { pub teacher_id: Uuid, pub schedule_id: Uuid, pub room_id: Uuid }

#[derive(Deserialize)]
pub struct SessionQuery { pub period_id: Option<Uuid> }

#[derive(Deserialize)]
pub struct ScheduleQuery { pub session_id: Uuid }

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/exam-sessions", request_body = CreateExamSessionRequest,
    responses((status = 201, body = ExamSessionDto)), security(("bearer" = [])), tag = "Exam")]
async fn create_session(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<CreateExamSessionRequest>,
) -> Result<Json<ApiResponse<ExamSessionDto>>, ApiError> {
    let uc = CreateExamSessionUseCase { session_repo: state.exam_session_repo.clone(), event_bus: state.event_bus.clone() };
    let s = uc.execute(&ctx, ctx.tenant_id, p.period_id, p.name, p.name_ar, p.start_date, p.end_date).await?;
    Ok(ApiResponse::ok(ExamSessionDto {
        id: s.id, org_id: s.org_id, period_id: s.period_id, name: s.name, name_ar: s.name_ar,
        start_date: s.start_date, end_date: s.end_date, status: s.status, is_active: s.is_active,
    }))
}

#[utoipa::path(get, path = "/api/v1/exam-sessions",
    responses((status = 200, body = Vec<ExamSessionDto>)), security(("bearer" = [])), tag = "Exam")]
async fn list_sessions(
    _ctx: AuthContext, State(state): State<AppState>, Query(q): Query<SessionQuery>,
) -> Result<Json<ApiResponse<Vec<ExamSessionDto>>>, ApiError> {
    let sessions = match q.period_id {
        Some(pid) => state.exam_session_repo.find_by_period(pid).await?,
        None => vec![],
    };
    let dtos = sessions.into_iter().map(|s| ExamSessionDto {
        id: s.id, org_id: s.org_id, period_id: s.period_id, name: s.name, name_ar: s.name_ar,
        start_date: s.start_date, end_date: s.end_date, status: s.status, is_active: s.is_active,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/exam-sessions/{id}", params(("id" = Uuid, Path)),
    responses((status = 200, body = ExamSessionDto)), security(("bearer" = [])), tag = "Exam")]
async fn get_session(
    _ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ExamSessionDto>>, ApiError> {
    let s = state.exam_session_repo.find_by_id(id).await?.ok_or_else(|| ApiError::not_found("Exam session not found"))?;
    Ok(ApiResponse::ok(ExamSessionDto {
        id: s.id, org_id: s.org_id, period_id: s.period_id, name: s.name, name_ar: s.name_ar,
        start_date: s.start_date, end_date: s.end_date, status: s.status, is_active: s.is_active,
    }))
}

#[utoipa::path(post, path = "/api/v1/exam-schedules", request_body = ScheduleExamRequest,
    responses((status = 201, body = ExamScheduleDto)), security(("bearer" = [])), tag = "Exam")]
async fn schedule_exam(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<ScheduleExamRequest>,
) -> Result<Json<ApiResponse<ExamScheduleDto>>, ApiError> {
    let uc = ScheduleExamUseCase { schedule_repo: state.exam_schedule_repo.clone(), event_bus: state.event_bus.clone() };
    let s = uc.execute(&ctx, p.session_id, p.subject_id, p.date, p.max_score, p.passing_score).await?;
    Ok(ApiResponse::ok(ExamScheduleDto {
        id: s.id, session_id: s.session_id, subject_id: s.subject_id,
        date: s.date, max_score: s.max_score, passing_score: s.passing_score,
    }))
}

#[utoipa::path(get, path = "/api/v1/exam-schedules",
    params(("session_id" = Uuid, Query)),
    responses((status = 200, body = Vec<ExamScheduleDto>)), security(("bearer" = [])), tag = "Exam")]
async fn list_schedules(
    _ctx: AuthContext, State(state): State<AppState>, Query(q): Query<ScheduleQuery>,
) -> Result<Json<ApiResponse<Vec<ExamScheduleDto>>>, ApiError> {
    let schedules = state.exam_schedule_repo.find_by_session(q.session_id).await?;
    let dtos = schedules.into_iter().map(|s| ExamScheduleDto {
        id: s.id, session_id: s.session_id, subject_id: s.subject_id,
        date: s.date, max_score: s.max_score, passing_score: s.passing_score,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/exam-seating", request_body = AssignSeatingRequest,
    responses((status = 201, body = SeatingDto)), security(("bearer" = [])), tag = "Exam")]
async fn assign_seating(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<AssignSeatingRequest>,
) -> Result<Json<ApiResponse<SeatingDto>>, ApiError> {
    let uc = AssignSeatingNumberUseCase { seating_repo: state.seating_number_repo.clone(), event_bus: state.event_bus.clone() };
    let s = uc.execute(&ctx, p.student_id, p.session_id, p.seating_number).await?;
    Ok(ApiResponse::ok(SeatingDto { student_id: s.student_id, session_id: s.session_id, seating_number: s.seating_number }))
}

#[utoipa::path(post, path = "/api/v1/exam-secrets", request_body = AssignSecretRequest,
    responses((status = 201, body = SecretDto)), security(("bearer" = [])), tag = "Exam")]
async fn assign_secret(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<AssignSecretRequest>,
) -> Result<Json<ApiResponse<SecretDto>>, ApiError> {
    let uc = AssignSecretNumberUseCase { secret_repo: state.secret_number_repo.clone() };
    let s = uc.execute(&ctx, p.student_id, p.schedule_id, p.secret_number).await?;
    Ok(ApiResponse::ok(SecretDto { student_id: s.student_id, schedule_id: s.schedule_id, secret_number: s.secret_number }))
}

#[utoipa::path(post, path = "/api/v1/exam-proctors", request_body = AssignProctorRequest,
    responses((status = 201, body = ProctorDto)), security(("bearer" = [])), tag = "Exam")]
async fn assign_proctor(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<AssignProctorRequest>,
) -> Result<Json<ApiResponse<ProctorDto>>, ApiError> {
    let uc = AssignProctorUseCase { proctor_repo: state.exam_proctor_repo.clone() };
    let pr = uc.execute(&ctx, p.teacher_id, p.schedule_id, p.room_id).await?;
    Ok(ApiResponse::ok(ProctorDto { teacher_id: pr.teacher_id, schedule_id: pr.schedule_id, room_id: pr.room_id }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/exam-sessions", post(create_session).get(list_sessions))
        .route("/api/v1/exam-sessions/{id}", get(get_session))
        .route("/api/v1/exam-schedules", post(schedule_exam).get(list_schedules))
        .route("/api/v1/exam-seating", post(assign_seating))
        .route("/api/v1/exam-secrets", post(assign_secret))
        .route("/api/v1/exam-proctors", post(assign_proctor))
}
