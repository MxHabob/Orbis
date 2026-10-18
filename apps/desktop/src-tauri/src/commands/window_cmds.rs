//! Window and system management Tauri commands.

use std::sync::Mutex;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

static STANDALONE_WINDOW_LOCK: Mutex<()> = Mutex::new(());

pub(crate) async fn do_open_agent_window<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
) -> Result<(), String> {
    if let Some(existing) = app.get_webview_window("orbis-agent") {
        let _ = existing.unminimize();
        existing.show().map_err(|e| e.to_string())?;
        existing.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }
    let main_window = app.get_webview_window("main");
    let mut builder =
        WebviewWindowBuilder::new(app, "orbis-agent", WebviewUrl::App("agent".into()))
            .title("Orbis AI Agent")
            .inner_size(460.0, 720.0)
            .min_inner_size(360.0, 500.0)
            .resizable(true)
            .shadow(true)
            .decorations(true)
            .visible(true);
    if let Some(ref main) = main_window {
        builder = builder.parent(main).map_err(|e| e.to_string())?;
    }
    let win = builder.build().map_err(|e| e.to_string())?;
    let _ = win.show();
    let _ = win.set_focus();
    Ok(())
}

pub(crate) async fn do_open_changelog_window<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
) -> Result<(), String> {
    if let Some(existing) = app.get_webview_window("orbis-changelog") {
        let _ = existing.unminimize();
        existing.show().map_err(|e| e.to_string())?;
        existing.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }
    let main_window = app.get_webview_window("main");
    let app_name = app.package_info().name.clone();
    let mut builder =
        WebviewWindowBuilder::new(app, "orbis-changelog", WebviewUrl::App("changelog".into()))
            .title(format!("{app_name} — What's New & Changelog"))
            .inner_size(860.0, 680.0)
            .min_inner_size(580.0, 460.0)
            .resizable(true)
            .shadow(true)
            .decorations(true)
            .visible(true);
    if let Some(ref main) = main_window {
        builder = builder.parent(main).map_err(|e| e.to_string())?;
    }
    let win = builder.build().map_err(|e| e.to_string())?;
    let _ = win.show();
    let _ = win.set_focus();
    Ok(())
}

fn standalone_app_path(path: &str) -> Result<String, String> {
    if path.len() > 4096
        || !path.starts_with('/')
        || path.starts_with("//")
        || path.contains('\\')
        || path.chars().any(char::is_control)
    {
        return Err("The requested page path is invalid.".to_string());
    }

    let route = path.split(['?', '#']).next().unwrap_or_default();
    let normalized_route = route.to_ascii_lowercase();
    if route
        .split('/')
        .any(|segment| matches!(segment, "." | ".."))
        || normalized_route.contains("%2e")
        || normalized_route.contains("%2f")
        || normalized_route.contains("%5c")
    {
        return Err("The requested page path is invalid.".to_string());
    }

    let base = tauri::Url::parse("https://tauri.localhost/")
        .map_err(|error| format!("Failed to validate the requested page path: {error}"))?;
    let resolved = base
        .join(path)
        .map_err(|error| format!("Failed to validate the requested page path: {error}"))?;
    if resolved.scheme() != "https" || resolved.host_str() != Some("tauri.localhost") {
        return Err("Only pages inside Orbis can be opened in a separate window.".to_string());
    }

    let app_path = path.trim_start_matches('/');
    Ok(if app_path.is_empty() {
        "index.html".to_string()
    } else {
        app_path.to_string()
    })
}

fn standalone_window_label(path: &str) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in path.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("orbis-page-{hash:016x}")
}

#[tauri::command]
pub async fn open_standalone_window<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    path: String,
) -> Result<(), String> {
    let app_path = standalone_app_path(&path)?;
    let label = standalone_window_label(&path);
    let _guard = STANDALONE_WINDOW_LOCK
        .lock()
        .map_err(|error| format!("Failed to acquire the standalone window lock: {error}"))?;

    if let Some(existing) = app.get_webview_window(&label) {
        existing.unminimize().map_err(|error| error.to_string())?;
        existing.show().map_err(|error| error.to_string())?;
        existing.set_focus().map_err(|error| error.to_string())?;
        return Ok(());
    }

    WebviewWindowBuilder::new(&app, label, WebviewUrl::App(app_path.into()))
        .title("Orbis")
        .inner_size(1200.0, 800.0)
        .min_inner_size(760.0, 520.0)
        .resizable(true)
        .decorations(false)
        .transparent(false)
        .shadow(true)
        .center()
        .visible(false)
        .on_page_load(|webview, payload| {
            if payload.event() == tauri::webview::PageLoadEvent::Finished {
                if let Err(error) = webview.show() {
                    tracing::warn!("Failed to show standalone page window: {error}");
                }
                if let Err(error) = webview.set_focus() {
                    tracing::warn!("Failed to focus standalone page window: {error}");
                }
            }
        })
        .build()
        .map_err(|error| format!("Failed to open the requested page: {error}"))?;
    Ok(())
}

pub(crate) async fn do_open_connection_window<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
) -> Result<(), String> {
    if let Some(existing) = app.get_webview_window("orbis-connection") {
        existing.show().map_err(|e| e.to_string())?;
        existing.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }
    let main_window = app.get_webview_window("main");
    let mut builder = WebviewWindowBuilder::new(
        app,
        "orbis-connection",
        WebviewUrl::App("connection".into()),
    )
    .title("Orbis Connection")
    .inner_size(780.0, 560.0)
    .resizable(true)
    .shadow(true)
    .decorations(true)
    .visible(false)
    .on_page_load(|webview, payload| {
        if payload.event() == tauri::webview::PageLoadEvent::Finished {
            let _ = webview.show();
            let _ = webview.set_focus();
        }
    });
    if let Some(ref main) = main_window {
        builder = builder.parent(main).map_err(|e| e.to_string())?;
    }
    builder.build().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn show_window<R: tauri::Runtime>(window: tauri::WebviewWindow<R>) -> Result<(), String> {
    let app_handle = window.app_handle();
    let main_window = match app_handle.get_webview_window("main") {
        Some(window) => window,
        None => {
            return Err("Main window not found".to_string());
        }
    };
    main_window
        .show()
        .map_err(|e| format!("Failed to show window: {}", e))?;
    main_window
        .set_focus()
        .map_err(|e| format!("Failed to set focus: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn open_agent_window<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    do_open_agent_window(&app).await
}

#[tauri::command]
pub async fn open_connection_window<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    do_open_connection_window(&app).await
}

#[tauri::command]
pub async fn open_changelog_window<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<(), String> {
    do_open_changelog_window(&app).await
}

#[tauri::command]
pub async fn check_for_updates<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
    use tauri_plugin_updater::UpdaterExt;
    #[cfg(desktop)]
    {
        let updater = match app.updater() {
            Ok(updater) => updater,
            Err(error) => {
                app.dialog()
                    .message(format!("Failed to initialize updater: {error}"))
                    .title("Update Check Failed")
                    .kind(MessageDialogKind::Error)
                    .buttons(MessageDialogButtons::Ok)
                    .blocking_show();
                return Err(error.to_string());
            }
        };

        match updater.check().await {
            Ok(Some(update)) => {
                let answer = app
                    .dialog()
                    .message(format!(
                        "A new version {} is available. Would you like to update now?",
                        update.version
                    ))
                    .title("Update Available")
                    .kind(MessageDialogKind::Info)
                    .buttons(MessageDialogButtons::OkCancel)
                    .blocking_show();
                if answer {
                    match update
                        .download_and_install(
                            |_chunk_length, _content_length| {},
                            || tracing::info!("Update download finished"),
                        )
                        .await
                    {
                        Ok(()) => app.restart(),
                        Err(error) => {
                            app.dialog()
                                .message(format!("Failed to install update: {error}"))
                                .title("Update Installation Failed")
                                .kind(MessageDialogKind::Error)
                                .buttons(MessageDialogButtons::Ok)
                                .blocking_show();
                            return Err(error.to_string());
                        }
                    }
                }
            }
            Ok(None) => {
                let version = app.package_info().version.to_string();
                app.dialog()
                    .message(format!("Orbis\nversion {}\n\nYou're up to date!", version))
                    .title("No Updates Available")
                    .kind(MessageDialogKind::Info)
                    .buttons(MessageDialogButtons::Ok)
                    .blocking_show();
            }
            Err(error) => {
                app.dialog()
                    .message(format!("Failed to check for updates: {error}"))
                    .title("Update Check Failed")
                    .kind(MessageDialogKind::Error)
                    .buttons(MessageDialogButtons::Ok)
                    .blocking_show();
                return Err(error.to_string());
            }
        }
    }
    #[cfg(not(desktop))]
    {
        app.dialog()
            .message("Updates are managed through your app store.")
            .title("Check App Store")
            .kind(MessageDialogKind::Info)
            .buttons(MessageDialogButtons::Ok)
            .blocking_show();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{standalone_app_path, standalone_window_label};

    #[test]
    fn standalone_path_accepts_internal_routes_with_query_and_fragment() {
        assert_eq!(
            standalone_app_path("/org-1/students/42?tab=grades#history").unwrap(),
            "org-1/students/42?tab=grades#history"
        );
    }

    #[test]
    fn standalone_path_maps_root_to_app_entrypoint() {
        assert_eq!(standalone_app_path("/").unwrap(), "index.html");
    }

    #[test]
    fn standalone_path_rejects_external_and_traversal_paths() {
        for path in [
            "//example.com/page",
            "/../secrets",
            "/%2e%2e/secrets",
            "/\\example.com/page",
        ] {
            assert!(standalone_app_path(path).is_err(), "accepted {path}");
        }
    }

    #[test]
    fn standalone_window_labels_are_stable_and_route_specific() {
        assert_eq!(
            standalone_window_label("/org-1/students"),
            standalone_window_label("/org-1/students")
        );
        assert_ne!(
            standalone_window_label("/org-1/students"),
            standalone_window_label("/org-1/settings")
        );
    }
}
