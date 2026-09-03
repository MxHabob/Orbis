use std::sync::Arc;

use tauri::State;

use crate::commands::error::{CmdResult, CommandError};
use crate::state::AppState;

#[tauri::command]
pub async fn create_template(
    state: State<'_, Arc<AppState>>,
    request: Option<serde_json::Value>,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let request = request
        .ok_or_else(|| CommandError::new("Missing template request"))
        .and_then(|value| {
            serde_json::from_value::<docforge_orbis::template_service::CreateTemplateRequest>(value)
                .map_err(|error| CommandError::new(format!("Invalid template request: {error}")))
        })?;

    let template = docforge_orbis::template_service::TemplateService::create(
        &ctx.db,
        &ctx.org_id,
        ctx.actor.as_str(),
        request,
    )
    .await
    .map_err(CommandError::from)?;

    serde_json::to_value(template)
        .map_err(|error| CommandError::new(format!("Failed to serialize template: {error}")))
}

#[tauri::command]
pub async fn get_template(
    state: State<'_, Arc<AppState>>,
    template_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let template = docforge_orbis::template_service::TemplateService::get_by_id(
        &ctx.db,
        &ctx.org_id,
        &template_id,
    )
    .await
    .map_err(CommandError::from)?;

    serde_json::to_value(template)
        .map_err(|error| CommandError::new(format!("Failed to serialize template: {error}")))
}

#[tauri::command]
pub async fn update_template(
    state: State<'_, Arc<AppState>>,
    template_id: String,
    request: Option<serde_json::Value>,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let request = request
        .ok_or_else(|| CommandError::new("Missing template update request"))
        .and_then(|value| {
            serde_json::from_value::<docforge_orbis::template_service::UpdateTemplateRequest>(value)
                .map_err(|error| CommandError::new(format!("Invalid template update request: {error}")))
        })?;

    let template = docforge_orbis::template_service::TemplateService::update(
        &ctx.db,
        &ctx.org_id,
        &template_id,
        ctx.actor.as_str(),
        request,
    )
    .await
    .map_err(CommandError::from)?;

    serde_json::to_value(template)
        .map_err(|error| CommandError::new(format!("Failed to serialize template: {error}")))
}

#[tauri::command]
pub async fn publish_template(
    state: State<'_, Arc<AppState>>,
    template_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let template = docforge_orbis::template_service::TemplateService::publish(
        &ctx.db,
        &ctx.org_id,
        &template_id,
        ctx.actor.as_str(),
    )
    .await
    .map_err(CommandError::from)?;

    serde_json::to_value(template)
        .map_err(|error| CommandError::new(format!("Failed to serialize template: {error}")))
}

#[tauri::command]
pub async fn archive_template(
    state: State<'_, Arc<AppState>>,
    template_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let template = docforge_orbis::template_service::TemplateService::archive(
        &ctx.db,
        &ctx.org_id,
        &template_id,
    )
    .await
    .map_err(CommandError::from)?;

    serde_json::to_value(template)
        .map_err(|error| CommandError::new(format!("Failed to serialize template: {error}")))
}

#[tauri::command]
pub async fn restore_template(
    state: State<'_, Arc<AppState>>,
    template_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let template = docforge_orbis::template_service::TemplateService::restore(
        &ctx.db,
        &ctx.org_id,
        &template_id,
    )
    .await
    .map_err(CommandError::from)?;

    serde_json::to_value(template)
        .map_err(|error| CommandError::new(format!("Failed to serialize template: {error}")))
}

#[tauri::command]
pub async fn delete_template(
    state: State<'_, Arc<AppState>>,
    template_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;

    docforge_orbis::template_service::TemplateService::delete(
        &ctx.db,
        &ctx.org_id,
        &template_id,
    )
    .await
    .map_err(CommandError::from)?;

    Ok(serde_json::json!({ "deleted": true }))
}

#[tauri::command]
pub async fn list_templates(
    state: State<'_, Arc<AppState>>,
    request: Option<serde_json::Value>,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let params = request
        .map(|value| {
            serde_json::from_value::<docforge_orbis::template_service::ListTemplatesParams>(value)
                .map_err(|error| CommandError::new(format!("Invalid template filters: {error}")))
        })
        .transpose()?;

    let mut params = params.unwrap_or(docforge_orbis::template_service::ListTemplatesParams {
        org_id: ctx.org_id.clone(),
        status: None,
        document_type: None,
        entity_type: None,
        category_id: None,
        search: None,
    });
    params.org_id = ctx.org_id.clone();

    let templates = docforge_orbis::template_service::TemplateService::list(&ctx.db, params)
        .await
        .map_err(CommandError::from)?;

    serde_json::to_value(templates)
        .map_err(|error| CommandError::new(format!("Failed to serialize templates: {error}")))
}

#[tauri::command]
pub async fn list_data_schemas(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<serde_json::Value> {
    let _ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;

    let schemas = docforge_orbis::datasource::DataSourceRegistry::new().list_schemas();
    serde_json::to_value(schemas)
        .map_err(|error| CommandError::new(format!("Failed to serialize data schemas: {error}")))
}

#[tauri::command]
pub async fn preview_multi_up(
    state: State<'_, Arc<AppState>>,
    template_id: String,
    entity_ids: Vec<String>,
    multi_up_config: String,
) -> CmdResult<Vec<u8>> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let config = serde_json::from_str::<docforge_engine::layout::multi_up::MultiUpConfig>(
        &multi_up_config,
    )
    .map_err(|error| CommandError::new(format!("Invalid multi-up configuration: {error}")))?;

    docforge_orbis::batch_multi_up::preview_multi_up(
        &ctx.db,
        &ctx.org_id,
        &template_id,
        &entity_ids,
        &config,
    )
    .await
    .map_err(CommandError::from)
}

#[tauri::command]
pub async fn start_multi_up_print(
    state: State<'_, Arc<AppState>>,
    jobs: State<'_, Arc<docforge_orbis::batch::BatchJobManager>>,
    template_id: String,
    entity_ids: Vec<String>,
    multi_up_config: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let multi_up = serde_json::from_str::<docforge_engine::layout::multi_up::MultiUpConfig>(
        &multi_up_config,
    )
    .map_err(|error| CommandError::new(format!("Invalid multi-up configuration: {error}")))?;
    let job_id = uuid::Uuid::new_v4().to_string();
    let output_path = std::env::temp_dir()
        .join("orbis-docforge")
        .join(&job_id)
        .to_string_lossy()
        .to_string();
    let job = jobs.create_job(job_id.clone(), entity_ids.len(), output_path.clone());
    let config = docforge_orbis::batch_multi_up::MultiUpPrintJobConfig {
        job_id: job_id.clone(),
        template_id,
        org_id: ctx.org_id,
        entity_ids,
        multi_up,
        output_mode: Default::default(),
        output_path,
    };
    let db = ctx.db.clone();
    let callback_jobs = Arc::clone(&jobs);
    let callback: Arc<dyn Fn(docforge_orbis::batch::BatchJobStatus) + Send + Sync> = Arc::new(
        move |status| {
            if let Some(existing) = callback_jobs.get_job(&status.job_id) {
                if let Ok(mut current) = existing.status.lock() {
                    *current = status;
                }
            }
        },
    );
    tokio::spawn(docforge_orbis::batch_multi_up::start_multi_up_print_job(
        config, job, db, callback,
    ));

    Ok(serde_json::json!({ "jobId": job_id }))
}

#[tauri::command]
pub async fn get_multi_up_print_status(
    jobs: State<'_, Arc<docforge_orbis::batch::BatchJobManager>>,
    job_id: String,
) -> CmdResult<serde_json::Value> {
    let status = jobs
        .get_status(&job_id)
        .ok_or_else(|| CommandError::new(format!("Print job not found: {job_id}")))?;
    serde_json::to_value(status)
        .map_err(|error| CommandError::new(format!("Failed to serialize print status: {error}")))
}

#[tauri::command]
pub async fn list_template_versions(
    state: State<'_, Arc<AppState>>,
    template_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let _ = docforge_orbis::template_service::TemplateService::get_by_id(
        &ctx.db,
        &ctx.org_id,
        &template_id,
    )
    .await
    .map_err(CommandError::from)?;
    let versions = docforge_orbis::template_service::TemplateService::get_versions(
        &ctx.db,
        &template_id,
    )
    .await
    .map_err(CommandError::from)?;

    serde_json::to_value(versions)
        .map_err(|error| CommandError::new(format!("Failed to serialize template versions: {error}")))
}

#[tauri::command]
pub async fn revert_template_version(
    state: State<'_, Arc<AppState>>,
    template_id: String,
    version: i32,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let template = docforge_orbis::template_service::TemplateService::revert_to_version(
        &ctx.db,
        &ctx.org_id,
        &template_id,
        version,
        ctx.actor.as_str(),
    )
    .await
    .map_err(CommandError::from)?;

    serde_json::to_value(template)
        .map_err(|error| CommandError::new(format!("Failed to serialize reverted template: {error}")))
}
