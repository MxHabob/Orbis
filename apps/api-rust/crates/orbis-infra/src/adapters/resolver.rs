use async_trait::async_trait;
use orbis_app::ports::gateways::{LlmGatewayPort, ReportEnginePort, PaymentGatewayPort, SurveillancePort};
use orbis_app::ports::repositories::IntegrationRepository;
use orbis_domain::errors::DomainError;
use orbis_app::context::RequestContext;
use std::sync::Arc;

use crate::adapters::{gemini::GeminiAdapter, mada::MadaAdapter, moyasar::MoyasarAdapter, hikvision::HikvisionAdapter, authen::AuthenAdapter, docforge::DocforgeAdapter};

pub struct IntegrationResolver {
    pub integration_repo: Arc<dyn IntegrationRepository>,
}

impl IntegrationResolver {
    pub fn new(integration_repo: Arc<dyn IntegrationRepository>) -> Self {
        Self { integration_repo }
    }
}

use crate::adapters::orbis_ai::OrbisAiAdapter;

#[async_trait]
impl LlmGatewayPort for IntegrationResolver {
    async fn generate_text(&self, ctx: &RequestContext, model: &str, system_prompt: &str, user_prompt: &str, temperature: f64) -> Result<String, DomainError> {
        let configs = self.integration_repo.find_configs_by_org(ctx.tenant_id).await?;
        let active_ai = configs.into_iter().find(|c| c.category == "ai" && c.is_enabled);
        
        match active_ai {
            Some(config) => {
                let secrets = self.integration_repo.get_secrets_by_config(config.id).await?;
                
                if config.provider_name == "gemini" {
                    let api_key = secrets.into_iter().find(|s| s.key_name == "api_key").map(|s| s.secret_key).unwrap_or_default();
                    let adapter = GeminiAdapter::new(api_key);
                    adapter.generate_text(ctx, model, system_prompt, user_prompt, temperature).await
                } else if config.provider_name == "orbis" {
                    let endpoint_url = config.settings.as_ref().and_then(|s| s.get("endpoint_url")).and_then(|v| v.as_str()).unwrap_or("http://localhost:11434").to_string();
                    let adapter = OrbisAiAdapter::new(endpoint_url);
                    adapter.generate_text(ctx, model, system_prompt, user_prompt, temperature).await
                } else {
                    Err(DomainError::Validation(format!("Unsupported AI Provider: {}", config.provider_name)))
                }
            },
            None => Err(DomainError::Validation("No active AI Provider integration found".into())),
        }
    }
}

#[async_trait]
impl PaymentGatewayPort for IntegrationResolver {
    async fn process_payment(&self, ctx: &RequestContext, amount: f64, currency: &str, source: &str) -> Result<String, DomainError> {
        let configs = self.integration_repo.find_configs_by_org(ctx.tenant_id).await?;
        let active_payment = configs.into_iter().find(|c| c.category == "payment-gateway" && c.is_enabled);
        
        match active_payment {
            Some(config) => {
                let secrets = self.integration_repo.get_secrets_by_config(config.id).await?;
                let secret_key = secrets.into_iter().find(|s| s.key_name == "secret_key").map(|s| s.secret_key).unwrap_or_default();
                
                if config.provider_name == "mada" {
                    let adapter = MadaAdapter::new(secret_key);
                    adapter.process_payment(ctx, amount, currency, source).await
                } else if config.provider_name == "moyasar" {
                    let adapter = MoyasarAdapter::new(secret_key);
                    adapter.process_payment(ctx, amount, currency, source).await
                } else {
                    Err(DomainError::Validation(format!("Unsupported Payment Provider: {}", config.provider_name)))
                }
            },
            None => Err(DomainError::Validation("No active Payment Gateway integration found".into())),
        }
    }
}


#[async_trait]
impl SurveillancePort for IntegrationResolver {
    async fn detect_face(&self, ctx: &RequestContext, image_data: &[u8]) -> Result<Option<String>, DomainError> {
        let configs = self.integration_repo.find_configs_by_org(ctx.tenant_id).await?;
        let active_camera = configs.into_iter().find(|c| c.category == "surveillance" && c.is_enabled);
        
        match active_camera {
            Some(config) => {
                let secrets = self.integration_repo.get_secrets_by_config(config.id).await?;
                let api_key = secrets.into_iter().find(|s| s.key_name == "api_key").map(|s| s.secret_key).unwrap_or_default();
                
                if config.provider_name == "hikvision" {
                    let api_url = config.settings.as_ref().and_then(|s| s.get("api_url")).and_then(|v| v.as_str()).unwrap_or_default().to_string();
                    let adapter = HikvisionAdapter::new(api_url, api_key);
                    adapter.detect_face(ctx, image_data).await
                } else if config.provider_name == "authen" {
                    let api_url = config.settings.as_ref().and_then(|s| s.get("api_url")).and_then(|v| v.as_str()).unwrap_or_default().to_string();
                    let adapter = AuthenAdapter::new(api_url, api_key);
                    adapter.detect_face(ctx, image_data).await
                } else {
                    Err(DomainError::Validation(format!("Unsupported Surveillance Provider: {}", config.provider_name)))
                }
            },
            None => Err(DomainError::Validation("No active Surveillance integration found".into())),
        }
    }
}

#[async_trait]
impl ReportEnginePort for IntegrationResolver {
    async fn generate_pdf(&self, ctx: &RequestContext, template_body: &str, data: &serde_json::Value) -> Result<Vec<u8>, DomainError> {
        // Here we could also dynamically resolve, but for now we just wrap Docforge
        let adapter = DocforgeAdapter {};
        adapter.generate_pdf(ctx, template_body, data).await
    }

    async fn generate_excel(&self, ctx: &RequestContext, template_body: &str, data: &serde_json::Value) -> Result<Vec<u8>, DomainError> {
        let adapter = DocforgeAdapter {};
        adapter.generate_excel(ctx, template_body, data).await
    }
}

use orbis_app::ports::gateways::NotificationPort;
use crate::adapters::{email::EmailAdapter, whatsapp::WhatsAppAdapter};

#[async_trait]
impl NotificationPort for IntegrationResolver {
    async fn send_email(&self, ctx: &RequestContext, to: &str, subject: &str, body: &str) -> Result<(), DomainError> {
        let configs = self.integration_repo.find_configs_by_org(ctx.tenant_id).await?;
        let active_email = configs.into_iter().find(|c| c.category == "email" && c.is_enabled);
        
        match active_email {
            Some(config) => {
                let secrets = self.integration_repo.get_secrets_by_config(config.id).await?;
                let api_key = secrets.into_iter().find(|s| s.key_name == "api_key").map(|s| s.secret_key).unwrap_or_default();
                
                if config.provider_name == "sendgrid" {
                    let adapter = EmailAdapter::new(api_key);
                    adapter.send_email(ctx, to, subject, body).await
                } else {
                    Err(DomainError::Validation(format!("Unsupported Email Provider: {}", config.provider_name)))
                }
            },
            None => Err(DomainError::Validation("No active Email integration found".into())),
        }
    }

    async fn send_whatsapp(&self, ctx: &RequestContext, to: &str, message: &str) -> Result<(), DomainError> {
        let configs = self.integration_repo.find_configs_by_org(ctx.tenant_id).await?;
        let active_wa = configs.into_iter().find(|c| c.category == "whatsapp" && c.is_enabled);
        
        match active_wa {
            Some(config) => {
                let secrets = self.integration_repo.get_secrets_by_config(config.id).await?;
                let sid = secrets.iter().find(|s| s.key_name == "account_sid").map(|s| s.secret_key.clone()).unwrap_or_default();
                let token = secrets.iter().find(|s| s.key_name == "auth_token").map(|s| s.secret_key.clone()).unwrap_or_default();
                
                let from_num = config.settings.as_ref().and_then(|s| s.get("from_number")).and_then(|v| v.as_str()).unwrap_or_default().to_string();

                if config.provider_name == "twilio" {
                    let adapter = WhatsAppAdapter::new(sid, token, from_num);
                    adapter.send_whatsapp(ctx, to, message).await
                } else {
                    Err(DomainError::Validation(format!("Unsupported WhatsApp Provider: {}", config.provider_name)))
                }
            },
            None => Err(DomainError::Validation("No active WhatsApp integration found".into())),
        }
    }
}
