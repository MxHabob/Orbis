use axum::{
    extract::{State, Path, Query, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use crate::service::Service;
use crate::middleware::AuthenticatedUser;
use crate::service::schedule::{
    CreateTimeSlotRequest, UpdateTimeSlotRequest,
    CreateClassSubjectRequest, UpdateClassSubjectRequest,
    CreateLessonRequest, UpdateLessonRequest,
};

#[utoipa::path(get, path = "/api/schedule/time-slots", responses((status = 200)))]
pub async fn get_time_slots(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::ScheduleParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ScheduleRead, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let cycle_id = params.cycle_id.ok_or_else(|| {
        (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "cycle_id is required" })))
    })?;

    let slots = service.get_time_slots(&user.org_id, &cycle_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": slots }))))
}

#[utoipa::path(post, path = "/api/schedule/time-slots", responses((status = 201)))]
pub async fn post_time_slots(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateTimeSlotRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ScheduleWrite, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let slot = service.create_time_slot(&user.org_id, req).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": slot }))))
}

#[utoipa::path(put, path = "/api/schedule/time-slots/{id}", responses((status = 200)))]
pub async fn put_time_slot(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<UpdateTimeSlotRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ScheduleWrite, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let slot = service.update_time_slot(&id, req).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": slot }))))
}

#[utoipa::path(delete, path = "/api/schedule/time-slots/{id}", responses((status = 200)))]
pub async fn delete_time_slot(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ScheduleDelete, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    service.delete_time_slot(&id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "message": "time slot deleted" }))))
}

#[utoipa::path(get, path = "/api/schedule/class-subjects", responses((status = 200)))]
pub async fn get_class_subjects(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::ClassSubjectParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ScheduleRead, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let items = service.get_class_subjects(
        params.class_id.as_deref(),
        params.cycle_id.as_deref(),
    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": items }))))
}

#[utoipa::path(post, path = "/api/schedule/class-subjects", responses((status = 201)))]
pub async fn post_class_subjects(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateClassSubjectRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ScheduleWrite, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let item = service.create_class_subject(req).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": item }))))
}

#[utoipa::path(put, path = "/api/schedule/class-subjects/{id}", responses((status = 200)))]
pub async fn put_class_subject(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<UpdateClassSubjectRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ScheduleWrite, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let item = service.update_class_subject(&id, req).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": item }))))
}

#[utoipa::path(delete, path = "/api/schedule/class-subjects/{id}", responses((status = 200)))]
pub async fn delete_class_subject(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ScheduleDelete, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    service.delete_class_subject(&id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "message": "class subject deleted" }))))
}

#[utoipa::path(get, path = "/api/schedule/lessons", responses((status = 200)))]
pub async fn get_lessons(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::ScheduleParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ScheduleRead, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let cycle_id = params.cycle_id.ok_or_else(|| {
        (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "cycle_id is required" })))
    })?;

    let lessons = if let Some(class_id) = params.class_id {
        service.get_class_schedule(&user.org_id, &cycle_id, &class_id).await
    } else if let Some(teacher_id) = params.teacher_id {
        service.get_teacher_schedule(&user.org_id, &cycle_id, &teacher_id).await
    } else {
        return Err((StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "class_id or teacher_id is required" }))));
    }.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": lessons }))))
}

#[utoipa::path(get, path = "/api/schedule/lessons/{id}", responses((status = 200)))]
pub async fn get_lesson(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ScheduleRead, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let lesson = service.get_lesson_by_id(&id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": lesson }))))
}

#[utoipa::path(post, path = "/api/schedule/lessons", responses((status = 201)))]
pub async fn post_lessons(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateLessonRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ScheduleWrite, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let lesson = service.create_lesson(&user.org_id, req).await
        .map_err(|e| (StatusCode::CONFLICT, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": lesson }))))
}

#[utoipa::path(put, path = "/api/schedule/lessons/{id}", responses((status = 200)))]
pub async fn put_lesson(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<UpdateLessonRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ScheduleWrite, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let lesson = service.update_lesson(&user.org_id, &id, req).await
        .map_err(|e| (StatusCode::CONFLICT, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": lesson }))))
}

#[utoipa::path(delete, path = "/api/schedule/lessons/{id}", responses((status = 200)))]
pub async fn delete_lesson(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ScheduleDelete, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    service.delete_lesson(&id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "message": "lesson cancelled" }))))
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
pub struct ConflictCheckRequest {
    pub cycle_id: String,
    pub time_slot_id: String,
    pub day_of_week: i32,
    pub teacher_id: String,
    pub class_id: String,
    pub room_id: Option<String>,
    pub exclude_id: Option<String>,
}

#[utoipa::path(post, path = "/api/schedule/conflicts", responses((status = 200)))]
pub async fn post_check_conflicts(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<ConflictCheckRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ScheduleRead, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let conflicts = service.check_schedule_conflicts(
        &user.org_id,
        &req.cycle_id,
        &req.time_slot_id,
        req.day_of_week,
        req.exclude_id.as_deref(),
        &req.teacher_id,
        &req.class_id,
        req.room_id.as_deref(),
    ).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": conflicts, "has_conflicts": !conflicts.is_empty() }))))
}
