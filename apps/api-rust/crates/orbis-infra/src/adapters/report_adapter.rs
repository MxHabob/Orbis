use async_trait::async_trait;
use orbis_app::ports::gateways::ReportEnginePort;
use orbis_app::context::RequestContext;
use orbis_domain::errors::DomainError;
use tera::{Tera, Context};

pub struct TeraReportAdapter {
    tera: Tera,
}

impl TeraReportAdapter {
    pub fn new() -> Self {
        Self {
            tera: Tera::default(),
        }
    }
}

impl Default for TeraReportAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ReportEnginePort for TeraReportAdapter {
    async fn generate_pdf(&self, _ctx: &RequestContext, layout_body: Option<&str>, template_body: &str, data: &serde_json::Value) -> Result<Vec<u8>, DomainError> {
        let mut context = Context::new();
        context.insert("data", data);
        
        let mut tera = self.tera.clone();
        
        if let Some(layout) = layout_body {
            tera.add_raw_template("layout", layout)
                .map_err(|e| DomainError::Internal(format!("Failed to load layout template: {}", e)))?;
        }
        
        tera.add_raw_template("report", template_body)
            .map_err(|e| DomainError::Internal(format!("Failed to load report template: {}", e)))?;

        let rendered_html = tera.render("report", &context)
            .map_err(|e| DomainError::Internal(format!("Failed to render report template: {}", e)))?;
            
        println!("Generated HTML for PDF (len: {})", rendered_html.len());
        Ok(rendered_html.into_bytes())
    }

    async fn generate_excel(&self, _ctx: &RequestContext, _layout_body: Option<&str>, _template_body: &str, _data: &serde_json::Value) -> Result<Vec<u8>, DomainError> {
        Ok(vec![])
    }
}
