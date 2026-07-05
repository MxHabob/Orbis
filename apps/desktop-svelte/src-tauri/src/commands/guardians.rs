use tauri::State;
use std::sync::Arc;
use core_domain::service::guardian::{CreateGuardianLinkRequest, UpdateGuardianLinkRequest};
use crate::state::AppState;
use super::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn get_guardians_by_student(
    state: State<'_, Arc<AppState>>,
    student_id: String,
) -> CmdResult<Vec<core_domain::entities::student_guardian::Model>> {
    state.service().await.get_guardians_by_student(&student_id).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn get_students_by_guardian(
    state: State<'_, Arc<AppState>>,
    guardian_id: String,
) -> CmdResult<Vec<core_domain::entities::student_guardian::Model>> {
    state.service().await.get_students_by_guardian(&guardian_id).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn create_guardian_link(
    state: State<'_, Arc<AppState>>,
    request: CreateGuardianLinkRequest,
) -> CmdResult<core_domain::entities::student_guardian::Model> {
    state.service().await.create_guardian_link(request).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn update_guardian_link(
    state: State<'_, Arc<AppState>>,
    id: String,
    request: UpdateGuardianLinkRequest,
) -> CmdResult<core_domain::entities::student_guardian::Model> {
    state.service().await.update_guardian_link(&id, request).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn delete_guardian_link(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<()> {
    state.service().await.delete_guardian_link(&id).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn search_guardians(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    query: String,
) -> CmdResult<Vec<core_domain::entities::user::Model>> {
    state.service().await.search_guardians(&org_id, &query).await
        .map_err(|e| CommandError::new(e))
}
