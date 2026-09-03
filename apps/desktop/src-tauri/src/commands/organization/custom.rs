use std::sync::Arc;

use tauri::State;

use crate::commands::error::{CmdResult, CommandError};
use crate::state::AppState;

/// Select or clear the active organizational unit for the current session.
#[tauri::command]
pub async fn organization_select_org_unit(
    state: State<'_, Arc<AppState>>,
    org_unit_id: Option<String>,
) -> CmdResult<serde_json::Value> {
    state
        .set_active_org_unit(org_unit_id.clone())
        .await
        .map_err(CommandError::from)?;

    Ok(serde_json::json!({
        "org_unit_id": org_unit_id,
    }))
}
