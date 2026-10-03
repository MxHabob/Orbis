// ═══════════════════════════════════════════════════════════════
// docforge-engine — Render Module
//
// Rust-native PDF rendering pipeline:
// 1. font_cache:     Font loading + caching (cosmic-text FontSystem)
// 2. pdf_renderer:   TemplateDocument → PDF bytes (krilla + rustybuzz)
// 3. barcode_gen:    Barcode/QR → SVG strings
// 4. image_loader:   PNG/JPEG decode
// 5. types:          PositionedElement, LayoutResult
// ═══════════════════════════════════════════════════════════════

pub mod barcode_gen;
pub mod font_cache;
pub mod image_loader;
pub mod pdf_renderer;
pub mod types;
