// ═══════════════════════════════════════════════════════════════
// Orbis Desktop — DocForge Tauri Commands
// Implements commands for template preview, rendering, expression
// evaluation, and retrieving data schemas for UI binding.
// ═══════════════════════════════════════════════════════════════

use tauri::State;
use std::sync::Arc;
use std::collections::HashMap;
use crate::state::AppState;
use super::error::{CmdResult, CommandError};
use docforge_engine::{
    render::font_cache::{FontCache, FontStyleKind},
    render::pdf_renderer::{render_to_pdf, PdfRenderConfig},
    layout::LayoutEngine,
    model::TemplateDocument,
};
use docforge_orbis::{
    datasource::DataSourceRegistry,
    template_service::TemplateService,
};

/// Previews a document template from raw editor JSON content and custom data context.
#[tauri::command]
pub async fn preview_document_template(
    _state: State<'_, Arc<AppState>>,
    template_json: String,
    context_json: String,
) -> CmdResult<Vec<u8>> {
    // 1. Parse template JSON
    let doc: TemplateDocument = serde_json::from_str(&template_json)
        .map_err(|e| CommandError::new(format!("Failed to parse template JSON: {}", e)))?;

    // 2. Parse context JSON
    let context = serde_json::from_str(&context_json)
        .map_err(|e| CommandError::new(format!("Failed to parse context JSON: {}", e)))?;

    // 3. Setup font cache and bundle/load standard system fonts
    let mut fonts = FontCache::new("Rubik");
    let font_path = "C:\\Windows\\Fonts\\tahoma.ttf";
    if std::path::Path::new(font_path).exists() {
        fonts.load_font_file("Tahoma", 400, FontStyleKind::Normal, font_path).ok();
        fonts.load_font_file("Rubik", 400, FontStyleKind::Normal, font_path).ok();
    }
    
    // 4. Compute layout
    let engine = LayoutEngine::new(&fonts);
    let layout_result = engine.layout(&doc, &context)
        .map_err(|e| CommandError::new(format!("Layout calculation failed: {:?}", e)))?;

    // 5. Render to PDF
    let pdf_output = render_to_pdf(&layout_result, &fonts, &PdfRenderConfig::default())
        .map_err(|e| CommandError::new(format!("PDF rendering failed: {:?}", e)))?;

    Ok(pdf_output.bytes)
}

/// Renders a template from database using resolved real entity database records.
#[tauri::command]
pub async fn render_document(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    template_id: String,
    entity_id: String,
) -> CmdResult<Vec<u8>> {
    let db = &state.service.repo.db;

    // 1. Fetch template record from DB
    let template = TemplateService::get_by_id(db, &org_id, &template_id)
        .await
        .map_err(|e| CommandError::new(format!("Failed to fetch template: {}", e)))?;

    // 2. Parse template JSON
    let doc: TemplateDocument = serde_json::from_str(&template.document_json)
        .map_err(|e| CommandError::new(format!("Failed to parse template JSON: {}", e)))?;

    // 3. Resolve data context from appropriate DataSource
    let registry = DataSourceRegistry::new();
    let data_source = registry.get(&template.entity_type)
        .ok_or_else(|| CommandError::new(format!("Unsupported entity type data source: {}", template.entity_type)))?;

    let context = data_source.resolve(db, &entity_id)
        .await
        .map_err(|e| CommandError::new(format!("Failed to resolve data source: {}", e)))?;

    // 4. Setup font cache
    let mut fonts = FontCache::new("Rubik");
    let font_path = "C:\\Windows\\Fonts\\tahoma.ttf";
    if std::path::Path::new(font_path).exists() {
        fonts.load_font_file("Tahoma", 400, FontStyleKind::Normal, font_path).ok();
        fonts.load_font_file("Rubik", 400, FontStyleKind::Normal, font_path).ok();
    }

    // 5. Compute layout
    let engine = LayoutEngine::new(&fonts);
    let layout_result = engine.layout(&doc, &context)
        .map_err(|e| CommandError::new(format!("Layout calculation failed: {:?}", e)))?;

    // 6. Render to PDF
    let pdf_output = render_to_pdf(&layout_result, &fonts, &PdfRenderConfig::default())
        .map_err(|e| CommandError::new(format!("PDF rendering failed: {:?}", e)))?;

    Ok(pdf_output.bytes)
}

/// Returns dynamic variables schemas for editor picker auto-completion.
#[tauri::command]
pub async fn get_datasource_schemas() -> CmdResult<HashMap<String, serde_json::Value>> {
    let registry = DataSourceRegistry::new();
    Ok(registry.list_schemas())
}

/// Evaluates a template string containing expressions against a JSON context.
#[tauri::command]
pub async fn evaluate_template_expressions(
    expression_template: String,
    context_json: String,
) -> CmdResult<String> {
    let context = serde_json::from_str(&context_json)
        .map_err(|e| CommandError::new(format!("Failed to parse context JSON: {}", e)))?;
    let val = docforge_engine::expr::evaluator::evaluate_expression(&expression_template, &context);
    Ok(val)
}

/// Starts a background batch printing job for multiple entity IDs using a template.
#[tauri::command]
pub async fn start_batch_print(
    state: State<'_, Arc<AppState>>,
    batch_manager: State<'_, Arc<docforge_orbis::batch::BatchJobManager>>,
    app_handle: tauri::AppHandle,
    org_id: String,
    template_id: String,
    entity_ids: Vec<String>,
    output_dir: String,
) -> CmdResult<docforge_orbis::batch::BatchJobStatus> {
    // Generate a unique job ID
    let job_id = uuid::Uuid::new_v4().to_string();
    
    // Create the job status
    let job = batch_manager.create_job(job_id, entity_ids.len(), output_dir.clone());
    
    // Spawn the background print orchestrator
    let db = state.service.repo.db.clone();
    let job_clone = job.clone();
    
    let app_handle_clone = app_handle.clone();
    let callback = Arc::new(move |status| {
        use tauri::Emitter;
        let _ = app_handle_clone.emit("batch-job-progress", status);
    });

    tokio::spawn(async move {
        docforge_orbis::batch::start_batch_print_job(
            callback,
            job_clone,
            db,
            org_id,
            template_id,
            entity_ids,
            output_dir,
        ).await;
    });

    let status = job.status.lock().unwrap().clone();
    Ok(status)
}

/// Pauses a batch print job.
#[tauri::command]
pub async fn pause_batch_print(
    batch_manager: State<'_, Arc<docforge_orbis::batch::BatchJobManager>>,
    job_id: String,
) -> CmdResult<bool> {
    Ok(batch_manager.pause_job(&job_id))
}

/// Resumes a paused batch print job.
#[tauri::command]
pub async fn resume_batch_print(
    batch_manager: State<'_, Arc<docforge_orbis::batch::BatchJobManager>>,
    job_id: String,
) -> CmdResult<bool> {
    Ok(batch_manager.resume_job(&job_id))
}

/// Cancels a running or paused batch print job.
#[tauri::command]
pub async fn cancel_batch_print(
    batch_manager: State<'_, Arc<docforge_orbis::batch::BatchJobManager>>,
    job_id: String,
) -> CmdResult<bool> {
    Ok(batch_manager.cancel_job(&job_id))
}

/// Returns the current progress or error status of a batch print job.
#[tauri::command]
pub async fn get_batch_print_status(
    batch_manager: State<'_, Arc<docforge_orbis::batch::BatchJobManager>>,
    job_id: String,
) -> CmdResult<docforge_orbis::batch::BatchJobStatus> {
    batch_manager.get_status(&job_id)
        .ok_or_else(|| CommandError::new(format!("Job not found: {}", job_id)))
}

// ═══════════════════════════════════════════════════════════════
// Multi-Up Printing Commands
// ═══════════════════════════════════════════════════════════════

/// Compute optimal multi-up layout for given config.
/// Returns layout info (columns, rows, items_per_sheet, slot positions).
/// Used by the frontend for instant WYSIWYG preview.
#[tauri::command]
pub async fn compute_multi_up_layout(
    config_json: String,
) -> CmdResult<serde_json::Value> {
    let config: docforge_engine::layout::multi_up::MultiUpConfig =
        serde_json::from_str(&config_json)
            .map_err(|e| CommandError::new(format!("Invalid multi-up config: {}", e)))?;

    let layout = config.compute_layout();
    let result = serde_json::to_value(&layout)
        .map_err(|e| CommandError::new(format!("Serialization error: {}", e)))?;

    Ok(result)
}

/// Generate a multi-up preview PDF containing only the first sheet.
/// Uses real entity data for WYSIWYG accuracy.
#[tauri::command]
pub async fn preview_multi_up(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    template_id: String,
    entity_ids: Vec<String>,
    config_json: String,
) -> CmdResult<Vec<u8>> {
    let db = &state.service.repo.db;

    let config: docforge_engine::layout::multi_up::MultiUpConfig =
        serde_json::from_str(&config_json)
            .map_err(|e| CommandError::new(format!("Invalid multi-up config: {}", e)))?;

    let pdf_bytes = docforge_orbis::batch_multi_up::preview_multi_up(
        db, &org_id, &template_id, &entity_ids, &config,
    )
    .await
    .map_err(|e| CommandError::new(e))?;

    Ok(pdf_bytes)
}

/// Start a background multi-up batch print job.
/// Emits "multi-up-batch-progress" events for frontend progress tracking.
#[tauri::command]
pub async fn start_multi_up_batch_print(
    state: State<'_, Arc<AppState>>,
    batch_manager: State<'_, Arc<docforge_orbis::batch::BatchJobManager>>,
    app_handle: tauri::AppHandle,
    org_id: String,
    template_id: String,
    entity_ids: Vec<String>,
    config_json: String,
    output_dir: String,
) -> CmdResult<docforge_orbis::batch::BatchJobStatus> {
    let config: docforge_engine::layout::multi_up::MultiUpConfig =
        serde_json::from_str(&config_json)
            .map_err(|e| CommandError::new(format!("Invalid multi-up config: {}", e)))?;

    // Generate a unique job ID
    let job_id = uuid::Uuid::new_v4().to_string();

    // Create the job status
    let job = batch_manager.create_job(job_id.clone(), entity_ids.len(), output_dir.clone());

    // Build the multi-up job config
    let multi_up_config = docforge_orbis::batch_multi_up::MultiUpPrintJobConfig {
        job_id: job_id.clone(),
        template_id,
        org_id,
        entity_ids,
        multi_up: config,
        output_mode: docforge_orbis::batch_multi_up::MultiUpOutputMode::MergedPdf,
        output_path: output_dir,
    };

    // Progress callback emits Tauri events
    let app_handle_clone = app_handle.clone();
    let callback = Arc::new(move |status: docforge_orbis::batch::BatchJobStatus| {
        use tauri::Emitter;
        let _ = app_handle_clone.emit("multi-up-batch-progress", &status);
    });

    // Spawn the background orchestrator
    let db = state.service.repo.db.clone();
    let job_clone = job.clone();

    tokio::spawn(async move {
        docforge_orbis::batch_multi_up::start_multi_up_print_job(
            multi_up_config,
            job_clone,
            db,
            callback,
        ).await;
    });

    let status = job.status.lock().unwrap().clone();
    Ok(status)
}
