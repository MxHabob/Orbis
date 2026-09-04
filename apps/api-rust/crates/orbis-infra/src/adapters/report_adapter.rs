use async_trait::async_trait;
use orbis_app::ports::gateways::ReportEnginePort;
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
    async fn generate_pdf(&self, _ctx: &orbis_app::context::RequestContext, template_body: &str, data: &serde_json::Value) -> Result<Vec<u8>, DomainError> {
        let mut context = Context::new();
        context.insert("data", data);
        
        let mut tera = self.tera.clone();
        tera.add_raw_template("report", template_body)
            .map_err(|e| DomainError::Validation(format!("Template compilation error: {}", e)))?;
            
        let rendered_html = tera.render("report", &context)
            .map_err(|e| DomainError::Validation(format!("Template render error: {}", e)))?;
            
        println!("Generated HTML for PDF (len: {})", rendered_html.len());
        Ok(rendered_html.into_bytes())
    }

    async fn generate_excel(&self, _ctx: &orbis_app::context::RequestContext, _template_body: &str, _data: &serde_json::Value) -> Result<Vec<u8>, DomainError> {
        Ok(vec![])
    }
}
