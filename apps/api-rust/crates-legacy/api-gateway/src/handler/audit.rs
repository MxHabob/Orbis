use axum::{extract::{State, Json}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use crate::app_state::AppState;
use crate::middleware::AuthenticatedUser;
use svc_iam::ops;

#[utoipa::path(get, path = "/api/audit-logs")]
pub async fn get_audit_logs(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::audit::list(&ctx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}
