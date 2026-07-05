use tauri::State;
use std::sync::Arc;

use core_domain::service::academic_context::{AcademicContext, AcademicContextService};

use crate::state::AppState;
use super::error::{CmdResult, CommandError};

/// Get the full academic context: current year, active cycle, all cycles, phase.
///
/// This is the primary entry point for the frontend's `AcademicContextStore`.
/// It combines registry (year) data and year-DB (cycle) data into a single response.
#[tauri::command]
pub async fn get_academic_context(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<AcademicContext> {
    let year_id = state.provider.current_year_id().await;
    let is_archived = state.provider.is_current_archived().await;
    let registry_db = state.provider.registry();
    let service = state.service().await;

    AcademicContextService::get_context(
        registry_db,
        &service.repo.db,
        &org_id,
        &year_id,
        is_archived,
    )
    .await
    .map_err(CommandError::from)
}

/// Resolve only the active cycle (lightweight — used for auto-selection).
#[tauri::command]
pub async fn resolve_active_cycle(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Option<serde_json::Value>> {
    let service = state.service().await;

    let cycle = AcademicContextService::resolve_active_cycle(
        &service.repo.db,
        &org_id,
    )
    .await
    .map_err(CommandError::from)?;

    match cycle {
        Some(c) => {
            let val = serde_json::to_value(&c).map_err(|e| CommandError::from(e.to_string()))?;
            Ok(Some(val))
        }
        None => Ok(None),
    }
}

/// Check if the current year is writable (not archived).
/// Returns Ok(true) if writable, Ok(false) if read-only.
#[tauri::command]
pub async fn is_year_writable(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<bool> {
    Ok(!state.provider.is_current_archived().await)
}
