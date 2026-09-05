//! Orbis Server — Axum router with full OpenAPI documentation.

pub mod api;
pub mod state;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use api::{
    iam_routes, org_routes, calendar_routes, academic_routes,
    student_routes, scheduling_routes, assessment_routes, exam_routes,
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
        (name = "Scheduling", description = "Rooms, time slots, scheduled lessons"),
        (name = "Assessment", description = "Assessment schemes, grades, attendance"),
        (name = "Exam", description = "Exam sessions, schedules, seating, proctors"),
        (name = "Finance", description = "Accounts, fee templates, student fees, payments"),
        (name = "Document", description = "Document management"),
        (name = "Integration", description = "Third-party app integrations"),
        (name = "Report", description = "Report definitions, templates, generation"),
    ),
    paths(
        // Auth
        api::iam_routes::__path_login,
        api::iam_routes::__path_get_session,
        api::iam_routes::__path_register_user,
        api::iam_routes::__path_list_users,
        api::iam_routes::__path_get_user,
        // Organization
        api::org_routes::__path_create_org,
        api::org_routes::__path_get_current_org,
        api::org_routes::__path_update_current_org,
        api::org_routes::__path_bootstrap_org,
        api::org_routes::__path_get_settings,
        api::org_routes::__path_update_settings,
        api::org_routes::__path_get_dashboard,
        api::org_routes::__path_create_org_unit,
        api::org_routes::__path_list_org_units,
        api::org_routes::__path_get_org_unit,
        api::org_routes::__path_delete_org_unit,
        // Calendar
        api::calendar_routes::__path_create_year,
        api::calendar_routes::__path_list_years,
        api::calendar_routes::__path_get_current_context,
        api::calendar_routes::__path_get_year,
        api::calendar_routes::__path_create_period,
        api::calendar_routes::__path_list_periods,
        api::calendar_routes::__path_get_period,
        api::calendar_routes::__path_list_period_templates,
        // Academic
        api::academic_routes::__path_create_subject,
        api::academic_routes::__path_list_subjects,
        api::academic_routes::__path_get_subject,
        api::academic_routes::__path_create_class,
        api::academic_routes::__path_list_classes,
        api::academic_routes::__path_get_class,
        api::academic_routes::__path_assign_class_subject,
        api::academic_routes::__path_list_class_subjects,
        // Student
        api::student_routes::__path_enroll_student,
        api::student_routes::__path_list_enrollments,
        api::student_routes::__path_get_enrollment,
        api::student_routes::__path_assign_guardian,
        api::student_routes::__path_list_guardians,
        // Scheduling
        api::scheduling_routes::__path_create_room,
        api::scheduling_routes::__path_list_rooms,
        api::scheduling_routes::__path_get_room,
        api::scheduling_routes::__path_create_time_slot,
        api::scheduling_routes::__path_list_time_slots,
        api::scheduling_routes::__path_schedule_lesson,
        api::scheduling_routes::__path_list_lessons,
        // Assessment
        api::assessment_routes::__path_create_scheme,
        api::assessment_routes::__path_list_schemes,
        api::assessment_routes::__path_get_scheme,
        api::assessment_routes::__path_record_grade,
        api::assessment_routes::__path_list_grades,
        api::assessment_routes::__path_calculate_grades,
        api::assessment_routes::__path_mark_attendance,
        api::assessment_routes::__path_list_attendance,
        // Exam
        api::exam_routes::__path_create_session,
        api::exam_routes::__path_list_sessions,
        api::exam_routes::__path_get_session,
        api::exam_routes::__path_schedule_exam,
        api::exam_routes::__path_list_schedules,
        api::exam_routes::__path_assign_seating,
        api::exam_routes::__path_assign_secret,
        api::exam_routes::__path_assign_proctor,
        // Finance
        api::finance_routes::__path_create_account,
        api::finance_routes::__path_list_accounts,
        api::finance_routes::__path_create_fee_template,
        api::finance_routes::__path_list_fee_templates,
        api::finance_routes::__path_generate_fee,
        api::finance_routes::__path_list_fees,
        api::finance_routes::__path_process_payment,
        api::finance_routes::__path_list_payments,
        api::finance_routes::__path_list_currencies,
        // Document
        api::document_routes::__path_upload_document,
        api::document_routes::__path_list_documents,
        api::document_routes::__path_get_document,
        api::document_routes::__path_delete_document,
        // Integration
        api::integration_routes::__path_connect_app,
        api::integration_routes::__path_list_integrations,
    ),
    components(
        schemas(
            // Shared
            api::OrbisErrorBody,
            // IAM
            api::iam_routes::LoginRequest,
            api::iam_routes::LoginResponse,
            api::iam_routes::RegisterUserRequest,
            api::iam_routes::UserDto,
            api::iam_routes::SessionResponse,
            // Org
            api::org_routes::CreateOrgRequest,
            api::org_routes::UpdateOrgRequest,
            api::org_routes::OrgDto,
            api::org_routes::CreateOrgUnitRequest,
            api::org_routes::OrgUnitDto,
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
            api::student_routes::EnrollmentDto,
            api::student_routes::EnrollStudentRequest,
            api::student_routes::GuardianDto,
            api::student_routes::AssignGuardianRequest,
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
            api::integration_routes::ConnectAppRequest,
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
        .merge(scheduling_routes::routes())
        .merge(assessment_routes::routes())
        .merge(exam_routes::routes())
        .merge(finance_routes::routes())
        .merge(document_routes::routes())
        .merge(integration_routes::routes())
        .merge(report_routes::routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state)
}
