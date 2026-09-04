use async_trait::async_trait;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;

#[async_trait]
pub trait LlmGatewayPort: Send + Sync {
    async fn generate_text(&self, ctx: &RequestContext, model: &str, system_prompt: &str, user_prompt: &str, temperature: f64) -> Result<String, DomainError>;
}



#[async_trait]
pub trait PaymentGatewayPort: Send + Sync {
    async fn process_payment(&self, ctx: &RequestContext, amount: f64, currency: &str, source: &str) -> Result<String, DomainError>;
}

#[async_trait]
pub trait SurveillancePort: Send + Sync {
    async fn detect_face(&self, ctx: &RequestContext, image_data: &[u8]) -> Result<Option<String>, DomainError>;
}

#[async_trait]
pub trait NotificationPort: Send + Sync {
    async fn send_email(&self, ctx: &RequestContext, to: &str, subject: &str, body: &str) -> Result<(), DomainError>;
    async fn send_whatsapp(&self, ctx: &RequestContext, to: &str, message: &str) -> Result<(), DomainError>;
}
