use axum::{
    extract::{State, Path, Query, Json},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::service::Service;
use crate::middleware::AuthenticatedUser;

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct CsvImportBody {
    pub csv_text: String,
}

// ── Import Endpoints ──

#[utoipa::path(post, path = "/api/import/students", request_body = CsvImportBody, responses((status = 200, description = "Students imported")))]
pub async fn post_import_students(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(body): Json<CsvImportBody>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::ImportData, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let result = service.import_students_csv(&user.org_id, &body.csv_text, &user.id).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": result }))))
}

#[utoipa::path(post, path = "/api/import/grades", request_body = CsvImportBody, responses((status = 200, description = "Grades imported")))]
pub async fn post_import_grades(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(body): Json<CsvImportBody>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::GradeWrite, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let result = service.import_grades_csv(&user.org_id, &body.csv_text, &user.id).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": result }))))
}

#[utoipa::path(post, path = "/api/import/attendance", request_body = CsvImportBody, responses((status = 200, description = "Attendance imported")))]
pub async fn post_import_attendance(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(body): Json<CsvImportBody>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::AttendanceWrite, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let result = service.import_attendance_csv(&user.org_id, &body.csv_text, &user.id).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": result }))))
}

// ── Export Endpoints ──

#[utoipa::path(get, path = "/api/export/students", responses((status = 200, description = "Students CSV")))]
pub async fn get_export_students(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::ExportParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::ExportData, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let lang = params.lang.as_deref().unwrap_or("en");
    let csv_data = service.export_students_csv(&user.org_id, lang).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/csv; charset=utf-8")
        .header(header::CONTENT_DISPOSITION, "attachment; filename=\"students.csv\"")
        .body(axum::body::Body::from(csv_data))
        .unwrap())
}

#[utoipa::path(get, path = "/api/export/grades", responses((status = 200, description = "Grades CSV")))]
pub async fn get_export_grades(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::GradeParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::GradeRead, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let csv_data = service.export_grades_csv(&user.org_id, params, "en").await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/csv; charset=utf-8")
        .header(header::CONTENT_DISPOSITION, "attachment; filename=\"grades.csv\"")
        .body(axum::body::Body::from(csv_data))
        .unwrap())
}

#[utoipa::path(get, path = "/api/export/attendance", responses((status = 200, description = "Attendance CSV")))]
pub async fn get_export_attendance(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::AttendanceParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::AttendanceRead, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let csv_data = service.export_attendance_csv(&user.org_id, params, "en").await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/csv; charset=utf-8")
        .header(header::CONTENT_DISPOSITION, "attachment; filename=\"attendance.csv\"")
        .body(axum::body::Body::from(csv_data))
        .unwrap())
}

#[utoipa::path(get, path = "/api/export/report-cards/{class_id}", responses((status = 200, description = "Report cards CSV")))]
pub async fn get_export_report_cards(
    State(service): State<Service>,
    Path(class_id): Path<String>,
    Query(params): Query<crate::handler::params::ExportParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::GradeReport, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let lang = params.lang.as_deref().unwrap_or("en");
    let csv_data = service.export_report_cards_csv(&user.org_id, &class_id, lang).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/csv; charset=utf-8")
        .header(header::CONTENT_DISPOSITION, "attachment; filename=\"report_cards.csv\"")
        .body(axum::body::Body::from(csv_data))
        .unwrap())
}
