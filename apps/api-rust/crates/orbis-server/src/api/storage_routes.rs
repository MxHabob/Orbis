//! Storage routes — File upload (multipart), file download/streaming, presigned URLs, and document archiving.

use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use super::document_routes::{doc_to_dto, DocumentDto};
use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
use orbis_app::services::UploadFileInput;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PresignedUrlResponseDto {
    pub url: String,
    pub expires_in_seconds: u64,
}

#[derive(Deserialize, ToSchema)]
pub struct PresignRequest {
    pub expires_in_seconds: Option<u64>,
}

#[derive(Deserialize, ToSchema)]
pub struct ArchiveRequest {
    pub reason: Option<String>,
}

#[derive(Deserialize, IntoParams)]
pub struct StorageListQuery {
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
}

// ── Handlers ────────────────────────────────────────────────────

/// Upload physical file via multipart/form-data.
#[utoipa::path(
    post,
    path = "/api/v1/storage/upload",
    responses(
        (status = 201, description = "File uploaded successfully", body = DocumentDto),
        (status = 400, description = "Invalid payload or file missing"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer" = [])),
    tag = "Storage"
)]
async fn upload_file(
    ctx: AuthContext,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<ApiResponse<DocumentDto>>), ApiError> {
    let mut file_name = String::new();
    let mut mime_type = String::from("application/octet-stream");
    let mut file_data = Vec::new();
    let mut category: Option<String> = None;
    let mut entity_type: Option<String> = None;
    let mut entity_id: Option<Uuid> = None;
    let mut org_unit_id: Option<Uuid> = None;
    let mut folder_id: Option<Uuid> = None;
    let mut asset_type: Option<String> = None;
    let mut description: Option<String> = None;
    let mut academic_year_id: Option<Uuid> = None;
    let mut term_id: Option<Uuid> = None;
    let mut tags: Option<String> = None;
    let mut visibility: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::bad_request(format!("Multipart error: {e}")))?
    {
        let name = field.name().unwrap_or_default().to_string();
        match name.as_str() {
            "file" => {
                if let Some(fname) = field.file_name() {
                    file_name = fname.to_string();
                }
                if let Some(ctype) = field.content_type() {
                    mime_type = ctype.to_string();
                }
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|e| ApiError::bad_request(format!("Failed to read file bytes: {e}")))?;
                file_data = bytes.to_vec();
            }
            "category" => {
                let text = field.text().await.unwrap_or_default();
                if !text.is_empty() {
                    category = Some(text);
                }
            }
            "entity_type" => {
                let text = field.text().await.unwrap_or_default();
                if !text.is_empty() {
                    entity_type = Some(text);
                }
            }
            "entity_id" => {
                let text = field.text().await.unwrap_or_default();
                if let Ok(uid) = Uuid::parse_str(&text) {
                    entity_id = Some(uid);
                }
            }
            "org_unit_id" => {
                let text = field.text().await.unwrap_or_default();
                if let Ok(uid) = Uuid::parse_str(&text) {
                    org_unit_id = Some(uid);
                }
            }
            "folder_id" => {
                let text = field.text().await.unwrap_or_default();
                if let Ok(uid) = Uuid::parse_str(&text) {
                    folder_id = Some(uid);
                }
            }
            "asset_type" => {
                let text = field.text().await.unwrap_or_default();
                if !text.is_empty() {
                    asset_type = Some(text);
                }
            }
            "description" => {
                let text = field.text().await.unwrap_or_default();
                if !text.is_empty() {
                    description = Some(text);
                }
            }
            "academic_year_id" => {
                let text = field.text().await.unwrap_or_default();
                if let Ok(uid) = Uuid::parse_str(&text) {
                    academic_year_id = Some(uid);
                }
            }
            "term_id" => {
                let text = field.text().await.unwrap_or_default();
                if let Ok(uid) = Uuid::parse_str(&text) {
                    term_id = Some(uid);
                }
            }
            "tags" => {
                let text = field.text().await.unwrap_or_default();
                if !text.is_empty() {
                    tags = Some(text);
                }
            }
            "visibility" => {
                let text = field.text().await.unwrap_or_default();
                if !text.is_empty() {
                    visibility = Some(text);
                }
            }
            "name" => {
                let text = field.text().await.unwrap_or_default();
                if !text.is_empty() {
                    file_name = text;
                }
            }
            _ => {}
        }
    }

    if file_data.is_empty() {
        return Err(ApiError::bad_request("No file data provided in multipart payload"));
    }
    if file_name.is_empty() {
        file_name = "unnamed_upload.bin".to_string();
    }

    let input = UploadFileInput {
        name: file_name,
        folder_id,
        entity_type,
        entity_id,
        org_unit_id,
        category,
        asset_type,
        description,
        academic_year_id,
        term_id,
        tags,
        visibility,
        mime_type,
        data: file_data,
    };

    let doc = state
        .file_storage_service
        .upload_file(&ctx, ctx.tenant_id, input)
        .await?;

    Ok((StatusCode::CREATED, ApiResponse::ok(doc_to_dto(doc))))
}

/// Download raw file binary.
#[utoipa::path(
    get,
    path = "/api/v1/storage/{id}/file",
    params(("id" = Uuid, Path, description = "Document ID")),
    responses(
        (status = 200, description = "Raw file stream / binary", content_type = "application/octet-stream"),
        (status = 404, description = "Document or physical file not found")
    ),
    security(("bearer" = [])),
    tag = "Storage"
)]
async fn download_file(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Response, ApiError> {
    let (doc, bytes) = state
        .file_storage_service
        .download_file_bytes(&ctx, id)
        .await?;

    let mime = doc.mime_type.unwrap_or_else(|| "application/octet-stream".to_string());
    let mut headers = HeaderMap::new();

    if let Ok(mime_val) = HeaderValue::from_str(&mime) {
        headers.insert(header::CONTENT_TYPE, mime_val);
    }

    let disposition = format!("inline; filename=\"{}\"", doc.name);
    if let Ok(disp_val) = HeaderValue::from_str(&disposition) {
        headers.insert(header::CONTENT_DISPOSITION, disp_val);
    }

    if let Some(size) = doc.file_size_bytes {
        headers.insert(header::CONTENT_LENGTH, HeaderValue::from(size));
    }

    Ok((headers, Body::from(bytes)).into_response())
}

/// Generate secure presigned URL for direct object access.
#[utoipa::path(
    post,
    path = "/api/v1/storage/{id}/presign",
    params(("id" = Uuid, Path, description = "Document ID")),
    request_body = PresignRequest,
    responses(
        (status = 200, description = "Presigned URL generated", body = PresignedUrlResponseDto),
        (status = 404, description = "Document not found")
    ),
    security(("bearer" = [])),
    tag = "Storage"
)]
async fn get_access_url(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<PresignRequest>,
) -> Result<Json<ApiResponse<PresignedUrlResponseDto>>, ApiError> {
    let expires = p.expires_in_seconds.unwrap_or(3600);
    let url = state
        .file_storage_service
        .generate_access_url(&ctx, id, expires)
        .await?;

    Ok(ApiResponse::ok(PresignedUrlResponseDto {
        url,
        expires_in_seconds: expires,
    }))
}

/// Archive a document.
#[utoipa::path(
    post,
    path = "/api/v1/storage/{id}/archive",
    params(("id" = Uuid, Path, description = "Document ID")),
    request_body = ArchiveRequest,
    responses(
        (status = 200, description = "Document archived", body = DocumentDto),
        (status = 404, description = "Document not found")
    ),
    security(("bearer" = [])),
    tag = "Storage"
)]
async fn archive_document(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<ArchiveRequest>,
) -> Result<Json<ApiResponse<DocumentDto>>, ApiError> {
    let doc = state
        .file_storage_service
        .archive_document(&ctx, id, p.reason)
        .await?;

    Ok(ApiResponse::ok(doc_to_dto(doc)))
}

/// Restore an archived or trashed document.
#[utoipa::path(
    post,
    path = "/api/v1/storage/{id}/restore",
    params(("id" = Uuid, Path, description = "Document ID")),
    responses(
        (status = 200, description = "Document restored", body = DocumentDto),
        (status = 404, description = "Document not found")
    ),
    security(("bearer" = [])),
    tag = "Storage"
)]
async fn restore_document(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<DocumentDto>>, ApiError> {
    let doc = state
        .file_storage_service
        .restore_document(&ctx, id)
        .await?;

    Ok(ApiResponse::ok(doc_to_dto(doc)))
}

/// Move document to trash (soft-delete).
#[utoipa::path(
    delete,
    path = "/api/v1/storage/{id}/trash",
    params(("id" = Uuid, Path, description = "Document ID")),
    responses(
        (status = 204, description = "Document moved to trash"),
        (status = 404, description = "Document not found")
    ),
    security(("bearer" = [])),
    tag = "Storage"
)]
async fn trash_document(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    state.file_storage_service.trash_document(&ctx, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Permanently purge document and delete physical binary.
#[utoipa::path(
    delete,
    path = "/api/v1/storage/{id}/purge",
    params(("id" = Uuid, Path, description = "Document ID")),
    responses(
        (status = 204, description = "Document permanently purged"),
        (status = 404, description = "Document not found")
    ),
    security(("bearer" = [])),
    tag = "Storage"
)]
async fn purge_document(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    state.file_storage_service.purge_document(&ctx, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// List all archived documents for the organization.
#[utoipa::path(
    get,
    path = "/api/v1/storage/archive",
    responses(
        (status = 200, description = "List of archived documents", body = Vec<DocumentDto>)
    ),
    security(("bearer" = [])),
    tag = "Storage"
)]
async fn list_archived_documents(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<DocumentDto>>>, ApiError> {
    let docs = state.document_repo.find_archived(ctx.tenant_id).await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    Ok(ApiResponse::ok(docs.into_iter().map(doc_to_dto).collect()))
}

/// List all trashed documents for the organization.
#[utoipa::path(
    get,
    path = "/api/v1/storage/trash",
    responses(
        (status = 200, description = "List of trashed documents", body = Vec<DocumentDto>)
    ),
    security(("bearer" = [])),
    tag = "Storage"
)]
async fn list_trash_documents(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<DocumentDto>>>, ApiError> {
    let docs = state.document_repo.find_trash(ctx.tenant_id).await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    Ok(ApiResponse::ok(docs.into_iter().map(doc_to_dto).collect()))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/storage/upload", post(upload_file))
        .route("/api/v1/storage/archive", get(list_archived_documents))
        .route("/api/v1/storage/trash", get(list_trash_documents))
        .route("/api/v1/storage/:id/file", get(download_file))
        .route("/api/v1/storage/:id/presign", post(get_access_url))
        .route("/api/v1/storage/:id/archive", post(archive_document))
        .route("/api/v1/storage/:id/restore", post(restore_document))
        .route("/api/v1/storage/:id/trash", delete(trash_document))
        .route("/api/v1/storage/:id/purge", delete(purge_document))
}
