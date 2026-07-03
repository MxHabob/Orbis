use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::middleware::AuthenticatedUser;
use crate::service::Service;
use crate::service::exam_session::{
    CreateExamSessionRequest, UpdateExamSessionRequest,
    CreateExamSubjectRequest, CreateExamScheduleRequest,
    CreateExamHallRequest,
};
use crate::service::student_ordering::RebuildOrderingResult;
use crate::service::exam_numbers::GenerationResult;
use crate::service::exam_logistics::{AllocationConfig, ProctorAssignConfig};

// ── Exam Sessions ───────────────────────────────────────────────────────────

#[utoipa::path(get, path = "/api/exam-sessions", responses((status = 200)))]
pub async fn get_exam_sessions(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(params): Query<crate::handler::params::ExamSessionParams>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(
            &user,
            crate::service::authz::Permission::ExamSessionRead,
            None,
        )
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let sessions = service
        .get_exam_sessions(&user.org_id, params.cycle_id.as_deref())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": sessions }))))
}

#[utoipa::path(post, path = "/api/exam-sessions", responses((status = 201)))]
pub async fn post_exam_sessions(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateExamSessionRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(
            &user,
            crate::service::authz::Permission::ExamSessionWrite,
            None,
        )
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let session = service
        .create_exam_session(&user.org_id, &user.id, req)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": session }))))
}

#[utoipa::path(get, path = "/api/exam-sessions/{id}", responses((status = 200)))]
pub async fn get_exam_session(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(
            &user,
            crate::service::authz::Permission::ExamSessionRead,
            None,
        )
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let session = service.get_exam_session(&user.org_id, &id).await.map_err(|e| {
        let status = if e.contains("not found") { StatusCode::NOT_FOUND } else { StatusCode::INTERNAL_SERVER_ERROR };
        (status, Json(json!({ "success": false, "error": e })))
    })?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": session }))))
}

#[utoipa::path(put, path = "/api/exam-sessions/{id}", responses((status = 200)))]
pub async fn put_exam_session(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<UpdateExamSessionRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(
            &user,
            crate::service::authz::Permission::ExamSessionWrite,
            None,
        )
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let session = service.update_exam_session(&user.org_id, &id, req).await.map_err(|e| {
        let status = if e.contains("not found") { StatusCode::NOT_FOUND } else { StatusCode::BAD_REQUEST };
        (status, Json(json!({ "success": false, "error": e })))
    })?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": session }))))
}

#[utoipa::path(post, path = "/api/exam-sessions/{id}/activate", responses((status = 200)))]
pub async fn post_activate_exam_session(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(
            &user,
            crate::service::authz::Permission::ExamSessionActivate,
            None,
        )
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let session = service.activate_exam_session(&user.org_id, &id, &user.id).await.map_err(|e| {
        (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e })))
    })?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": session }))))
}

#[utoipa::path(post, path = "/api/exam-sessions/{id}/complete", responses((status = 200)))]
pub async fn post_complete_exam_session(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(
            &user,
            crate::service::authz::Permission::ExamSessionWrite,
            None,
        )
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let session = service.complete_exam_session(&user.org_id, &id).await.map_err(|e| {
        (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e })))
    })?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": session }))))
}

// ── Exam Subjects ───────────────────────────────────────────────────────────

#[utoipa::path(get, path = "/api/exam-sessions/{id}/subjects", responses((status = 200)))]
pub async fn get_exam_subjects(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::ExamSessionRead, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let subjects = service.get_exam_subjects(&user.org_id, &id).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e })))
    })?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": subjects }))))
}

#[utoipa::path(post, path = "/api/exam-sessions/{id}/subjects", responses((status = 201)))]
pub async fn post_exam_subjects(
    State(service): State<Service>,
    Path(_id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateExamSubjectRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::ExamSessionWrite, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let subject = service.create_exam_subject(&user.org_id, req).await.map_err(|e| {
        (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e })))
    })?;

    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": subject }))))
}

// ── Exam Schedules ──────────────────────────────────────────────────────────

#[utoipa::path(get, path = "/api/exam-sessions/{id}/schedule", responses((status = 200)))]
pub async fn get_exam_schedule(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::ExamScheduleRead, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let schedules = service.get_exam_schedules(&user.org_id, &id).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e })))
    })?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": schedules }))))
}

#[utoipa::path(post, path = "/api/exam-sessions/{id}/schedule", responses((status = 201)))]
pub async fn post_exam_schedule(
    State(service): State<Service>,
    Path(_id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateExamScheduleRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::ExamScheduleWrite, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let schedule = service.create_exam_schedule(&user.org_id, req).await.map_err(|e| {
        (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e })))
    })?;

    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": schedule }))))
}

// ── Student Ordering ────────────────────────────────────────────────────────

#[utoipa::path(get, path = "/api/student-ordering", responses((status = 200)))]
pub async fn get_student_ordering(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(params): Query<crate::handler::params::StudentOrderingParams>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::StudentOrderRead, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let cycle_id = params.cycle_id.ok_or_else(|| {
        (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "cycle_id is required" })))
    })?;

    let students = service
        .get_ordered_students(&user.org_id, &cycle_id, params.class_id.as_deref())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": students }))))
}

#[utoipa::path(post, path = "/api/student-ordering/rebuild", responses((status = 200)))]
pub async fn post_rebuild_ordering(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(params): Query<crate::handler::params::StudentOrderingParams>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::StudentOrderRebuild, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let cycle_id = params.cycle_id.ok_or_else(|| {
        (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "cycle_id is required" })))
    })?;

    let strategy = params.strategy.as_deref().unwrap_or("name_ar_asc");

    let result = service
        .rebuild_ordering(&user.org_id, &cycle_id, strategy)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": result }))))
}

#[utoipa::path(post, path = "/api/student-ordering/freeze", responses((status = 200)))]
pub async fn post_freeze_ordering(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(params): Query<crate::handler::params::StudentOrderingParams>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::StudentOrderFreeze, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let cycle_id = params.cycle_id.ok_or_else(|| {
        (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "cycle_id is required" })))
    })?;

    let count = service
        .freeze_ordering(&user.org_id, &cycle_id, &user.id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": { "frozen_count": count } }))))
}

// ── Seating Numbers ─────────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/exam-sessions/{id}/seating-numbers/generate", responses((status = 200)))]
pub async fn post_generate_seating_numbers(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::SeatingNumberGenerate, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let result = service
        .generate_seating_numbers(&user.org_id, &id, &user.id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": result }))))
}

#[utoipa::path(get, path = "/api/exam-sessions/{id}/seating-numbers", responses((status = 200)))]
pub async fn get_seating_numbers(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Query(params): Query<crate::handler::params::SeatingNumberParams>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::SeatingNumberRead, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let numbers = service
        .get_seating_numbers(&user.org_id, &id, params.class_id.as_deref())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": numbers }))))
}

// ── Secret Numbers ──────────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/exam-sessions/{id}/secret-numbers/generate", responses((status = 200)))]
pub async fn post_generate_secret_numbers(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::SecretNumberGenerate, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let result = service
        .generate_secret_numbers(&user.org_id, &id, &user.id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": result }))))
}

#[utoipa::path(post, path = "/api/exam-sessions/{id}/secret-numbers/reveal", responses((status = 200)))]
pub async fn post_reveal_secret_numbers(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::SecretNumberReveal, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let count = service
        .reveal_secret_numbers(&user.org_id, &id, &user.id)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": { "revealed_count": count } }))))
}

// ── Exam Halls ──────────────────────────────────────────────────────────────

#[utoipa::path(get, path = "/api/exam-sessions/{id}/halls", responses((status = 200)))]
pub async fn get_exam_halls(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::ExamHallRead, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let halls = service.get_exam_halls(&user.org_id, &id).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e })))
    })?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": halls }))))
}

#[utoipa::path(post, path = "/api/exam-sessions/{id}/halls", responses((status = 201)))]
pub async fn post_exam_halls(
    State(service): State<Service>,
    Path(_id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateExamHallRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::ExamHallWrite, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let hall = service.create_exam_hall(&user.org_id, req).await.map_err(|e| {
        (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e })))
    })?;

    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": hall }))))
}

#[utoipa::path(post, path = "/api/exam-sessions/{id}/halls/allocate", responses((status = 200)))]
pub async fn post_allocate_halls(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(config): Json<AllocationConfig>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::ExamHallAllocate, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let result = service
        .allocate_students_to_halls(&user.org_id, &id, config)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": result }))))
}

#[utoipa::path(get, path = "/api/exam-sessions/{session_id}/halls/{hall_id}/students", responses((status = 200)))]
pub async fn get_hall_students(
    State(service): State<Service>,
    Path((session_id, hall_id)): Path<(String, String)>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::ExamHallRead, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let students = service
        .get_hall_students(&user.org_id, &session_id, &hall_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": students }))))
}

// ── Proctors ────────────────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/exam-sessions/{session_id}/proctors/assign/{schedule_id}", responses((status = 200)))]
pub async fn post_assign_proctors(
    State(service): State<Service>,
    Path((session_id, schedule_id)): Path<(String, String)>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(config): Json<ProctorAssignConfig>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::ProctorAssign, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let result = service
        .auto_assign_proctors(&user.org_id, &session_id, &schedule_id, config)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": result }))))
}

#[utoipa::path(get, path = "/api/exam-sessions/{id}/proctors", responses((status = 200)))]
pub async fn get_exam_proctors(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::ProctorRead, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let proctors = service
        .get_exam_proctors(&user.org_id, &id, None)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": proctors }))))
}

// ── Phase 6: Auto-Scheduling ────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/exam-sessions/{id}/auto-schedule", responses((status = 200)))]
pub async fn post_auto_schedule(
    State(service): State<Service>,
    Path(_id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<crate::service::exam_auto_schedule::AutoScheduleRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::ExamScheduleWrite, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let result = service
        .start_exam_solve(&user.org_id, &user.id, req)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": result }))))
}

#[utoipa::path(post, path = "/api/exam-sessions/{session_id}/schedule/{schedule_id}/postpone", responses((status = 200)))]
pub async fn post_postpone_exam(
    State(service): State<Service>,
    Path((session_id, schedule_id)): Path<(String, String)>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::ExamScheduleWrite, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let new_date = body["new_date"].as_str().ok_or_else(|| {
        (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "new_date is required" })))
    })?;
    let reason = body["reason"].as_str().unwrap_or("No reason provided");

    let schedule = service
        .postpone_exam(&user.org_id, &session_id, &schedule_id, new_date, reason)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": schedule }))))
}

// ── Phase 7: Dashboard & Reports ────────────────────────────────────────────

#[utoipa::path(get, path = "/api/exam-sessions/{id}/dashboard", responses((status = 200)))]
pub async fn get_exam_dashboard(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::ExamSessionRead, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let dashboard = service
        .get_exam_dashboard(&user.org_id, &id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": dashboard }))))
}

#[utoipa::path(post, path = "/api/exam-sessions/{id}/reports", responses((status = 200)))]
pub async fn post_exam_report(
    State(service): State<Service>,
    Path(_id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<crate::service::exam_reports::GenerateReportRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::ExamSessionRead, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let report = service
        .generate_exam_report(&user.org_id, req)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": report }))))
}

// ── Phase 8: Analytics ──────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/exam-sessions/{id}/analytics", responses((status = 200)))]
pub async fn post_exam_analytics(
    State(service): State<Service>,
    Path(_id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<crate::service::exam_analytics::ExamAnalyticsRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(&user, crate::service::authz::Permission::ExamSessionRead, None)
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let analytics = service
        .analyze_exam_session(&user.org_id, req)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": analytics }))))
}
