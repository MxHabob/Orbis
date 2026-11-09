// ═══════════════════════════════════════════════════════════════
// docforge-engine — Font Cache
// Loads and caches font files for use in text measurement and PDF rendering.
// Uses cosmic-text's FontSystem for text shaping + layout.
// ═══════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::error::DocforgeError;

/// A cached font entry holding raw bytes + metadata
#[derive(Debug, Clone)]
pub struct CachedFont {
    pub family: String,
    pub weight: u16,
    pub style: FontStyleKind,
    pub data: Arc<Vec<u8>>,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontStyleKind {
    Normal,
    Italic,
}

/// Font cache key
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FontKey {
    family: String,
    weight: u16,
    style: FontStyleKind,
}

/// Manages font loading and caching.
/// Fonts are loaded once from disk and kept in memory for the lifetime of the renderer.
///
/// Memory budget: ~20MB for typical Arabic + Latin font set
pub struct FontCache {
    fonts: HashMap<FontKey, CachedFont>,
    /// Directories to search for font files
    search_dirs: Vec<PathBuf>,
    /// Default font family name
    default_family: String,
}

impl FontCache {
    /// Create a new empty font cache
    pub fn new(default_family: &str) -> Self {
        let mut cache = FontCache {
            fonts: HashMap::new(),
            search_dirs: Vec::new(),
            default_family: default_family.to_string(),
        };

        let candidate_paths = [
            ("Tahoma", "C:\\Windows\\Fonts\\tahoma.ttf"),
            ("Arial", "C:\\Windows\\Fonts\\arial.ttf"),
            ("Segoe UI", "C:\\Windows\\Fonts\\segoeui.ttf"),
            (
                "DejaVu Sans",
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            ),
            (
                "Noto Sans Arabic",
                "/usr/share/fonts/truetype/noto/NotoSansArabic-Regular.ttf",
            ),
            ("Arial", "/System/Library/Fonts/Supplemental/Arial.ttf"),
            ("Geeza Pro", "/System/Library/Fonts/GeezaPro.ttc"),
        ];

        for (family, path) in candidate_paths {
            if !Path::new(path).exists() {
                continue;
            }
            let _ = cache.load_font_file(family, 400, FontStyleKind::Normal, path);
            if family == "Tahoma" || family == "DejaVu Sans" || family == "Arial" {
                let _ = cache.load_font_file(family, 700, FontStyleKind::Normal, path);
            }
        }

        cache
    }

    /// Create a font cache pre-loaded with standard OS Arabic and Latin fonts
    pub fn with_system_fonts() -> Self {
        let mut cache = FontCache::new("Noto Sans Arabic");

        let candidates = [
            // Windows
            (
                "Tahoma",
                400,
                FontStyleKind::Normal,
                "C:\\Windows\\Fonts\\tahoma.ttf",
            ),
            (
                "Tahoma",
                700,
                FontStyleKind::Normal,
                "C:\\Windows\\Fonts\\tahomabd.ttf",
            ),
            (
                "Arial",
                400,
                FontStyleKind::Normal,
                "C:\\Windows\\Fonts\\arial.ttf",
            ),
            (
                "Arial",
                700,
                FontStyleKind::Normal,
                "C:\\Windows\\Fonts\\arialbd.ttf",
            ),
            (
                "Segoe UI",
                400,
                FontStyleKind::Normal,
                "C:\\Windows\\Fonts\\segoeui.ttf",
            ),
            (
                "Segoe UI",
                700,
                FontStyleKind::Normal,
                "C:\\Windows\\Fonts\\segoeuib.ttf",
            ),
            // Linux
            (
                "DejaVu Sans",
                400,
                FontStyleKind::Normal,
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            ),
            (
                "Noto Sans Arabic",
                400,
                FontStyleKind::Normal,
                "/usr/share/fonts/truetype/noto/NotoSansArabic-Regular.ttf",
            ),
            // macOS
            (
                "Arial",
                400,
                FontStyleKind::Normal,
                "/System/Library/Fonts/Supplemental/Arial.ttf",
            ),
            (
                "Geeza Pro",
                400,
                FontStyleKind::Normal,
                "/System/Library/Fonts/GeezaPro.ttc",
            ),
        ];

        let mut loaded_any = false;
        for (family, weight, style, path) in candidates {
            if Path::new(path).exists() {
                if let Ok(()) = cache.load_font_file(family, weight, style, path) {
                    loaded_any = true;
                    // Also alias under Noto Sans Arabic if not yet loaded
                    let alias_key = FontKey {
                        family: "Noto Sans Arabic".to_string(),
                        weight,
                        style,
                    };
                    if !cache.fonts.contains_key(&alias_key) {
                        if let Ok(data) = std::fs::read(path) {
                            cache.load_font_bytes(
                                "Noto Sans Arabic",
                                weight,
                                style,
                                data,
                                PathBuf::from(path),
                            );
                        }
                    }
                }
            }
        }

        if !loaded_any {
            tracing::warn!("FontCache: No system fonts could be found at standard paths");
        }

        cache
    }

    /// Add a directory to search for fonts
    pub fn add_search_dir(&mut self, dir: impl AsRef<Path>) {
        self.search_dirs.push(dir.as_ref().to_path_buf());
    }

    /// Load a font file directly from bytes
    pub fn load_font_bytes(
        &mut self,
        family: &str,
        weight: u16,
        style: FontStyleKind,
        data: Vec<u8>,
        path: PathBuf,
    ) {
        let key = FontKey {
            family: family.to_string(),
            weight,
            style,
        };
        let cached = CachedFont {
            family: family.to_string(),
            weight,
            style,
            data: Arc::new(data),
            path,
        };
        self.fonts.insert(key, cached);
    }

    /// Load a font file from disk
    pub fn load_font_file(
        &mut self,
        family: &str,
        weight: u16,
        style: FontStyleKind,
        path: impl AsRef<Path>,
    ) -> Result<(), DocforgeError> {
        let path = path.as_ref();
        let data = std::fs::read(path).map_err(|e| {
            DocforgeError::Render(format!(
                "Failed to load font '{}' from {}: {}",
                family,
                path.display(),
                e
            ))
        })?;

        self.load_font_bytes(family, weight, style, data, path.to_path_buf());
        tracing::info!(
            "Loaded font: {} (weight: {}, path: {})",
            family,
            weight,
            path.display()
        );
        Ok(())
    }

    /// Get font bytes for a given family/weight/style
    pub fn get_font(&self, family: &str, weight: u16, style: FontStyleKind) -> Option<&CachedFont> {
        // Exact match first
        let key = FontKey {
            family: family.to_string(),
            weight,
            style,
        };
        if let Some(font) = self.fonts.get(&key) {
            return Some(font);
        }

        // Fallback: same family, Normal weight
        let fallback_key = FontKey {
            family: family.to_string(),
            weight: 400,
            style: FontStyleKind::Normal,
        };
        if let Some(font) = self.fonts.get(&fallback_key) {
            return Some(font);
        }

        // Fallback: default family
        let default_key = FontKey {
            family: self.default_family.clone(),
            weight: 400,
            style: FontStyleKind::Normal,
        };
        self.fonts
            .get(&default_key)
            .or_else(|| self.fonts.values().next())
    }

    /// Get the default font, panics if no fonts loaded
    pub fn default_font(&self) -> &CachedFont {
        let key = FontKey {
            family: self.default_family.clone(),
            weight: 400,
            style: FontStyleKind::Normal,
        };
        self.fonts
            .get(&key)
            .or_else(|| self.fonts.values().next())
            .expect("FontCache has no fonts loaded")
    }

    /// Get all loaded font families
    pub fn families(&self) -> Vec<String> {
        let mut families: Vec<String> = self
            .fonts
            .keys()
            .map(|k| k.family.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        families.sort();
        families
    }

    /// Get the raw font bytes for embedding in PDF
    pub fn get_raw_bytes(
        &self,
        family: &str,
        weight: u16,
        style: FontStyleKind,
    ) -> Option<Arc<Vec<u8>>> {
        self.get_font(family, weight, style)
            .map(|f| Arc::clone(&f.data))
    }

    /// Number of fonts loaded
    pub fn len(&self) -> usize {
        self.fonts.len()
    }

    /// Whether the cache is empty
    pub fn is_empty(&self) -> bool {
        self.fonts.is_empty()
    }
}

/// Create a shaping system containing only the font that will be embedded in the PDF.
/// The returned family is read from the font itself so shaping cannot select a
/// different system-installed font with the same requested template family.
pub fn create_font_system_for_font(cached_font: &CachedFont) -> (cosmic_text::FontSystem, String) {
    let mut font_db = cosmic_text::fontdb::Database::new();
    font_db.load_font_data(cached_font.data.as_ref().clone());
    let font_system = cosmic_text::FontSystem::new_with_locale_and_db("en-US".to_string(), font_db);
    let family = font_system
        .db()
        .faces()
        .next()
        .and_then(|face| face.families.first())
        .map(|(family, _)| family.clone())
        .unwrap_or_else(|| cached_font.family.clone());

    (font_system, family)
}
