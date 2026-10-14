use image;
#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;
use tauri::image::Image;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_deep_link::DeepLinkExt;
use tauri_plugin_dialog;
use tauri_plugin_process;
use tauri_plugin_updater;
pub mod commands;
pub mod connection;
pub mod events;
pub mod features;
pub mod state;
use connection::ConnectionConfig;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_fs::init())
        .on_page_load(|webview, payload| {
            if payload.event() == tauri::webview::PageLoadEvent::Finished
                && webview.label() == "main"
            {
                let _ = webview.show();
                let _ = webview.set_focus();
            }
        })
        .invoke_handler(crate::commands::generate_handlers())
        .setup(move |app| {
            // Add updater plugin conditionally for desktop
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_updater::Builder::new().build())?;
            // 1. Initialize local AppData directory
            let app_data_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("Failed to resolve app data dir: {e}"))?;
            std::fs::create_dir_all(&app_data_dir)
                .map_err(|e| format!("Failed to create app data dir {:?}: {e}", app_data_dir))?;

            // 2. Load connection configuration (works before database is connected)
            let conn_config = ConnectionConfig::load(&app_data_dir);
            let conn_state = commands::connection_cmds::ConnectionState {
                app_data_dir: app_data_dir.clone(),
                config: std::sync::Arc::new(std::sync::RwLock::new(conn_config.clone())),
            };
            app.manage(conn_state);

            // 3. Auto-connect if a valid local config exists (seamless for returning users)
            //    New users will see the connection/setup flow.
            let validation = conn_config.validate(&app_data_dir);
            if validation.is_valid
                && matches!(
                    conn_config.mode,
                    connection::ConnectionMode::Local | connection::ConnectionMode::Direct
                )
            {
                let database_url = conn_config.resolve_database_url(&app_data_dir);
                let init_result = tauri::async_runtime::block_on(async {
                    let database_url = database_url?;
                    let database_url = database_url
                        .ok_or_else(|| "Database URL is not configured".to_string())?;
                    AppState::init(
                        &app_data_dir,
                        &database_url,
                        conn_config.mode == connection::ConnectionMode::Local,
                    )
                    .await
                });
                match init_result {
                    Ok(app_state) => {
                        app.manage(app_state);
                    }
                    Err(e) => {
                        tracing::warn!("Auto-connect failed (user will see setup): {e}");
                        // Don't crash — the frontend will detect missing AppState
                        // and show the connection setup screen.
                    }
                }
            }
            let app_handle = app.handle().clone();
            let initial_url = WebviewUrl::App("index.html".into());
            // Clone app_handle before it gets moved into closures
            let app_handle_for_deep_links = app_handle.clone();
            let app_handle_for_navigation = app_handle.clone();
            // Register deep links at runtime for development
            #[cfg(debug_assertions)]
            {
                let _ = app_handle.deep_link().register_all();
            }
            // Handle deep link events
            app_handle.deep_link().on_open_url(move |event| {
                let url_strings: Vec<String> =
                    event.urls().iter().map(|url| url.to_string()).collect();
                features::deep_link::handle_deep_link(&app_handle_for_deep_links, url_strings);
            });
            // Dynamically calculate window size based on primary monitor resolution (flexible size)
            let (width, height) = if let Ok(Some(monitor)) = app.primary_monitor() {
                let size = monitor.size();
                let scale = monitor.scale_factor();
                let mut w = (size.width as f64 * 0.8) / scale;
                let mut h = (size.height as f64 * 0.8) / scale;
                if w < 900.0 { w = 900.0; }
                if h < 600.0 { h = 600.0; }
                (w, h)
            } else {
                (1200.0, 750.0)
            };
            let win_builder = WebviewWindowBuilder::new(
                app,
                "main",
                initial_url,
            )
            .title("Orbis")
            .inner_size(width, height)
            .min_inner_size(900.0, 600.0)
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 OrbisDesktop/1.0")
            .visible(false)
            .center()
            .shadow(true);
            #[cfg(debug_assertions)]
            let win_builder = win_builder.devtools(true);
            #[cfg(target_os = "macos")]
            let win_builder = win_builder
                .decorations(false)
                .transparent(false)
                .hidden_title(true)
                .title_bar_style(tauri::TitleBarStyle::Overlay);
            #[cfg(not(target_os = "macos"))]
            let win_builder = win_builder.decorations(false).transparent(false);
            let window = win_builder
                .disable_drag_drop_handler()
                .on_download(|_window, _event| {
                    println!("Download triggered!");
                    true
                })
                .on_navigation(move |url: &tauri::Url| {
                    if url.scheme() == "tauri" || url.host_str() == Some("tauri.localhost") {
                        return true;
                    }
                    // Allow Vite dev server during `tauri dev`
                    #[cfg(debug_assertions)]
                    if matches!(url.host_str(), Some("localhost") | Some("127.0.0.1")) {
                        return true;
                    }
                    let is_http_or_https = url.scheme() == "http" || url.scheme() == "https";
                    if is_http_or_https {
                        let url_string = url.as_str().to_string();
                        let app_handle_clone = app_handle_for_navigation.clone();
                        tauri::async_runtime::spawn(async move {
                            let _ = tauri_plugin_opener::OpenerExt::opener(&app_handle_clone)
                                .open_url(url_string, None::<String>);
                        });
                        return false;
                    }
                    true
                })
                .build()
                .unwrap();
            // Fallback timer to ensure main window shows on first launch
            let window_clone = window.clone();
            tauri::async_runtime::spawn(async move {
                std::thread::sleep(std::time::Duration::from_secs(2));
                if let Ok(is_visible) = window_clone.is_visible() {
                    if !is_visible {
                        let _ = window_clone.show();
                        let _ = window_clone.set_focus();
                    }
                }
            });
            // Set the default app menu to restore the Orbis menu (macOS only, prevents window menu bar on Windows/Linux)
            #[cfg(target_os = "macos")]
            {
                let app_menu = Menu::default(app.handle())?;
                app.set_menu(app_menu)?;
            }
            // Setup simple system tray for search toggle only
            let tray_icon = {
                let icon_bytes = include_bytes!("../icons/tray-icon.png");
                let img = image::load_from_memory(icon_bytes).map_err(|e| format!("Failed to load tray icon: {}", e))?;
                let rgba = img.to_rgba8();
                let (width, height) = rgba.dimensions();
                Image::new_owned(rgba.into_raw(), width, height)
            };
            let check_updates_item = MenuItem::with_id(app, "check_updates", "Check for Updates...", true, None::<&str>)?;
            let exit_item = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&check_updates_item, &exit_item])?;
            let _tray = TrayIconBuilder::new()
                .icon(tray_icon)
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    if event.id == "check_updates" {
                        let app_handle = app.clone();
                        tauri::async_runtime::spawn(async move {
                            let _ = commands::window_cmds::check_for_updates(app_handle).await;
                        });
                    } else if event.id == "exit" {
                        if let Some(state) = app.try_state::<AppState>() {
                            let sidecar = state.solver_sidecar.clone();
                            tauri::async_runtime::block_on(async move {
                                sidecar.stop().await;
                            });
                        }
                        app.exit(0);
                    }
                })
                .on_tray_icon_event(move |tray, event| {
                    match event {
                        TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        } => {
                            let app_handle = tray.app_handle();
                            if let Some(main_window) = app_handle.get_webview_window("main") {
                                if main_window.is_visible().unwrap_or(false) {
                                    let _ = main_window.hide();
                                } else {
                                    let _ = main_window.show();
                                    let _ = main_window.set_focus();
                                }
                            }
                        },
                        _ => {}
                    }
                })
                .build(app)?;
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri app")
        .run(|app_handle, event| match event {
            #[cfg(target_os = "macos")]
            tauri::RunEvent::Reopen { .. } => {
                if let Some(main_window) = app_handle.get_webview_window("main") {
                    let _ = main_window.show();
                    let _ = main_window.set_focus();
                }
            }
            tauri::RunEvent::ExitRequested { api, code, .. } => {
                if code.is_none() {
                    api.prevent_exit();
                    if let Some(main_window) = app_handle.get_webview_window("main") {
                        let _ = main_window.hide();
                    }
                }
            }
            tauri::RunEvent::Exit => {
                if let Some(state) = app_handle.try_state::<AppState>() {
                    let sidecar = state.solver_sidecar.clone();
                    tauri::async_runtime::block_on(async move {
                        sidecar.stop().await;
                    });
                }
            }
            _ => {}
        });
}
