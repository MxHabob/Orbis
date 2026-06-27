use std::net::SocketAddr;
use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::Json,
    http::{StatusCode, Method, HeaderValue},
    response::IntoResponse,
};
use serde_json::json;
use tower_http::cors::CorsLayer;
use tracing::{info, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use core_domain::{config, database, entities, repository, service};
mod handler;
mod middleware;

use utoipa::OpenApi;

use config::Config;
use repository::Repository;
use service::Service;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth
        handler::auth::post_register,
        handler::auth::post_login,
        handler::auth::get_me,
        handler::auth::post_logout,
        // Organizations
        handler::organization::get_organization,
        handler::organization::put_organization,
        handler::organization::get_organization_stats,
        // Users
        handler::user::get_users,
        handler::user::post_users,
        handler::user::get_user,
        handler::user::put_user,
        handler::user::delete_user,
        handler::user::post_user_role,
        handler::user::post_user_password,
        // Academic Cycles
        handler::academic_cycle::get_cycles,
        handler::academic_cycle::post_cycles,
        handler::academic_cycle::get_cycle,
        handler::academic_cycle::put_cycle,
        handler::academic_cycle::post_activate_cycle,
        handler::academic_cycle::post_lock_cycle,
        handler::academic_cycle::post_unlock_cycle,
        handler::academic_cycle::post_open_grade_entry,
        handler::academic_cycle::post_close_grade_entry,
        // Org Units
        handler::org_unit::get_units,
        handler::org_unit::post_units,
        handler::org_unit::put_unit,
        handler::org_unit::delete_unit,
        // Classes
        handler::class::get_classes,
        handler::class::post_classes,
        handler::class::put_class,
        handler::class::delete_class,
        handler::class::get_class_students,
        // Enrollments
        handler::enrollment::get_enrollments,
        handler::enrollment::post_enrollments,
        handler::enrollment::get_enrollment,
        handler::enrollment::post_withdraw,
        handler::enrollment::post_transfer,
        handler::enrollment::post_promote,
        // Grades
        handler::grade::get_grades,
        handler::grade::post_grades,
        handler::grade::post_grades_bulk,
        handler::grade::post_grades_bulk_subject,
        handler::grade::put_grade,
        handler::grade::get_report_card,
        handler::grade::get_grade_analytics,
        handler::grade::get_grade_matrix,
        // Attendance
        handler::attendance::get_attendance,
        handler::attendance::post_attendance,
        handler::attendance::post_attendance_bulk,
        handler::attendance::get_attendance_report,
        handler::attendance::get_attendance_analytics,
        // Behavior
        handler::behavior::get_behavior,
        handler::behavior::post_ai_behavior,
        handler::behavior::put_behavior,
        handler::behavior::delete_behavior,
        // Notifications
        handler::notification::get_notifications,
        handler::notification::post_mark_read,
        // Dashboard
        handler::dashboard::get_dashboard_stats,
        // Audit
        handler::audit::get_audit_logs,
        // Import/Export
        handler::import_export::post_import_students,
        handler::import_export::post_import_grades,
        handler::import_export::post_import_attendance,
        handler::import_export::get_export_students,
        handler::import_export::get_export_grades,
        handler::import_export::get_export_attendance,
        handler::import_export::get_export_report_cards,
        // Finance
        handler::finance::get_ledger_accounts,
        handler::finance::post_ledger_accounts,
        handler::finance::get_treasuries,
        handler::finance::post_treasuries,
        handler::finance::get_student_fees,
        handler::finance::post_student_fees,
        handler::finance::get_payments,
        handler::finance::post_payments,
        // Subjects
        handler::subject::get_subjects,
        handler::subject::get_subject,
        handler::subject::post_subjects,
        handler::subject::put_subject,
        handler::subject::delete_subject,
        // Schedule
        handler::schedule::get_time_slots,
        handler::schedule::post_time_slots,
        handler::schedule::put_time_slot,
        handler::schedule::delete_time_slot,
        handler::schedule::get_class_subjects,
        handler::schedule::post_class_subjects,
        handler::schedule::put_class_subject,
        handler::schedule::delete_class_subject,
        handler::schedule::get_lessons,
        handler::schedule::get_lesson,
        handler::schedule::post_lessons,
        handler::schedule::put_lesson,
        handler::schedule::delete_lesson,
        handler::schedule::post_check_conflicts,
        // Schedule Solver
        handler::schedule_solve::post_schedule_solve,
        handler::schedule_solve::get_schedule_solve_jobs,
        handler::schedule_solve::get_schedule_solve_job,
        handler::schedule_solve::delete_schedule_solve_job,
        handler::schedule_solve::post_apply_schedule_solve,
        handler::room::get_rooms,
        handler::room::post_rooms,
        handler::room::put_room,
        handler::room::delete_room,
        // Assessments
        handler::assessment::get_assessments,
        handler::assessment::get_assessment,
        handler::assessment::post_assessments,
        handler::assessment::put_assessment,
        handler::assessment::delete_assessment,
        // Guardians
        handler::guardian::get_guardians,
        handler::guardian::post_guardians,
        handler::guardian::put_guardian,
        handler::guardian::delete_guardian,
        // Documents
        handler::document::get_documents,
        handler::document::get_document,
        handler::document::post_documents,
        handler::document::put_document,
        handler::document::delete_document,
        // Examination Management
        handler::exam::get_exam_sessions,
        handler::exam::post_exam_sessions,
        handler::exam::get_exam_session,
        handler::exam::put_exam_session,
        handler::exam::post_activate_exam_session,
        handler::exam::post_complete_exam_session,
        handler::exam::get_exam_subjects,
        handler::exam::post_exam_subjects,
        handler::exam::get_exam_schedule,
        handler::exam::post_exam_schedule,
        handler::exam::get_student_ordering,
        handler::exam::post_rebuild_ordering,
        handler::exam::post_freeze_ordering,
        handler::exam::post_generate_seating_numbers,
        handler::exam::get_seating_numbers,
        handler::exam::post_generate_secret_numbers,
        handler::exam::post_reveal_secret_numbers,
        handler::exam::get_exam_halls,
        handler::exam::post_exam_halls,
        handler::exam::post_allocate_halls,
        handler::exam::get_hall_students,
        handler::exam::post_assign_proctors,
        handler::exam::get_exam_proctors,
    ),
    components(
        schemas(
            handler::auth::RegisterRequest,
            handler::auth::LoginRequest,
            service::auth::LoginResponse,
            handler::class::CreateClassRequest,
            service::class::UpdateClassRequest,
            service::behavior::AIBehaviorSubmission,
            service::behavior::UpdateBehaviorRequest,
            service::organization::UpdateOrganizationRequest,
            service::user::UpdateUserRequest,
            service::user::ChangeRoleRequest,
            service::user::ChangePasswordRequest,
            service::user::CreateUserRequest,
            service::academic_cycle::CreateCycleRequest,
            service::academic_cycle::UpdateCycleRequest,
            service::org_unit::CreateOrgUnitRequest,
            service::org_unit::UpdateOrgUnitRequest,
            service::enrollment::CreateEnrollmentRequest,
            service::enrollment::WithdrawRequest,
            service::enrollment::TransferRequest,
            service::enrollment::PromoteRequest,
            service::enrollment::PromotionResult,
            service::enrollment::PromotionDetail,
            service::grade::CreateGradeRequest,
            service::grade::BulkGradeRequest,
            service::grade::UpdateGradeRequest,
            service::grade::BulkSubjectGradeEntry,
            service::grade::StudentScore,
            service::grade::BulkSubjectGradeResult,
            service::grade::GradeMatrixResponse,
            service::grade::StudentGradeRow,
            service::grade::GradeCell,
            service::grade::GradeReportCard,
            service::grade::SubjectGradeSummary,
            service::grade::GradeAnalytics,
            service::grade::GradeDistribution,
            service::attendance_svc::CreateAttendanceRequest,
            service::attendance_svc::BulkAttendanceRequest,
            service::attendance_svc::BulkAttendanceRecord,
            service::attendance_svc::AttendanceReport,
            service::attendance_svc::AttendanceAnalytics,
            service::dashboard::DashboardStats,
            service::dashboard::BehaviorSummary,
            service::dashboard::AttendanceOverview,
            service::import_export::ImportResult,
            handler::import_export::CsvImportBody,
            // Finance Request Schemas
            service::finance::CreateLedgerAccountRequest,
            service::finance::CreateTreasuryRequest,
            service::finance::AssignStudentFeeRequest,
            service::finance::RecordPaymentRequest,
            service::subject::CreateSubjectRequest,
            service::subject::UpdateSubjectRequest,
            service::schedule::CreateTimeSlotRequest,
            service::schedule::UpdateTimeSlotRequest,
            service::schedule::CreateClassSubjectRequest,
            service::schedule::UpdateClassSubjectRequest,
            service::schedule::CreateLessonRequest,
            service::schedule::UpdateLessonRequest,
            service::schedule::ScheduleConflict,
            handler::schedule::ConflictCheckRequest,
            service::schedule_solver::StartSolveRequest,
            service::schedule_solver::SolveJobResponse,
            infra_scheduler::SolveMode,
            service::room::CreateRoomRequest,
            service::room::UpdateRoomRequest,
            entities::room::Model,
            entities::solve_job::Model,
            service::assessment::CreateAssessmentRequest,
            service::assessment::UpdateAssessmentRequest,
            service::guardian::CreateGuardianLinkRequest,
            service::guardian::UpdateGuardianLinkRequest,
            service::document::CreateDocumentRequest,
            service::document::UpdateDocumentRequest,
            // Examination Management schemas
            service::exam_session::CreateExamSessionRequest,
            service::exam_session::UpdateExamSessionRequest,
            service::exam_session::CreateExamSubjectRequest,
            service::exam_session::CreateExamScheduleRequest,
            service::exam_session::CreateExamHallRequest,
            service::student_ordering::OrderedStudent,
            service::student_ordering::RebuildOrderingResult,
            service::exam_numbers::SeatingNumberView,
            service::exam_numbers::SecretNumberView,
            service::exam_numbers::GenerationResult,
            service::exam_logistics::AllocationConfig,
            service::exam_logistics::AllocationResult,
            service::exam_logistics::HallAllocationSummary,
            service::exam_logistics::ProctorAssignConfig,
            service::exam_logistics::ProctorAssignResult,
            entities::exam_session::Model,
            entities::exam_subject::Model,
            entities::exam_schedule::Model,
            entities::seating_number::Model,
            entities::secret_number::Model,
            entities::exam_hall::Model,
            entities::exam_hall_allocation::Model,
            entities::exam_proctor::Model,
            entities::student_ordering::Model,
            entities::subject::Model,
            entities::time_slot::Model,
            entities::class_subject::Model,
            entities::scheduled_lesson::Model,
            service::schedule::ScheduledLessonView,
            entities::assessment::Model,
            entities::student_guardian::Model,
            entities::document::Model,
            entities::organization::Model,
            entities::user::Model,
            entities::user_session::Model,
            entities::academic_cycle::Model,
            entities::org_unit::Model,
            entities::class::Model,
            entities::enrollment::Model,
            entities::behavior_record::Model,
            entities::notification::Model,
            entities::audit_log::Model,
            entities::grade::Model,
            entities::attendance::Model,
        )
    )
)]
struct ApiDoc;

pub async fn start_server(database_url: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    info!("🚀 Bootstrapping Embedded Orbis Axum + SeaORM Rust Backend");

    // Load Configuration and override DB URL / Port
    let mut cfg = Config::load();
    cfg.database_url = database_url.to_string();
    cfg.port = port;

    // Write OpenAPI spec file for the SDK code generation
    #[cfg(debug_assertions)]
    {
        if let Ok(openapi_json) = ApiDoc::openapi().to_json() {
            let _ = std::fs::write("packages/sdk/openapi.json", &openapi_json);
            let _ = std::fs::write("../sdk/openapi.json", &openapi_json);
            info!("📝 Generated openapi.json for hey-api SDK");
        }
    }

    // Connect to Database (SeaORM + SQLite)
    let db = database::connect(&cfg.database_url).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // Initialize Dependency Injection
    let repo = Repository::new(db);
    let service = Service::new(repo, cfg.clone());

    // Define CORS rules
    let cors = CorsLayer::new()
        .allow_origin(any_origin_or_credentials())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(vec![
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
        ])
        .allow_credentials(true);

    // Define Routes
    let app = Router::new()
        .route("/health", get(get_status))
        .route("/api/status", get(get_status))
        .route("/api/auth/register", post(handler::auth::post_register))
        .route("/api/auth/login", post(handler::auth::post_login))
        .route("/api/auth/me", get(handler::auth::get_me))
        .route("/api/auth/logout", post(handler::auth::post_logout))
        .route("/api/organizations/:id", get(handler::organization::get_organization).put(handler::organization::put_organization))
        .route("/api/organizations/:id/stats", get(handler::organization::get_organization_stats))
        .route("/api/users", get(handler::user::get_users).post(handler::user::post_users))
        .route("/api/users/:id", get(handler::user::get_user).put(handler::user::put_user).delete(handler::user::delete_user))
        .route("/api/users/:id/role", post(handler::user::post_user_role))
        .route("/api/users/:id/password", post(handler::user::post_user_password))
        .route("/api/cycles", get(handler::academic_cycle::get_cycles).post(handler::academic_cycle::post_cycles))
        .route("/api/cycles/:id", get(handler::academic_cycle::get_cycle).put(handler::academic_cycle::put_cycle))
        .route("/api/cycles/:id/activate", post(handler::academic_cycle::post_activate_cycle))
        .route("/api/cycles/:id/lock", post(handler::academic_cycle::post_lock_cycle))
        .route("/api/cycles/:id/unlock", post(handler::academic_cycle::post_unlock_cycle))
        .route("/api/cycles/:id/grade-entry/open", post(handler::academic_cycle::post_open_grade_entry))
        .route("/api/cycles/:id/grade-entry/close", post(handler::academic_cycle::post_close_grade_entry))
        .route("/api/units", get(handler::org_unit::get_units).post(handler::org_unit::post_units))
        .route("/api/units/:id", put(handler::org_unit::put_unit).delete(handler::org_unit::delete_unit))
        .route("/api/classes", get(handler::class::get_classes).post(handler::class::post_classes))
        .route("/api/classes/:id", put(handler::class::put_class).delete(handler::class::delete_class))
        .route("/api/classes/:id/students", get(handler::class::get_class_students))
        .route("/api/enrollments", get(handler::enrollment::get_enrollments).post(handler::enrollment::post_enrollments))
        .route("/api/enrollments/promote", post(handler::enrollment::post_promote))
        .route("/api/enrollments/:id", get(handler::enrollment::get_enrollment))
        .route("/api/enrollments/:id/withdraw", post(handler::enrollment::post_withdraw))
        .route("/api/enrollments/:id/transfer", post(handler::enrollment::post_transfer))
        .route("/api/grades", get(handler::grade::get_grades).post(handler::grade::post_grades))
        .route("/api/grades/bulk", post(handler::grade::post_grades_bulk))
        .route("/api/grades/bulk-subject", post(handler::grade::post_grades_bulk_subject))
        .route("/api/grades/matrix", get(handler::grade::get_grade_matrix))
        .route("/api/grades/analytics", get(handler::grade::get_grade_analytics))
        .route("/api/grades/report/:enrollment_id", get(handler::grade::get_report_card))
        .route("/api/grades/:id", put(handler::grade::put_grade))
        .route("/api/attendance", get(handler::attendance::get_attendance).post(handler::attendance::post_attendance))
        .route("/api/attendance/bulk", post(handler::attendance::post_attendance_bulk))
        .route("/api/attendance/analytics", get(handler::attendance::get_attendance_analytics))
        .route("/api/attendance/report/:enrollment_id", get(handler::attendance::get_attendance_report))
        .route("/api/behavior", get(handler::behavior::get_behavior))
        .route("/api/behavior/:id", put(handler::behavior::put_behavior).delete(handler::behavior::delete_behavior))
        .route("/api/ai/behavior", post(handler::behavior::post_ai_behavior))
        .route("/api/notifications", get(handler::notification::get_notifications))
        .route("/api/notifications/:id/read", post(handler::notification::post_mark_read))
        .route("/api/dashboard/stats", get(handler::dashboard::get_dashboard_stats))
        .route("/api/audit-logs", get(handler::audit::get_audit_logs))
        .route("/api/import/students", post(handler::import_export::post_import_students))
        .route("/api/import/grades", post(handler::import_export::post_import_grades))
        .route("/api/import/attendance", post(handler::import_export::post_import_attendance))
        .route("/api/export/students", get(handler::import_export::get_export_students))
        .route("/api/export/grades", get(handler::import_export::get_export_grades))
        .route("/api/export/attendance", get(handler::import_export::get_export_attendance))
        .route("/api/export/report-cards/:class_id", get(handler::import_export::get_export_report_cards))
        .route("/api/finance/ledger-accounts", get(handler::finance::get_ledger_accounts).post(handler::finance::post_ledger_accounts))
        .route("/api/finance/treasuries", get(handler::finance::get_treasuries).post(handler::finance::post_treasuries))
        .route("/api/finance/student-fees", get(handler::finance::get_student_fees).post(handler::finance::post_student_fees))
        .route("/api/finance/payments", get(handler::finance::get_payments).post(handler::finance::post_payments))
        .route("/api/subjects", get(handler::subject::get_subjects).post(handler::subject::post_subjects))
        .route("/api/subjects/:id", get(handler::subject::get_subject).put(handler::subject::put_subject).delete(handler::subject::delete_subject))
        .route("/api/schedule/time-slots", get(handler::schedule::get_time_slots).post(handler::schedule::post_time_slots))
        .route("/api/schedule/time-slots/:id", put(handler::schedule::put_time_slot).delete(handler::schedule::delete_time_slot))
        .route("/api/schedule/class-subjects", get(handler::schedule::get_class_subjects).post(handler::schedule::post_class_subjects))
        .route("/api/schedule/class-subjects/:id", put(handler::schedule::put_class_subject).delete(handler::schedule::delete_class_subject))
        .route("/api/schedule/lessons", get(handler::schedule::get_lessons).post(handler::schedule::post_lessons))
        .route("/api/schedule/lessons/:id", get(handler::schedule::get_lesson).put(handler::schedule::put_lesson).delete(handler::schedule::delete_lesson))
        .route("/api/schedule/conflicts", post(handler::schedule::post_check_conflicts))
        .route("/api/schedule/solve", get(handler::schedule_solve::get_schedule_solve_jobs).post(handler::schedule_solve::post_schedule_solve))
        .route("/api/schedule/solve/:id", get(handler::schedule_solve::get_schedule_solve_job).delete(handler::schedule_solve::delete_schedule_solve_job))
        .route("/api/schedule/solve/:id/apply", post(handler::schedule_solve::post_apply_schedule_solve))
        .route("/api/schedule/rooms", get(handler::room::get_rooms).post(handler::room::post_rooms))
        .route("/api/schedule/rooms/:id", put(handler::room::put_room).delete(handler::room::delete_room))
        .route("/api/assessments", get(handler::assessment::get_assessments).post(handler::assessment::post_assessments))
        .route("/api/assessments/:id", get(handler::assessment::get_assessment).put(handler::assessment::put_assessment).delete(handler::assessment::delete_assessment))
        .route("/api/guardians", get(handler::guardian::get_guardians).post(handler::guardian::post_guardians))
        .route("/api/guardians/:id", put(handler::guardian::put_guardian).delete(handler::guardian::delete_guardian))
        .route("/api/documents", get(handler::document::get_documents).post(handler::document::post_documents))
        .route("/api/documents/:id", get(handler::document::get_document).put(handler::document::put_document).delete(handler::document::delete_document))
        // ── Examination Management Routes ──
        .route("/api/exam-sessions", get(handler::exam::get_exam_sessions).post(handler::exam::post_exam_sessions))
        .route("/api/exam-sessions/:id", get(handler::exam::get_exam_session).put(handler::exam::put_exam_session))
        .route("/api/exam-sessions/:id/activate", post(handler::exam::post_activate_exam_session))
        .route("/api/exam-sessions/:id/complete", post(handler::exam::post_complete_exam_session))
        .route("/api/exam-sessions/:id/subjects", get(handler::exam::get_exam_subjects).post(handler::exam::post_exam_subjects))
        .route("/api/exam-sessions/:id/schedule", get(handler::exam::get_exam_schedule).post(handler::exam::post_exam_schedule))
        .route("/api/exam-sessions/:id/seating-numbers", get(handler::exam::get_seating_numbers))
        .route("/api/exam-sessions/:id/seating-numbers/generate", post(handler::exam::post_generate_seating_numbers))
        .route("/api/exam-sessions/:id/secret-numbers/generate", post(handler::exam::post_generate_secret_numbers))
        .route("/api/exam-sessions/:id/secret-numbers/reveal", post(handler::exam::post_reveal_secret_numbers))
        .route("/api/exam-sessions/:id/halls", get(handler::exam::get_exam_halls).post(handler::exam::post_exam_halls))
        .route("/api/exam-sessions/:id/halls/allocate", post(handler::exam::post_allocate_halls))
        .route("/api/exam-sessions/:session_id/halls/:hall_id/students", get(handler::exam::get_hall_students))
        .route("/api/exam-sessions/:session_id/proctors/assign/:schedule_id", post(handler::exam::post_assign_proctors))
        .route("/api/exam-sessions/:id/proctors", get(handler::exam::get_exam_proctors))
        .route("/api/student-ordering", get(handler::exam::get_student_ordering))
        .route("/api/student-ordering/rebuild", post(handler::exam::post_rebuild_ordering))
        .route("/api/student-ordering/freeze", post(handler::exam::post_freeze_ordering))
        // ── Phase 6-8: Auto-Scheduling, Reports, Analytics ──
        .route("/api/exam-sessions/:id/auto-schedule", post(handler::exam::post_auto_schedule))
        .route("/api/exam-sessions/:session_id/schedule/:schedule_id/postpone", post(handler::exam::post_postpone_exam))
        .route("/api/exam-sessions/:id/dashboard", get(handler::exam::get_exam_dashboard))
        .route("/api/exam-sessions/:id/reports", post(handler::exam::post_exam_report))
        .route("/api/exam-sessions/:id/analytics", post(handler::exam::post_exam_analytics))
        .with_state(service)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("📡 Standalone Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    axum::serve(listener, app).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    Ok(())
}

fn any_origin_or_credentials() -> tower_http::cors::AllowOrigin {
    tower_http::cors::AllowOrigin::any()
}

async fn get_status() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "version": "2.0.0",
            "engine": "Axum/Rust/SQLite (SeaORM) Embedded",
            "endpoints": 82
        })),
    )
}
