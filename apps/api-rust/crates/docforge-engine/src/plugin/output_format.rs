// ═══════════════════════════════════════════════════════════════
// docforge-engine — OutputFormat Trait
// Allows registering custom output exporters (e.g., DOCX, HTML, SVG)
// ═══════════════════════════════════════════════════════════════

use async_trait::async_trait;
use crate::error::DocforgeError;
use crate::model::TemplateDocument;
use crate::plugin::data_source::DataContext;

#[async_trait]
pub trait OutputFormat: Send + Sync {
    /// Unique name of the output format (e.g., "html", "docx")
    fn name(&self) -> &str;

    /// File extension associated with this format (e.g., "html", "docx")
    fn extension(&self) -> &str;

    /// MIME type associated with this format
    fn mime_type(&self) -> &str;

    /// Render the template document to raw bytes for this output format
    async fn render(
        &self,
        document: &TemplateDocument,
        data: &DataContext,
    ) -> Result<Vec<u8>, DocforgeError>;
}
