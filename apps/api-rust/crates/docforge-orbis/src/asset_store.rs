// ═══════════════════════════════════════════════════════════════
// docforge-orbis — AssetStore Implementation
// SQLite + local filesystem storage backend for docforge assets
// ═══════════════════════════════════════════════════════════════

use docforge_engine::error::DocforgeError;
use docforge_engine::plugin::{AssetStore, FontMeta};
use std::path::PathBuf;

pub struct OrbisAssetStore {
    base_path: PathBuf,
}

impl OrbisAssetStore {
    pub fn new(base_path: PathBuf) -> Self {
        // Ensure directories exist
        let _ = std::fs::create_dir_all(&base_path);
        let _ = std::fs::create_dir_all(base_path.join("fonts"));
        let _ = std::fs::create_dir_all(base_path.join("assets"));
        let _ = std::fs::create_dir_all(base_path.join("outputs"));

        Self { base_path }
    }
}

impl AssetStore for OrbisAssetStore {
    fn get_asset_bytes(&self, id: &str) -> Result<Vec<u8>, DocforgeError> {
        let path = self.base_path.join("assets").join(id);
        std::fs::read(&path).map_err(|e| {
            DocforgeError::AssetNotFound(format!("Asset {id} not found at {path:?}: {e}"))
        })
    }

    fn get_font_bytes(
        &self,
        family: &str,
        weight: &str,
        style: &str,
    ) -> Result<Vec<u8>, DocforgeError> {
        let font_filename = format!("{}-{}-{}.ttf", family, weight, style);
        let path = self.base_path.join("fonts").join(&font_filename);

        std::fs::read(&path).map_err(|_| DocforgeError::FontNotFound {
            family: family.to_string(),
            weight: weight.to_string(),
        })
    }

    fn list_fonts(&self) -> Vec<FontMeta> {
        // Return system/embedded fonts list.
        // During actual rendering (Phase 2), we can search system directories dynamically.
        vec![
            FontMeta {
                family: "Rubik".to_string(),
                weight: "normal".to_string(),
                style: "normal".to_string(),
                file_path: "Rubik-Regular.ttf".to_string(),
            },
            FontMeta {
                family: "Amiri".to_string(),
                weight: "normal".to_string(),
                style: "normal".to_string(),
                file_path: "Amiri-Regular.ttf".to_string(),
            },
        ]
    }

    fn store_generated(&self, key: &str, data: &[u8]) -> Result<String, DocforgeError> {
        let path = self.base_path.join("outputs").join(key);
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(&path, data).map_err(|e| DocforgeError::Io(e))?;
        Ok(path.to_string_lossy().to_string())
    }
}
