//! Document routes — Upload, List, Get, Delete.

use axum::{extract::{State, Path}, routing::{get, post, delete}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_app::usecases::document::{
    upload::UploadDocumentUseCase,
    delete::DeleteDocumentUseCase,
};

#[derive(Serialize, ToSchema)]
pub struct DocumentDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub file_name: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct UploadDocumentRequest {
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub file_name: String,
    pub mime_type: String,
    pub data_base64: String,
}

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/documents", request_body = UploadDocumentRequest,
    responses((status = 201, body = DocumentDto)), security(("bearer" = [])), tag = "Document")]
async fn upload_document(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<UploadDocumentRequest>,
) -> Result<Json<ApiResponse<DocumentDto>>, ApiError> {
    let uc = UploadDocumentUseCase {
        document_repo: state.document_repo.clone(),
        event_bus: state.event_bus.clone(),
    };
    let doc = uc.execute(&ctx, ctx.tenant_id, p.entity_type, p.entity_id, p.file_name, p.mime_type, p.data_base64).await?;
    Ok(ApiResponse::ok(DocumentDto {
        id: doc.id, org_id: doc.org_id, entity_type: doc.entity_type,
        entity_id: doc.entity_id, file_name: doc.file_name, mime_type: doc.mime_type,
        size_bytes: doc.size_bytes, created_at: doc.created_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/documents",
    responses((status = 200, body = Vec<DocumentDto>)), security(("bearer" = [])), tag = "Document")]
async fn list_documents(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<DocumentDto>>>, ApiError> {
    let docs = state.document_repo.find_by_org(ctx.tenant_id).await?;
    let dtos = docs.into_iter().map(|d| DocumentDto {
        id: d.id, org_id: d.org_id, entity_type: d.entity_type,
        entity_id: d.entity_id, file_name: d.file_name, mime_type: d.mime_type,
        size_bytes: d.size_bytes, created_at: d.created_at,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/documents/{id}", params(("id" = Uuid, Path)),
    responses((status = 200, body = DocumentDto)), security(("bearer" = [])), tag = "Document")]
async fn get_document(
    _ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<DocumentDto>>, ApiError> {
    let d = state.document_repo.find_by_id(id).await?.ok_or_else(|| ApiError::not_found("Document not found"))?;
    Ok(ApiResponse::ok(DocumentDto {
        id: d.id, org_id: d.org_id, entity_type: d.entity_type,
        entity_id: d.entity_id, file_name: d.file_name, mime_type: d.mime_type,
        size_bytes: d.size_bytes, created_at: d.created_at,
    }))
}

#[utoipa::path(delete, path = "/api/v1/documents/{id}", params(("id" = Uuid, Path)),
    responses((status = 204, description = "Deleted")), security(("bearer" = [])), tag = "Document")]
async fn delete_document(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    let uc = DeleteDocumentUseCase {
        document_repo: state.document_repo.clone(),
        event_bus: state.event_bus.clone(),
    };
    uc.execute(&ctx, id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/documents", post(upload_document).get(list_documents))
        .route("/api/v1/documents/{id}", get(get_document).delete(delete_document))
}
