// ═══════════════════════════════════════════════════════════════
// docforge-engine — Batch Processing Traits
// ProgressReporter + BatchStatusStore for 50K+ record processing
// ═══════════════════════════════════════════════════════════════

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::error::DocforgeError;

// ── Progress Reporting ───────────────────────────────────────

/// Trait for reporting batch processing progress.
/// Orbis implements this with Tauri events; other consumers
/// could use WebSocket, SSE, or stdout.
#[async_trait]
pub trait ProgressReporter: Send + Sync {
    async fn report(&self, event: ProgressEvent) -> Result<(), DocforgeError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressEvent {
    pub job_id: String,
    /// Completion percentage (0-100)
    pub progress: u32,
    /// Total number of records processed so far
    pub processed: u64,
    /// Number of successfully rendered records
    pub succeeded: u64,
    /// Number of failed records
    pub failed: u64,
    /// Total number of records in the job
    pub total: u64,
    /// Estimated time of arrival (ISO 8601)
    pub eta: Option<String>,
}

// ── Batch Status Store ───────────────────────────────────────

/// Trait for persisting batch job state (for resume/retry).
/// Orbis implements this with SQLite; other consumers could
/// use Postgres, Redis, or a file system.
#[async_trait]
pub trait BatchStatusStore: Send + Sync {
    /// Save a checkpoint for a job (for resume)
    async fn save_checkpoint(
        &self,
        job_id: &str,
        checkpoint: CheckpointData,
    ) -> Result<(), DocforgeError>;

    /// Load the last checkpoint for a job
    async fn load_checkpoint(&self, job_id: &str) -> Result<Option<CheckpointData>, DocforgeError>;

    /// Check if a job has been cancelled
    async fn is_cancelled(&self, job_id: &str) -> Result<bool, DocforgeError>;

    /// Save a successful item result
    async fn save_item_success(
        &self,
        job_id: &str,
        record_id: &str,
        sequence: u64,
        output: &[u8],
        duration_ms: u64,
    ) -> Result<(), DocforgeError>;

    /// Save a failed item result
    async fn save_item_failure(
        &self,
        job_id: &str,
        record_id: &str,
        error: &str,
    ) -> Result<(), DocforgeError>;

    /// Mark an item for retry
    async fn mark_for_retry(
        &self,
        job_id: &str,
        record_id: &str,
        error: &str,
    ) -> Result<(), DocforgeError>;

    /// Get retry count for an item
    async fn get_retry_count(&self, job_id: &str, record_id: &str) -> Result<u32, DocforgeError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointData {
    pub last_sequence: u64,
    pub succeeded: u64,
    pub failed: u64,
}

/// Configuration for a batch rendering job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchJobConfig {
    pub job_id: String,
    pub template_id: String,
    pub template_version: u32,
    pub document_json: String,
    pub output_format: BatchOutputFormat,
    pub output_mode: OutputMode,
    pub max_retries: u32,
    pub concurrency: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BatchOutputFormat {
    Pdf,
    Png,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputMode {
    /// Each record produces a separate file
    Individual,
    /// All records merged into a single multi-page PDF
    Merged,
    /// Individual files bundled into a ZIP archive
    Zip,
}
