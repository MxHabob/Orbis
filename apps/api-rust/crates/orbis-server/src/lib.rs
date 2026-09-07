//! Orbis Server — Axum router with full OpenAPI documentation.

pub mod api;
pub mod state;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use api::{
    iam_routes, org_routes, calendar_routes, academic_routes,
    student_routes, override_routes, scheduling_routes, assessment_routes, exam_routes,
    finance_routes, document_routes, integration_routes, report_routes,
};

// ── OpenAPI Spec ────────────────────────────────────────────────

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Orbis API",
        version = "1.0.0",
        description = "Orbis Education Platform — Unified REST API",
    ),
    tags(
        (name = "Auth", description = "Authentication & session management"),
        (name = "IAM", description = "Users, roles, permissions"),
        (name = "Organization", description = "Organization CRUD, settings, dashboard"),
        (name = "OrgUnits", description = "Organizational units / branches"),
        (name = "Calendar", description = "Academic years, periods, templates"),
        (name = "Academic", description = "Subjects, classes, class-subject assignments"),
        (name = "Student", description = "Enrollments, guardians"),
        (name = "Override", description = "Academic & Financial override requests and approval workflow"),
        (name = "Scheduling", description = "Rooms, time slots, scheduled lessons"),
        (name = "Assessment", description = "Assessment schemes, grades, attendance"),
        (name = "Exam", description = "Exam sessions, schedules, seating, proctors"),
        (name = "Finance", description = "Accounts, fee templates, student fees, payments"),
        (name = "Document", description = "Document management"),
        (name = "Integration", description = "Third-party app integrations"),
        (name = "Report", description = "Report definitions, templates, generation"),
    ),
    paths(
        // Auth & MFA & Devices
        api::iam_routes::login,
        api::iam_routes::get_session,
        api::iam_routes::mfa_enroll,
        api::iam_routes::mfa_verify_enrollment,
        api::iam_routes::mfa_challenge,
        api::iam_routes::google_link,
        api::iam_routes::google_login,
        api::iam_routes::list_devices,
        api::iam_routes::revoke_device,
        api::iam_routes::register_user,
        api::iam_routes::list_users,
        api::iam_routes::get_user,
        api::iam_routes::update_user,
        // Organization
        api::org_routes::create_org,
        api::org_routes::get_current_org,
        api::org_routes::update_current_org,
        api::org_routes::bootstrap_org,
        api::org_routes::get_settings,
        api::org_routes::update_settings,
        api::org_routes::get_dashboard,
        api::org_routes::create_org_unit,
        api::org_routes::list_org_units,
        api::org_routes::get_org_unit,
        api::org_routes::delete_org_unit,
        // Calendar
        api::calendar_routes::create_year,
        api::calendar_routes::list_years,
        api::calendar_routes::get_current_context,
        api::calendar_routes::get_year,
        api::calendar_routes::create_period,
        api::calendar_routes::list_periods,
        api::calendar_routes::get_period,
        api::calendar_routes::list_period_templates,
        // Academic
        api::academic_routes::create_subject,
        api::academic_routes::list_subjects,
        api::academic_routes::get_subject,
        api::academic_routes::create_class,
        api::academic_routes::list_classes,
        api::academic_routes::get_class,
        api::academic_routes::assign_class_subject,
        api::academic_routes::list_class_subjects,
        // Student
        api::student_routes::register_student,
        api::student_routes::list_students,
        api::student_routes::get_student,
        api::student_routes::update_student_status,
        api::student_routes::enroll_student,
        api::student_routes::list_enrollments,
        api::student_routes::get_enrollment,
        api::student_routes::change_study_mode,
        api::student_routes::assign_guardian,
        api::student_routes::list_guardians,
        // Override & Workflow
        api::override_routes::submit_override,
        api::override_routes::list_overrides,
        api::override_routes::get_override,
        api::override_routes::approve_override,
        api::override_routes::reject_override,
        // Scheduling
        api::scheduling_routes::create_room,
        api::scheduling_routes::list_rooms,
        api::scheduling_routes::get_room,
        api::scheduling_routes::create_time_slot,
        api::scheduling_routes::list_time_slots,
        api::scheduling_routes::schedule_lesson,
        api::scheduling_routes::list_lessons,
        // Assessment
        api::assessment_routes::create_scheme,
        api::assessment_routes::list_schemes,
        api::assessment_routes::get_scheme,
        api::assessment_routes::record_grade,
        api::assessment_routes::list_grades,
        api::assessment_routes::calculate_grades,
        api::assessment_routes::mark_attendance,
        api::assessment_routes::list_attendance,
        // Exam
        api::exam_routes::create_session,
        api::exam_routes::list_sessions,
        api::exam_routes::get_session,
        api::exam_routes::schedule_exam,
        api::exam_routes::list_schedules,
        api::exam_routes::assign_seating,
        api::exam_routes::assign_secret,
        api::exam_routes::assign_proctor,
        // Finance
        api::finance_routes::create_account,
        api::finance_routes::list_accounts,
        api::finance_routes::create_fee_template,
        api::finance_routes::list_fee_templates,
        api::finance_routes::generate_fee,
        api::finance_routes::list_fees,
        api::finance_routes::process_payment,
        api::finance_routes::list_payments,
        api::finance_routes::list_currencies,
        // Document
        api::document_routes::upload_document,
        api::document_routes::list_documents,
        api::document_routes::get_document,
        api::document_routes::delete_document,
        // Integration
        api::integration_routes::connect_app,
        api::integration_routes::list_integrations,
        api::integration_routes::get_integration_status,
        api::integration_routes::disconnect_app,
        api::integration_routes::toggle_integration,
        // Report & Templates
        api::report_routes::list_templates,
        api::report_routes::create_template,
        api::report_routes::get_template,
        api::report_routes::update_template,
        api::report_routes::delete_template,
        api::report_routes::publish_template,
        api::report_routes::archive_template,
        api::report_routes::get_template_versions,
        api::report_routes::revert_template_version,
        api::report_routes::preview_template,
        api::report_routes::validate_template,
        api::report_routes::list_report_definitions,
        api::report_routes::get_report_definition,
        api::report_routes::list_template_bindings,
        api::report_routes::create_template_binding,
        api::report_routes::generate_report,
        api::report_routes::get_report_job_status,
        api::report_routes::download_report_file,
        api::report_routes::batch_print,
    ),
    components(
        schemas(
            // Shared
            api::OrbisErrorBody,
            // IAM & MFA & Devices
            api::iam_routes::LoginRequest,
            api::iam_routes::LoginResponse,
            api::iam_routes::RegisterUserRequest,
            api::iam_routes::UpdateUserRequest,
            api::iam_routes::UserDto,
            api::iam_routes::SessionResponse,
            api::iam_routes::MfaEnrollRequest,
            api::iam_routes::MfaEnrollResponse,
            api::iam_routes::MfaVerifyEnrollmentRequest,
            api::iam_routes::MfaChallengeRequest,
            api::iam_routes::GoogleAuthRequest,
            api::iam_routes::UserDeviceDto,
            api::iam_routes::UserIdentityDto,
            // Org
            api::org_routes::CreateOrgRequest,
            api::org_routes::UpdateOrgRequest,
            api::org_routes::OrgDto,
            api::org_routes::CreateOrgUnitRequest,
            api::org_routes::OrgUnitDto,
            api::org_routes::OrganizationSettingsDto,
            api::org_routes::AcademicSettingsDto,
            api::org_routes::EnrollmentSettingsDto,
            api::org_routes::FinanceSettingsDto,
            api::org_routes::NotificationSettingsDto,
            api::org_routes::AutopilotSettingsDto,
            api::org_routes::DashboardCountsDto,
            // Calendar
            api::calendar_routes::AcademicYearDto,
            api::calendar_routes::AcademicPeriodDto,
            api::calendar_routes::AcademicContextResponse,
            api::calendar_routes::CreateYearRequest,
            api::calendar_routes::CreatePeriodRequest,
            api::calendar_routes::PeriodTemplateDto,
            // Academic
            api::academic_routes::SubjectDto,
            api::academic_routes::CreateSubjectRequest,
            api::academic_routes::ClassDto,
            api::academic_routes::CreateClassRequest,
            api::academic_routes::ClassSubjectDto,
            api::academic_routes::AssignClassSubjectRequest,
            // Student
            api::student_routes::StudentDto,
            api::student_routes::RegisterStudentRequest,
            api::student_routes::EnrollmentDto,
            api::student_routes::EnrollStudentRequest,
            api::student_routes::ChangeStudyModeRequest,
            api::student_routes::UpdateStudentStatusRequest,
            api::student_routes::GuardianDto,
            api::student_routes::AssignGuardianRequest,
            // Override & Workflow
            api::override_routes::OverrideRequestDto,
            api::override_routes::SubmitOverrideRequest,
            api::override_routes::ResolveOverrideRequest,
            api::override_routes::ApprovalStepDto,
            // Scheduling
            api::scheduling_routes::RoomDto,
            api::scheduling_routes::CreateRoomRequest,
            api::scheduling_routes::TimeSlotDto,
            api::scheduling_routes::CreateTimeSlotRequest,
            api::scheduling_routes::ScheduledLessonDto,
            api::scheduling_routes::ScheduleLessonRequest,
            // Assessment
            api::assessment_routes::AssessmentSchemeDto,
            api::assessment_routes::CreateSchemeRequest,
            api::assessment_routes::GradeDto,
            api::assessment_routes::RecordGradeRequest,
            api::assessment_routes::AttendanceDto,
            api::assessment_routes::MarkAttendanceRequest,
            api::assessment_routes::CalculateGradesRequest,
            api::assessment_routes::CalculationResultDto,
            // Exam
            api::exam_routes::ExamSessionDto,
            api::exam_routes::CreateExamSessionRequest,
            api::exam_routes::ExamScheduleDto,
            api::exam_routes::ScheduleExamRequest,
            api::exam_routes::SeatingDto,
            api::exam_routes::AssignSeatingRequest,
            api::exam_routes::SecretDto,
            api::exam_routes::AssignSecretRequest,
            api::exam_routes::ProctorDto,
            api::exam_routes::AssignProctorRequest,
            // Finance
            api::finance_routes::AccountDto,
            api::finance_routes::CreateAccountRequest,
            api::finance_routes::FeeTemplateDto,
            api::finance_routes::CreateFeeTemplateRequest,
            api::finance_routes::StudentFeeDto,
            api::finance_routes::GenerateFeeRequest,
            api::finance_routes::PaymentDto,
            api::finance_routes::ProcessPaymentRequest,
            api::finance_routes::CurrencyDto,
            // Document
            api::document_routes::DocumentDto,
            api::document_routes::UploadDocumentRequest,
            // Integration
            api::integration_routes::IntegrationDto,
            api::integration_routes::IntegrationStatusDto,
            api::integration_routes::ConnectAppRequest,
            api::integration_routes::ToggleIntegrationRequest,
            // Report & Template
            api::report_routes::TemplateDto,
            api::report_routes::CreateTemplateRequest,
            api::report_routes::UpdateTemplateRequest,
            api::report_routes::TemplateVersionDto,
            api::report_routes::TemplateBindingDto,
            api::report_routes::CreateTemplateBindingRequest,
            api::report_routes::GenerateReportRequest,
            api::report_routes::GenerateReportResponse,
            api::report_routes::ReportJobStatusResponse,
            api::report_routes::BatchPrintRequest,
            api::report_routes::PreviewTemplateRequest,
            api::report_routes::ValidateTemplateRequest,
            api::report_routes::ValidationResultDto,
            api::report_routes::ValidationIssueDto,
            api::report_routes::ReportDefinitionDto,
            api::report_routes::RevertVersionRequest,
        )
    ),
    modifiers(),
    security(
        ("bearer" = [])
    )
)]
pub struct ApiDoc;

// ── App Router ──────────────────────────────────────────────────

pub fn app_router(state: state::AppState) -> Router {
    Router::new()
        .merge(iam_routes::routes())
        .merge(org_routes::routes())
        .merge(calendar_routes::routes())
        .merge(academic_routes::routes())
        .merge(student_routes::routes())
        .merge(override_routes::routes())
        .merge(scheduling_routes::routes())
        .merge(assessment_routes::routes())
        .merge(exam_routes::routes())
        .merge(finance_routes::routes())
        .merge(document_routes::routes())
        .merge(integration_routes::routes())
        .merge(report_routes::routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(state)
}
