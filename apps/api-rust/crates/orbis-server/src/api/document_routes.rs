//! Document routes — Upload, List, Get, Delete.

use axum::{extract::{State, Path}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_app::services::CreateDocumentInput;

#[derive(Serialize, ToSchema)]
pub struct DocumentDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub uploaded_by: Option<Uuid>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub asset_type: String,
    pub school_year: Option<String>,
    pub term: Option<String>,
    pub tags: Option<String>,
    pub visibility: String,
    #[serde(alias = "size")]
    pub file_size_bytes: Option<i64>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub storage_provider: String,
    #[serde(alias = "path")]
    pub file_path: String,
    pub url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub preview_path: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct UploadDocumentRequest {
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub asset_type: Option<String>,
    pub school_year: Option<String>,
    pub term: Option<String>,
    pub tags: Option<String>,
    pub visibility: Option<String>,
    #[serde(alias = "size")]
    pub file_size_bytes: Option<i64>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub storage_provider: Option<String>,
    #[serde(alias = "path")]
    pub file_path: String,
    pub url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub preview_path: Option<String>,
}

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/documents", request_body = UploadDocumentRequest,
    responses((status = 201, body = DocumentDto)), security(("bearer" = [])), tag = "Document")]
async fn upload_document(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<UploadDocumentRequest>,
) -> Result<Json<ApiResponse<DocumentDto>>, ApiError> {
    let uploaded_by = match ctx.actor {
        orbis_domain::iam::actor::Actor::User(uid) => Some(uid),
        _ => None,
    };
    let input = CreateDocumentInput {
        uploaded_by,
        entity_type: p.entity_type,
        entity_id: p.entity_id,
        name: p.name,
        description: p.description,
        category: p.category.unwrap_or_else(|| "general".to_string()),
        asset_type: p.asset_type.unwrap_or_else(|| "file".to_string()),
        school_year: p.school_year,
        term: p.term,
        tags: p.tags,
        visibility: p.visibility.unwrap_or_else(|| "internal".to_string()),
        file_size_bytes: p.file_size_bytes,
        mime_type: p.mime_type,
        extension: p.extension,
        storage_provider: p.storage_provider.unwrap_or_else(|| "local".to_string()),
        file_path: p.file_path,
        url: p.url,
        thumbnail_path: p.thumbnail_path,
        preview_path: p.preview_path,
    };
    let doc = state.document_service.create_document(&ctx, ctx.tenant_id, input).await?;
    Ok(ApiResponse::ok(DocumentDto {
        id: doc.id,
        org_id: doc.org_id,
        uploaded_by: doc.uploaded_by,
        entity_type: doc.entity_type,
        entity_id: doc.entity_id,
        name: doc.name,
        description: doc.description,
        category: doc.category,
        asset_type: doc.asset_type,
        school_year: doc.school_year,
        term: doc.term,
        tags: doc.tags,
        visibility: doc.visibility,
        file_size_bytes: doc.file_size_bytes,
        mime_type: doc.mime_type,
        extension: doc.extension,
        storage_provider: doc.storage_provider,
        file_path: doc.file_path,
        url: doc.url,
        thumbnail_path: doc.thumbnail_path,
        preview_path: doc.preview_path,
        created_at: doc.created_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/documents",
    responses((status = 200, body = Vec<DocumentDto>)), security(("bearer" = [])), tag = "Document")]
async fn list_documents(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<DocumentDto>>>, ApiError> {
    let docs = state.document_service.list_documents_by_org(ctx.tenant_id).await?;
    let dtos = docs.into_iter().map(|doc| DocumentDto {
        id: doc.id,
        org_id: doc.org_id,
        uploaded_by: doc.uploaded_by,
        entity_type: doc.entity_type,
        entity_id: doc.entity_id,
        name: doc.name,
        description: doc.description,
        category: doc.category,
        asset_type: doc.asset_type,
        school_year: doc.school_year,
        term: doc.term,
        tags: doc.tags,
        visibility: doc.visibility,
        file_size_bytes: doc.file_size_bytes,
        mime_type: doc.mime_type,
        extension: doc.extension,
        storage_provider: doc.storage_provider,
        file_path: doc.file_path,
        url: doc.url,
        thumbnail_path: doc.thumbnail_path,
        preview_path: doc.preview_path,
        created_at: doc.created_at,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/documents/{id}", params(("id" = Uuid, Path, description = "Document ID")),
    responses((status = 200, body = DocumentDto)), security(("bearer" = [])), tag = "Document")]
async fn get_document(
    _ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<DocumentDto>>, ApiError> {
    let doc = state.document_service.get_document(id).await?.ok_or_else(|| ApiError::not_found("Document not found"))?;
    Ok(ApiResponse::ok(DocumentDto {
        id: doc.id,
        org_id: doc.org_id,
        uploaded_by: doc.uploaded_by,
        entity_type: doc.entity_type,
        entity_id: doc.entity_id,
        name: doc.name,
        description: doc.description,
        category: doc.category,
        asset_type: doc.asset_type,
        school_year: doc.school_year,
        term: doc.term,
        tags: doc.tags,
        visibility: doc.visibility,
        file_size_bytes: doc.file_size_bytes,
        mime_type: doc.mime_type,
        extension: doc.extension,
        storage_provider: doc.storage_provider,
        file_path: doc.file_path,
        url: doc.url,
        thumbnail_path: doc.thumbnail_path,
        preview_path: doc.preview_path,
        created_at: doc.created_at,
    }))
}

#[utoipa::path(delete, path = "/api/v1/documents/{id}", params(("id" = Uuid, Path, description = "Document deleted")),
    responses((status = 204, description = "Deleted")), security(("bearer" = [])), tag = "Document")]
async fn delete_document(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    state.document_service.delete_document(&ctx, id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/documents", post(upload_document).get(list_documents))
        .route("/api/v1/documents/:id", get(get_document).delete(delete_document))
}
