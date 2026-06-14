use axum::{
    extract::{State, Path, Query, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use crate::service::Service;
use crate::service::grade::{CreateGradeRequest, BulkGradeRequest, UpdateGradeRequest};
use crate::middleware::AuthenticatedUser;

#[utoipa::path(get, path = "/api/grades", responses((status = 200, description = "Grades list")))]
pub async fn get_grades(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::GradeParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::GradeRead, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let grades = service.get_grades(&user.org_id, params).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": grades }))))
}

#[utoipa::path(post, path = "/api/grades", request_body = CreateGradeRequest, responses((status = 201, description = "Grade submitted")))]
pub async fn post_grades(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateGradeRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::GradeWrite, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let grade = service.submit_grade(&user.org_id, &user.id, req).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": grade }))))
}

#[utoipa::path(post, path = "/api/grades/bulk", request_body = BulkGradeRequest, responses((status = 201, description = "Grades submitted in bulk")))]
pub async fn post_grades_bulk(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<BulkGradeRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::GradeWrite, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let grades = service.submit_grades_bulk(&user.org_id, &user.id, req).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": grades, "count": grades.len() }))))
}

#[utoipa::path(put, path = "/api/grades/{id}", request_body = UpdateGradeRequest, responses((status = 200, description = "Grade updated")))]
pub async fn put_grade(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<UpdateGradeRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::GradeWrite, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let grade = service.update_grade(&id, req).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": grade }))))
}

#[utoipa::path(get, path = "/api/grades/report/{enrollment_id}", responses((status = 200, description = "Student report card")))]
pub async fn get_report_card(
    State(service): State<Service>,
    Path(enrollment_id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = crate::service::authz::AuthzContext {
        target_user_id: Some(enrollment_id.clone()),
        ..Default::default()
    };
    // Map GradeReport to checked permission
    service.authz.check_permission(&user, crate::service::authz::Permission::GradeReport, Some(&ctx))
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let report = service.generate_report_card(&enrollment_id).await
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": report }))))
}

#[utoipa::path(get, path = "/api/grades/analytics", responses((status = 200, description = "Grade analytics")))]
pub async fn get_grade_analytics(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::GradeParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::GradeAnalytics, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let analytics = service.get_grade_analytics(&user.org_id, params).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": analytics }))))
}
