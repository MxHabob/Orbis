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

#[derive(serde::Serialize)]
pub struct ReportJobStatus {
    pub job_id: String,
    pub status: String,
}

#[tauri::command]
pub async fn get_report_job_status(
    job_id: String,
    state: State<'_, AppState>,
) -> Result<ReportJobStatus, String> {
    let parsed_job_id = Uuid::parse_str(&job_id).map_err(|_| "Invalid job ID")?;
    
    let job = state.report_repo.find_job_by_id(parsed_job_id).await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Job not found".to_string())?;

    Ok(ReportJobStatus {
        job_id: job.id.to_string(),
        status: job.status,
    })
}

#[tauri::command]
pub async fn download_report_file(
    job_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<u8>, String> {
    let parsed_job_id = Uuid::parse_str(&job_id).map_err(|_| "Invalid job ID")?;
    
    let job = state.report_repo.find_job_by_id(parsed_job_id).await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Job not found".to_string())?;

    if job.status != "completed" {
        return Err("Report generation is not completed yet".to_string());
    }

    let file_path = job.file_path.ok_or_else(|| "File path missing".to_string())?;
    
    // In production, file_path would be relative to some data directory.
    // Assuming for now it's relative to the current working directory or handled securely.
    tokio::fs::read(&file_path).await.map_err(|e| format!("Failed to read file: {}", e))
}
