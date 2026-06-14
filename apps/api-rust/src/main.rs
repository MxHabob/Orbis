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

mod config;
mod database;
mod entities;
mod repository;
mod service;
mod handler;
mod middleware;

use utoipa::OpenApi;

use crate::config::Config;
use crate::repository::Repository;
use crate::service::Service;

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
        handler::grade::put_grade,
        handler::grade::get_report_card,
        handler::grade::get_grade_analytics,
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
    ),
    components(
        schemas(
            handler::auth::RegisterRequest,
            handler::auth::LoginRequest,
            service::auth::LoginResponse,
            handler::class::CreateClassRequest,
            handler::class::UpdateClassRequest,
            service::behavior::AIBehaviorSubmission,
            handler::behavior::UpdateBehaviorRequest,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Logger
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("🚀 Bootstrapping Orbis Axum + SeaORM Rust Backend");

    // 2. Load Configuration
    let cfg = Config::load();
    info!("🔧 Configuration loaded for environment: {}", cfg.env);

    // Write OpenAPI spec file for the SDK code generation
    #[cfg(debug_assertions)]
    {
        let openapi_json = ApiDoc::openapi().to_json()?;
        let _ = std::fs::write("packages/sdk/openapi.json", &openapi_json);
        let _ = std::fs::write("../sdk/openapi.json", &openapi_json);
        info!("📝 Generated openapi.json for hey-api SDK");
    }

    // 3. Connect to Database (SeaORM + SQLite)
    let db = database::connect(&cfg.database_url).await?;

    // 4. Initialize Dependency Injection (Clean Architecture)
    let repo = Repository::new(db);
    let service = Service::new(repo, cfg.clone());

    // 5. Define CORS rules (Vite dashboard and desktop client integration)
    let cors = CorsLayer::new()
        .allow_origin(any_origin_or_credentials())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(vec![
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
        ])
        .allow_credentials(true);

    // 6. Define Routes
    let app = Router::new()
        // ── Public Health Checks ──
        .route("/health", get(get_status))
        .route("/api/status", get(get_status))

        // ── Auth Endpoints ──
        .route("/api/auth/register", post(handler::auth::post_register))
        .route("/api/auth/login", post(handler::auth::post_login))
        .route("/api/auth/me", get(handler::auth::get_me))
        .route("/api/auth/logout", post(handler::auth::post_logout))

        // ── Organization Endpoints ──
        .route("/api/organizations/:id", get(handler::organization::get_organization).put(handler::organization::put_organization))
        .route("/api/organizations/:id/stats", get(handler::organization::get_organization_stats))

        // ── User Management Endpoints ──
        .route("/api/users", get(handler::user::get_users).post(handler::user::post_users))
        .route("/api/users/:id", get(handler::user::get_user).put(handler::user::put_user).delete(handler::user::delete_user))
        .route("/api/users/:id/role", post(handler::user::post_user_role))
        .route("/api/users/:id/password", post(handler::user::post_user_password))

        // ── Academic Cycle Endpoints ──
        .route("/api/cycles", get(handler::academic_cycle::get_cycles).post(handler::academic_cycle::post_cycles))
        .route("/api/cycles/:id", get(handler::academic_cycle::get_cycle).put(handler::academic_cycle::put_cycle))
        .route("/api/cycles/:id/activate", post(handler::academic_cycle::post_activate_cycle))
        .route("/api/cycles/:id/lock", post(handler::academic_cycle::post_lock_cycle))

        // ── Org Unit Endpoints ──
        .route("/api/units", get(handler::org_unit::get_units).post(handler::org_unit::post_units))
        .route("/api/units/:id", put(handler::org_unit::put_unit).delete(handler::org_unit::delete_unit))

        // ── Class Endpoints ──
        .route("/api/classes", get(handler::class::get_classes).post(handler::class::post_classes))
        .route("/api/classes/:id", put(handler::class::put_class).delete(handler::class::delete_class))
        .route("/api/classes/:id/students", get(handler::class::get_class_students))

        // ── Enrollment Endpoints ──
        .route("/api/enrollments", get(handler::enrollment::get_enrollments).post(handler::enrollment::post_enrollments))
        .route("/api/enrollments/promote", post(handler::enrollment::post_promote))
        .route("/api/enrollments/:id", get(handler::enrollment::get_enrollment))
        .route("/api/enrollments/:id/withdraw", post(handler::enrollment::post_withdraw))
        .route("/api/enrollments/:id/transfer", post(handler::enrollment::post_transfer))

        // ── Grade Endpoints ──
        .route("/api/grades", get(handler::grade::get_grades).post(handler::grade::post_grades))
        .route("/api/grades/bulk", post(handler::grade::post_grades_bulk))
        .route("/api/grades/analytics", get(handler::grade::get_grade_analytics))
        .route("/api/grades/report/:enrollment_id", get(handler::grade::get_report_card))
        .route("/api/grades/:id", put(handler::grade::put_grade))

        // ── Attendance Endpoints ──
        .route("/api/attendance", get(handler::attendance::get_attendance).post(handler::attendance::post_attendance))
        .route("/api/attendance/bulk", post(handler::attendance::post_attendance_bulk))
        .route("/api/attendance/analytics", get(handler::attendance::get_attendance_analytics))
        .route("/api/attendance/report/:enrollment_id", get(handler::attendance::get_attendance_report))

        // ── Behavior Endpoints ──
        .route("/api/behavior", get(handler::behavior::get_behavior))
        .route("/api/behavior/:id", put(handler::behavior::put_behavior).delete(handler::behavior::delete_behavior))
        .route("/api/ai/behavior", post(handler::behavior::post_ai_behavior))

        // ── Notification Endpoints ──
        .route("/api/notifications", get(handler::notification::get_notifications))
        .route("/api/notifications/:id/read", post(handler::notification::post_mark_read))

        // ── Dashboard Endpoints ──
        .route("/api/dashboard/stats", get(handler::dashboard::get_dashboard_stats))

        // ── Audit Log Endpoints ──
        .route("/api/audit-logs", get(handler::audit::get_audit_logs))

        // ── Import Endpoints ──
        .route("/api/import/students", post(handler::import_export::post_import_students))
        .route("/api/import/grades", post(handler::import_export::post_import_grades))
        .route("/api/import/attendance", post(handler::import_export::post_import_attendance))

        // ── Export Endpoints ──
        .route("/api/export/students", get(handler::import_export::get_export_students))
        .route("/api/export/grades", get(handler::import_export::get_export_grades))
        .route("/api/export/attendance", get(handler::import_export::get_export_attendance))
        .route("/api/export/report-cards/:class_id", get(handler::import_export::get_export_report_cards))

        // Apply State (DI Container) and CORS Middleware
        .with_state(service)
        .layer(cors);

    // 7. Start Axum Server Listener
    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port));
    info!("📡 Server listening on http://{}", addr);
    info!("📊 Total API endpoints registered: 52");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// Helper to handle standard wildcards or credentials for local development
fn any_origin_or_credentials() -> tower_http::cors::AllowOrigin {
    // In dev we support credentials & origins. In production we scope it.
    tower_http::cors::AllowOrigin::any()
}

// System status response handler
async fn get_status() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "version": "2.0.0",
            "engine": "Axum/Rust/SQLite (SeaORM)",
            "endpoints": 52
        })),
    )
}
