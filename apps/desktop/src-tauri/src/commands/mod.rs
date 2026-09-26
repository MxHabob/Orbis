//! Tauri command registration — all IPC handlers.

pub mod org_cmds;
pub mod iam_cmds;
pub mod calendar_cmds;
pub mod academic_cmds;
pub mod student_cmds;
pub mod employee_cmds;
pub mod group_cmds;
pub mod scheduling_cmds;
pub mod assessment_cmds;
pub mod exam_cmds;
pub mod finance_cmds;
pub mod document_cmds;
pub mod integration_cmds;
pub mod workflow_cmds;
pub mod progression_cmds;
pub mod report_cmds;
pub mod template_cmds;
pub mod window_cmds;
pub mod connection_cmds;
pub mod database_cmds;
pub mod bootstrap_cmds;
pub mod ai_cmds;
pub mod autopilot_cmds;

use tauri::generate_handler;

pub(crate) fn make_ctx(state: &crate::state::AppState) -> orbis_app::context::RequestContext {
    let session = state.get_active_session();
    let (tenant_id, actor) = match session {
        Some(s) => (s.org_id, orbis_domain::iam::actor::Actor::User(s.user_id)),
        None => (uuid::Uuid::nil(), orbis_domain::iam::actor::Actor::System),
    };
    orbis_app::context::RequestContext {
        tenant_id,
        actor,
        request_id: uuid::Uuid::new_v4().to_string(),
    }
}

pub fn generate_handlers() -> impl Fn(tauri::ipc::Invoke) -> bool {
    generate_handler![
        // ── Auth & IAM ──────────────────────────────────────────
        iam_cmds::auth_login,
        iam_cmds::auth_get_session,
        iam_cmds::auth_logout,
        iam_cmds::auth_mfa_enroll,
        iam_cmds::auth_mfa_verify_enrollment,
        iam_cmds::auth_mfa_challenge,
        iam_cmds::auth_link_google,
        iam_cmds::auth_login_google,
        iam_cmds::auth_list_devices,
        iam_cmds::auth_revoke_device,
        iam_cmds::iam_register_user,
        iam_cmds::iam_list_users,
        iam_cmds::iam_get_user,
        iam_cmds::iam_update_user,
        iam_cmds::iam_list_roles,
        iam_cmds::iam_save_role,
        iam_cmds::iam_delete_role,
        iam_cmds::iam_list_permissions,
        iam_cmds::iam_save_permissions,
        // ── Organization ────────────────────────────────────────
        org_cmds::org_get_current,
        org_cmds::org_update_current,
        org_cmds::org_get_settings,
        org_cmds::org_update_settings,
        org_cmds::org_get_dashboard,
        org_cmds::org_create_unit,
        org_cmds::org_list_units,
        org_cmds::org_get_unit,
        org_cmds::org_delete_unit,
        org_cmds::org_select_unit,
        // ── Calendar ────────────────────────────────────────────
        calendar_cmds::calendar_create_year,
        calendar_cmds::calendar_list_years,
        calendar_cmds::calendar_get_context,
        calendar_cmds::calendar_get_year,
        calendar_cmds::calendar_create_period,
        calendar_cmds::calendar_list_periods,
        calendar_cmds::calendar_get_period,
        calendar_cmds::calendar_lock_period,
        calendar_cmds::calendar_unlock_period,
        calendar_cmds::calendar_close_period,
        calendar_cmds::calendar_list_period_templates,
        // ── Academic ────────────────────────────────────────────
        academic_cmds::academic_create_subject,
        academic_cmds::academic_list_subjects,
        academic_cmds::academic_get_subject,
        academic_cmds::academic_create_class,
        academic_cmds::academic_list_classes,
        academic_cmds::academic_get_class,
        academic_cmds::academic_assign_class_subject,
        academic_cmds::academic_list_class_subjects,
        academic_cmds::academic_check_prerequisites,
        // ── Student ─────────────────────────────────────────────
        student_cmds::student_register_student,
        student_cmds::student_list_students,
        student_cmds::student_get_student,
        student_cmds::student_update_student_status,
        student_cmds::student_update_profile,
        student_cmds::student_change_study_mode,
        student_cmds::student_transfer_class,
        student_cmds::student_enroll_student,
        student_cmds::student_list_enrollments,
        student_cmds::student_get_enrollment,
        student_cmds::student_assign_guardian,
        student_cmds::student_list_guardians,
        // ── Employee Affairs ────────────────────────────────────
        employee_cmds::employee_register_employee,
        employee_cmds::employee_list_employees,
        employee_cmds::employee_get_employee,
        employee_cmds::employee_get_employee_stats,
        employee_cmds::employee_update_employee,
        employee_cmds::employee_update_employee_status,
        employee_cmds::employee_provision_account,
        employee_cmds::employee_assign_groups,
        employee_cmds::employee_reset_password,
        // ── User Groups ─────────────────────────────────────────
        group_cmds::group_create_group,
        group_cmds::group_list_groups,
        group_cmds::group_get_group,
        group_cmds::group_update_group,
        group_cmds::group_delete_group,
        group_cmds::group_get_members,
        group_cmds::group_add_member,
        group_cmds::group_remove_member,
        group_cmds::group_bulk_assign,
        group_cmds::group_set_permissions,
        // ── Student Cohort Progression ──────────────────────────
        progression_cmds::progression_evaluate_cohort,
        progression_cmds::progression_promote_cohort,
        progression_cmds::progression_list_jobs,
        progression_cmds::progression_get_job,
        // ── Scheduling & Solver ────────────────────────────────
        scheduling_cmds::scheduling_create_room,
        scheduling_cmds::scheduling_list_rooms,
        scheduling_cmds::scheduling_get_room,
        scheduling_cmds::scheduling_create_time_slot,
        scheduling_cmds::scheduling_list_time_slots,
        scheduling_cmds::scheduling_schedule_lesson,
        scheduling_cmds::scheduling_list_lessons,
        scheduling_cmds::solver_get_sidecar_status,
        scheduling_cmds::solver_stop_sidecar,
        scheduling_cmds::solver_submit_timetable,
        scheduling_cmds::solver_get_timetable_status,
        scheduling_cmds::solver_submit_exam,
        scheduling_cmds::solver_get_exam_status,
        // ── Assessment ──────────────────────────────────────────
        assessment_cmds::assessment_create_scheme,
        assessment_cmds::assessment_list_schemes,
        assessment_cmds::assessment_get_scheme,
        assessment_cmds::assessment_update_scheme,
        assessment_cmds::assessment_delete_scheme,
        assessment_cmds::assessment_record_grade,
        assessment_cmds::assessment_list_grades,
        assessment_cmds::assessment_calculate_grades,
        assessment_cmds::assessment_mark_attendance,
        assessment_cmds::assessment_list_attendance,
        assessment_cmds::assessment_calculate_gpa,
        assessment_cmds::assessment_calculate_rankings,
        assessment_cmds::assessment_record_behavior,
        assessment_cmds::assessment_list_student_behavior_records,
        // ── Exam ────────────────────────────────────────────────
        exam_cmds::exam_create_session,
        exam_cmds::exam_list_sessions,
        exam_cmds::exam_get_session,
        exam_cmds::exam_schedule_exam,
        exam_cmds::exam_list_schedules,
        exam_cmds::exam_assign_seating,
        exam_cmds::exam_list_seating,
        exam_cmds::exam_generate_seating,
        exam_cmds::exam_assign_secret,
        exam_cmds::exam_list_secrets,
        exam_cmds::exam_generate_secrets,
        exam_cmds::exam_assign_proctor,
        // ── Finance ─────────────────────────────────────────────
        finance_cmds::finance_create_account,
        finance_cmds::finance_list_accounts,
        finance_cmds::finance_create_fee_template,
        finance_cmds::finance_list_fee_templates,
        finance_cmds::finance_generate_fee,
        finance_cmds::finance_list_student_fees,
        finance_cmds::finance_process_payment,
        finance_cmds::finance_list_payments,
        finance_cmds::finance_list_currencies,
        finance_cmds::finance_list_journal_entries,
        finance_cmds::finance_get_journal_entry,
        finance_cmds::finance_create_discount_template,
        finance_cmds::finance_list_discount_templates,
        // ── Document ────────────────────────────────────────────
        document_cmds::document_upload,
        document_cmds::document_list,
        document_cmds::document_get,
        document_cmds::document_delete,
        // ── Integration ─────────────────────────────────────────
        integration_cmds::integration_connect_app,
        integration_cmds::integration_list,
        integration_cmds::integration_get_status,
        integration_cmds::integration_disconnect,
        integration_cmds::integration_toggle,
        // ── Academic Override & Workflow ────────────────────────
        workflow_cmds::workflow_submit_override,
        workflow_cmds::workflow_approve_override,
        workflow_cmds::workflow_reject_override,
        workflow_cmds::workflow_list_overrides,
        workflow_cmds::workflow_get_override,
        // ── Report Generation & Audit Logs ─────────────────────
        report_cmds::generate_report,
        report_cmds::get_report_job_status,
        report_cmds::download_report_file,
        report_cmds::batch_print,
        report_cmds::audit_list_logs,
        // ── Template Management ─────────────────────────────────
        template_cmds::template_create,
        template_cmds::template_get,
        template_cmds::template_list,
        template_cmds::template_update,
        template_cmds::template_delete,
        template_cmds::template_publish,
        template_cmds::template_archive,
        template_cmds::template_get_versions,
        template_cmds::template_revert_version,
        template_cmds::template_list_bindings,
        template_cmds::template_create_binding,
        template_cmds::template_preview,
        template_cmds::template_validate,
        // ── Report Definitions (read-only) ──────────────────────
        template_cmds::report_list_definitions,
        template_cmds::report_get_definition,
        // ── Window & System ─────────────────────────────────────
        window_cmds::show_window,
        window_cmds::open_agent_window,
        window_cmds::open_connection_window,
        window_cmds::check_for_updates,
        // ── Connection ──────────────────────────────────────────
        connection_cmds::connection_get_config,
        connection_cmds::connection_set_config,
        connection_cmds::connection_reset,
        connection_cmds::connection_validate,
        connection_cmds::connection_check_remote_health,
        connection_cmds::connection_connect,
        // ── Database Management ─────────────────────────────────
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
        // ── Bootstrap & System ──────────────────────────────────
        bootstrap_cmds::system_health,
        bootstrap_cmds::system_has_organization,
        bootstrap_cmds::system_get_readiness,
        bootstrap_cmds::bootstrap_organization,
        bootstrap_cmds::bootstrap_initial_setup,
        bootstrap_cmds::onboarding_complete,
        bootstrap_cmds::system_get_entitlement,
        // ── AI Platform ─────────────────────────────────────────
        ai_cmds::ai_chat,
        ai_cmds::ai_list_tools,
        ai_cmds::ai_save_session,
        ai_cmds::ai_load_sessions,
        ai_cmds::ai_delete_session,
        ai_cmds::ai_save_context,
        ai_cmds::ai_load_context,
        // ── Autopilot System ────────────────────────────────────
        autopilot_cmds::autopilot_list_tasks,
        autopilot_cmds::autopilot_run_tick,
        autopilot_cmds::autopilot_get_task,
        autopilot_cmds::autopilot_execute_task,
        autopilot_cmds::autopilot_retry_task,
        autopilot_cmds::autopilot_dismiss_task,
        autopilot_cmds::autopilot_get_diagnostics,
    ]
}
