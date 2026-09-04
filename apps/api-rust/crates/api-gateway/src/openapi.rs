use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Orbis API",
        version = "1.0.0",
        description = "OpenAPI contract generated from the Orbis API Gateway handlers"
    ),
    paths(
        crate::handler::academic_period::get_periods,
        crate::handler::academic_period::post_periods,
        crate::handler::academic_period::get_period,
        crate::handler::academic_period::get_periods_by_year,
        crate::handler::academic_period::post_lock_period,
        crate::handler::academic_period::post_unlock_period,
        crate::handler::academic_period::post_open_enrollment,
        crate::handler::academic_period::post_close_enrollment,
        crate::handler::academic_period::post_open_grade_entry,
        crate::handler::academic_period::post_close_grade_entry,
        crate::handler::attendance::get_attendance,
        crate::handler::attendance::post_attendance,
        crate::handler::attendance::post_attendance_bulk,
        crate::handler::attendance::get_attendance_report,
        crate::handler::audit::get_audit_logs,
        crate::handler::auth::post_login,
        crate::handler::auth::post_refresh,
        crate::handler::auth::post_register,
        crate::handler::auth::get_me,
        crate::handler::auth::post_logout,
        crate::handler::behavior::post_behavior,
        crate::handler::behavior::get_behavior,
        crate::handler::class::get_classes,
        crate::handler::class::post_classes,
        crate::handler::class::put_class,
        crate::handler::class::delete_class,
        crate::handler::class::get_class_students,
        crate::handler::class::get_class_students_enriched,
        crate::handler::dashboard::get_dashboard,
        crate::handler::document::get_documents,
        crate::handler::document::post_documents,
        crate::handler::document::download_document,
        crate::handler::document::delete_document,
        crate::handler::enrollment::get_enrollments,
        crate::handler::enrollment::post_enrollments,
        crate::handler::enrollment::get_enrollment,
        crate::handler::enrollment::post_withdraw,
        crate::handler::enrollment::post_transfer,
        crate::handler::enrollment::post_promote,
        crate::handler::exam::get_sessions,
        crate::handler::exam::post_session,
        crate::handler::exam::get_exam_subjects,
        crate::handler::exam::get_halls,
        crate::handler::exam::post_generate_seating,
        crate::handler::exam::post_generate_schedule,
        crate::handler::finance::get_fee_templates,
        crate::handler::finance::post_fee_templates,
        crate::handler::finance::post_assign_fee,
        crate::handler::finance::get_student_fees,
        crate::handler::finance::post_payment,
        crate::handler::finance::get_payments,
        crate::handler::finance::get_ledger_accounts,
        crate::handler::grade::get_grades,
        crate::handler::grade::post_grades,
        crate::handler::grade::post_grades_bulk,
        crate::handler::grade::post_grade_upsert,
        crate::handler::grade::get_grade_matrix,
        crate::handler::grade::get_report_card,
        crate::handler::grade::post_compile_semester,
        crate::handler::guardian::get_guardians,
        crate::handler::guardian::post_guardians,
        crate::handler::guardian::delete_guardian,
        crate::handler::import_export::post_import_students,
        crate::handler::import_export::post_import_grades,
        crate::handler::import_export::get_export_students,
        crate::handler::import_export::get_export_grades,
        crate::handler::notification::get_notifications,
        crate::handler::org_unit::get_units,
        crate::handler::org_unit::post_units,
        crate::handler::org_unit::put_unit,
        crate::handler::org_unit::delete_unit,
        crate::handler::organization::get_organization,
        crate::handler::organization::put_organization,
        crate::handler::organization::get_organization_stats,
        crate::handler::report::get_reports,
        crate::handler::report::get_report_definition,
        crate::handler::report::post_generate_report,
        crate::handler::report::post_export_report,
        crate::handler::room::get_rooms,
        crate::handler::room::post_room,
        crate::handler::room::put_room,
        crate::handler::room::delete_room,
        crate::handler::schedule_solve::post_solve,
        crate::handler::schedule_solve::get_solve_jobs,
        crate::handler::schedule_solve::post_apply_solve,
        crate::handler::schedule_solve::post_cancel_solve,
        crate::handler::schedule::get_class_schedule,
        crate::handler::schedule::get_teacher_schedule,
        crate::handler::schedule::post_lesson,
        crate::handler::schedule::put_lesson,
        crate::handler::schedule::delete_lesson,
        crate::handler::schedule::get_time_slots,
        crate::handler::subject::get_subjects,
        crate::handler::subject::post_subjects,
        crate::handler::subject::put_subject,
        crate::handler::subject::delete_subject,
        crate::handler::user::get_users,
        crate::handler::user::post_users,
        crate::handler::user::get_user,
        crate::handler::user::put_user,
        crate::handler::user::delete_user,
        crate::handler::user::post_user_role,
        crate::handler::user::post_user_password,
    ),
    components(
        schemas(
            svc_iam::ops::auth::RegisterPayload,
            svc_iam::ops::auth::ChangePasswordPayload,
            svc_iam::ops::user::CreateUserPayload,
            svc_iam::ops::user::UpdateUserPayload,
            svc_iam::ops::user::ChangeRolePayload,
            svc_academic::ops::class::CreateClassPayload,
            svc_academic::ops::class::UpdateClassPayload,
            svc_academic::ops::enrollment::CreateEnrollmentPayload,
            svc_academic::ops::enrollment::PromotePayload,
            svc_academic::ops::guardian::CreateGuardianPayload,
            svc_academic::ops::subject::CreateSubjectPayload,
            svc_academic::ops::subject::UpdateSubjectPayload,
            svc_assessment::ops::attendance::CreateAttendancePayload,
            svc_assessment::ops::attendance::BulkAttendancePayload,
            svc_assessment::ops::attendance::BulkAttendanceItem,
            svc_assessment::ops::behavior::SubmitAIBehaviorRequest,
            svc_assessment::ops::grade::CreateGradePayload,
            svc_assessment::ops::grade::SubmitBulkPayload,
            svc_assessment::services::grade::UpsertCellRequest,
            svc_calendar::ops::academic_period::CreatePeriodPayload,
            svc_scheduling::ops::lesson::CreateLessonRequest,
            svc_scheduling::ops::lesson::UpdateLessonRequest,
            svc_scheduling::ops::room::CreateRoomRequest,
            svc_scheduling::ops::room::UpdateRoomRequest
        )
    ),
    tags(
        (name = "auth", description = "Authentication and sessions"),
        (name = "dashboard", description = "Dashboard data"),
        (name = "academic", description = "Academic resources"),
        (name = "attendance", description = "Attendance data"),
        (name = "documents", description = "Document data")
    )
)]
pub struct ApiDoc;

pub fn json() -> String {
    let mut document = serde_json::to_value(ApiDoc::openapi())
        .expect("OpenAPI document must serialize");

    normalize_schema_refs(&mut document);

    if let Some(paths) = document.get_mut("paths").and_then(serde_json::Value::as_object_mut) {
        for path in paths.values_mut() {
            let Some(operations) = path.as_object_mut() else { continue };
            for operation in operations.values_mut() {
                let Some(operation) = operation.as_object_mut() else { continue };
                let responses = operation
                    .entry("responses")
                    .or_insert_with(|| serde_json::json!({}));
                if responses.as_object().is_some_and(serde_json::Map::is_empty) {
                    *responses = serde_json::json!({
                        "200": {
                            "description": "Successful response",
                            "content": {
                                "application/json": {
                                    "schema": {}
                                }
                            }
                        }
                    });
                }
            }
        }
    }

    serde_json::to_string_pretty(&document).expect("OpenAPI document must serialize")
}

fn normalize_schema_refs(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Array(values) => {
            for value in values {
                normalize_schema_refs(value);
            }
        }
        serde_json::Value::Object(values) => {
            if let Some(serde_json::Value::String(reference)) = values.get_mut("$ref") {
                if let Some(name) = reference.strip_prefix("#/components/schemas/") {
                    if let Some(short_name) = name.rsplit('.').next() {
                        *reference = format!("#/components/schemas/{short_name}");
                    }
                }
            }
            for value in values.values_mut() {
                normalize_schema_refs(value);
            }
        }
        _ => {}
    }
}