use axum::{
    extract::{State, Path, Query, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use crate::service::Service;
use crate::service::attendance_svc::{CreateAttendanceRequest, BulkAttendanceRequest};
use crate::middleware::AuthenticatedUser;

#[utoipa::path(get, path = "/api/attendance", responses((status = 200, description = "Attendance records")))]
pub async fn get_attendance(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::AttendanceParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::AttendanceRead, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let records = service.get_attendance(&user.org_id, params).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": records }))))
}

#[utoipa::path(post, path = "/api/attendance", request_body = CreateAttendanceRequest, responses((status = 201, description = "Attendance recorded")))]
pub async fn post_attendance(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateAttendanceRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::AttendanceWrite, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let record = service.record_attendance(&user.org_id, &user.id, req).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": record }))))
}

#[utoipa::path(post, path = "/api/attendance/bulk", request_body = BulkAttendanceRequest, responses((status = 201, description = "Bulk attendance recorded")))]
pub async fn post_attendance_bulk(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<BulkAttendanceRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::AttendanceWrite, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let records = service.record_bulk_attendance(&user.org_id, &user.id, req).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": records, "count": records.len() }))))
}

#[utoipa::path(get, path = "/api/attendance/report/{enrollment_id}", responses((status = 200, description = "Student attendance report")))]
pub async fn get_attendance_report(
    State(service): State<Service>,
    Path(enrollment_id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = crate::service::authz::AuthzContext {
        target_user_id: Some(enrollment_id.clone()),
        ..Default::default()
    };
    service.check_permission_and_audit(&user, crate::service::authz::Permission::AttendanceReport, Some(&ctx)).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let report = service.get_attendance_report(&enrollment_id).await
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": report }))))
}

#[utoipa::path(get, path = "/api/attendance/analytics", responses((status = 200, description = "Attendance analytics")))]
pub async fn get_attendance_analytics(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::AttendanceParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::AttendanceAnalytics, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let analytics = service.get_attendance_analytics(&user.org_id, params).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": analytics }))))
}
