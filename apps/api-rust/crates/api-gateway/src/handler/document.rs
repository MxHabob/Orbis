use axum::{extract::{State, Path, Json}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use crate::app_state::AppState;
use crate::middleware::AuthenticatedUser;
use svc_document::ops;

#[utoipa::path(get, path = "/api/documents/{owner_id}")]
pub async fn get_documents(State(state): State<AppState>, Path(owner_id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    // If owner_id is "all", pass None to list all org documents
    let target = if owner_id == "all" { None } else { Some(owner_id.as_str()) };
    let request = shared_kernel::pagination::CursorRequest { cursor: None, limit: 100 };
    let r = ops::document::list(&ctx, target, None, None, request).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}

#[utoipa::path(post, path = "/api/documents")]
pub async fn post_documents(
    State(state): State<AppState>, 
    AuthenticatedUser(user): AuthenticatedUser, 
    mut multipart: axum::extract::Multipart
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    
    // We expect certain text fields (entity_type, entity_id, category, visibility) 
    // and one file field (file).
    let mut entity_type = String::new();
    let mut entity_id = String::new();
    let mut category = String::new();
    let mut visibility = String::from("internal");
    
    let mut file_name = String::new();
    let mut mime_type = String::new();
    let mut file_data = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "file" {
            file_name = field.file_name().unwrap_or("unnamed").to_string();
            mime_type = field.content_type().unwrap_or("application/octet-stream").to_string();
            file_data = field.bytes().await.map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))))?.to_vec();
        } else if name == "entity_type" {
            entity_type = field.text().await.unwrap_or_default();
        } else if name == "entity_id" {
            entity_id = field.text().await.unwrap_or_default();
        } else if name == "category" {
            category = field.text().await.unwrap_or_default();
        } else if name == "visibility" {
            visibility = field.text().await.unwrap_or_default();
        }
    }

    if file_data.is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "No file uploaded"}))));
    }

    let req = svc_document::ops::document::UploadRequest {
        file_name,
        mime_type,
        data: &file_data,
        entity_type,
        entity_id,
        category,
        visibility,
    };

    let r = svc_document::ops::document::upload_file(&ctx, state.platform.storage.as_ref(), req)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
        
    ok!(StatusCode::CREATED, r)
}

#[utoipa::path(delete, path = "/api/documents/{id}")]
pub async fn delete_document(State(state): State<AppState>, Path(id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    ops::document::delete(&ctx, &id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(json!({"deleted": true}))
}
