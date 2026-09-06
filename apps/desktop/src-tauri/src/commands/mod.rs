//! Tauri command registration — all IPC handlers.

pub mod org_cmds;
pub mod iam_cmds;
pub mod calendar_cmds;
pub mod academic_cmds;
pub mod student_cmds;
pub mod scheduling_cmds;
pub mod assessment_cmds;
pub mod exam_cmds;
pub mod finance_cmds;
pub mod document_cmds;
pub mod integration_cmds;
pub mod report_cmds;
pub mod template_cmds;
pub mod window_cmds;

use tauri::generate_handler;

/// Build a `RequestContext` from the managed `AppState`.
///
/// In a full implementation, this reads the active session from
/// `AppState` (set during `auth_login`). For now, it uses the
/// default org and System actor — to be replaced once session
/// management is wired up in the Tauri managed state.
pub(crate) fn make_ctx(state: &crate::state::AppState) -> orbis_app::context::RequestContext {
    // TODO: Read active session from managed state once login flow stores it.
    orbis_app::context::RequestContext {
        tenant_id: state.active_org_id.unwrap_or_default(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: uuid::Uuid::new_v4().to_string(),
    }
}

pub fn generate_handlers() -> impl Fn(tauri::ipc::Invoke) -> bool {
    generate_handler![
        // ── Auth & IAM ──────────────────────────────────────────
        iam_cmds::auth_login,
        iam_cmds::auth_get_session,
        iam_cmds::auth_logout,
        iam_cmds::iam_register_user,
        iam_cmds::iam_list_users,
        iam_cmds::iam_get_user,
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
        // ── Calendar ────────────────────────────────────────────
        calendar_cmds::calendar_list_years,
        calendar_cmds::calendar_get_context,
        calendar_cmds::calendar_get_year,
        calendar_cmds::calendar_list_periods,
        calendar_cmds::calendar_get_period,
        calendar_cmds::calendar_list_period_templates,
        // ── Academic ────────────────────────────────────────────
        academic_cmds::academic_list_subjects,
        academic_cmds::academic_get_subject,
        academic_cmds::academic_list_classes,
        academic_cmds::academic_get_class,
        academic_cmds::academic_list_class_subjects,
        // ── Student ─────────────────────────────────────────────
        student_cmds::student_list_enrollments,
        student_cmds::student_get_enrollment,
        student_cmds::student_list_guardians,
        // ── Scheduling ──────────────────────────────────────────
        scheduling_cmds::scheduling_list_rooms,
        scheduling_cmds::scheduling_get_room,
        scheduling_cmds::scheduling_list_time_slots,
        scheduling_cmds::scheduling_list_lessons,
        // ── Assessment ──────────────────────────────────────────
        assessment_cmds::assessment_list_schemes,
        assessment_cmds::assessment_get_scheme,
        assessment_cmds::assessment_list_grades,
        assessment_cmds::assessment_list_attendance,
        // ── Exam ────────────────────────────────────────────────
        exam_cmds::exam_list_sessions,
        exam_cmds::exam_get_session,
        exam_cmds::exam_list_schedules,
        // ── Finance ─────────────────────────────────────────────
        finance_cmds::finance_list_accounts,
        finance_cmds::finance_list_fee_templates,
        finance_cmds::finance_list_student_fees,
        finance_cmds::finance_list_payments,
        finance_cmds::finance_list_currencies,
        // ── Document ────────────────────────────────────────────
        document_cmds::document_list,
        document_cmds::document_get,
        document_cmds::document_delete,
        // ── Integration ─────────────────────────────────────────
        integration_cmds::integration_list,
        // ── Report Generation ───────────────────────────────────
        report_cmds::generate_report,
        report_cmds::get_report_job_status,
        report_cmds::download_report_file,
        report_cmds::batch_print,
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
        window_cmds::check_for_updates,
    ]
}

