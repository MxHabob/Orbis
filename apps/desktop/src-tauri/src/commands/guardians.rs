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
    state.service.get_guardians_by_student(&student_id).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn get_students_by_guardian(
    state: State<'_, Arc<AppState>>,
    guardian_id: String,
) -> CmdResult<Vec<core_domain::entities::student_guardian::Model>> {
    state.service.get_students_by_guardian(&guardian_id).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn create_guardian_link(
    state: State<'_, Arc<AppState>>,
    request: CreateGuardianLinkRequest,
) -> CmdResult<core_domain::entities::student_guardian::Model> {
    state.service.create_guardian_link(request).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn update_guardian_link(
    state: State<'_, Arc<AppState>>,
    id: String,
    request: UpdateGuardianLinkRequest,
) -> CmdResult<core_domain::entities::student_guardian::Model> {
    state.service.update_guardian_link(&id, request).await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn delete_guardian_link(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<()> {
    state.service.delete_guardian_link(&id).await
        .map_err(|e| CommandError::new(e))
}
