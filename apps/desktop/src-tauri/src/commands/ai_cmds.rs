//! Tauri IPC commands — AI Platform (Stateless ReAct Chat, Tool Discovery & File-based Conversation/Context Persistence).

use super::make_ctx;
use super::org_cmds::ApiResponse;
use crate::state::AppState;
use orbis_domain::entities::ai::{
    AiChatRequest, AiChatResponse, AiCustomReportRequest, AiCustomReportResponse,
    AiStoredSession, AiToolDefinition,
};
use std::path::{Path, PathBuf};
use tauri::State;

fn ok<T: serde::Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse { data }
}

fn get_conversations_dir(app_data_dir: &Path, org_id: &str) -> PathBuf {
    let safe_org_id = if org_id.trim().is_empty() {
        "default"
    } else {
        org_id.trim()
    };
    app_data_dir.join("ai_conversations").join(safe_org_id)
}

fn get_context_dir(app_data_dir: &Path, org_id: &str) -> PathBuf {
    let safe_org_id = if org_id.trim().is_empty() {
        "default"
    } else {
        org_id.trim()
    };
    app_data_dir.join("ai_context").join(safe_org_id)
}

#[tauri::command]
pub async fn ai_chat(
    state: State<'_, AppState>,
    request: AiChatRequest,
) -> Result<ApiResponse<AiChatResponse>, String> {
    let ctx = make_ctx(&state);
    let response = state
        .ai_service
        .chat(&ctx, request)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(response))
}

#[tauri::command]
pub async fn ai_list_tools(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<AiToolDefinition>>, String> {
    let ctx = make_ctx(&state);
    let tools = state
        .ai_service
        .get_authorized_tools(&ctx)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(tools))
}

#[tauri::command]
pub async fn ai_create_custom_report(
    state: State<'_, AppState>,
    request: AiCustomReportRequest,
) -> Result<ApiResponse<AiCustomReportResponse>, String> {
    let ctx = make_ctx(&state);
    let response = state
        .ai_service
        .create_custom_report(&ctx, request)
        .await
        .map_err(|error| error.to_string())?;
    Ok(ok(response))
}

/// Saves an AI conversation session as a file on disk: `<app_data>/ai_conversations/<org_id>/<session_id>.json`
#[tauri::command]
pub async fn ai_save_session(
    state: State<'_, AppState>,
    session: AiStoredSession,
) -> Result<ApiResponse<bool>, String> {
    let dir = get_conversations_dir(&state.app_data_dir, &session.org_id);
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| format!("Failed to create AI conversations dir: {e}"))?;

    let file_path = dir.join(format!("{}.json", session.id));
    let content = serde_json::to_string_pretty(&session)
        .map_err(|e| format!("Failed to serialize AI session: {e}"))?;

    tokio::fs::write(&file_path, content)
        .await
        .map_err(|e| format!("Failed to write AI session file: {e}"))?;

    Ok(ok(true))
}

/// Loads all AI conversation session files for an organization from disk.
#[tauri::command]
pub async fn ai_load_sessions(
    state: State<'_, AppState>,
    org_id: String,
) -> Result<ApiResponse<Vec<AiStoredSession>>, String> {
    let dir = get_conversations_dir(&state.app_data_dir, &org_id);
    if !dir.exists() {
        return Ok(ok(Vec::new()));
    }

    let mut sessions = Vec::new();
    let mut entries = tokio::fs::read_dir(&dir)
        .await
        .map_err(|e| format!("Failed to read AI conversations dir: {e}"))?;

    while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            if let Ok(content) = tokio::fs::read_to_string(&path).await {
                if let Ok(session) = serde_json::from_str::<AiStoredSession>(&content) {
                    sessions.push(session);
                }
            }
        }
    }

    // Sort by updated_at descending
    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    Ok(ok(sessions))
}

/// Deletes an AI conversation session file from disk.
#[tauri::command]
pub async fn ai_delete_session(
    state: State<'_, AppState>,
    org_id: String,
    session_id: String,
) -> Result<ApiResponse<bool>, String> {
    let dir = get_conversations_dir(&state.app_data_dir, &org_id);
    let file_path = dir.join(format!("{}.json", session_id));
    if file_path.exists() {
        tokio::fs::remove_file(&file_path)
            .await
            .map_err(|e| format!("Failed to delete AI session file: {e}"))?;
    }
    Ok(ok(true))
}

/// Saves the current AI page/screen context as a file: `<app_data>/ai_context/<org_id>/context.json`
#[tauri::command]
pub async fn ai_save_context(
    state: State<'_, AppState>,
    org_id: String,
    context: serde_json::Value,
) -> Result<ApiResponse<bool>, String> {
    let dir = get_context_dir(&state.app_data_dir, &org_id);
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| format!("Failed to create AI context dir: {e}"))?;

    let file_path = dir.join("context.json");
    let content = serde_json::to_string_pretty(&context)
        .map_err(|e| format!("Failed to serialize AI context: {e}"))?;

    tokio::fs::write(&file_path, content)
        .await
        .map_err(|e| format!("Failed to write AI context file: {e}"))?;

    Ok(ok(true))
}

/// Loads the persisted AI context file for an organization.
#[tauri::command]
pub async fn ai_load_context(
    state: State<'_, AppState>,
    org_id: String,
) -> Result<ApiResponse<Option<serde_json::Value>>, String> {
    let dir = get_context_dir(&state.app_data_dir, &org_id);
    let file_path = dir.join("context.json");
    if !file_path.exists() {
        return Ok(ok(None));
    }

    let content = tokio::fs::read_to_string(&file_path)
        .await
        .map_err(|e| format!("Failed to read AI context file: {e}"))?;

    let val: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse AI context file: {e}"))?;

    Ok(ok(Some(val)))
}
