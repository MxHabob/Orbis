use axum::{
    extract::{State, Query, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use crate::service::Service;
use crate::middleware::AuthenticatedUser;

#[utoipa::path(get, path = "/api/audit-logs", responses((status = 200, description = "Audit logs")))]
pub async fn get_audit_logs(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::AuditLogParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = crate::service::authz::AuthzContext {
        target_org_id: Some(user.org_id.clone()),
        ..Default::default()
    };
    service.check_permission_and_audit(&user, crate::service::authz::Permission::AuditRead, Some(&ctx)).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let logs = service.repo.get_audit_logs(&user.org_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e.to_string() }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": logs }))))
}
