use std::net::SocketAddr;
use axum::{
    routing::{get, post},
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
        handler::auth::post_register,
        handler::auth::post_login,
        handler::auth::get_me,
        handler::auth::post_logout,
        handler::class::get_classes,
        handler::class::post_classes,
        handler::behavior::get_behavior,
        handler::behavior::post_ai_behavior,
        handler::notification::get_notifications,
        handler::notification::post_mark_read,
    ),
    components(
        schemas(
            handler::auth::RegisterRequest,
            handler::auth::LoginRequest,
            service::auth::LoginResponse,
            handler::class::CreateClassRequest,
            service::behavior::AIBehaviorSubmission,
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
        // Public Health Checks
        .route("/health", get(get_status))
        .route("/api/status", get(get_status))
        // Public Auth Enpoints
        .route("/api/auth/register", post(handler::auth::post_register))
        .route("/api/auth/login", post(handler::auth::post_login))
        // Protected Auth Endpoints
        .route("/api/auth/me", get(handler::auth::get_me))
        .route("/api/auth/logout", post(handler::auth::post_logout))
        // Protected Academic Classes Endpoints
        .route("/api/classes", get(handler::class::get_classes).post(handler::class::post_classes))
        // Protected AI Behavior Endpoints
        .route("/api/behavior", get(handler::behavior::get_behavior))
        .route("/api/ai/behavior", post(handler::behavior::post_ai_behavior))
        // Protected Notifications Endpoints
        .route("/api/notifications", get(handler::notification::get_notifications))
        .route("/api/notifications/:id/read", post(handler::notification::post_mark_read))
        // Apply State (DI Container) and CORS Middleware
        .with_state(service)
        .layer(cors);

    // 7. Start Axum Server Listener
    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port));
    info!("📡 Server listening on http://{}", addr);

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
            "engine": "Axum/Rust/SQLite (SeaORM)"
        })),
    )
}
