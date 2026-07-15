use std::sync::Arc;
use tauri::{
    Manager, WebviewUrl,
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
pub mod events;
use state::AppState;
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
async fn open_agent_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(existing) = app.get_webview_window("orbis-agent") {
        existing.show().map_err(|e| e.to_string())?;
        existing.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }
    let main_window = app.get_webview_window("main");
    let mut builder = WebviewWindowBuilder::new(
        &app,
        "orbis-agent",
        WebviewUrl::App("agent".into()),
    )
    .title("Orbis Agent")
    .inner_size(420.0, 650.0)
    .resizable(true)
    .shadow(true)
    .decorations(true);
    if let Some(ref main) = main_window {
        builder = builder.parent(main).map_err(|e| e.to_string())?;
    }
    builder.build().map_err(|e| e.to_string())?;
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
        .on_page_load(|webview, payload| {
            if payload.event() == tauri::webview::PageLoadEvent::Finished
                && webview.label() == "main"
            {
                let _ = webview.show();
                let _ = webview.set_focus();
            }
        })
        .invoke_handler(tauri::generate_handler![
            // Core window & update commands
            show_window,
            open_agent_window,
            check_for_updates,
            // Document
            commands::document::thin::upload_document,
            commands::document::thin::get_documents,
            commands::document::thin::storage_delete_document,
            // IAM commands
            commands::iam::thin::login,
            commands::iam::thin::logout_session,
            commands::iam::thin::restore_session,
            commands::iam::thin::list_user_sessions,
            commands::iam::thin::list_users,
            commands::iam::thin::list_users_paginated,
            commands::iam::thin::get_user,
            commands::iam::thin::create_user,
            commands::iam::thin::update_user,
            commands::iam::thin::delete_user,
            commands::iam::thin::change_password,
            commands::iam::thin::change_role,
            commands::iam::thin::list_user_roles,
            commands::iam::thin::save_user_roles,
            commands::iam::thin::assign_role,
            commands::iam::thin::revoke_role,
            commands::iam::thin::list_audit_logs,
            commands::iam::thin::search_users,
            commands::iam::thin::get_role_permissions,
            commands::iam::thin::save_role_permissions,
            // Academics commands
            commands::academic::thin::list_classes,
            commands::academic::thin::get_class,
            commands::academic::thin::create_class,
            commands::academic::thin::update_class,
            commands::academic::thin::delete_class,
            commands::academic::thin::list_subjects,
            commands::academic::thin::get_subject,
            commands::academic::thin::create_subject,
            commands::academic::thin::update_subject,
            commands::academic::thin::delete_subject,
            commands::organization::thin::list_org_units,
            commands::organization::thin::create_org_unit,
            commands::organization::thin::update_org_unit,
            commands::organization::thin::delete_org_unit,
            commands::academic::thin::list_enrollments,
            commands::academic::thin::list_enrollments_paginated,
            commands::academic::thin::get_enrollment,
            commands::academic::thin::create_enrollment,
            commands::academic::thin::withdraw_enrollment,
            commands::academic::thin::transfer_enrollment,
            commands::academic::thin::check_duplicate_enrollment,
            commands::academic::thin::list_class_subjects,
            commands::academic::thin::create_class_subject,
            commands::academic::thin::update_class_subject,
            commands::academic::thin::delete_class_subject,
            commands::academic::thin::list_guardians_by_student,
            commands::academic::thin::search_guardians,
            commands::academic::thin::create_guardian_link,
            commands::academic::thin::update_guardian_link,
            commands::academic::thin::delete_guardian_link,
            commands::academic::thin::list_documents,
            commands::academic::thin::create_document,
            commands::academic::thin::delete_document,
            commands::academic::thin::list_notifications,
            commands::academic::thin::mark_notification_read,
            commands::academic::thin::assign_teacher,
            commands::academic::thin::list_teacher_subjects,
            commands::academic::thin::list_periods,
            commands::academic::thin::get_period,
            commands::academic::thin::create_period,
            commands::academic::thin::update_period,
            commands::academic::thin::delete_period,
            commands::academic::thin::get_current_period,
            commands::academic::thin::set_current_period,
            commands::academic::thin::lock_period,
            commands::academic::thin::unlock_period,
            commands::academic::thin::toggle_grade_entry,
            commands::academic::thin::preview_document_template,
            commands::academic::thin::render_document,
            commands::academic::thin::get_datasource_schemas,
            commands::academic::thin::evaluate_template_expressions,
            commands::academic::thin::start_batch_print,
            commands::academic::thin::pause_batch_print,
            commands::academic::thin::resume_batch_print,
            commands::academic::thin::cancel_batch_print,
            commands::academic::thin::get_batch_print_status,
            commands::academic::thin::compute_multi_up_layout,
            commands::academic::thin::preview_multi_up,
            // Assessment commands
            commands::assessment::thin::list_assessments,
            commands::assessment::thin::create_assessment,
            commands::assessment::thin::update_assessment,
            commands::assessment::thin::delete_assessment,
            commands::assessment::thin::submit_grade,
            commands::assessment::thin::submit_bulk_grades,
            commands::assessment::thin::update_grade,
            commands::assessment::thin::list_grades,
            commands::assessment::thin::record_attendance,
            commands::assessment::thin::record_bulk_attendance,
            commands::assessment::thin::list_attendance,
            commands::assessment::thin::submit_behavior,
            commands::assessment::thin::list_behavior,
            commands::assessment::thin::update_behavior,
            commands::assessment::thin::delete_behavior,
            commands::assessment::thin::get_report_card,
            commands::assessment::thin::get_grade_analytics,
            commands::assessment::thin::get_attendance_report,
            commands::assessment::thin::upsert_grade_cell,
            commands::assessment::thin::compile_semester_grades,
            commands::assessment::thin::get_grade_matrix,
            // Finance commands
            commands::finance::thin::list_fee_templates,
            commands::finance::thin::create_fee_template,
            commands::finance::thin::update_fee_template,
            commands::finance::thin::delete_fee_template,
            commands::finance::thin::list_discount_templates,
            commands::finance::thin::assign_fee,
            commands::finance::thin::list_student_fees,
            commands::finance::thin::list_all_student_fees,
            commands::finance::thin::record_payment,
            commands::finance::thin::list_payments,
            commands::finance::thin::list_ledger_accounts,
            commands::finance::thin::list_journal_entries,
            commands::finance::thin::list_treasuries,
            commands::finance::thin::create_treasury,
            commands::finance::thin::list_transaction_accounts,
            commands::finance::thin::get_account_statement,
            commands::finance::thin::list_active_fee_templates,
            commands::finance::thin::list_currencies,
            commands::finance::thin::create_currency,
            commands::finance::thin::update_currency,
            commands::finance::thin::delete_currency,
            commands::finance::thin::list_fiscal_years,
            // Lifeperiod commands
            commands::organization::thin::get_organization,
            commands::organization::thin::register_student,
            commands::organization::thin::get_main_organization,
            commands::organization::thin::update_organization,
            commands::organization::thin::get_settings,
            commands::organization::thin::update_settings,
            commands::organization::thin::get_dashboard_stats,
            commands::organization::thin::get_setup_status,
            commands::organization::thin::initialize_school_setup,
            commands::organization::thin::get_period_templates,
            commands::organization::thin::get_current_academic_year,
            commands::organization::thin::list_academic_years,
            commands::organization::thin::get_promotion_jobs,
            commands::organization::thin::get_promotion_job,
            commands::organization::thin::get_promotion_details,
            commands::organization::thin::promotion_dry_run,
            commands::organization::thin::execute_promotion,
            commands::organization::thin::rollback_promotion,
            commands::organization::thin::override_promotion_decision,
            commands::organization::thin::get_promotion_policy,
            commands::organization::thin::get_calendar_events,
            commands::organization::thin::get_pending_calendar_events,
            commands::organization::thin::create_calendar_event,
            commands::organization::thin::execute_calendar_event,
            commands::organization::thin::delete_calendar_event,
            commands::organization::thin::generate_default_calendar,
            commands::organization::thin::list_override_requests,
            commands::organization::thin::get_override_request,
            commands::organization::thin::process_approval,
            commands::organization::thin::cancel_override,
            commands::organization::thin::get_pending_approvals,
            commands::organization::thin::get_org_policy,
            commands::organization::thin::update_org_policy,
            commands::organization::thin::list_document_templates,
            commands::organization::thin::get_template_schemas,
            commands::organization::thin::get_document_template,
            commands::organization::thin::create_document_template,
            commands::organization::thin::update_document_template,
            commands::organization::thin::delete_document_template,
            commands::organization::thin::publish_document_template,
            commands::organization::thin::archive_document_template,
            commands::organization::thin::restore_document_template,
            commands::organization::thin::transition_enrollment_status,
            commands::organization::thin::import_students,
            commands::organization::thin::import_grades,
            commands::organization::thin::export_students,
            commands::organization::thin::export_grades,
            // AI commands
            commands::ai::thin::autopilot_get_tasks,
            commands::ai::thin::autopilot_get_diagnostics,
            commands::ai::thin::autopilot_run_tick,
            commands::ai::thin::agent_get_sessions,
            commands::ai::thin::agent_get_session_logs,
            commands::ai::thin::agent_approve_action,
            commands::ai::thin::agent_reject_action,
            commands::ai::thin::agent_create_session,
            commands::ai::thin::agent_query,
            commands::ai::thin::agent_open_standalone,
            // Scheduling commands
            commands::scheduling::thin::list_time_slots,
            commands::scheduling::thin::create_time_slot,
            commands::scheduling::thin::update_time_slot,
            commands::scheduling::thin::delete_time_slot,
            commands::scheduling::thin::get_class_schedule,
            commands::scheduling::thin::get_teacher_schedule,
            commands::scheduling::thin::get_lesson_by_id,
            commands::scheduling::thin::create_lesson,
            commands::scheduling::thin::update_lesson,
            commands::scheduling::thin::delete_lesson,
            commands::scheduling::thin::list_rooms,
            commands::scheduling::thin::create_room,
            commands::scheduling::thin::update_room,
            commands::scheduling::thin::delete_room,
            commands::scheduling::thin::start_solver,
            commands::scheduling::thin::list_solver_jobs,
            commands::scheduling::thin::get_solver_job,
            commands::scheduling::thin::cancel_solver,
            commands::scheduling::thin::apply_solver,
            commands::scheduling::thin::delete_solver_job,
            commands::scheduling::thin::check_schedule_conflicts,
            // Exam commands
            commands::exam::thin::list_exam_sessions,
            commands::exam::thin::create_exam_session,
            commands::exam::thin::get_exam_session,
            commands::exam::thin::update_exam_session,
            commands::exam::thin::list_exam_subjects,
            commands::exam::thin::create_exam_subject,
            commands::exam::thin::get_exam_halls,
            commands::exam::thin::create_exam_hall,
            commands::exam::thin::generate_ordering,
            commands::exam::thin::list_seating,
            commands::exam::thin::generate_exam_schedule,
            commands::exam::thin::list_schedule,
            commands::exam::thin::get_student_ordering,
            commands::exam::thin::rebuild_student_ordering,
        ])
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
            // 2. Initialize managed AppState with DatabaseProvider (multi-year support)
            let app_state = tauri::async_runtime::block_on(async {
                let state = AppState::init(&app_data_dir)
                    .await
                    .map_err(|e| format!("Database init failed: {e}"))?;
                // Run docforge template database migrations on the current year's DB
                let db = state.db();
                docforge_orbis::schema::migrate_template_schema(&db)
                    .await
                    .map_err(|e| format!("Template schema migration failed: {e}"))?;
                Ok::<AppState, String>(state)
            })?;
            app.manage(Arc::new(app_state));
            // Create batch job manager state
            let batch_job_manager = Arc::new(docforge_orbis::batch::BatchJobManager::new());
            app.manage(batch_job_manager);
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
                            let _ = check_for_updates(app_handle).await;
                        });
                    } else if event.id == "exit" {
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
            _ => {}
        });
}


