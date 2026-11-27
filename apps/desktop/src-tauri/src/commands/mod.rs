//! Tauri command registration — native desktop IPC handlers + in-memory API router bridge.

pub mod api_distribution_cmds;
pub mod camera_stream_cmds;
pub mod connection_cmds;
pub mod database_cmds;
pub mod solver_cmds;
pub mod system_cmds;
pub mod window_cmds;

use tauri::{Manager, generate_handler};

pub fn generate_handlers() -> impl Fn(tauri::ipc::Invoke) -> bool + Send + Sync + 'static {
    let handler = build_handler();

    move |invoke: tauri::ipc::Invoke| -> bool {
        let command = invoke.message.command().to_string();
        if invoke
            .message
            .webview_ref()
            .try_state::<crate::state::AppState>()
            .is_none()
            && !is_allowed_before_backend_initialization(&command)
        {
            invoke
                .resolver
                .reject("The application backend is not initialized.".to_string());
            return true;
        }

        handler(invoke)
    }
}

#[derive(serde::Serialize)]
struct IpcResponseMeta {
    status: u16,
    headers: std::collections::HashMap<String, String>,
}

/// Executes an HTTP request against the in-memory `orbis_server` Axum router.
///
/// Returns a binary frame: `[4-byte big-endian meta_len][UTF-8 JSON meta][raw body bytes]`
/// so JSON, PDFs, images, and exports transfer without UTF-8 corruption or double-encoding.
#[tauri::command]
pub async fn ipc_api_request(
    method: String,
    path: String,
    headers: std::collections::HashMap<String, String>,
    body: Option<Vec<u8>>,
    state: tauri::State<'_, crate::state::AppState>,
) -> Result<tauri::ipc::Response, String> {
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    let parsed_method = axum::http::Method::from_bytes(method.as_bytes())
        .map_err(|e| format!("Invalid HTTP method: {e}"))?;
    let mut builder = axum::http::Request::builder()
        .method(parsed_method)
        .uri(&path);
    for (k, v) in headers {
        builder = builder.header(k, v);
    }
    let req_body = match body {
        Some(bytes) if !bytes.is_empty() => axum::body::Body::from(bytes),
        _ => axum::body::Body::empty(),
    };
    let request = builder
        .body(req_body)
        .map_err(|e| format!("Failed to build IPC request: {e}"))?;

    let response = state
        .server_router
        .clone()
        .oneshot(request)
        .await
        .map_err(|e| format!("In-memory router error: {e}"))?;

    let status = response.status().as_u16();
    let mut resp_headers = std::collections::HashMap::new();
    for (name, val) in response.headers() {
        if let Ok(v) = val.to_str() {
            resp_headers.insert(name.as_str().to_string(), v.to_string());
        }
    }
    let collected = response
        .into_body()
        .collect()
        .await
        .map_err(|e| format!("Failed to read IPC response body: {e}"))?;
    let body_bytes = collected.to_bytes();

    let meta_bytes = serde_json::to_vec(&IpcResponseMeta {
        status,
        headers: resp_headers,
    })
    .map_err(|e| format!("Failed to serialize IPC response metadata: {e}"))?;
    let meta_len = u32::try_from(meta_bytes.len())
        .map_err(|_| "IPC response metadata exceeds 4GB".to_string())?;

    let mut frame = Vec::with_capacity(4 + meta_bytes.len() + body_bytes.len());
    frame.extend_from_slice(&meta_len.to_be_bytes());
    frame.extend_from_slice(&meta_bytes);
    frame.extend_from_slice(&body_bytes);

    Ok(tauri::ipc::Response::new(frame))
}

fn build_handler() -> impl Fn(tauri::ipc::Invoke) -> bool + Send + Sync + 'static {
    generate_handler![
        ipc_api_request,
        api_distribution_cmds::api_distribution_verify_manifest,
        camera_stream_cmds::camera_stream_start,
        camera_stream_cmds::camera_stream_stop,
        // ── Solver Sidecar ──────────────────────────────────────
        solver_cmds::solver_get_sidecar_status,
        solver_cmds::solver_get_setup_status,
        solver_cmds::solver_start_setup,
        solver_cmds::solver_stop_sidecar,
        solver_cmds::solver_submit_timetable,
        solver_cmds::solver_get_timetable_status,
        solver_cmds::solver_submit_exam,
        solver_cmds::solver_get_exam_status,
        // ── Window & System ─────────────────────────────────────
        window_cmds::show_window,
        window_cmds::open_agent_window,
        window_cmds::open_connection_window,
        window_cmds::open_changelog_window,
        window_cmds::open_standalone_window,
        window_cmds::check_for_updates,
        // ── Connection ──────────────────────────────────────────
        connection_cmds::connection_get_config,
        connection_cmds::connection_set_config,
        connection_cmds::connection_reset,
        connection_cmds::connection_validate,
        connection_cmds::connection_check_remote_health,
        connection_cmds::connection_connect,
        // ── Database Management (Local SQLite) ──────────────────
        database_cmds::database_get_status,
        database_cmds::database_create_local,
        database_cmds::database_open_existing,
        database_cmds::database_backup,
        database_cmds::database_validate_backup_file,
        database_cmds::database_restore,
        database_cmds::database_run_migrations,
        database_cmds::database_check_integrity,
        database_cmds::database_optimize,
        database_cmds::database_get_settings,
        database_cmds::database_update_settings,
        database_cmds::database_pick_file,
        database_cmds::database_pick_folder,
        database_cmds::database_pick_save_path,
        // ── System & Updater ────────────────────────────────────
        system_cmds::system_health,
        system_cmds::system_has_organization,
        system_cmds::system_get_readiness,
        system_cmds::system_get_entitlement,
        system_cmds::system_get_app_info,
        system_cmds::system_get_changelog,
        system_cmds::system_get_release_notes_state,
        system_cmds::system_acknowledge_release_notes,
        system_cmds::system_check_for_updates,
        system_cmds::system_download_update,
        system_cmds::system_install_update,
    ]
}

fn is_allowed_before_backend_initialization(command: &str) -> bool {
    command.starts_with("connection_")
        || command.starts_with("database_")
        || command.starts_with("window_")
        || matches!(
            command,
            "system_check_for_updates" | "system_download_update" | "system_install_update"
        )
        || command == "api_distribution_verify_manifest"
}
