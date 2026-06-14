use tauri::State;
use std::sync::Arc;

use core_domain::{
    entities::{academic_cycle, class},
    params::{CycleParams, ClassParams},
    service::academic_cycle::{CreateCycleRequest, UpdateCycleRequest},
    service::class::UpdateClassRequest,
};

use crate::state::AppState;
use super::error::{CmdResult, CommandError};

// ── Academic Cycles ────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_cycles(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: CycleParams,
) -> CmdResult<Vec<academic_cycle::Model>> {
    state
        .service
        .get_cycles(&org_id, params)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_cycle(
    state: State<'_, Arc<AppState>>,
    cycle_id: String,
) -> CmdResult<academic_cycle::Model> {
    state
        .service
        .get_cycle(&cycle_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn create_cycle(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateCycleRequest,
) -> CmdResult<academic_cycle::Model> {
    state
        .service
        .create_cycle(&org_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn update_cycle(
    state: State<'_, Arc<AppState>>,
    cycle_id: String,
    request: UpdateCycleRequest,
) -> CmdResult<academic_cycle::Model> {
    state
        .service
        .update_cycle(&cycle_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn activate_cycle(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
) -> CmdResult<academic_cycle::Model> {
    state
        .service
        .activate_cycle(&org_id, &cycle_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn lock_cycle(
    state: State<'_, Arc<AppState>>,
    cycle_id: String,
) -> CmdResult<academic_cycle::Model> {
    state
        .service
        .lock_cycle(&cycle_id)
        .await
        .map_err(CommandError::from)
}

// ── Classes ────────────────────────────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateClassRequest {
    pub name: String,
    pub name_ar: String,
    pub level: String,
    pub division: String,
}

#[tauri::command]
pub async fn get_classes(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: ClassParams,
) -> CmdResult<Vec<class::Model>> {
    state
        .service
        .get_classes(&org_id, params)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn create_class(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateClassRequest,
) -> CmdResult<class::Model> {
    state
        .service
        .create_class(
            &request.name,
            &request.name_ar,
            &request.level,
            &request.division,
            &org_id,
        )
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn update_class(
    state: State<'_, Arc<AppState>>,
    class_id: String,
    request: UpdateClassRequest,
) -> CmdResult<class::Model> {
    state
        .service
        .update_class(&class_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn delete_class(
    state: State<'_, Arc<AppState>>,
    class_id: String,
) -> CmdResult<()> {
    state
        .service
        .delete_class(&class_id)
        .await
        .map_err(CommandError::from)
}
