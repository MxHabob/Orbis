//! Exam routes — Sessions, Schedules, Seating, Secrets, Proctors.

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get, post},
};
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ExamSessionDto {
    pub id: Uuid,
    pub org_id: Uuid,
    #[serde(alias = "academic_period_id")]
    pub period_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub session_type: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateExamSessionRequest {
    #[serde(alias = "academic_period_id")]
    pub period_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    #[serde(default = "default_exam_session_type")]
    pub session_type: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

fn default_exam_session_type() -> String {
    "final".to_string()
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateExamSessionRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub session_type: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ExamScheduleDto {
    pub id: Uuid,
    pub exam_session_id: Uuid,
    pub section_subject_id: Uuid,
    pub exam_date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct ScheduleExamRequest {
    pub exam_session_id: Uuid,
    pub section_subject_id: Uuid,
    pub exam_date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SeatingDto {
    pub id: Uuid,
    pub exam_session_id: Uuid,
    pub student_id: Uuid,
    pub room_id: Option<Uuid>,
    pub seating_number: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct AssignSeatingRequest {
    pub exam_session_id: Uuid,
    pub student_id: Uuid,
    pub room_id: Option<Uuid>,
    pub seating_number: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SecretDto {
    pub id: Uuid,
    pub exam_schedule_id: Uuid,
    pub student_id: Uuid,
    pub secret_number: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct AssignSecretRequest {
    pub exam_schedule_id: Uuid,
    pub student_id: Uuid,
    pub secret_number: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ProctorDto {
    pub id: Uuid,
    pub exam_schedule_id: Uuid,
    pub room_id: Uuid,
    pub teacher_id: Uuid,
    pub is_chief: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct AssignProctorRequest {
    pub exam_schedule_id: Uuid,
    pub room_id: Uuid,
    pub teacher_id: Uuid,
    pub is_chief: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct GenerateSeatingRequest {
    pub exam_session_id: Uuid,
    pub student_ids: Vec<Uuid>,
    #[serde(default)]
    pub room_ids: Vec<Uuid>,
    pub prefix: Option<String>,
    pub start_number: Option<u32>,
}

#[derive(Deserialize, ToSchema)]
pub struct GenerateSecretsRequest {
    pub exam_schedule_id: Uuid,
    pub student_ids: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct SessionQuery {
    pub period_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct ScheduleQuery {
    pub session_id: Uuid,
}

#[derive(Deserialize)]
pub struct SeatingQuery {
    pub session_id: Uuid,
}

#[derive(Deserialize)]
pub struct SecretQuery {
    pub schedule_id: Uuid,
}

#[derive(Deserialize)]
pub struct ProctorQuery {
    pub schedule_id: Uuid,
}

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/exam-sessions", request_body = CreateExamSessionRequest,
    responses((status = 201, body = ExamSessionDto)), security(("bearer" = [])), tag = "Exam")]
async fn create_session(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<CreateExamSessionRequest>,
) -> Result<Json<ApiResponse<ExamSessionDto>>, ApiError> {
    let s = state
        .scheduling_service
        .create_exam_session(
            &ctx,
            ctx.tenant_id,
            p.period_id,
            p.name,
            p.name_ar,
            p.session_type,
            p.start_date,
            p.end_date,
        )
        .await?;
    Ok(ApiResponse::ok(ExamSessionDto {
        id: s.id,
        org_id: s.org_id,
        period_id: s.period_id,
        name: s.name,
        name_ar: s.name_ar,
        session_type: s.session_type,
        start_date: s.start_date,
        end_date: s.end_date,
        status: s.status,
        created_at: s.created_at,
        updated_at: s.updated_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/exam-sessions",
    responses((status = 200, body = Vec<ExamSessionDto>)), security(("bearer" = [])), tag = "Exam")]
async fn list_sessions(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Query(q): Query<SessionQuery>,
) -> Result<Json<ApiResponse<Vec<ExamSessionDto>>>, ApiError> {
    let sessions = match q.period_id {
        Some(pid) => state.scheduling_service.list_exam_sessions(pid).await?,
        None => vec![],
    };
    let dtos = sessions
        .into_iter()
        .map(|s| ExamSessionDto {
            id: s.id,
            org_id: s.org_id,
            period_id: s.period_id,
            name: s.name,
            name_ar: s.name_ar,
            session_type: s.session_type,
            start_date: s.start_date,
            end_date: s.end_date,
            status: s.status,
            created_at: s.created_at,
            updated_at: s.updated_at,
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/exam-sessions/{id}", params(("id" = Uuid, Path, description = "Exam Session ID")),
    responses((status = 200, body = ExamSessionDto)), security(("bearer" = [])), tag = "Exam")]
async fn get_session(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ExamSessionDto>>, ApiError> {
    let s = state
        .scheduling_service
        .get_exam_session(id)
        .await?
        .ok_or_else(|| ApiError::not_found("Exam session not found"))?;
    Ok(ApiResponse::ok(ExamSessionDto {
        id: s.id,
        org_id: s.org_id,
        period_id: s.period_id,
        name: s.name,
        name_ar: s.name_ar,
        session_type: s.session_type,
        start_date: s.start_date,
        end_date: s.end_date,
        status: s.status,
        created_at: s.created_at,
        updated_at: s.updated_at,
    }))
}

#[utoipa::path(patch, path = "/api/v1/exam-sessions/{id}", request_body = UpdateExamSessionRequest,
    params(("id" = Uuid, Path, description = "Exam Session ID")),
    responses((status = 200, body = ExamSessionDto)), security(("bearer" = [])), tag = "Exam")]
async fn update_session(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateExamSessionRequest>,
) -> Result<Json<ApiResponse<ExamSessionDto>>, ApiError> {
    let s = state
        .scheduling_service
        .update_exam_session(
            &ctx,
            id,
            p.name,
            p.name_ar,
            p.session_type,
            p.start_date,
            p.end_date,
            p.status,
        )
        .await?;
    Ok(ApiResponse::ok(ExamSessionDto {
        id: s.id,
        org_id: s.org_id,
        period_id: s.period_id,
        name: s.name,
        name_ar: s.name_ar,
        session_type: s.session_type,
        start_date: s.start_date,
        end_date: s.end_date,
        status: s.status,
        created_at: s.created_at,
        updated_at: s.updated_at,
    }))
}

#[utoipa::path(delete, path = "/api/v1/exam-sessions/{id}", params(("id" = Uuid, Path, description = "Exam Session ID")),
    responses((status = 200, body = ())), security(("bearer" = [])), tag = "Exam")]
async fn delete_session(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state
        .scheduling_service
        .delete_exam_session(&ctx, id)
        .await?;
    Ok(ApiResponse::ok(()))
}

#[utoipa::path(post, path = "/api/v1/exam-schedules", request_body = ScheduleExamRequest,
    responses((status = 201, body = ExamScheduleDto)), security(("bearer" = [])), tag = "Exam")]
async fn schedule_exam(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<ScheduleExamRequest>,
) -> Result<Json<ApiResponse<ExamScheduleDto>>, ApiError> {
    let s = state
        .scheduling_service
        .schedule_exam(
            &ctx,
            p.exam_session_id,
            p.section_subject_id,
            p.exam_date,
            p.start_time,
            p.end_time,
        )
        .await?;
    Ok(ApiResponse::ok(ExamScheduleDto {
        id: s.id,
        exam_session_id: s.exam_session_id,
        section_subject_id: s.section_subject_id,
        exam_date: s.exam_date,
        start_time: s.start_time,
        end_time: s.end_time,
        created_at: s.created_at,
        updated_at: s.updated_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/exam-schedules",
    params(("session_id" = Uuid, Query, description = "Exam Session ID")),
    responses((status = 200, body = Vec<ExamScheduleDto>)), security(("bearer" = [])), tag = "Exam")]
async fn list_schedules(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Query(q): Query<ScheduleQuery>,
) -> Result<Json<ApiResponse<Vec<ExamScheduleDto>>>, ApiError> {
    let schedules = state
        .scheduling_service
        .list_exam_schedules(q.session_id)
        .await?;
    let dtos = schedules
        .into_iter()
        .map(|s| ExamScheduleDto {
            id: s.id,
            exam_session_id: s.exam_session_id,
            section_subject_id: s.section_subject_id,
            exam_date: s.exam_date,
            start_time: s.start_time,
            end_time: s.end_time,
            created_at: s.created_at,
            updated_at: s.updated_at,
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(delete, path = "/api/v1/exam-schedules/{id}", params(("id" = Uuid, Path, description = "Exam Schedule ID")),
    responses((status = 200, body = ())), security(("bearer" = [])), tag = "Exam")]
async fn delete_schedule(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state
        .scheduling_service
        .delete_exam_schedule(&ctx, id)
        .await?;
    Ok(ApiResponse::ok(()))
}

#[utoipa::path(post, path = "/api/v1/exam-seating", request_body = AssignSeatingRequest,
    responses((status = 201, body = SeatingDto)), security(("bearer" = [])), tag = "Exam")]
async fn assign_seating(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<AssignSeatingRequest>,
) -> Result<Json<ApiResponse<SeatingDto>>, ApiError> {
    let s = state
        .scheduling_service
        .assign_seating_number(
            &ctx,
            p.exam_session_id,
            p.student_id,
            p.seating_number,
            p.room_id,
        )
        .await?;
    Ok(ApiResponse::ok(SeatingDto {
        id: s.id,
        exam_session_id: s.exam_session_id,
        student_id: s.student_id,
        seating_number: s.seating_number,
        room_id: s.room_id,
        created_at: s.created_at,
        updated_at: s.updated_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/exam-seating",
    params(("session_id" = Uuid, Query, description = "Exam Session ID")),
    responses((status = 200, body = Vec<SeatingDto>)), security(("bearer" = [])), tag = "Exam")]
async fn list_seating(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Query(q): Query<SeatingQuery>,
) -> Result<Json<ApiResponse<Vec<SeatingDto>>>, ApiError> {
    let list = state
        .scheduling_service
        .list_seating_numbers(q.session_id)
        .await?;
    let dtos = list
        .into_iter()
        .map(|s| SeatingDto {
            id: s.id,
            exam_session_id: s.exam_session_id,
            student_id: s.student_id,
            seating_number: s.seating_number,
            room_id: s.room_id,
            created_at: s.created_at,
            updated_at: s.updated_at,
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(delete, path = "/api/v1/exam-seating/{id}", params(("id" = Uuid, Path, description = "Seating Number ID")),
    responses((status = 200, body = ())), security(("bearer" = [])), tag = "Exam")]
async fn delete_seating(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state
        .scheduling_service
        .delete_seating_number(&ctx, id)
        .await?;
    Ok(ApiResponse::ok(()))
}

#[utoipa::path(post, path = "/api/v1/exam-seating/actions/generate", request_body = GenerateSeatingRequest,
    responses((status = 200, body = Vec<SeatingDto>)), security(("bearer" = [])), tag = "Exam")]
async fn generate_seating(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<GenerateSeatingRequest>,
) -> Result<Json<ApiResponse<Vec<SeatingDto>>>, ApiError> {
    let seatings = state
        .scheduling_service
        .generate_seating_numbers(
            &ctx,
            p.exam_session_id,
            p.student_ids,
            p.room_ids,
            p.prefix,
            p.start_number,
        )
        .await?;
    let dtos = seatings
        .into_iter()
        .map(|s| SeatingDto {
            id: s.id,
            exam_session_id: s.exam_session_id,
            student_id: s.student_id,
            seating_number: s.seating_number,
            room_id: s.room_id,
            created_at: s.created_at,
            updated_at: s.updated_at,
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/exam-secrets", request_body = AssignSecretRequest,
    responses((status = 201, body = SecretDto)), security(("bearer" = [])), tag = "Exam")]
async fn assign_secret(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<AssignSecretRequest>,
) -> Result<Json<ApiResponse<SecretDto>>, ApiError> {
    let s = state
        .scheduling_service
        .assign_secret_number(&ctx, p.exam_schedule_id, p.student_id, p.secret_number)
        .await?;
    Ok(ApiResponse::ok(SecretDto {
        id: s.id,
        exam_schedule_id: s.exam_schedule_id,
        student_id: s.student_id,
        secret_number: s.secret_number,
        created_at: s.created_at,
        updated_at: s.updated_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/exam-secrets",
    params(("schedule_id" = Uuid, Query, description = "Exam Schedule ID")),
    responses((status = 200, body = Vec<SecretDto>)), security(("bearer" = [])), tag = "Exam")]
async fn list_secrets(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Query(q): Query<SecretQuery>,
) -> Result<Json<ApiResponse<Vec<SecretDto>>>, ApiError> {
    let list = state
        .scheduling_service
        .list_secret_numbers(q.schedule_id)
        .await?;
    let dtos = list
        .into_iter()
        .map(|s| SecretDto {
            id: s.id,
            exam_schedule_id: s.exam_schedule_id,
            student_id: s.student_id,
            secret_number: s.secret_number,
            created_at: s.created_at,
            updated_at: s.updated_at,
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(delete, path = "/api/v1/exam-secrets/{id}", params(("id" = Uuid, Path, description = "Secret Number ID")),
    responses((status = 200, body = ())), security(("bearer" = [])), tag = "Exam")]
async fn delete_secret(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state
        .scheduling_service
        .delete_secret_number(&ctx, id)
        .await?;
    Ok(ApiResponse::ok(()))
}

#[utoipa::path(post, path = "/api/v1/exam-secrets/actions/generate", request_body = GenerateSecretsRequest,
    responses((status = 200, body = Vec<SecretDto>)), security(("bearer" = [])), tag = "Exam")]
async fn generate_secrets(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<GenerateSecretsRequest>,
) -> Result<Json<ApiResponse<Vec<SecretDto>>>, ApiError> {
    let secrets = state
        .scheduling_service
        .generate_secret_numbers(&ctx, p.exam_schedule_id, p.student_ids)
        .await?;
    let dtos = secrets
        .into_iter()
        .map(|s| SecretDto {
            id: s.id,
            exam_schedule_id: s.exam_schedule_id,
            student_id: s.student_id,
            secret_number: s.secret_number,
            created_at: s.created_at,
            updated_at: s.updated_at,
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/exam-proctors", request_body = AssignProctorRequest,
    responses((status = 201, body = ProctorDto)), security(("bearer" = [])), tag = "Exam")]
async fn assign_proctor(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<AssignProctorRequest>,
) -> Result<Json<ApiResponse<ProctorDto>>, ApiError> {
    let pr = state
        .scheduling_service
        .assign_proctor(
            &ctx,
            p.exam_schedule_id,
            p.room_id,
            p.teacher_id,
            p.is_chief,
        )
        .await?;
    Ok(ApiResponse::ok(ProctorDto {
        id: pr.id,
        exam_schedule_id: pr.exam_schedule_id,
        room_id: pr.room_id,
        teacher_id: pr.teacher_id,
        is_chief: pr.is_chief,
        created_at: pr.created_at,
        updated_at: pr.updated_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/exam-proctors",
    params(("schedule_id" = Uuid, Query, description = "Exam Schedule ID")),
    responses((status = 200, body = Vec<ProctorDto>)), security(("bearer" = [])), tag = "Exam")]
async fn list_proctors(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Query(q): Query<ProctorQuery>,
) -> Result<Json<ApiResponse<Vec<ProctorDto>>>, ApiError> {
    let list = state
        .scheduling_service
        .list_proctors(q.schedule_id)
        .await?;
    let dtos = list
        .into_iter()
        .map(|pr| ProctorDto {
            id: pr.id,
            exam_schedule_id: pr.exam_schedule_id,
            room_id: pr.room_id,
            teacher_id: pr.teacher_id,
            is_chief: pr.is_chief,
            created_at: pr.created_at,
            updated_at: pr.updated_at,
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(delete, path = "/api/v1/exam-proctors/{id}", params(("id" = Uuid, Path, description = "Proctor Assignment ID")),
    responses((status = 200, body = ())), security(("bearer" = [])), tag = "Exam")]
async fn delete_proctor(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state.scheduling_service.delete_proctor(&ctx, id).await?;
    Ok(ApiResponse::ok(()))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/exam-sessions",
            post(create_session).get(list_sessions),
        )
        .route(
            "/api/v1/exam-sessions/:id",
            get(get_session)
                .patch(update_session)
                .delete(delete_session),
        )
        .route(
            "/api/v1/exam-schedules",
            post(schedule_exam).get(list_schedules),
        )
        .route("/api/v1/exam-schedules/:id", delete(delete_schedule))
        .route(
            "/api/v1/exam-seating",
            post(assign_seating).get(list_seating),
        )
        .route("/api/v1/exam-seating/:id", delete(delete_seating))
        .route(
            "/api/v1/exam-seating/actions/generate",
            post(generate_seating),
        )
        .route(
            "/api/v1/exam-secrets",
            post(assign_secret).get(list_secrets),
        )
        .route("/api/v1/exam-secrets/:id", delete(delete_secret))
        .route(
            "/api/v1/exam-secrets/actions/generate",
            post(generate_secrets),
        )
        .route(
            "/api/v1/exam-proctors",
            post(assign_proctor).get(list_proctors),
        )
        .route("/api/v1/exam-proctors/:id", delete(delete_proctor))
}

#[cfg(test)]
mod exam_session_request_tests {
    use super::CreateExamSessionRequest;
    use uuid::Uuid;

    #[test]
    fn accepts_period_id_and_legacy_academic_period_id() {
        let period_id = Uuid::new_v4();
        for request_body in [
            serde_json::json!({
                "period_id": period_id,
                "name": "Final Exams",
                "name_ar": "الاختبارات النهائية",
                "start_date": "2026-10-16",
                "end_date": "2026-10-28"
            }),
            serde_json::json!({
                "academic_period_id": period_id,
                "name": "Final Exams",
                "name_ar": "الاختبارات النهائية",
                "start_date": "2026-10-16",
                "end_date": "2026-10-28"
            }),
        ] {
            let request: CreateExamSessionRequest = serde_json::from_value(request_body).unwrap();
            assert_eq!(request.period_id, period_id);
            assert_eq!(request.session_type, "final");
        }
    }
}
