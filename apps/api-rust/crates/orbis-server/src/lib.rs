//! Orbis Server — Axum router with full OpenAPI documentation.

pub mod api;
pub mod state;

use axum::{
    Router,
    body::Body,
    extract::{FromRequestParts, State},
    http::Request,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
};
use sea_orm::ConnectionTrait;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use api::{
    academic_framework_routes, academic_routes, ai_routes, assessment_routes, autopilot_routes,
    bootstrap_routes, calendar_routes, document_routes, employee_routes, exam_routes,
    finance_routes, folder_routes, group_routes, iam_routes, integration_routes, mulverse_routes,
    notification_routes, onboarding_routes, org_routes, override_routes, progression_routes,
    report_routes, scheduling_routes, storage_routes, student_routes,
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
        (name = "Storage", description = "Physical file storage, upload, download, presigned URLs, archive"),
        (name = "Folder", description = "Hierarchical folders, student folders, and AI curriculum knowledge base"),
        (name = "Integration", description = "Third-party app integrations"),
        (name = "Notification", description = "Notification center, preferences, and unread counts"),
        (name = "Report", description = "Report definitions, templates, generation"),
        (name = "Autopilot", description = "Autonomous lifecycle, finance & reporting automation engine"),
        (name = "Bootstrap", description = "First-time organization initialization"),
        (name = "Onboarding", description = "Post-bootstrap institution configuration & readiness DAG"),
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
        api::iam_routes::provision_user,
        api::iam_routes::list_users,
        api::iam_routes::get_user,
        api::iam_routes::update_user,
        api::iam_routes::reset_user_password,
        api::iam_routes::list_roles,
        api::iam_routes::save_role,
        api::iam_routes::delete_role,
        api::iam_routes::list_permissions,
        api::iam_routes::save_permissions,
        // Organization & System
        api::org_routes::create_org,
        api::org_routes::get_current_org,
        api::org_routes::update_current_org,
        api::org_routes::get_system_readiness_endpoint,
        api::org_routes::get_system_has_org_endpoint,
        api::org_routes::get_settings,
        api::org_routes::update_settings,
        api::org_routes::get_dashboard,
        api::org_routes::create_org_unit,
        api::org_routes::list_org_units,
        api::org_routes::get_org_unit,
        api::org_routes::update_org_unit,
        api::org_routes::delete_org_unit,
        // Calendar
        api::calendar_routes::create_year,
        api::calendar_routes::list_years,
        api::calendar_routes::get_current_context,
        api::calendar_routes::get_year,
        api::calendar_routes::delete_year,
        api::calendar_routes::update_year,
        api::calendar_routes::activate_year,
        api::calendar_routes::close_year_handler,
        api::calendar_routes::archive_year_handler,
        api::calendar_routes::create_period,
        api::calendar_routes::list_periods,
        api::calendar_routes::get_period,
        api::calendar_routes::update_period,
        api::calendar_routes::delete_period_handler,
        api::calendar_routes::lock_period,
        api::calendar_routes::unlock_period,
        api::calendar_routes::close_period,
        api::calendar_routes::list_period_types,
        // Configurable Academic Framework
        api::academic_framework_routes::list_calendars,
        api::academic_framework_routes::create_calendar,
        api::academic_framework_routes::list_period_types,
        api::academic_framework_routes::create_period_type,
        api::academic_framework_routes::list_calendar_periods,
        api::academic_framework_routes::create_calendar_period,
        api::academic_framework_routes::list_courses,
        api::academic_framework_routes::create_course,
        api::academic_framework_routes::list_section_subjects,
        api::academic_framework_routes::create_section_subject,
        api::academic_framework_routes::list_enrollments,
        api::academic_framework_routes::enroll_student,
        // Academic
        api::academic_routes::create_subject,
        api::academic_routes::update_subject,
        api::academic_routes::list_subjects,
        api::academic_routes::get_subject,
        api::academic_routes::create_section,
        api::academic_routes::update_section,
        api::academic_routes::list_sections,
        api::academic_routes::get_section,
        api::academic_routes::assign_section_subject,
        api::academic_routes::get_section_subject,
        api::academic_routes::update_section_subject,
        api::academic_routes::list_section_subjects,
        // Student
        api::student_routes::register_student,
        api::student_routes::check_duplicate,
        api::student_routes::list_students,
        api::student_routes::get_my_student,
        api::student_routes::get_student,
        api::student_routes::update_student_status,
        api::student_routes::update_student_profile,
        api::student_routes::delete_student,
        api::student_routes::restore_student,
        api::student_routes::bulk_register_students,
        api::student_routes::enroll_student,
        api::student_routes::list_enrollments,
        api::student_routes::get_enrollment,
        api::student_routes::change_study_mode,
        api::student_routes::transfer_class,
        api::student_routes::assign_guardian,
        api::student_routes::list_guardians,
        api::student_routes::remove_guardian,
        // Employee Affairs
        api::employee_routes::register_employee,
        api::employee_routes::bulk_register_employees,
        api::employee_routes::list_employees,
        api::employee_routes::get_employee,
        api::employee_routes::get_employee_stats,
        api::employee_routes::update_employee,
        api::employee_routes::update_employee_status,
        api::employee_routes::delete_employee,
        api::employee_routes::restore_employee,
        api::employee_routes::provision_account,
        api::employee_routes::assign_groups,
        // User Groups
        api::group_routes::create_group,
        api::group_routes::list_groups,
        api::group_routes::get_group,
        api::group_routes::update_group,
        api::group_routes::delete_group,
        api::group_routes::get_group_members,
        api::group_routes::add_group_member,
        api::group_routes::remove_group_member,
        api::group_routes::bulk_assign_group,
        api::group_routes::set_group_permissions,
        // Override & Workflow
        api::override_routes::submit_override,
        api::override_routes::list_overrides,
        api::override_routes::get_override,
        api::override_routes::approve_override,
        api::override_routes::reject_override,
        // Progression
        api::progression_routes::evaluate_cohort,
        api::progression_routes::promote_cohort,
        api::progression_routes::list_jobs,
        api::progression_routes::get_job,
        // Scheduling
        api::scheduling_routes::create_room,
        api::scheduling_routes::list_rooms,
        api::scheduling_routes::get_room,
        api::scheduling_routes::update_room,
        api::scheduling_routes::delete_room,
        api::scheduling_routes::create_time_slot,
        api::scheduling_routes::batch_create_time_slots,
        api::scheduling_routes::list_time_slots,
        api::scheduling_routes::get_time_slot,
        api::scheduling_routes::update_time_slot,
        api::scheduling_routes::delete_time_slot,
        api::scheduling_routes::schedule_lesson,
        api::scheduling_routes::bulk_schedule_lessons,
        api::scheduling_routes::list_lessons,
        api::scheduling_routes::delete_lesson,
        // Assessment
        api::assessment_routes::create_scheme,
        api::assessment_routes::list_schemes,
        api::assessment_routes::get_scheme,
        api::assessment_routes::update_scheme,
        api::assessment_routes::delete_scheme,
        api::assessment_routes::get_current_academic_policy,
        api::assessment_routes::preview_academic_result,
        api::assessment_routes::record_grade,
        api::assessment_routes::batch_record_grades,
        api::assessment_routes::delete_grade,
        api::assessment_routes::list_grades,
        api::assessment_routes::calculate_grades,
        api::assessment_routes::calculate_gpa,
        api::assessment_routes::calculate_rankings,
        api::assessment_routes::mark_attendance,
        api::assessment_routes::batch_mark_attendance,
        api::assessment_routes::delete_attendance,
        api::assessment_routes::list_attendance,
        api::assessment_routes::list_student_attendance,
        api::assessment_routes::record_behavior,
        api::assessment_routes::delete_behavior,
        api::assessment_routes::list_all_behavior,
        api::assessment_routes::list_student_behavior,
        // Exam
        api::exam_routes::create_session,
        api::exam_routes::list_sessions,
        api::exam_routes::get_session,
        api::exam_routes::update_session,
        api::exam_routes::delete_session,
        api::exam_routes::schedule_exam,
        api::exam_routes::list_schedules,
        api::exam_routes::delete_schedule,
        api::exam_routes::assign_seating,
        api::exam_routes::list_seating,
        api::exam_routes::delete_seating,
        api::exam_routes::generate_seating,
        api::exam_routes::assign_secret,
        api::exam_routes::list_secrets,
        api::exam_routes::delete_secret,
        api::exam_routes::generate_secrets,
        api::exam_routes::assign_proctor,
        api::exam_routes::list_proctors,
        api::exam_routes::delete_proctor,
        // Finance
        api::finance_routes::create_account,
        api::finance_routes::list_accounts,
        api::finance_routes::get_account,
        api::finance_routes::update_account,
        api::finance_routes::delete_account,
        api::finance_routes::create_fee_template,
        api::finance_routes::list_fee_templates,
        api::finance_routes::get_fee_template,
        api::finance_routes::update_fee_template,
        api::finance_routes::delete_fee_template,
        api::finance_routes::create_discount_template,
        api::finance_routes::list_discount_templates,
        api::finance_routes::delete_discount_template,
        api::finance_routes::generate_fee,
        api::finance_routes::list_fees,
        api::finance_routes::delete_student_fee,
        api::finance_routes::process_payment,
        api::finance_routes::list_payments,
        api::finance_routes::delete_payment,
        api::finance_routes::list_currencies,
        api::finance_routes::list_journal_entries,
        api::finance_routes::get_journal_entry,
        api::finance_routes::create_journal_entry,
        api::finance_routes::reverse_journal_entry,
        api::finance_routes::delete_journal_entry,
        api::finance_routes::split_fee,
        api::finance_routes::refund_fee,
        // Document
        api::document_routes::upload_document,
        api::document_routes::list_documents,
        api::document_routes::get_document,
        api::document_routes::update_document,
        api::document_routes::delete_document,
        api::document_routes::create_document_requirement,
        api::document_routes::list_document_requirements,
        api::document_routes::get_document_requirement,
        api::document_routes::update_document_requirement,
        api::document_routes::delete_document_requirement,
        api::document_routes::evaluate_student_requirements,
        // Storage & File Upload
        api::storage_routes::upload_file,
        api::storage_routes::download_file,
        api::storage_routes::get_access_url,
        api::storage_routes::archive_document,
        api::storage_routes::restore_document,
        api::storage_routes::trash_document,
        api::storage_routes::purge_document,
        api::storage_routes::list_archived_documents,
        api::storage_routes::list_trash_documents,
        // Folder & Hierarchy & AI Curriculum
        api::folder_routes::create_folder,
        api::folder_routes::list_folders,
        api::folder_routes::get_folder,
        api::folder_routes::get_folder_contents,
        api::folder_routes::get_or_create_student_folder,
        api::folder_routes::list_curriculum_folders,
        api::folder_routes::get_or_create_curriculum_folder,
        api::folder_routes::update_folder,
        api::folder_routes::delete_folder,
        // Integration
        api::integration_routes::connect_app,
        api::integration_routes::list_integrations,
        api::integration_routes::list_notification_integrations,
        api::integration_routes::get_integration_status,
        api::integration_routes::disconnect_app,
        api::integration_routes::toggle_integration,
        // Notification
        api::notification_routes::list_notifications,
        api::notification_routes::get_unread_count,
        api::notification_routes::mark_read,
        api::notification_routes::mark_all_read,
        api::notification_routes::dismiss_notification,
        api::notification_routes::get_preferences,
        api::notification_routes::update_preferences,
        api::notification_routes::notification_stream,
        api::mulverse_routes::get_status,
        api::mulverse_routes::connect,
        api::mulverse_routes::disconnect,
        api::mulverse_routes::list_deployments,
        api::mulverse_routes::create_deployment,
        api::mulverse_routes::update_deployment,
        api::mulverse_routes::list_endpoints,
        api::mulverse_routes::create_endpoint,
        api::mulverse_routes::update_endpoint,
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
        api::report_routes::create_custom_report,
        api::report_routes::get_report_job_status,
        api::report_routes::cancel_report_job,
        api::report_routes::download_report_file,
        api::report_routes::batch_print,
        api::report_routes::get_provider_schema,
        api::report_routes::list_audit_logs,
        // AI Platform
        api::ai_routes::chat,
        api::ai_routes::list_tools,
        // Autopilot
        api::autopilot_routes::list_tasks,
        api::autopilot_routes::run_tick,
        api::autopilot_routes::get_task,
        api::autopilot_routes::execute_task,
        api::autopilot_routes::retry_task,
        api::autopilot_routes::dismiss_task,
        api::autopilot_routes::get_diagnostics,
        // Bootstrap
        api::bootstrap_routes::get_bootstrap_status,
        api::bootstrap_routes::execute_bootstrap,
        // Onboarding
        api::onboarding_routes::get_onboarding_state,
        api::onboarding_routes::provision_blueprint_defaults,
    ),
    components(
        schemas(
            // Shared
            api::OrbisErrorBody,
            // IAM & MFA & Devices
            api::iam_routes::LoginRequest,
            api::iam_routes::LoginResponse,
            api::iam_routes::RegisterUserRequest,
            api::iam_routes::ProvisionUserRequest,
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
            api::iam_routes::RoleDto,
            api::iam_routes::SaveRoleRequest,
            api::iam_routes::PermissionRuleDto,
            api::iam_routes::EffectiveContextDto,
            api::iam_routes::LinkedDependentDto,
            // Org
            api::org_routes::CreateOrgRequest,
            api::org_routes::UpdateOrgRequest,
            api::org_routes::OrgDto,
            api::org_routes::CreateOrgUnitRequest,
            api::org_routes::UpdateOrgUnitRequest,
            api::org_routes::OrgUnitDto,
            api::org_routes::OrganizationSettingsDto,
            api::org_routes::AiSettingsDto,
            api::org_routes::AcademicSettingsDto,
            api::org_routes::EnrollmentSettingsDto,
            api::org_routes::AdmissionConfigurationDto,
            api::org_routes::NumberingSchemaDto,
            api::org_routes::AdmissionPolicyDto,
            api::org_routes::FormFieldConfigDto,
            api::org_routes::FinanceSettingsDto,
            api::org_routes::NotificationSettingsDto,
            api::org_routes::AutopilotSettingsDto,
            api::org_routes::DashboardCountsDto,
            // Calendar
            api::calendar_routes::AcademicCalendarDto,
            api::calendar_routes::AcademicPeriodDto,
            orbis_domain::entities::academic_period::PeriodKind,
            orbis_domain::entities::academic_period::PeriodType,
            orbis_domain::entities::academic_period::AcademicPeriodState,
            api::calendar_routes::AcademicContextResponse,
            api::calendar_routes::CreateYearRequest,
            api::calendar_routes::UpdateYearRequest,
            api::calendar_routes::CreatePeriodRequest,
            api::calendar_routes::UpdatePeriodRequest,
            api::calendar_routes::PeriodTypeDto,
            api::academic_framework_routes::AcademicCalendarDto,
            api::academic_framework_routes::PeriodTypeDto,
            api::academic_framework_routes::AcademicPeriodDto,
            api::academic_framework_routes::CourseDto,
            api::academic_framework_routes::SectionSubjectDto,
            api::academic_framework_routes::EnrollmentDto,
            api::academic_framework_routes::CreateAcademicCalendarRequest,
            api::academic_framework_routes::CreatePeriodTypeRequest,
            api::academic_framework_routes::CreateAcademicPeriodRequest,
            api::academic_framework_routes::CreateCourseRequest,
            api::academic_framework_routes::CreateSectionSubjectRequest,
            api::academic_framework_routes::CreateEnrollmentRequest,
            // Academic
            api::academic_routes::SubjectDto,
            api::academic_routes::CreateSubjectRequest,
            api::academic_routes::UpdateSubjectRequest,
            api::academic_routes::SectionDto,
            api::academic_routes::CreateSectionRequest,
            api::academic_routes::UpdateSectionRequest,
            api::academic_routes::SectionSubjectDto,
            api::academic_routes::AssignSectionSubjectRequest,
            api::academic_routes::UpdateSectionSubjectRequest,
            // Student
            api::student_routes::StudentDto,
            api::student_routes::RegisterStudentRequest,
            api::student_routes::UpdateStudentProfileRequest,
            api::student_routes::DuplicateCheckRequest,
            api::student_routes::DuplicateMatchDto,
            api::student_routes::DuplicateCheckResponse,
            api::student_routes::EnrollmentDto,
            api::student_routes::EnrollStudentRequest,
            api::student_routes::ChangeStudyModeRequest,
            api::student_routes::TransferClassRequest,
            api::student_routes::UpdateStudentStatusRequest,
            api::student_routes::GuardianDto,
            api::student_routes::AssignGuardianRequest,
            api::student_routes::BulkRegisterStudentItem,
            api::student_routes::BulkRegisterStudentsRequest,
            api::student_routes::BulkRegisterItemResultDto,
            api::student_routes::BulkRegisterStudentsResponse,
            // Override & Workflow
            api::override_routes::OverrideRequestDto,
            api::override_routes::SubmitOverrideRequest,
            api::override_routes::ResolveOverrideRequest,
            api::override_routes::ApprovalStepDto,
            // Scheduling
            api::scheduling_routes::RoomDto,
            api::scheduling_routes::CreateRoomRequest,
            api::scheduling_routes::UpdateRoomRequest,
            api::scheduling_routes::TimeSlotDto,
            api::scheduling_routes::CreateTimeSlotRequest,
            api::scheduling_routes::CreateTimeSlotItemRequest,
            api::scheduling_routes::BatchCreateTimeSlotsRequest,
            api::scheduling_routes::UpdateTimeSlotRequest,
            api::scheduling_routes::ScheduledLessonDto,
            api::scheduling_routes::ScheduleLessonRequest,
            api::scheduling_routes::BulkScheduleLessonsRequest,
            // Assessment
            api::assessment_routes::AssessmentSchemeDto,
            orbis_domain::entities::assessment_scheme::AssessmentLevel,
            orbis_domain::entities::assessment_scheme::AssessmentComponent,
            orbis_domain::entities::assessment_scheme::InstitutionScope,
            api::assessment_routes::AcademicPolicyDto,
            api::assessment_routes::AcademicResultViewDto,
            api::assessment_routes::AcademicResultPreviewRequest,
            api::assessment_routes::CreateSchemeRequest,
            api::assessment_routes::UpdateSchemeRequest,
            api::assessment_routes::GradeDto,
            api::assessment_routes::RecordGradeRequest,
            api::assessment_routes::RecordGradeItemRequest,
            api::assessment_routes::BatchRecordGradesRequest,
            api::assessment_routes::BatchRecordGradesResponseDto,
            api::assessment_routes::CalculateGpaRequest,
            api::assessment_routes::CumulativeGpaRecordDto,
            api::assessment_routes::AttendanceDto,
            api::assessment_routes::MarkAttendanceRequest,
            api::assessment_routes::MarkAttendanceItemRequest,
            api::assessment_routes::BatchMarkAttendanceRequest,
            api::assessment_routes::BatchMarkAttendanceResponseDto,
            api::assessment_routes::CalculateGradesRequest,
            api::assessment_routes::CalculationResultDto,
            api::assessment_routes::CalculateRankingsRequest,
            api::assessment_routes::ScoreItemRequest,
            api::assessment_routes::RankingItemDto,
            api::assessment_routes::BehaviorRecordDto,
            api::assessment_routes::RecordBehaviorRequest,
            // Exam
            api::exam_routes::ExamSessionDto,
            api::exam_routes::CreateExamSessionRequest,
            api::exam_routes::UpdateExamSessionRequest,
            api::exam_routes::ExamScheduleDto,
            api::exam_routes::ScheduleExamRequest,
            api::exam_routes::SeatingDto,
            api::exam_routes::AssignSeatingRequest,
            api::exam_routes::GenerateSeatingRequest,
            api::exam_routes::SecretDto,
            api::exam_routes::AssignSecretRequest,
            api::exam_routes::GenerateSecretsRequest,
            api::exam_routes::ProctorDto,
            api::exam_routes::AssignProctorRequest,
            // Finance
            api::finance_routes::AccountDto,
            api::finance_routes::CreateAccountRequest,
            api::finance_routes::UpdateAccountRequest,
            api::finance_routes::FeeTemplateDto,
            api::finance_routes::CreateFeeTemplateRequest,
            api::finance_routes::UpdateFeeTemplateRequest,
            api::finance_routes::DiscountTemplateDto,
            api::finance_routes::CreateDiscountTemplateRequest,
            api::finance_routes::StudentFeeDto,
            api::finance_routes::GenerateFeeRequest,
            api::finance_routes::PaymentDto,
            api::finance_routes::ProcessPaymentRequest,
            api::finance_routes::CurrencyDto,
            api::finance_routes::JournalEntryDto,
            api::finance_routes::JournalEntryLineDto,
            api::finance_routes::CreateJournalEntryRequest,
            api::finance_routes::CreateJournalEntryLineRequest,
            api::finance_routes::ReverseJournalRequest,
            api::finance_routes::SplitFeeRequest,
            api::finance_routes::RefundStudentFeeRequest,
            // Document
            api::document_routes::DocumentDto,
            api::document_routes::UploadDocumentRequest,
            api::document_routes::UpdateDocumentRequest,
            api::document_routes::DocumentRequirementDto,
            api::document_routes::CreateDocumentRequirementRequest,
            api::document_routes::UpdateDocumentRequirementRequest,
            api::document_routes::StudentDocumentFulfillmentDto,
            // Storage
            api::storage_routes::PresignedUrlResponseDto,
            api::storage_routes::PresignRequest,
            api::storage_routes::ArchiveRequest,
            // Folder
            api::folder_routes::FolderDto,
            api::folder_routes::FolderContentsDto,
            api::folder_routes::CreateFolderRequest,
            api::folder_routes::UpdateFolderRequest,
            api::folder_routes::CreateCurriculumFolderRequest,
            // Integration
            api::integration_routes::IntegrationDto,
            api::integration_routes::IntegrationStatusDto,
            api::integration_routes::ConnectAppRequest,
            api::integration_routes::ToggleIntegrationRequest,
            api::integration_routes::NotificationIntegrationQuery,
            api::integration_routes::NotificationIntegrationCheckDto,
            // Notification
            api::notification_routes::NotificationDto,
            api::notification_routes::UnreadCountDto,
            api::notification_routes::MarkAllReadDto,
            api::notification_routes::ListNotificationsQuery,
            api::notification_routes::NotificationPreferenceDto,
            api::notification_routes::NotificationPreferenceResponseDto,
            api::notification_routes::UpdatePreferencesRequest,
            api::mulverse_routes::MulverseConnectionDto,
            api::mulverse_routes::ConnectMulverseRequest,
            api::mulverse_routes::DeploymentDto,
            api::mulverse_routes::EndpointDto,
            api::mulverse_routes::CreateDeploymentRequest,
            api::mulverse_routes::UpdateDeploymentRequest,
            api::mulverse_routes::CreateEndpointRequest,
            api::mulverse_routes::UpdateEndpointRequest,
            // Report & Template
            api::report_routes::TemplateDto,
            api::report_routes::CreateTemplateRequest,
            api::report_routes::UpdateTemplateRequest,
            api::report_routes::TemplateVersionDto,
            api::report_routes::TemplateBindingDto,
            api::report_routes::CreateTemplateBindingRequest,
            api::report_routes::GenerateReportRequest,
            api::report_routes::GenerateReportResponse,
            orbis_domain::entities::ai::AiCustomReportRequest,
            orbis_domain::entities::ai::AiCustomReportResponse,
            api::report_routes::ReportJobStatusResponse,
            api::report_routes::CancelReportResponse,
            api::report_routes::BatchPrintRequest,
            api::report_routes::PreviewTemplateRequest,
            api::report_routes::ValidateTemplateRequest,
            api::report_routes::ValidationResultDto,
            api::report_routes::ValidationIssueDto,
            api::report_routes::ReportDefinitionDto,
            api::report_routes::RevertVersionRequest,
            api::report_routes::ProviderSchemaResponse,
            api::report_routes::ProviderSchemaField,
            api::report_routes::AuditLogDto,
            // AI Platform
            orbis_domain::entities::ai::AiToolCall,
            orbis_domain::entities::ai::AiToolResult,
            orbis_domain::entities::ai::AiMessage,
            orbis_domain::entities::ai::ToolCategory,
            orbis_domain::entities::ai::TeachingAssignmentInfo,
            orbis_domain::entities::ai::UserAiContext,
            orbis_domain::entities::ai::AcademicAiContext,
            orbis_domain::entities::ai::UiContext,
            orbis_domain::entities::ai::AiToolDefinition,
            orbis_domain::entities::ai::AiProposedAction,
            orbis_domain::entities::ai::AiChatRequest,
            orbis_domain::entities::ai::AiChatResponse,
            orbis_domain::entities::ai::AiStoredMessage,
            orbis_domain::entities::ai::AiStoredSession,
            // Autopilot
            api::autopilot_routes::AutopilotTaskDto,
            api::autopilot_routes::AutopilotDiagnosticsDto,
            api::autopilot_routes::RunTickRequest,
            api::autopilot_routes::ListTasksResponse,
            // Progression
            api::progression_routes::ProgressionJobDto,
            api::progression_routes::StudentProgressionItemInput,
            api::progression_routes::PromoteCohortRequest,
            api::progression_routes::EvaluateCohortRequest,
            // Employee Affairs
            api::employee_routes::EmployeeDetailDto,
            api::employee_routes::AccountProvisioningResultDto,
            api::employee_routes::AccountProvisioningConfigRequest,
            api::employee_routes::RegisterEmployeeResponseDto,
            api::employee_routes::RegisterEmployeeRequest,
            api::employee_routes::BulkRegisterEmployeeItemRequest,
            api::employee_routes::BulkRegisterEmployeesRequest,
            api::employee_routes::BulkRegisterEmployeeItemResultDto,
            api::employee_routes::BulkRegisterEmployeesResponseDto,
            api::employee_routes::UpdateEmployeeRequest,
            api::employee_routes::UpdateEmployeeStatusRequest,
            api::employee_routes::ProvisionEmployeeAccountRequest,
            api::employee_routes::AssignEmployeeGroupsRequest,
            api::employee_routes::PaginatedEmployeesDto,
            // User Groups
            api::group_routes::GroupDetailDto,
            api::group_routes::GroupMemberInfoDto,
            api::group_routes::CreateGroupRequest,
            api::group_routes::UpdateGroupRequest,
            api::group_routes::AddGroupMemberRequest,
            api::group_routes::BulkAssignGroupRequest,
            api::group_routes::SetGroupPermissionsRequest,
            // Bootstrap
            api::bootstrap_routes::BootstrapRequest,
            api::bootstrap_routes::BootstrapResultDto,
            api::bootstrap_routes::BootstrapStatusDto,
            // Onboarding
            api::onboarding_routes::SetupTaskDto,
            api::onboarding_routes::CategoryProgressDto,
            api::onboarding_routes::OnboardingStateDto,
            api::onboarding_routes::ProvisionDefaultsReqDto,
            api::onboarding_routes::ProvisionDefaultsResultDto,
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
        .route("/api/v1/health", get(api_health))
        .merge(iam_routes::routes())
        .merge(org_routes::routes())
        .merge(calendar_routes::routes())
        .merge(academic_framework_routes::routes())
        .merge(academic_routes::routes())
        .merge(student_routes::routes())
        .merge(employee_routes::routes())
        .merge(group_routes::routes())
        .merge(override_routes::routes())
        .merge(progression_routes::routes())
        .merge(scheduling_routes::routes())
        .merge(assessment_routes::routes())
        .merge(exam_routes::routes())
        .merge(finance_routes::routes())
        .merge(document_routes::routes())
        .merge(storage_routes::routes())
        .merge(folder_routes::folder_routes())
        .merge(integration_routes::routes())
        .merge(mulverse_routes::routes())
        .merge(report_routes::routes())
        .merge(ai_routes::routes())
        .merge(autopilot_routes::routes())
        .merge(bootstrap_routes::routes())
        .merge(notification_routes::routes())
        .merge(onboarding_routes::routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            enforce_api_license,
        ))
        .with_state(state)
}

async fn api_health(State(state): State<state::AppState>) -> Response {
    let database_connected = state.db.execute_unprepared("SELECT 1").await.is_ok();
    let status = if database_connected {
        axum::http::StatusCode::OK
    } else {
        axum::http::StatusCode::SERVICE_UNAVAILABLE
    };
    (
        status,
        axum::Json(serde_json::json!({
            "status": if database_connected { "ok" } else { "degraded" },
            "version": env!("CARGO_PKG_VERSION"),
            "api_version": "1.0.0",
            "database_connected": database_connected,
        })),
    )
        .into_response()
}

async fn enforce_api_license(
    State(state): State<state::AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let path = request.uri().path();
    if path.starts_with("/api/v1/auth/")
        || path.starts_with("/api/v1/bootstrap")
        || path == "/api/v1/system/readiness"
        || path == "/api/v1/system/has-organization"
        || path == "/api/v1/health"
        || path.starts_with("/swagger-ui")
        || path.starts_with("/api-docs")
    {
        return next.run(request).await;
    }

    let feature = required_feature_for_path(path);
    let (mut parts, body) = request.into_parts();
    let auth = match api::extractors::AuthContext::from_request_parts(&mut parts, &state).await {
        Ok(auth) => auth,
        Err(error) => return error.into_response(),
    };

    if let Err(error) = state.license_runtime.authorize(auth.tenant_id, feature) {
        return api::ApiError::from(error).into_response();
    }

    next.run(Request::from_parts(parts, body)).await
}

fn required_feature_for_path(path: &str) -> Option<&'static str> {
    if path.starts_with("/api/v1/students") {
        Some("students")
    } else if path.starts_with("/api/v1/ai") {
        Some("ai_grading")
    } else if path.starts_with("/api/v1/exam") {
        Some("exam_tools")
    } else if path.starts_with("/api/v1/org-units") {
        Some("multi_branch")
    } else if path.starts_with("/api/v1/reports") || path.starts_with("/api/v1/analytics") {
        Some("campus_analytics")
    } else if path.starts_with("/api/v1/parent") {
        Some("parent_portal")
    } else if path.starts_with("/api/v1/sso") {
        Some("sso")
    } else if path.contains("offline-sync") {
        Some("offline_sync")
    } else {
        None
    }
}
