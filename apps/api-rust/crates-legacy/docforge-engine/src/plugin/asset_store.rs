// ═══════════════════════════════════════════════════════════════
// docforge-engine — AssetStore Trait
// Consumers implement this for asset storage and retrieval
// ═══════════════════════════════════════════════════════════════

use crate::error::DocforgeError;
use serde::{Deserialize, Serialize};

/// Trait for loading assets (images, fonts) needed during rendering.
/// Implemented by the host application (e.g., Orbis uses SQLite + filesystem).
pub trait AssetStore: Send + Sync {
    /// Get raw bytes for an asset by ID
    fn get_asset_bytes(&self, id: &str) -> Result<Vec<u8>, DocforgeError>;

    /// Get font file bytes by family, weight, and style
    fn get_font_bytes(&self, family: &str, weight: &str, style: &str) -> Result<Vec<u8>, DocforgeError>;

    /// List all available font families
    fn list_fonts(&self) -> Vec<FontMeta>;

    /// Store generated output (e.g., rendered PDF) and return a storage key
    fn store_generated(&self, key: &str, data: &[u8]) -> Result<String, DocforgeError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontMeta {
    pub family: String,
    pub weight: String,
    pub style: String,
    pub file_path: String,
}
