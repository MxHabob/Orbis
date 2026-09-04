use tauri::State;
use crate::state::AppState;
use orbis_app::usecases::report::generate_report::GenerateReportUseCase;
use orbis_app::context::RequestContext;
use uuid::Uuid;
use orbis_domain::queries::pagination::CursorRequest;

#[tauri::command]
pub async fn generate_report(
    template_id: String,
    format: String,
    parameters: serde_json::Value,
    _query: Option<CursorRequest>, // we can pass the filter query for data extraction
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_template_id = Uuid::parse_str(&template_id).map_err(|_| "Invalid template ID")?;

    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(), // Mocked
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = GenerateReportUseCase {
        report_repo: state.report_repo.clone(),
        report_engine: state.integration_resolver.clone(), // Integration resolver is our ReportEnginePort
    };

    match usecase.execute(&ctx, parsed_template_id, parameters, &format).await {
        Ok(job) => Ok(job.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}
