//! Tauri IPC commands — Folder Hierarchy, Student Folders, and Curriculum Knowledge Base.

use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use super::document_cmds::{DocumentDto, doc_dto, ok};
use super::make_ctx;
use super::org_cmds::ApiResponse;
use crate::state::AppState;
use orbis_app::services::{CreateFolderInput, UpdateFolderInput};

#[derive(Serialize)]
pub struct FolderDto {
    pub id: String,
    pub org_id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub path: String,
    pub folder_type: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub org_unit_id: Option<String>,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub is_system: bool,
    pub metadata: Option<serde_json::Value>,
    pub created_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn folder_dto(f: orbis_domain::entities::folder::Folder) -> FolderDto {
    FolderDto {
        id: f.id.to_string(),
        org_id: f.org_id.to_string(),
        parent_id: f.parent_id.map(|p| p.to_string()),
        name: f.name,
        path: f.path,
        folder_type: f.folder_type,
        entity_type: f.entity_type,
        entity_id: f.entity_id.map(|e| e.to_string()),
        org_unit_id: f.org_unit_id.map(|u| u.to_string()),
        academic_year_id: f.academic_year_id.map(|y| y.to_string()),
        term_id: f.term_id.map(|t| t.to_string()),
        is_system: f.is_system,
        metadata: f.metadata,
        created_by: f.created_by.map(|c| c.to_string()),
        created_at: f.created_at.to_rfc3339(),
        updated_at: f.updated_at.to_rfc3339(),
    }
}

#[derive(Serialize)]
pub struct FolderContentsDto {
    pub folder: FolderDto,
    pub subfolders: Vec<FolderDto>,
    pub documents: Vec<DocumentDto>,
}

#[derive(Deserialize)]
pub struct CreateFolderInputDto {
    pub name: String,
    pub parent_id: Option<String>,
    pub folder_type: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub org_unit_id: Option<String>,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct UpdateFolderInputDto {
    pub name: Option<String>,
    pub parent_id: Option<String>,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct CreateCurriculumFolderInputDto {
    pub name: String,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[tauri::command]
pub async fn folder_create(
    request: CreateFolderInputDto,
    state: State<'_, AppState>,
) -> Result<ApiResponse<FolderDto>, String> {
    let ctx = make_ctx(&state);
    let parent_id = request.parent_id.and_then(|p| Uuid::parse_str(&p).ok());
    let entity_id = request.entity_id.and_then(|e| Uuid::parse_str(&e).ok());
    let org_unit_id = request.org_unit_id.and_then(|u| Uuid::parse_str(&u).ok());
    let academic_year_id = request
        .academic_year_id
        .and_then(|y| Uuid::parse_str(&y).ok());
    let term_id = request.term_id.and_then(|t| Uuid::parse_str(&t).ok());

    let input = CreateFolderInput {
        name: request.name,
        parent_id,
        folder_type: request.folder_type,
        entity_type: request.entity_type,
        entity_id,
        org_unit_id,
        academic_year_id,
        term_id,
        metadata: request.metadata,
    };

    let folder = state
        .folder_service
        .create_folder(&ctx, input)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(folder_dto(folder)))
}

#[tauri::command]
pub async fn folder_list(
    parent_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<FolderDto>>, String> {
    let ctx = make_ctx(&state);
    let pid = parent_id.and_then(|p| Uuid::parse_str(&p).ok());

    let folders = state
        .folder_service
        .list_folders(&ctx, pid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(folders.into_iter().map(folder_dto).collect()))
}

#[tauri::command]
pub async fn folder_get(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<FolderDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid Folder ID")?;

    let folder = state
        .folder_service
        .get_folder(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(folder_dto(folder)))
}

#[tauri::command]
pub async fn folder_get_contents(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<FolderContentsDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid Folder ID")?;

    let contents = state
        .folder_service
        .get_folder_contents(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(FolderContentsDto {
        folder: folder_dto(contents.folder),
        subfolders: contents.subfolders.into_iter().map(folder_dto).collect(),
        documents: contents.documents.into_iter().map(doc_dto).collect(),
    }))
}

#[tauri::command]
pub async fn folder_get_or_create_student(
    student_id: String,
    student_name: Option<String>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<FolderDto>, String> {
    let ctx = make_ctx(&state);
    let sid = Uuid::parse_str(&student_id).map_err(|_| "Invalid Student ID")?;
    let sname = student_name.unwrap_or_else(|| student_id.clone());

    let folder = state
        .folder_service
        .get_or_create_student_folder(&ctx, sid, &sname)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(folder_dto(folder)))
}

#[tauri::command]
pub async fn folder_list_curriculum(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<FolderDto>>, String> {
    let ctx = make_ctx(&state);

    let folders = state
        .folder_service
        .list_curriculum_folders(&ctx)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(folders.into_iter().map(folder_dto).collect()))
}

#[tauri::command]
pub async fn folder_get_or_create_curriculum(
    request: CreateCurriculumFolderInputDto,
    state: State<'_, AppState>,
) -> Result<ApiResponse<FolderDto>, String> {
    let ctx = make_ctx(&state);
    let academic_year_id = request
        .academic_year_id
        .and_then(|y| Uuid::parse_str(&y).ok());
    let term_id = request.term_id.and_then(|t| Uuid::parse_str(&t).ok());

    let folder = state
        .folder_service
        .get_or_create_curriculum_folder(
            &ctx,
            &request.name,
            academic_year_id,
            term_id,
            request.metadata,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(folder_dto(folder)))
}

#[tauri::command]
pub async fn folder_update(
    id: String,
    request: UpdateFolderInputDto,
    state: State<'_, AppState>,
) -> Result<ApiResponse<FolderDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid Folder ID")?;
    let parent_id = request.parent_id.and_then(|p| Uuid::parse_str(&p).ok());
    let academic_year_id = request
        .academic_year_id
        .and_then(|y| Uuid::parse_str(&y).ok());
    let term_id = request.term_id.and_then(|t| Uuid::parse_str(&t).ok());

    let input = UpdateFolderInput {
        name: request.name,
        parent_id,
        academic_year_id,
        term_id,
        metadata: request.metadata,
    };

    let folder = state
        .folder_service
        .update_folder(&ctx, uid, input)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(folder_dto(folder)))
}

#[tauri::command]
pub async fn folder_delete(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid Folder ID")?;

    state
        .folder_service
        .delete_folder(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(()))
}
