//! Window and system management Tauri commands.

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

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
    let mut builder =
        WebviewWindowBuilder::new(app, "orbis-changelog", WebviewUrl::App("changelog".into()))
            .title("Orbis — What's New & Changelog")
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
