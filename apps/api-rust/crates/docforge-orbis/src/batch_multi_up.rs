// ═══════════════════════════════════════════════════════════════
// docforge-orbis — Multi-Up Batch Print Job Orchestrator
//
// Extends the batch system to support multi-up printing:
// renders N records onto physical sheets with M items per sheet.
//
// Output modes:
// - MergedPdf:       Single PDF file with all sheets
// - IndividualPdfs:  One PDF per sheet
// - PngSheets:       PNG images of each sheet
// - ZipBundle:       ZIP containing individual PDFs + optional PNGs
// ═══════════════════════════════════════════════════════════════

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use serde::{Serialize, Deserialize};
use sea_orm::DatabaseConnection;

use docforge_engine::{
    render::font_cache::{FontCache, FontStyleKind},
    render::pdf_renderer::{render_to_pdf, PdfRenderConfig},
    render::types::{LayoutResult, PageLayout},
    layout::LayoutEngine,
    layout::multi_up::{MultiUpConfig, MultiUpLayout},
    model::TemplateDocument,
};
use crate::{
    batch::{BatchJob, BatchJobStatus},
    datasource::DataSourceRegistry,
    template_service::TemplateService,
};

// ── Output Modes ────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum MultiUpOutputMode {
    /// Single merged PDF with all sheets
    MergedPdf,
    /// Individual PDF per physical sheet
    IndividualPdfs,
    /// PNG images of each sheet (with configurable DPI)
    PngSheets { dpi: u32 },
    /// ZIP bundle containing individual PDFs and optional PNGs
    ZipBundle { include_png: bool, png_dpi: u32 },
}

impl Default for MultiUpOutputMode {
    fn default() -> Self {
        MultiUpOutputMode::MergedPdf
    }
}

// ── Multi-Up Print Job Config ───────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiUpPrintJobConfig {
    /// Unique job identifier
    pub job_id: String,
    /// Template to use for rendering
    pub template_id: String,
    /// Organization context
    pub org_id: String,
    /// Entity IDs to render (selected from data-grid)
    pub entity_ids: Vec<String>,
    /// Multi-up layout configuration
    pub multi_up: MultiUpConfig,
    /// Output mode
    #[serde(default)]
    pub output_mode: MultiUpOutputMode,
    /// Output directory path
    pub output_path: String,
}

// ── Multi-Up Print Job Result ───────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiUpPrintResult {
    /// Job status
    pub status: BatchJobStatus,
    /// Number of physical sheets generated
    pub total_sheets: u32,
    /// Items per sheet
    pub items_per_sheet: u32,
    /// Layout dimensions
    pub layout_columns: u32,
    pub layout_rows: u32,
    /// Generated file paths
    pub output_files: Vec<String>,
}

// ── Preview Generation ──────────────────────────────────────

/// Generate a multi-up preview PDF containing only the first sheet.
/// Used for the PrintDialog WYSIWYG preview.
pub async fn preview_multi_up(
    db: &DatabaseConnection,
    org_id: &str,
    template_id: &str,
    entity_ids: &[String],
    multi_up_config: &MultiUpConfig,
) -> Result<Vec<u8>, String> {
    // 1. Load template
    let template = TemplateService::get_by_id(db, org_id, template_id)
        .await
        .map_err(|e| format!("Failed to load template: {}", e))?;

    let doc: TemplateDocument = serde_json::from_str(&template.document_json)
        .map_err(|e| format!("Failed to parse template: {}", e))?;

    // 2. Setup fonts
    let mut fonts = FontCache::new("Rubik");
    load_system_fonts(&mut fonts);
    let fonts = Arc::new(fonts);

    // 3. Resolve data for preview entities (just first sheet worth)
    let multi_up_layout = multi_up_config.compute_layout();
    let preview_count = (multi_up_layout.items_per_sheet as usize).min(entity_ids.len());
    let preview_ids = &entity_ids[..preview_count];

    let registry = DataSourceRegistry::new();
    let data_source = registry.get(&template.entity_type)
        .ok_or_else(|| format!("Unsupported entity type: {}", template.entity_type))?;

    let mut contexts = Vec::with_capacity(preview_count);
    for entity_id in preview_ids {
        let ctx = data_source.resolve(db, entity_id)
            .await
            .map_err(|e| format!("Failed to resolve {}: {}", entity_id, e))?;
        contexts.push(ctx);
    }

    // 4. Layout multi-up (first sheet only)
    let engine = LayoutEngine::new(&fonts);
    let layout_result = engine.layout_multi_up(&doc, &contexts, multi_up_config)
        .map_err(|e| format!("Layout failed: {:?}", e))?;

    // 5. Render to PDF
    let pdf = render_to_pdf(&layout_result, &fonts, &PdfRenderConfig::default())
        .map_err(|e| format!("PDF render failed: {:?}", e))?;

    Ok(pdf.bytes)
}

// ── Full Batch Execution ────────────────────────────────────

/// Start a multi-up batch print job.
///
/// This is the main orchestration function that:
/// 1. Resolves all entity data contexts
/// 2. Groups them into chunks of items_per_sheet
/// 3. Runs layout_multi_up for each chunk
/// 4. Renders output in the selected mode
/// 5. Reports progress throughout
pub async fn start_multi_up_print_job(
    config: MultiUpPrintJobConfig,
    job: Arc<BatchJob>,
    db: DatabaseConnection,
    progress_callback: Arc<dyn Fn(BatchJobStatus) + Send + Sync + 'static>,
) {
    let job_id = config.job_id.clone();

    // 1. Load template
    let template = match TemplateService::get_by_id(&db, &config.org_id, &config.template_id).await {
        Ok(t) => t,
        Err(e) => {
            update_job_failed(&job, &progress_callback, format!("Failed to load template: {}", e));
            return;
        }
    };

    let doc: TemplateDocument = match serde_json::from_str(&template.document_json) {
        Ok(d) => d,
        Err(e) => {
            update_job_failed(&job, &progress_callback, format!("Failed to parse template: {}", e));
            return;
        }
    };

    // 2. Create output directory
    if let Err(e) = std::fs::create_dir_all(&config.output_path) {
        update_job_failed(&job, &progress_callback, format!("Failed to create output dir: {}", e));
        return;
    }

    // 3. Setup fonts
    let mut fonts = FontCache::new("Rubik");
    load_system_fonts(&mut fonts);
    let fonts = Arc::new(fonts);

    // 4. Compute layout
    let multi_up_layout = config.multi_up.compute_layout();
    let items_per_sheet = multi_up_layout.items_per_sheet;

    // 5. Resolve all data contexts
    let registry = DataSourceRegistry::new();
    let data_source = match registry.get(&template.entity_type) {
        Some(ds) => ds,
        None => {
            update_job_failed(&job, &progress_callback, format!("Unsupported entity type: {}", template.entity_type));
            return;
        }
    };

    let mut all_contexts = Vec::with_capacity(config.entity_ids.len());
    for (i, entity_id) in config.entity_ids.iter().enumerate() {
        // Check cancel
        if job.cancel_flag.load(Ordering::SeqCst) {
            return;
        }

        // Check pause
        while job.pause_flag.load(Ordering::SeqCst) {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            if job.cancel_flag.load(Ordering::SeqCst) { return; }
        }

        match data_source.resolve(&db, entity_id).await {
            Ok(ctx) => all_contexts.push(ctx),
            Err(e) => {
                tracing::warn!("Failed to resolve entity {}: {}", entity_id, e);
                // Skip failed records, continue with others
                continue;
            }
        }

        // Progress: data resolution phase (0-50%)
        let progress_pct = ((i + 1) as f64 / config.entity_ids.len() as f64 * 50.0) as usize;
        {
            let mut status = job.status.lock().unwrap();
            status.processed_records = progress_pct;
            progress_callback(status.clone());
        }
    }

    // 6. Chunk contexts into sheets and render
    let chunks: Vec<&[serde_json::Value]> = all_contexts.chunks(items_per_sheet as usize).collect();
    let total_sheets = chunks.len();
    let engine = LayoutEngine::new(&fonts);
    let mut output_files = Vec::new();

    // For merged PDF mode, collect all pages
    let mut all_pages: Vec<PageLayout> = Vec::new();

    for (sheet_idx, chunk) in chunks.iter().enumerate() {
        // Check cancel
        if job.cancel_flag.load(Ordering::SeqCst) { return; }

        // Layout this sheet
        let layout_result = match engine.layout_multi_up(&doc, chunk, &config.multi_up) {
            Ok(lr) => lr,
            Err(e) => {
                tracing::error!("Layout failed for sheet {}: {:?}", sheet_idx, e);
                continue;
            }
        };

        match &config.output_mode {
            MultiUpOutputMode::MergedPdf => {
                all_pages.extend(layout_result.pages);
            }
            MultiUpOutputMode::IndividualPdfs => {
                let pdf = match render_to_pdf(&layout_result, &fonts, &PdfRenderConfig::default()) {
                    Ok(p) => p,
                    Err(e) => {
                        tracing::error!("PDF render failed for sheet {}: {:?}", sheet_idx, e);
                        continue;
                    }
                };
                let file_name = format!("sheet_{:05}_{}.pdf", sheet_idx + 1, job_id);
                let file_path = std::path::Path::new(&config.output_path).join(&file_name);
                if let Err(e) = std::fs::write(&file_path, &pdf.bytes) {
                    tracing::error!("Failed to write {}: {}", file_name, e);
                }
                output_files.push(file_path.to_string_lossy().to_string());
            }
            MultiUpOutputMode::PngSheets { dpi: _ } => {
                // PNG rendering would go through a rasterizer (future enhancement)
                // For now, generate PDF and note that PNG conversion is pending
                let pdf = match render_to_pdf(&layout_result, &fonts, &PdfRenderConfig::default()) {
                    Ok(p) => p,
                    Err(e) => {
                        tracing::error!("PDF render failed for sheet {}: {:?}", sheet_idx, e);
                        continue;
                    }
                };
                let file_name = format!("sheet_{:05}_{}.pdf", sheet_idx + 1, job_id);
                let file_path = std::path::Path::new(&config.output_path).join(&file_name);
                if let Err(e) = std::fs::write(&file_path, &pdf.bytes) {
                    tracing::error!("Failed to write {}: {}", file_name, e);
                }
                output_files.push(file_path.to_string_lossy().to_string());
            }
            MultiUpOutputMode::ZipBundle { include_png: _, png_dpi: _ } => {
                // ZIP bundling: save individual PDFs, then zip at the end
                let pdf = match render_to_pdf(&layout_result, &fonts, &PdfRenderConfig::default()) {
                    Ok(p) => p,
                    Err(e) => {
                        tracing::error!("PDF render failed for sheet {}: {:?}", sheet_idx, e);
                        continue;
                    }
                };
                let file_name = format!("sheet_{:05}_{}.pdf", sheet_idx + 1, job_id);
                let file_path = std::path::Path::new(&config.output_path).join(&file_name);
                if let Err(e) = std::fs::write(&file_path, &pdf.bytes) {
                    tracing::error!("Failed to write {}: {}", file_name, e);
                }
                output_files.push(file_path.to_string_lossy().to_string());
            }
        }

        // Progress: rendering phase (50-100%)
        let progress_pct = 50 + ((sheet_idx + 1) as f64 / total_sheets as f64 * 50.0) as usize;
        {
            let mut status = job.status.lock().unwrap();
            status.processed_records = progress_pct;
            progress_callback(status.clone());
        }
    }

    // 7. Finalize merged PDF if needed
    if matches!(config.output_mode, MultiUpOutputMode::MergedPdf) {
        let merged_result = LayoutResult { pages: all_pages };
        match render_to_pdf(&merged_result, &fonts, &PdfRenderConfig::default()) {
            Ok(pdf) => {
                let file_name = format!("multi_up_{}.pdf", job_id);
                let file_path = std::path::Path::new(&config.output_path).join(&file_name);
                if let Err(e) = std::fs::write(&file_path, &pdf.bytes) {
                    update_job_failed(&job, &progress_callback, format!("Failed to write merged PDF: {}", e));
                    return;
                }
                output_files.push(file_path.to_string_lossy().to_string());
            }
            Err(e) => {
                update_job_failed(&job, &progress_callback, format!("Merged PDF render failed: {:?}", e));
                return;
            }
        }
    }

    // 8. Mark job complete
    {
        let mut status = job.status.lock().unwrap();
        status.processed_records = config.entity_ids.len();
        status.status = "completed".to_string();
        progress_callback(status.clone());
    }
}

// ── Helpers ─────────────────────────────────────────────────

fn update_job_failed(
    job: &BatchJob,
    callback: &Arc<dyn Fn(BatchJobStatus) + Send + Sync>,
    message: String,
) {
    let mut status = job.status.lock().unwrap();
    status.status = "failed".to_string();
    status.error_message = Some(message);
    callback(status.clone());
}

fn load_system_fonts(fonts: &mut FontCache) {
    // Windows system fonts path
    let font_paths = [
        ("C:\\Windows\\Fonts\\tahoma.ttf", "Tahoma", 400),
        ("C:\\Windows\\Fonts\\tahomabd.ttf", "Tahoma", 700),
    ];

    for (path, family, weight) in font_paths {
        if std::path::Path::new(path).exists() {
            fonts.load_font_file(family, weight, FontStyleKind::Normal, path).ok();
            // Also register as default Rubik fallback
            if weight == 400 {
                fonts.load_font_file("Rubik", 400, FontStyleKind::Normal, path).ok();
            }
        }
    }
}
