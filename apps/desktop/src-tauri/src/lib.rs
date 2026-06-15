use serde_json;
use std::sync::{Arc, Mutex};
use tauri::{
    Emitter, Listener, Manager, PhysicalPosition, Position, WebviewUrl,
    WebviewWindowBuilder,
};
#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;
use tauri_plugin_deep_link::DeepLinkExt;
use tauri_plugin_updater;
use tauri_plugin_dialog;
use tauri_plugin_process;
use tauri::menu::{Menu, MenuItem};
use tauri::image::Image;
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use image;

pub mod state;
pub mod commands;
pub mod features;

use state::AppState;

// Global state for search window availability
type SearchWindowState = Arc<Mutex<bool>>;

#[tauri::command]
fn show_window(window: tauri::Window) -> Result<(), String> {
    // Always target the main window specifically, not the calling window
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
async fn check_for_updates(app: tauri::AppHandle) -> Result<(), String> {
    use tauri_plugin_dialog::{DialogExt, MessageDialogKind, MessageDialogButtons};
    use tauri_plugin_updater::UpdaterExt;
    
    #[cfg(desktop)]
    {
        // Try to check for updates
        if let Ok(updater) = app.updater() {
            match updater.check().await {
                Ok(Some(update)) => {
                    // Update available - show update dialog
                    let answer = app.dialog()
                        .message(format!("A new version {} is available. Would you like to update now?", update.version))
                        .title("Update Available")
                        .kind(MessageDialogKind::Info)
                        .buttons(MessageDialogButtons::OkCancel)
                        .blocking_show();
                    
                    if answer {
                        // User wants to update - provide required callbacks
                        let _ = update.download_and_install(
                            |_chunk_length, _content_length| {
                                // Progress callback - could show progress here
                            },
                            || {
                                // Download finished callback
                                println!("Update download finished");
                            }
                        ).await;
                    }
                }
                Ok(None) => {
                    // No update available - show current version info
                    let version = app.package_info().version.to_string();
                    app.dialog()
                        .message(format!("Orbis\nversion {}\n\nYou're up to date!", version))
                        .title("No Updates Available")
                        .kind(MessageDialogKind::Info)
                        .buttons(MessageDialogButtons::Ok)
                        .blocking_show();
                }
                Err(e) => {
                    // Error checking for updates
                    app.dialog()
                        .message(format!("Failed to check for updates: {}", e))
                        .title("Update Check Failed")
                        .kind(MessageDialogKind::Error)
                        .buttons(MessageDialogButtons::Ok)
                        .blocking_show();
                }
            }
        } else {
            // Updater not available
            app.dialog()
                .message("Update checking is not available in this build.")
                .title("Updates Not Available")
                .kind(MessageDialogKind::Warning)
                .buttons(MessageDialogButtons::Ok)
                .blocking_show();
        }
    }
    
    #[cfg(not(desktop))]
    {
        // Mobile platforms don't support auto-updates
        app.dialog()
            .message("Updates are managed through your app store.")
            .title("Check App Store")
            .kind(MessageDialogKind::Info)
            .buttons(MessageDialogButtons::Ok)
            .blocking_show();
    }
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            // Core window & update commands
            show_window,
            check_for_updates,

            // Auth commands
            commands::auth::register,
            commands::auth::login,
            commands::auth::validate_token,
            commands::auth::logout,

            // Users & organization commands
            commands::users::get_organization,
            commands::users::update_organization,
            commands::users::get_users,
            commands::users::get_user,
            commands::users::create_user,
            commands::users::update_user,
            commands::users::delete_user,
            commands::users::change_user_role,
            commands::users::change_user_password,

            // Academics commands
            commands::academics::get_cycles,
            commands::academics::get_cycle,
            commands::academics::create_cycle,
            commands::academics::update_cycle,
            commands::academics::activate_cycle,
            commands::academics::lock_cycle,
            commands::academics::get_classes,
            commands::academics::create_class,
            commands::academics::update_class,
            commands::academics::delete_class,

            // Enrollments commands
            commands::enrollments::get_enrollments,
            commands::enrollments::get_enrollment,
            commands::enrollments::create_enrollment,
            commands::enrollments::withdraw_enrollment,
            commands::enrollments::transfer_enrollment,
            commands::enrollments::promote_students,

            // Grades commands
            commands::grades::get_grades,
            commands::grades::submit_grade,
            commands::grades::submit_grades_bulk,
            commands::grades::update_grade,
            commands::grades::get_report_card,
            commands::grades::get_grade_analytics,

            // Attendance commands
            commands::attendance::get_attendance,
            commands::attendance::record_attendance,
            commands::attendance::record_bulk_attendance,
            commands::attendance::get_attendance_report,
            commands::attendance::get_attendance_analytics,

            // Finance commands
            commands::finance::get_ledger_accounts,
            commands::finance::create_ledger_account,
            commands::finance::get_treasuries,
            commands::finance::create_treasury,
            commands::finance::get_student_fees,
            commands::finance::assign_student_fee,
            commands::finance::get_payments,
            commands::finance::record_payment,

            // Misc commands
            commands::misc::get_behavior_records,
            commands::misc::submit_ai_behavior,
            commands::misc::update_behavior_record,
            commands::misc::delete_behavior_record,
            commands::misc::get_notifications,
            commands::misc::mark_notification_read,
            commands::misc::get_org_units,
            commands::misc::create_org_unit,
            commands::misc::update_org_unit,
            commands::misc::delete_org_unit,
            commands::misc::get_dashboard_stats,
            commands::misc::get_audit_logs,

            // Subjects
            commands::subjects::get_subjects,
            commands::subjects::get_subject,
            commands::subjects::create_subject,
            commands::subjects::update_subject,
            commands::subjects::delete_subject,

            // Assessments
            commands::assessments::get_assessments,
            commands::assessments::create_assessment,
            commands::assessments::update_assessment,
            commands::assessments::delete_assessment,

            // Documents
            commands::documents::get_documents,
            commands::documents::get_document,
            commands::documents::create_document,
            commands::documents::delete_document,

            // Certificates
            commands::certificates::get_certificates,
            commands::certificates::get_certificate,
            commands::certificates::create_certificate,
            commands::certificates::update_certificate,
            commands::certificates::delete_certificate,

            // Widgets
            commands::widgets::get_widget_preferences,
            commands::widgets::update_widget_preferences,
        ])
        .setup(move |app| {
            // Add updater plugin conditionally for desktop
            #[cfg(desktop)]
            app.handle().plugin(tauri_plugin_updater::Builder::new().build())?;

            // 1. Initialize local SQLite database path in local AppData folder
            let app_data_dir = app.path().app_data_dir().unwrap();
            let _ = std::fs::create_dir_all(&app_data_dir);
            let db_path = app_data_dir.join("orbis.db");
            let database_url = format!("sqlite://{}", db_path.to_string_lossy());

            // 2. Initialize managed AppState with direct Rust access (embedded core)
            let app_state = tauri::async_runtime::block_on(async {
                AppState::init(&database_url)
                    .await
                    .expect("Failed to initialize database and AppState")
            });
            app.manage(Arc::new(app_state));

            let app_handle = app.handle().clone();
            let initial_url = WebviewUrl::App("index.html".into());

            // Create shared search window state
            let search_state: SearchWindowState = Arc::new(Mutex::new(false));
            
            // Add search state to managed state so commands can access it
            app.manage(search_state.clone());

            // Clone app_handle before it gets moved into closures
            let app_handle_for_deep_links = app_handle.clone();
            let app_handle_for_navigation = app_handle.clone();

            // Initialize global shortcuts
            {
                use tauri_plugin_global_shortcut::{
                    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
                };

                let search_shortcut =
                    Shortcut::new(Some(Modifiers::SHIFT | Modifiers::ALT), Code::KeyK);

                if let Ok(_) = app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app_handle, shortcut, event| {
                            if shortcut == &search_shortcut
                                && event.state() == ShortcutState::Pressed
                            {
                                println!("🔍 Global shortcut triggered - checking search state");
                                if let Some(managed_search_state) = app_handle.try_state::<SearchWindowState>() {
                                    let _ = features::search::toggle_search_window(app_handle, &managed_search_state);
                                }
                            }
                        })
                        .build(),
                ) {
                    let _ = app.global_shortcut().register(search_shortcut);
                }
            }

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

            let win_builder = WebviewWindowBuilder::new(
                app,
                "main",
                initial_url,
            )
            .title("Orbis")
            .inner_size(1200.0, 750.0)
            .min_inner_size(900.0, 600.0)
            .user_agent("Mozilla/5.0 (compatible; Orbis Desktop App)")
            .decorations(false)
            .visible(false)
            .transparent(true)
            .center()
            .shadow(true);

            #[cfg(target_os = "macos")]
            let win_builder = win_builder
                .hidden_title(true)
                .title_bar_style(tauri::TitleBarStyle::Overlay);

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

            // Listen for search window state events from the frontend
            let search_state_for_events = search_state.clone();
            let app_handle_for_events = app_handle.clone();
            window.listen("search-window-enabled", move |event: tauri::Event| {
                if let Ok(enabled) = serde_json::from_str::<bool>(&event.payload()) {
                    *search_state_for_events.lock().unwrap() = enabled;
                    if !enabled {
                        if let Some(search_window) = app_handle_for_events.get_webview_window("search") {
                            let _ = search_window.close();
                        }
                    }
                }
            });

            // Listen for search window close requests from the frontend
            let app_handle_for_close = app_handle.clone();
            window.listen("search-window-close-requested", move |_event| {
                if let Some(search_window) = app_handle_for_close.get_webview_window("search") {
                    let _ = search_window.emit("search-window-open", false);
                    let _ = search_window.set_always_on_top(false);
                    let _ = search_window.hide();
                }
            });

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

            // Set the default app menu to restore the Orbis menu
            let app_menu = Menu::default(app.handle())?;
            app.set_menu(app_menu)?;

            // Setup simple system tray for search toggle only
            let tray_icon = {
                let icon_bytes = include_bytes!("../icons/tray-icon.png");
                let img = image::load_from_memory(icon_bytes).map_err(|e| format!("Failed to load tray icon: {}", e))?;
                let rgba = img.to_rgba8();
                let (width, height) = rgba.dimensions();
                Image::new_owned(rgba.into_raw(), width, height)
            };

            let check_updates_item = MenuItem::with_id(app, "check_updates", "Check for Updates...", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&check_updates_item])?;

            let _tray = TrayIconBuilder::new()
                .icon(tray_icon)
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    if event.id == "check_updates" {
                        let app_handle = app.clone();
                        tauri::async_runtime::spawn(async move {
                            let _ = check_for_updates(app_handle).await;
                        });
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
                            if let Some(managed_search_state) = app_handle.try_state::<SearchWindowState>() {
                                let _ = features::search::toggle_search_window(app_handle, &managed_search_state);
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
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
                if let Some(main_window) = app_handle.get_webview_window("main") {
                    let _ = main_window.hide();
                }
                if let Some(search_window) = app_handle.get_webview_window("search") {
                    let _ = search_window.hide();
                }
            }
            _ => {}
        });
}
