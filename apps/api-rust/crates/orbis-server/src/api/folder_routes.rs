//! Folder routes — Folders hierarchy, Student folders, and AI-curriculum knowledge base folders.

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use super::document_routes::{DocumentDto, doc_to_dto};
use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
use orbis_app::services::{CreateFolderInput, UpdateFolderInput};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct FolderDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub path: String,
    pub folder_type: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub org_unit_id: Option<Uuid>,
    pub calendar_id: Option<Uuid>,
    pub term_id: Option<Uuid>,
    pub is_system: bool,
    pub metadata: Option<serde_json::Value>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn folder_to_dto(f: orbis_domain::entities::folder::Folder) -> FolderDto {
    FolderDto {
        id: f.id,
        org_id: f.org_id,
        parent_id: f.parent_id,
        name: f.name,
        path: f.path,
        folder_type: f.folder_type,
        entity_type: f.entity_type,
        entity_id: f.entity_id,
        org_unit_id: f.org_unit_id,
        calendar_id: f.calendar_id,
        term_id: f.term_id,
        is_system: f.is_system,
        metadata: f.metadata,
        created_by: f.created_by,
        created_at: f.created_at,
        updated_at: f.updated_at,
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct FolderContentsDto {
    pub folder: FolderDto,
    pub subfolders: Vec<FolderDto>,
    pub documents: Vec<DocumentDto>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateFolderRequest {
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub folder_type: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub org_unit_id: Option<Uuid>,
    pub calendar_id: Option<Uuid>,
    pub term_id: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateFolderRequest {
    pub name: Option<String>,
    pub parent_id: Option<Uuid>,
    pub calendar_id: Option<Uuid>,
    pub term_id: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateCurriculumFolderRequest {
    pub name: String,
    pub calendar_id: Option<Uuid>,
    pub term_id: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Deserialize, IntoParams)]
pub struct ListFoldersQuery {
    pub parent_id: Option<Uuid>,
}

#[derive(Deserialize, IntoParams)]
pub struct StudentFolderQuery {
    pub student_name: Option<String>,
}

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/folders",
    request_body = CreateFolderRequest,
    responses((status = 201, body = FolderDto)),
    security(("bearer" = [])),
    tag = "Folder"
)]
async fn create_folder(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<CreateFolderRequest>,
) -> Result<Json<ApiResponse<FolderDto>>, ApiError> {
    let input = CreateFolderInput {
        name: p.name,
        parent_id: p.parent_id,
        folder_type: p.folder_type,
        entity_type: p.entity_type,
        entity_id: p.entity_id,
        org_unit_id: p.org_unit_id,
        calendar_id: p.calendar_id,
        term_id: p.term_id,
        metadata: p.metadata,
    };
    let folder = state.folder_service.create_folder(&ctx, input).await?;
    Ok(ApiResponse::ok(folder_to_dto(folder)))
}

#[utoipa::path(
    get,
    path = "/api/v1/folders",
    params(ListFoldersQuery),
    responses((status = 200, body = Vec<FolderDto>)),
    security(("bearer" = [])),
    tag = "Folder"
)]
async fn list_folders(
    ctx: AuthContext,
    State(state): State<AppState>,
    Query(query): Query<ListFoldersQuery>,
) -> Result<Json<ApiResponse<Vec<FolderDto>>>, ApiError> {
    let folders = state
        .folder_service
        .list_folders(&ctx, query.parent_id)
        .await?;
    Ok(ApiResponse::ok(
        folders.into_iter().map(folder_to_dto).collect(),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/folders/{id}",
    params(("id" = Uuid, Path, description = "Folder ID")),
    responses((status = 200, body = FolderDto)),
    security(("bearer" = [])),
    tag = "Folder"
)]
async fn get_folder(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<FolderDto>>, ApiError> {
    let folder = state.folder_service.get_folder(&ctx, id).await?;
    Ok(ApiResponse::ok(folder_to_dto(folder)))
}

#[utoipa::path(
    get,
    path = "/api/v1/folders/{id}/contents",
    params(("id" = Uuid, Path, description = "Folder ID")),
    responses((status = 200, body = FolderContentsDto)),
    security(("bearer" = [])),
    tag = "Folder"
)]
async fn get_folder_contents(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<FolderContentsDto>>, ApiError> {
    let contents = state.folder_service.get_folder_contents(&ctx, id).await?;
    Ok(ApiResponse::ok(FolderContentsDto {
        folder: folder_to_dto(contents.folder),
        subfolders: contents.subfolders.into_iter().map(folder_to_dto).collect(),
        documents: contents.documents.into_iter().map(doc_to_dto).collect(),
    }))
}

#[utoipa::path(
    get,
    path = "/api/v1/folders/student/{student_id}",
    params(
        ("student_id" = Uuid, Path, description = "Student ID"),
        StudentFolderQuery
    ),
    responses((status = 200, body = FolderDto)),
    security(("bearer" = [])),
    tag = "Folder"
)]
async fn get_or_create_student_folder(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(student_id): Path<Uuid>,
    Query(query): Query<StudentFolderQuery>,
) -> Result<Json<ApiResponse<FolderDto>>, ApiError> {
    let student_name = query.student_name.unwrap_or_else(|| student_id.to_string());
    let folder = state
        .folder_service
        .get_or_create_student_folder(&ctx, student_id, &student_name)
        .await?;
    Ok(ApiResponse::ok(folder_to_dto(folder)))
}

#[utoipa::path(
    get,
    path = "/api/v1/folders/curriculum",
    responses((status = 200, body = Vec<FolderDto>)),
    security(("bearer" = [])),
    tag = "Folder"
)]
async fn list_curriculum_folders(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<FolderDto>>>, ApiError> {
    let folders = state.folder_service.list_curriculum_folders(&ctx).await?;
    Ok(ApiResponse::ok(
        folders.into_iter().map(folder_to_dto).collect(),
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/folders/curriculum",
    request_body = CreateCurriculumFolderRequest,
    responses((status = 201, body = FolderDto)),
    security(("bearer" = [])),
    tag = "Folder"
)]
async fn get_or_create_curriculum_folder(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<CreateCurriculumFolderRequest>,
) -> Result<Json<ApiResponse<FolderDto>>, ApiError> {
    let folder = state
        .folder_service
        .get_or_create_curriculum_folder(&ctx, &p.name, p.calendar_id, p.term_id, p.metadata)
        .await?;
    Ok(ApiResponse::ok(folder_to_dto(folder)))
}

#[utoipa::path(
    patch,
    path = "/api/v1/folders/{id}",
    params(("id" = Uuid, Path, description = "Folder ID")),
    request_body = UpdateFolderRequest,
    responses((status = 200, body = FolderDto)),
    security(("bearer" = [])),
    tag = "Folder"
)]
async fn update_folder(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateFolderRequest>,
) -> Result<Json<ApiResponse<FolderDto>>, ApiError> {
    let input = UpdateFolderInput {
        name: p.name,
        parent_id: p.parent_id,
        calendar_id: p.calendar_id,
        term_id: p.term_id,
        metadata: p.metadata,
    };
    let folder = state.folder_service.update_folder(&ctx, id, input).await?;
    Ok(ApiResponse::ok(folder_to_dto(folder)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/folders/{id}",
    params(("id" = Uuid, Path, description = "Folder ID")),
    responses((status = 204, description = "Folder deleted")),
    security(("bearer" = [])),
    tag = "Folder"
)]
async fn delete_folder(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    state.folder_service.delete_folder(&ctx, id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

pub fn folder_routes() -> Router<AppState> {
    Router::new()
        .route("/folders", post(create_folder).get(list_folders))
        .route(
            "/folders/curriculum",
            get(list_curriculum_folders).post(get_or_create_curriculum_folder),
        )
        .route(
            "/folders/student/{student_id}",
            get(get_or_create_student_folder),
        )
        .route(
            "/folders/{id}",
            get(get_folder).patch(update_folder).delete(delete_folder),
        )
        .route("/folders/{id}/contents", get(get_folder_contents))
}
