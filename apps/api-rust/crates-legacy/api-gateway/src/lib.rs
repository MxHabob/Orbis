use std::net::SocketAddr;
use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::DefaultBodyLimit,
    extract::{Json, State},
    http::{StatusCode, Method},
    response::IntoResponse,
};
use serde_json::json;
use tower_http::cors::CorsLayer;
use tracing::info;

pub mod entities;
pub mod app_state;
pub mod platform;
pub mod openapi;

#[macro_use]
mod handler;
mod middleware;

use app_state::AppState;

pub async fn start_server(database_url: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    info!("🚀 Bootstrapping Orbis API Gateway (ops:: architecture)");

    let state = AppState::init_with_database_url(Some(database_url)).await?;

    let configured_origins = std::env::var("CORS_ORIGINS").unwrap_or_else(|_| {
        if state.config.is_development() {
            "http://localhost:1520,http://localhost:3000".to_string()
        } else {
            String::new()
        }
    });
    let allowed_origins = configured_origins
        .split(',')
        .filter_map(|origin| origin.trim().parse().ok())
        .collect::<Vec<axum::http::HeaderValue>>();
    if allowed_origins.is_empty() && !state.config.is_development() {
        return Err("CORS_ORIGINS must contain at least one trusted origin outside development".into());
    }
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::AllowOrigin::list(allowed_origins))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(vec![
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
        ])
        .allow_credentials(true);

    let is_development = state.config.is_development();
    let bind_host = std::env::var("BIND_HOST")
        .unwrap_or_else(|_| if is_development { "127.0.0.1".into() } else { "0.0.0.0".into() });

    let app = Router::new()
        // Health
        .route("/health", get(get_status))
        .route("/ready", get(get_ready))
        .route("/api/status", get(get_status))
        .route("/api/openapi.json", get(get_openapi))
        // Auth
        .route("/api/auth/register", post(handler::auth::post_register))
        .route("/api/auth/login", post(handler::auth::post_login))
        .route("/api/auth/refresh", post(handler::auth::post_refresh))
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
        .route("/api/classes/:id/students/enriched", get(handler::class::get_class_students_enriched))
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
        .route("/api/documents/:id/download", get(handler::document::download_document))
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
        // Reports
        .route("/api/reports", get(handler::report::get_reports))
        .route("/api/reports/:code", get(handler::report::get_report_definition))
        .route("/api/reports/generate", post(handler::report::post_generate_report))
        .route("/api/reports/export", post(handler::report::post_export_report))
        // Storage is served only through authenticated document handlers.
        .with_state(state)
        .layer(cors)
        .layer(DefaultBodyLimit::max(
            std::env::var("MAX_REQUEST_BYTES")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(25 * 1024 * 1024),
        ));

    let addr = format!("{}:{}", bind_host, port).parse::<SocketAddr>()?;
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

async fn get_ready(State(state): axum::extract::State<AppState>) -> impl IntoResponse {
    match state.db.ping().await {
        Ok(()) => (
            StatusCode::OK,
            Json(json!({ "success": true, "status": "ready" })),
        ),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "success": false, "status": "not_ready" })),
        ),
    }
}

async fn get_openapi() -> impl IntoResponse {
    (
        [(axum::http::header::CONTENT_TYPE, "application/json")],
        openapi::json(),
    )
}
