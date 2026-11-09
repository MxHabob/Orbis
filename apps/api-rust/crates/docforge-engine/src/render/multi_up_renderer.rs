use crate::{
    error::DocforgeError,
    layout::{multi_up::MultiUpConfig, LayoutEngine},
    model::document::TemplateDocument,
    render::{
        font_cache::FontCache,
        pdf_renderer::{render_to_pdf, PdfOutput, PdfRenderConfig},
    },
};
use serde_json::Value;

/// Render one DocForge template for each data context and arrange the results on sheets.
pub fn render_multi_up(
    document_json: &str,
    contexts: &[Value],
    config: &MultiUpConfig,
    title: Option<String>,
) -> Result<PdfOutput, DocforgeError> {
    if contexts.is_empty() {
        return Err(DocforgeError::Render(
            "At least one data context is required for batch printing".into(),
        ));
    }

    let document: TemplateDocument = serde_json::from_str(document_json)
        .map_err(|error| DocforgeError::Render(format!("Invalid template JSON: {error}")))?;
    let fonts = FontCache::with_system_fonts();
    let layout = LayoutEngine::new(&fonts).layout_multi_up(&document, contexts, config)?;
    render_to_pdf(
        &layout,
        &fonts,
        &PdfRenderConfig {
            title,
            ..Default::default()
        },
    )
}
