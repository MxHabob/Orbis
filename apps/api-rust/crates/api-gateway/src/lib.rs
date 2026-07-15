use std::net::SocketAddr;
use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::Json,
    http::{StatusCode, Method},
    response::IntoResponse,
};
use serde_json::json;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tracing::info;

pub mod entities;
pub mod app_state;
pub mod platform;

#[macro_use]
mod handler;
mod middleware;

use app_state::AppState;

pub async fn start_server(_database_url: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    info!("🚀 Bootstrapping Orbis API Gateway (ops:: architecture)");

    let state = AppState::init().await?;

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::AllowOrigin::any())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(vec![
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
        ])
        .allow_credentials(true);

    let app = Router::new()
        // Health
        .route("/health", get(get_status))
        .route("/api/status", get(get_status))
        // Auth
        .route("/api/auth/register", post(handler::auth::post_register))
        .route("/api/auth/login", post(handler::auth::post_login))
        .route("/api/auth/me", get(handler::auth::get_me))
        .route("/api/auth/logout", post(handler::auth::post_logout))
        // Organization
        .route("/api/organization", get(handler::organization::get_organization).put(handler::organization::put_organization))
        .route("/api/organization/stats", get(handler::organization::get_organization_stats))
        // Users
        .route("/api/users", get(handler::user::get_users).post(handler::user::post_users))
        .route("/api/users/:id", get(handler::user::get_user).put(handler::user::put_user).delete(handler::user::delete_user))
        .route("/api/users/:id/role", post(handler::user::post_user_role))
        .route("/api/users/:id/password", post(handler::user::post_user_password))
        // Academic Periods
        .route("/api/periods", get(handler::academic_period::get_periods).post(handler::academic_period::post_periods))
        .route("/api/periods/:id", get(handler::academic_period::get_period))
        .route("/api/periods/year/:period_id", get(handler::academic_period::get_periods_by_year))
        .route("/api/periods/:id/lock", post(handler::academic_period::post_lock_period))
        .route("/api/periods/:id/unlock", post(handler::academic_period::post_unlock_period))
        .route("/api/periods/:id/enrollment/open", post(handler::academic_period::post_open_enrollment))
        .route("/api/periods/:id/enrollment/close", post(handler::academic_period::post_close_enrollment))
        .route("/api/periods/:id/grade-entry/open", post(handler::academic_period::post_open_grade_entry))
        .route("/api/periods/:id/grade-entry/close", post(handler::academic_period::post_close_grade_entry))
        // Org Units
        .route("/api/units", get(handler::org_unit::get_units).post(handler::org_unit::post_units))
        .route("/api/units/:id", put(handler::org_unit::put_unit).delete(handler::org_unit::delete_unit))
        // Classes
        .route("/api/classes", get(handler::class::get_classes).post(handler::class::post_classes))
        .route("/api/classes/:id", put(handler::class::put_class).delete(handler::class::delete_class))
        .route("/api/classes/:id/students", get(handler::class::get_class_students))
        // Subjects
        .route("/api/subjects", get(handler::subject::get_subjects).post(handler::subject::post_subjects))
        .route("/api/subjects/:id", put(handler::subject::put_subject).delete(handler::subject::delete_subject))
        // Enrollments
        .route("/api/enrollments", get(handler::enrollment::get_enrollments).post(handler::enrollment::post_enrollments))
        .route("/api/enrollments/:id", get(handler::enrollment::get_enrollment))
        .route("/api/enrollments/:id/withdraw", post(handler::enrollment::post_withdraw))
        .route("/api/enrollments/:id/transfer", post(handler::enrollment::post_transfer))
        .route("/api/enrollments/:id/promote", post(handler::enrollment::post_promote))
        // Guardians
        .route("/api/guardians/:student_id", get(handler::guardian::get_guardians))
        .route("/api/guardians", post(handler::guardian::post_guardians))
        .route("/api/guardians/:id", delete(handler::guardian::delete_guardian))
        // Documents
        .route("/api/documents/:owner_id", get(handler::document::get_documents))
        .route("/api/documents", post(handler::document::post_documents))
        .route("/api/documents/:id", delete(handler::document::delete_document))
        // Grades
        .route("/api/grades", get(handler::grade::get_grades).post(handler::grade::post_grades))
        .route("/api/grades/bulk", post(handler::grade::post_grades_bulk))
        .route("/api/grades/upsert", post(handler::grade::post_grade_upsert))
        .route("/api/grades/matrix/:class_id/:period_id", get(handler::grade::get_grade_matrix))
        .route("/api/grades/compile/:period_id/:class_id", post(handler::grade::post_compile_semester))
        .route("/api/report-card/:enrollment_id/:period_id", get(handler::grade::get_report_card))
        // Attendance
        .route("/api/attendance/:class_id/:date", get(handler::attendance::get_attendance))
        .route("/api/attendance", post(handler::attendance::post_attendance))
        .route("/api/attendance/bulk", post(handler::attendance::post_attendance_bulk))
        .route("/api/attendance/report/:class_id", get(handler::attendance::get_attendance_report))
        // Assessments
        .route("/api/assessments", get(handler::assessment::get_assessments).post(handler::assessment::post_assessments))
        .route("/api/assessments/:id", put(handler::assessment::put_assessment).delete(handler::assessment::delete_assessment))
        // Behavior
        .route("/api/behavior", post(handler::behavior::post_behavior))
        .route("/api/behavior/:student_id", get(handler::behavior::get_behavior))
        // Dashboard
        .route("/api/dashboard", get(handler::dashboard::get_dashboard))
        // Notifications
        .route("/api/notifications", get(handler::notification::get_notifications))
        // Audit
        .route("/api/audit-logs", get(handler::audit::get_audit_logs))
        // Finance
        .route("/api/fee-templates", get(handler::finance::get_fee_templates).post(handler::finance::post_fee_templates))
        .route("/api/student-fees/:enrollment_id", get(handler::finance::get_student_fees))
        .route("/api/student-fees", post(handler::finance::post_assign_fee))
        .route("/api/payments", get(handler::finance::get_payments).post(handler::finance::post_payment))
        .route("/api/treasuries", get(handler::finance::get_treasuries))
        .route("/api/ledger-accounts", get(handler::finance::get_ledger_accounts))
        // Schedule
        .route("/api/schedule/:class_id/:period_id", get(handler::schedule::get_class_schedule))
        .route("/api/schedule/teacher/:teacher_id/:period_id", get(handler::schedule::get_teacher_schedule))
        .route("/api/lessons", post(handler::schedule::post_lesson))
        .route("/api/lessons/:id", put(handler::schedule::put_lesson).delete(handler::schedule::delete_lesson))
        .route("/api/time-slots", get(handler::schedule::get_time_slots))
        // Schedule Solver
        .route("/api/schedule/solve", post(handler::schedule_solve::post_solve))
        .route("/api/schedule/solve/:period_id", get(handler::schedule_solve::get_solve_jobs))
        .route("/api/schedule/solve/:job_id/apply", post(handler::schedule_solve::post_apply_solve))
        .route("/api/schedule/solve/:job_id/cancel", post(handler::schedule_solve::post_cancel_solve))
        // Rooms
        .route("/api/rooms", get(handler::room::get_rooms).post(handler::room::post_room))
        .route("/api/rooms/:id", put(handler::room::put_room).delete(handler::room::delete_room))
        // Exams
        .route("/api/exam/sessions/:period_id", get(handler::exam::get_sessions))
        .route("/api/exam/sessions", post(handler::exam::post_session))
        .route("/api/exam/sessions/:session_id/subjects", get(handler::exam::get_exam_subjects))
        .route("/api/exam/halls", get(handler::exam::get_halls))
        .route("/api/exam/sessions/:session_id/seating", post(handler::exam::post_generate_seating))
        .route("/api/exam/sessions/:session_id/schedule", post(handler::exam::post_generate_schedule))
        // Import/Export
        .route("/api/import/students", post(handler::import_export::post_import_students))
        .route("/api/import/grades", post(handler::import_export::post_import_grades))
        .route("/api/export/students", get(handler::import_export::get_export_students))
        .route("/api/export/grades", get(handler::import_export::get_export_grades))
        // Static Storage
        .nest_service("/storage", ServeDir::new("app_data/storage"))
        .with_state(state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("📡 API Gateway listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_status() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "version": "1.0.0",
            "engine": "Orbis API Gateway (ops:: architecture)",
        })),
    )
}
