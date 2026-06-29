// ═══════════════════════════════════════════════════════════════
// docforge-orbis — Batch Print Job Orchestrator
// Implements parallel rendering of 50,000+ records to disk
// with pause, resume, cancel, and progress tracking.
// ═══════════════════════════════════════════════════════════════

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tokio::sync::mpsc;
use sea_orm::DatabaseConnection;

use docforge_engine::{
    render::font_cache::{FontCache, FontStyleKind},
    render::pdf_renderer::{render_to_pdf, PdfRenderConfig},
    layout::LayoutEngine,
    model::TemplateDocument,
};
use crate::{
    datasource::DataSourceRegistry,
    template_service::TemplateService,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchJobStatus {
    pub job_id: String,
    pub total_records: usize,
    pub processed_records: usize,
    pub status: String, // "running", "paused", "cancelled", "completed", "failed"
    pub error_message: Option<String>,
    pub output_path: String,
}

pub struct BatchJob {
    pub status: Mutex<BatchJobStatus>,
    pub pause_flag: Arc<AtomicBool>,
    pub cancel_flag: Arc<AtomicBool>,
}

pub struct BatchJobManager {
    pub jobs: Mutex<HashMap<String, Arc<BatchJob>>>,
}

impl BatchJobManager {
    pub fn new() -> Self {
        Self {
            jobs: Mutex::new(HashMap::new()),
        }
    }

    pub fn create_job(
        &self,
        job_id: String,
        total_records: usize,
        output_path: String,
    ) -> Arc<BatchJob> {
        let job = Arc::new(BatchJob {
            status: Mutex::new(BatchJobStatus {
                job_id: job_id.clone(),
                total_records,
                processed_records: 0,
                status: "running".to_string(),
                error_message: None,
                output_path,
            }),
            pause_flag: Arc::new(AtomicBool::new(false)),
            cancel_flag: Arc::new(AtomicBool::new(false)),
        });

        let mut jobs = self.jobs.lock().unwrap();
        jobs.insert(job_id, job.clone());
        job
    }

    pub fn get_job(&self, job_id: &str) -> Option<Arc<BatchJob>> {
        let jobs = self.jobs.lock().unwrap();
        jobs.get(job_id).cloned()
    }

    pub fn get_status(&self, job_id: &str) -> Option<BatchJobStatus> {
        let jobs = self.jobs.lock().unwrap();
        jobs.get(job_id).map(|j| j.status.lock().unwrap().clone())
    }

    pub fn pause_job(&self, job_id: &str) -> bool {
        if let Some(job) = self.get_job(job_id) {
            job.pause_flag.store(true, Ordering::SeqCst);
            let mut status = job.status.lock().unwrap();
            status.status = "paused".to_string();
            true
        } else {
            false
        }
    }

    pub fn resume_job(&self, job_id: &str) -> bool {
        if let Some(job) = self.get_job(job_id) {
            job.pause_flag.store(false, Ordering::SeqCst);
            let mut status = job.status.lock().unwrap();
            status.status = "running".to_string();
            true
        } else {
            false
        }
    }

    pub fn cancel_job(&self, job_id: &str) -> bool {
        if let Some(job) = self.get_job(job_id) {
            job.cancel_flag.store(true, Ordering::SeqCst);
            let mut status = job.status.lock().unwrap();
            status.status = "cancelled".to_string();
            true
        } else {
            false
        }
    }
}

/// Orchestrates batch processing in a separate thread.
pub async fn start_batch_print_job(
    progress_callback: Arc<dyn Fn(BatchJobStatus) + Send + Sync + 'static>,
    job: Arc<BatchJob>,
    db: DatabaseConnection,
    org_id: String,
    template_id: String,
    entity_ids: Vec<String>,
    output_dir: String,
) {
    let job_id = {
        let status = job.status.lock().unwrap();
        status.job_id.clone()
    };

    // 1. Resolve template record from DB
    let template = match TemplateService::get_by_id(&db, &org_id, &template_id).await {
        Ok(t) => t,
        Err(e) => {
            let mut status = job.status.lock().unwrap();
            status.status = "failed".to_string();
            status.error_message = Some(format!("Failed to load template: {}", e));
            progress_callback(status.clone());
            return;
        }
    };

    // 2. Parse template JSON
    let doc: TemplateDocument = match serde_json::from_str(&template.document_json) {
        Ok(d) => d,
        Err(e) => {
            let mut status = job.status.lock().unwrap();
            status.status = "failed".to_string();
            status.error_message = Some(format!("Failed to parse template JSON: {}", e));
            progress_callback(status.clone());
            return;
        }
    };

    // 3. Create output directory if it doesn't exist
    if let Err(e) = std::fs::create_dir_all(&output_dir) {
        let mut status = job.status.lock().unwrap();
        status.status = "failed".to_string();
        status.error_message = Some(format!("Failed to create output directory: {}", e));
        progress_callback(status.clone());
        return;
    }

    // 4. Setup fonts shaper cache
    let mut fonts = FontCache::new("Rubik");
    let font_path = "C:\\Windows\\Fonts\\tahoma.ttf";
    if std::path::Path::new(font_path).exists() {
        fonts.load_font_file("Tahoma", 400, FontStyleKind::Normal, font_path).ok();
        fonts.load_font_file("Rubik", 400, FontStyleKind::Normal, font_path).ok();
    }
    let fonts = Arc::new(fonts);

    // 5. Worker queue orchestration (up to 8 parallel rendering tasks)
    let concurrency = 8;
    let (tx, mut rx) = mpsc::channel::<(usize, Result<Vec<u8>, String>)>(concurrency);
    let entity_ids_len = entity_ids.len();

    // Spawn a coordinator manager task that feeds the worker pool
    let db_arc = Arc::new(db);
    let entity_type_arc = Arc::new(template.entity_type);
    let doc_arc = Arc::new(doc);
    let job_clone = job.clone();

    tokio::spawn(async move {
        let registry = DataSourceRegistry::new();
        let data_source = match registry.get(&*entity_type_arc) {
            Some(ds) => ds,
            None => {
                let _ = tx.send((0, Err(format!("Unsupported entity type: {}", entity_type_arc)))).await;
                return;
            }
        };

        for (index, entity_id) in entity_ids.into_iter().enumerate() {
            // Check cancel flag
            if job_clone.cancel_flag.load(Ordering::SeqCst) {
                break;
            }

            // Check pause flag and wait
            while job_clone.pause_flag.load(Ordering::SeqCst) {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                if job_clone.cancel_flag.load(Ordering::SeqCst) {
                    break;
                }
            }

            let db_conn = db_arc.clone();
            let ds = data_source.clone();
            let doc_ref = doc_arc.clone();
            let fonts_ref = fonts.clone();
            let tx_clone = tx.clone();

            tokio::spawn(async move {
                // Fetch record context
                let context = match ds.resolve(&db_conn, &entity_id).await {
                    Ok(ctx) => ctx,
                    Err(e) => {
                        let _ = tx_clone.send((index, Err(format!("Failed to resolve context for {}: {}", entity_id, e)))).await;
                        return;
                    }
                };

                // Compute element layouts
                let engine = LayoutEngine::new(&fonts_ref);
                let layout_result = match engine.layout(&doc_ref, &context) {
                    Ok(l) => l,
                    Err(e) => {
                        let _ = tx_clone.send((index, Err(format!("Layout failed: {:?}", e)))).await;
                        return;
                    }
                };

                // Render document pages to PDF bytes
                match render_to_pdf(&layout_result, &fonts_ref, &PdfRenderConfig::default()) {
                    Ok(pdf) => {
                        let _ = tx_clone.send((index, Ok(pdf.bytes))).await;
                    }
                    Err(e) => {
                        let _ = tx_clone.send((index, Err(format!("PDF Render failed: {:?}", e)))).await;
                    }
                }
            });
        }
    });

    // 6. Receive processed PDFs and stream write them to disk
    let mut processed = 0;
    while let Some((index, result)) = rx.recv().await {
        processed += 1;
        match result {
            Ok(bytes) => {
                let file_path = std::path::Path::new(&output_dir)
                    .join(format!("doc_{:05}_{}.pdf", index + 1, job_id));
                if let Err(e) = std::fs::write(&file_path, bytes) {
                    eprintln!("Failed to write pdf file: {}", e);
                }
            }
            Err(err) => {
                eprintln!("Batch error for record index {}: {}", index, err);
            }
        }

        // Update active status
        {
            let mut status = job.status.lock().unwrap();
            status.processed_records = processed;
            if processed == entity_ids_len {
                status.status = "completed".to_string();
            }
            progress_callback(status.clone());
        }

        if processed == entity_ids_len {
            break;
        }
    }
}
