use axum::{extract::{State, Path, Json}, http::{header, StatusCode}, response::IntoResponse};
use serde_json::json;
use crate::app_state::AppState;
use crate::middleware::AuthenticatedUser;
use svc_document::ops;

#[utoipa::path(get, path = "/api/documents/{owner_id}")]
pub async fn get_documents(State(state): State<AppState>, Path(owner_id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    // If owner_id is "all", pass None to list all org documents
    let target = if owner_id == "all" { None } else { Some(owner_id.as_str()) };
    let request = shared_kernel::pagination::CursorRequest { cursor: None, limit: 100, filter: None };
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
    const MAX_UPLOAD_BYTES: usize = 25 * 1024 * 1024;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "file" {
            file_name = field.file_name().unwrap_or("unnamed").to_string();
            mime_type = field.content_type().unwrap_or("application/octet-stream").to_string();
            let bytes = field.bytes().await.map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))))?;
            if bytes.len() > MAX_UPLOAD_BYTES {
                return Err((StatusCode::PAYLOAD_TOO_LARGE, Json(json!({"success": false, "error": "File exceeds the 25 MiB upload limit"}))));
            }
            file_data = bytes.to_vec();
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
        school_year: None,
        term: None,
        tags: None,
        visibility,
    };

    let r = svc_document::ops::document::upload_file(&ctx, state.platform.storage.as_ref(), req)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
        
    ok!(StatusCode::CREATED, r)
}

#[utoipa::path(get, path = "/api/documents/{id}/download")]
pub async fn download_document(
    State(state): State<AppState>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let ctx = state.build_context(&user.org_id, &user.id);
    ctx.permissions.check(&ctx.actor, shared_kernel::traits::Permission::DocumentRead).await
        .map_err(|_| (StatusCode::FORBIDDEN, Json(json!({"success": false, "error": "Access denied"}))))?;

    let document = svc_document::entities::documents::Entity::find_by_id(id)
        .filter(svc_document::entities::documents::Column::OrgId.eq(ctx.org_id.clone()))
        .filter(svc_document::entities::documents::Column::DeletedAt.is_null())
        .one(&ctx.db)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success": false, "error": "Document lookup failed"}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"success": false, "error": "Document not found"}))))?;

    let path = std::path::Path::new(&document.path);
    let bucket = path.parent().and_then(|value| value.to_str()).ok_or_else(|| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success": false, "error": "Invalid document path"})))
    })?;
    let key = path.file_name().and_then(|value| value.to_str()).ok_or_else(|| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success": false, "error": "Invalid document path"})))
    })?;
    let data = state.platform.storage.read(bucket, key).await
        .map_err(|_| (StatusCode::NOT_FOUND, Json(json!({"success": false, "error": "Document content not found"}))))?;
    let content_type = document.mime_type.as_deref().unwrap_or("application/octet-stream")
        .parse::<header::HeaderValue>()
        .unwrap_or_else(|_| header::HeaderValue::from_static("application/octet-stream"));

    Ok(([(header::CONTENT_TYPE, content_type)], data))
}

#[utoipa::path(delete, path = "/api/documents/{id}")]
pub async fn delete_document(State(state): State<AppState>, Path(id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    ops::document::delete(&ctx, &id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(json!({"deleted": true}))
}
