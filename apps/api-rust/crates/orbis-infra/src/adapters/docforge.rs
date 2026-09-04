use async_trait::async_trait;
use orbis_app::ports::gateways::ReportEnginePort;
use orbis_domain::errors::DomainError;

pub struct DocforgeAdapter;

impl DocforgeAdapter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ReportEnginePort for DocforgeAdapter {
    async fn generate_pdf(&self, _ctx: &orbis_app::context::RequestContext, template_body: &str, _data: &serde_json::Value) -> Result<Vec<u8>, DomainError> {
        println!("Calling DocForge API for PDF with template length {}", template_body.len());
        Ok(vec![1, 2, 3, 4]) // Mock PDF bytes
    }

    async fn generate_excel(&self, _ctx: &orbis_app::context::RequestContext, template_body: &str, _data: &serde_json::Value) -> Result<Vec<u8>, DomainError> {
        println!("Calling DocForge API for EXCEL with template length {}", template_body.len());
        Ok(vec![5, 6, 7, 8]) // Mock Excel bytes
    }
}
