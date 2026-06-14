use axum::{
    extract::{State, Path, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use crate::service::Service;
use crate::service::organization::UpdateOrganizationRequest;
use crate::middleware::AuthenticatedUser;

#[utoipa::path(get, path = "/api/organizations/{id}", responses((status = 200, description = "Organization details")))]
pub async fn get_organization(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = crate::service::authz::AuthzContext {
        target_org_id: Some(id.clone()),
        ..Default::default()
    };
    service.authz.check_permission(&user, crate::service::authz::Permission::OrganizationRead, Some(&ctx))
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let org = service.get_organization(&id).await
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": org }))))
}

#[utoipa::path(put, path = "/api/organizations/{id}", request_body = UpdateOrganizationRequest, responses((status = 200, description = "Organization updated")))]
pub async fn put_organization(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<UpdateOrganizationRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = crate::service::authz::AuthzContext {
        target_org_id: Some(id.clone()),
        ..Default::default()
    };
    service.authz.check_permission(&user, crate::service::authz::Permission::OrganizationWrite, Some(&ctx))
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let org = service.update_organization(&id, req).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": org }))))
}

#[utoipa::path(get, path = "/api/organizations/{id}/stats", responses((status = 200, description = "Organization statistics")))]
pub async fn get_organization_stats(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = crate::service::authz::AuthzContext {
        target_org_id: Some(id.clone()),
        ..Default::default()
    };
    service.authz.check_permission(&user, crate::service::authz::Permission::DashboardRead, Some(&ctx))
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let stats = service.get_dashboard_stats(&id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": stats }))))
}
