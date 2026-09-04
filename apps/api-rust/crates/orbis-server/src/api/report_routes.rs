use axum::{
    extract::{State, Path},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::state::AppState;
use orbis_domain::errors::DomainError;
use orbis_domain::queries::pagination::CursorRequest;
use orbis_app::usecases::report::generate_report::GenerateReportUseCase;

#[derive(Deserialize)]
pub struct GenerateReportRequest {
    pub template_id: Uuid,
    pub format: String,
    pub parameters: serde_json::Value,
    pub query: Option<CursorRequest>,
}

#[derive(Serialize)]
pub struct ReportJobResponse {
    pub job_id: Uuid,
    pub status: String,
    pub file_path: Option<String>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/generate", post(generate_report_handler))
        .route("/jobs/:id", get(get_job_status_handler))
}

async fn generate_report_handler(
    State(state): State<AppState>,
    Json(payload): Json<GenerateReportRequest>,
) -> Result<Json<ReportJobResponse>, String> {
    let ctx = orbis_app::context::RequestContext {
        tenant_id: Uuid::new_v4(), // Mocked for now
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };
    let usecase = GenerateReportUseCase {
        report_repo: state.report_repo.clone(),
        report_engine: state.integration_resolver.clone(),
    };

    let job = match usecase
        .execute(&ctx, payload.template_id, payload.parameters, &payload.format)
        .await {
            Ok(j) => j,
            Err(e) => return Err(e.to_string()),
        };

    Ok(Json(ReportJobResponse {
        job_id: job.id,
        status: job.status,
        file_path: job.file_path,
    }))
}

async fn get_job_status_handler(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<ReportJobResponse>, String> {
    Err("Not implemented".into())
}
