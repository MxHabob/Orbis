use axum::{
    extract::{State, Path, Query, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use crate::service::Service;
use crate::service::org_unit::{CreateOrgUnitRequest, UpdateOrgUnitRequest};
use crate::middleware::AuthenticatedUser;

#[utoipa::path(get, path = "/api/units", responses((status = 200, description = "Org units list")))]
pub async fn get_units(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::OrgUnitParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::OrgUnitRead, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let units = service.get_org_units(&user.org_id, params).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": units }))))
}

#[utoipa::path(post, path = "/api/units", request_body = CreateOrgUnitRequest, responses((status = 201, description = "Unit created")))]
pub async fn post_units(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateOrgUnitRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::OrgUnitWrite, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let unit = service.create_org_unit(&user.org_id, req).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": unit }))))
}

#[utoipa::path(put, path = "/api/units/{id}", request_body = UpdateOrgUnitRequest, responses((status = 200, description = "Unit updated")))]
pub async fn put_unit(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<UpdateOrgUnitRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::OrgUnitWrite, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let unit = service.update_org_unit(&id, req).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": unit }))))
}

#[utoipa::path(delete, path = "/api/units/{id}", responses((status = 200, description = "Unit deleted")))]
pub async fn delete_unit(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::OrgUnitDelete, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    service.delete_org_unit(&id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "message": "unit deleted" }))))
}
