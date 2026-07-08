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

            // Auth commands
            commands::iam::auth::register,
            commands::iam::auth::login,
            commands::iam::auth::validate_token,
            commands::iam::auth::logout,

            // Permissions commands
            commands::iam::permissions::get_permissions,
            commands::iam::permissions::get_role_permissions,
            commands::iam::permissions::save_role_permissions,
            commands::iam::permissions::get_user_permissions,
            commands::iam::permissions::save_user_permissions,
            commands::iam::permissions::get_available_roles,


            // Users & organization commands
            commands::iam::users::get_organization,
            commands::iam::users::update_organization,
            commands::iam::users::get_main_organization,
            commands::iam::users::get_users,
            commands::iam::users::get_user,
            commands::iam::users::create_user,
            commands::iam::users::update_user,
            commands::iam::users::delete_user,
            commands::iam::users::change_user_role,
            commands::iam::users::change_user_password,
            commands::iam::users::get_user_roles,
            commands::iam::users::save_user_roles,
            commands::iam::users::get_user_sessions,

            // Academics commands
            commands::academic::academics::get_cycles,
            commands::academic::academics::get_cycle,
            commands::academic::academics::create_cycle,
            commands::academic::academics::update_cycle,
            commands::academic::academics::activate_cycle,
            commands::academic::academics::lock_cycle,
            commands::academic::academics::unlock_cycle,
            commands::academic::academics::open_grade_entry,
            commands::academic::academics::close_grade_entry,
            commands::academic::academics::get_classes,
            commands::academic::academics::create_class,
            commands::academic::academics::update_class,
            commands::academic::academics::delete_class,

            // Enrollments commands
            commands::academic::enrollments::get_enrollments,
            commands::academic::enrollments::get_enrollment,
            commands::academic::enrollments::create_enrollment,
            commands::academic::enrollments::withdraw_enrollment,
            commands::academic::enrollments::transfer_enrollment,
            commands::academic::enrollments::promote_students,
            commands::academic::enrollments::transition_enrollment_status,
            commands::academic::enrollments::check_student_duplicate,

            // Grades commands
            commands::assessment::grades::get_grades,
            commands::assessment::grades::submit_grade,
            commands::assessment::grades::submit_grades_bulk,
            commands::assessment::grades::update_grade,
            commands::assessment::grades::get_report_card,
            commands::assessment::grades::get_grade_analytics,
            commands::assessment::grades::bulk_subject_grade_entry,
            commands::assessment::grades::get_grade_matrix,
            commands::assessment::grades::compile_semester_grades,
            commands::assessment::grades::upsert_grade_cell,

            // Attendance commands
            commands::assessment::attendance::get_attendance,
            commands::assessment::attendance::record_attendance,
            commands::assessment::attendance::record_bulk_attendance,
            commands::assessment::attendance::get_attendance_report,
            commands::assessment::attendance::get_attendance_analytics,

            // Finance commands
            commands::finance::finance::get_ledger_accounts,
            commands::finance::finance::get_transaction_accounts,
            commands::finance::finance::create_ledger_account,
            commands::finance::finance::get_treasuries,
            commands::finance::finance::create_treasury,
            commands::finance::finance::get_student_fees,
            commands::finance::finance::get_active_fee_templates,
            commands::finance::finance::get_fee_templates,
            commands::finance::finance::create_fee_template,
            commands::finance::finance::update_fee_template,
            commands::finance::finance::delete_fee_template,
            commands::finance::finance::assign_student_fee,
            commands::finance::finance::get_payments,
            commands::finance::finance::record_payment,
            commands::finance::finance::get_journal_entries,
            commands::finance::finance::create_journal_entry,
            commands::finance::finance::get_active_discount_templates,
            commands::finance::finance::get_discount_templates,
            commands::finance::finance::create_discount_template,
            commands::finance::finance::update_discount_template,
            commands::finance::finance::delete_discount_template,
            commands::finance::finance::get_currencies,
            commands::finance::finance::get_currency,
            commands::finance::finance::create_currency,
            commands::finance::finance::update_currency,
            commands::finance::finance::delete_currency,
            commands::finance::finance::get_account_statement,
            commands::finance::finance::seed_default_financials,

            // Misc commands
            // Behavior commands (mapped to assessment)
            commands::assessment::assessments::get_behavior_records,
            commands::lifecycle::setup::initialize_school_setup,
            commands::lifecycle::setup::get_setup_status,
            commands::assessment::assessments::submit_ai_behavior,
            commands::assessment::assessments::update_behavior_record,
            commands::assessment::assessments::delete_behavior_record,
            commands::academic::academics::get_notifications,
            commands::academic::academics::mark_notification_read,
            commands::academic::academics::get_org_units,
            commands::academic::academics::create_org_unit,
            commands::academic::academics::update_org_unit,
            commands::academic::academics::delete_org_unit,
            commands::academic::academics::get_dashboard_stats,
            commands::lifecycle::setup::get_audit_logs,

            // Subjects
            commands::academic::subjects::get_subjects,
            commands::academic::subjects::get_subject,
            commands::academic::subjects::create_subject,
            commands::academic::subjects::update_subject,
            commands::academic::subjects::delete_subject,

            // Assessments
            commands::assessment::assessments::get_assessments,
            commands::assessment::assessments::create_assessment,
            commands::assessment::assessments::update_assessment,
            commands::assessment::assessments::delete_assessment,

            // Documents
            commands::academic::documents::get_documents,
            commands::academic::documents::get_document,
            commands::academic::documents::create_document,
            commands::academic::documents::delete_document,

            // Document Templates
            commands::academic::document_templates::get_document_templates,
            commands::academic::document_templates::get_document_template,
            commands::academic::document_templates::create_document_template,
            commands::academic::document_templates::update_document_template,
            commands::academic::document_templates::delete_document_template,
            commands::academic::document_templates::publish_document_template,
            commands::academic::document_templates::archive_document_template,
            commands::academic::document_templates::restore_document_template,
            commands::academic::document_templates::get_document_template_versions,
            commands::academic::document_templates::revert_document_template_to_version,

            // Widgets
            commands::widgets::widgets::get_widget_preferences,
            commands::widgets::widgets::update_widget_preferences,

            // Schedule
            commands::scheduling::schedule::get_time_slots,
            commands::scheduling::schedule::create_time_slot,
            commands::scheduling::schedule::update_time_slot,
            commands::scheduling::schedule::delete_time_slot,
            commands::scheduling::schedule::get_class_subjects,
            commands::scheduling::schedule::create_class_subject,
            commands::scheduling::schedule::update_class_subject,
            commands::scheduling::schedule::delete_class_subject,
            commands::scheduling::schedule::get_class_schedule,
            commands::scheduling::schedule::get_teacher_schedule,
            commands::scheduling::schedule::get_lesson_by_id,
            commands::scheduling::schedule::create_lesson,
            commands::scheduling::schedule::update_lesson,
            commands::scheduling::schedule::delete_lesson,
            commands::scheduling::schedule::check_schedule_conflicts,
            commands::scheduling::schedule::start_schedule_solve,
            commands::scheduling::schedule::get_schedule_solve_job,
            commands::scheduling::schedule::list_schedule_solve_jobs,
            commands::scheduling::schedule::cancel_schedule_solve,
            commands::scheduling::schedule::apply_schedule_solve,
            commands::scheduling::schedule::delete_schedule_solve_job,
            commands::scheduling::schedule::get_rooms,
            commands::scheduling::schedule::create_room,
            commands::scheduling::schedule::update_room,
            commands::scheduling::schedule::delete_room,

            // Guardians
            commands::academic::guardians::get_guardians_by_student,
            commands::academic::guardians::get_students_by_guardian,
            commands::academic::guardians::create_guardian_link,
            commands::academic::guardians::update_guardian_link,
            commands::academic::guardians::delete_guardian_link,
            commands::academic::guardians::search_guardians,

            // Exams
            commands::exam::exams::get_exam_sessions,
            commands::exam::exams::get_exam_session,
            commands::exam::exams::create_exam_session,
            commands::exam::exams::update_exam_session,
            commands::exam::exams::get_exam_subjects,
            commands::exam::exams::create_exam_subject,
            commands::exam::exams::get_student_ordering,
            commands::exam::exams::rebuild_student_ordering,
            commands::exam::exams::freeze_student_ordering,
            commands::exam::exams::generate_seating_numbers,
            commands::exam::exams::generate_secret_numbers,
            commands::exam::exams::get_exam_schedules,
            commands::exam::exams::start_exam_solve,
            commands::exam::exams::get_exam_solve_job,
            commands::exam::exams::list_exam_solve_jobs,
            commands::exam::exams::cancel_exam_solve,
            commands::exam::exams::apply_exam_solve,
            commands::exam::exams::delete_exam_schedule,
            commands::exam::exams::update_exam_schedule_status,
            commands::exam::exams::allocate_students_to_halls,
            commands::exam::exams::auto_assign_proctors,
            commands::exam::exams::get_exam_halls,
            commands::exam::exams::create_exam_hall,
            commands::exam::exams::get_exam_proctors,
            commands::exam::exams::get_hall_students,
            commands::exam::exams::get_seating_numbers,

            // DocForge Layout & Render commands
            commands::academic::docforge::preview_document_template,
            commands::academic::docforge::render_document,
            commands::academic::docforge::get_datasource_schemas,
            commands::academic::docforge::evaluate_template_expressions,
            commands::academic::docforge::start_batch_print,
            commands::academic::docforge::pause_batch_print,
            commands::academic::docforge::resume_batch_print,
            commands::academic::docforge::cancel_batch_print,
            commands::academic::docforge::get_batch_print_status,

            // Bulk Import commands
            commands::academic::bulk_import::bulk_create_students,
            commands::academic::bulk_import::bulk_create_teachers,
            commands::academic::bulk_import::bulk_create_grades,

            // Academic Year Management commands
            commands::lifecycle::academic_years::list_academic_years,
            commands::lifecycle::academic_years::get_current_academic_year,
            commands::lifecycle::academic_years::create_academic_year,
            commands::lifecycle::academic_years::switch_academic_year,
            commands::lifecycle::academic_years::archive_academic_year,
            commands::lifecycle::academic_years::is_current_year_archived,
            commands::lifecycle::academic_years::set_active_academic_year,

            // Academic Context commands
            commands::lifecycle::academic_context::get_academic_context,
            commands::lifecycle::academic_context::resolve_active_cycle,
            commands::lifecycle::academic_context::is_year_writable,

            // Calendar commands
            commands::lifecycle::calendar::get_calendar_events,
            commands::lifecycle::calendar::get_pending_calendar_events,
            commands::lifecycle::calendar::create_calendar_event,
            commands::lifecycle::calendar::update_calendar_event,
            commands::lifecycle::calendar::delete_calendar_event,
            commands::lifecycle::calendar::execute_calendar_event,
            commands::lifecycle::calendar::generate_default_events,
            commands::lifecycle::calendar::process_pending_events,
            commands::lifecycle::calendar::get_org_academic_policy,
            commands::lifecycle::calendar::update_org_academic_policy,

            // Promotion commands
            commands::lifecycle::promotion::promotion_dry_run,
            commands::lifecycle::promotion::get_promotion_jobs,
            commands::lifecycle::promotion::get_promotion_job,
            commands::lifecycle::promotion::get_promotion_details,
            commands::lifecycle::promotion::override_promotion_decision,
            commands::lifecycle::promotion::execute_promotion,
            commands::lifecycle::promotion::rollback_promotion,
            commands::lifecycle::promotion::get_promotion_policy,
            commands::lifecycle::promotion::update_promotion_policy,

            // Override & Approval commands
            commands::lifecycle::override_approval::create_override_request,
            commands::lifecycle::override_approval::process_approval,
            commands::lifecycle::override_approval::cancel_override_request,
            commands::lifecycle::override_approval::list_override_requests,
            commands::lifecycle::override_approval::get_override_request,
            commands::lifecycle::override_approval::get_pending_approvals,

            // AI / Autopilot commands
            commands::ai::autopilot::get_autopilot_status,
            commands::ai::autopilot::get_autopilot_tasks,
            commands::ai::autopilot::get_autopilot_diagnostics,
            commands::ai::autopilot::configure_autopilot,
            commands::ai::autopilot::run_autopilot_tick,
            commands::ai::agent::get_agent_sessions,
            commands::ai::agent::get_agent_session_logs,
            commands::ai::agent::approve_agent_action,
            commands::ai::agent::reject_agent_action,
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
                let db = state.active_db().await;
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

            // 3. Spawn startup automation (fire-and-forget, non-blocking)
            let app_handle_for_automation = app_handle.clone();
            tauri::async_runtime::spawn(async move {
                features::startup_automation::process_startup_events(
                    app_handle_for_automation,
                ).await;
            });



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

