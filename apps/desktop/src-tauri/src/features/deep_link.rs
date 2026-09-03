use tauri::{AppHandle, Emitter, Manager};

/// Handles incoming orbis:// deep link URLs.
/// Extracts the path and emits a navigation event to the main window.
pub fn handle_deep_link(app: &AppHandle, urls: Vec<String>) {
    for url in &urls {
        if let Some(path) = url.strip_prefix("orbis://") {
            let clean = path.trim_start_matches('/');
            if let Some(window) = app.get_webview_window("main") {
                if window.emit("deep-link-navigate", clean).is_ok() {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        }
    }
}
